mod amount;
mod currency;
mod date;
mod note;
mod transaction_type;

use self::amount::read_amount;
use self::currency::read_currency;
use self::date::read_date;
use self::note::NoteWizard;
use self::transaction_type::read_transaction_type;
use self::transaction_type::read_transaction_type_or_skip;
use crate::currency::Currency;
use crate::error::Res;
use crate::persistence::TransactionRepository;
use crate::printer::PrinterTrait;
use crate::transaction::amount::Amount;
use crate::transaction::transaction_type::TransactionType;
use crate::transaction::Transaction;
use chrono::NaiveDate;
use dialoguer::console::Style;
use dialoguer::theme::{ColorfulTheme, Theme};
use dialoguer::Confirm;

// Trait for "sub"-wizards
trait WizardTrait<T> {
    fn read(&self, theme: &dyn Theme, transactions: &[Transaction]) -> Res<T>;
}

pub struct Wizard {
    theme: Box<dyn Theme>,
    note_wizard: NoteWizard,
}

impl Wizard {
    pub fn new() -> Wizard {
        // let mut theme = ColorfulTheme::default();
        // theme.defaults_style = Style::new().blink();
        let theme = ColorfulTheme {
            defaults_style: Style::new().blink(),
            ..Default::default()
        };

        Wizard {
            theme: Box::new(theme),
            note_wizard: NoteWizard::default(),
        }
    }

    pub async fn run<P: PrinterTrait>(
        &self,
        printer: &mut P,
        base_currency: &Currency,
        repository: &TransactionRepository,
        transactions: &[Transaction],
    ) -> Res<()> {
        println!("Welcome to the transaction wizard");

        println!("Answer the following questions to insert a new transaction");
        println!("(Press ctrl+c to exit)");

        loop {
            let transaction = self.create_transaction(transactions)?;

            println!();
            println!("Read the following transaction:");
            printer.print_transaction(base_currency, &transaction);

            let confirm = Confirm::with_theme(self.theme.as_ref());
            if confirm
                .clone()
                .with_prompt("Save this transaction?")
                .default(true)
                .interact()?
            {
                match repository.add(&transaction).await {
                    Ok(id) => println!("Saved the new transaction #{}", id),
                    Err(_) => eprintln!("Could not store the transaction"),
                }

                if !confirm
                    .with_prompt("Do you want to insert another transaction?")
                    .default(true)
                    .interact()?
                {
                    return Ok(());
                }
            } else {
                println!();
                println!("Build another transaction instead");
            }
        }
    }

    pub fn read_date(&self) -> Res<NaiveDate> {
        read_date(self.theme.as_ref())
    }

    pub fn read_currency(&self) -> Res<Currency> {
        read_currency(self.theme.as_ref())
    }

    pub fn read_amount(&self) -> Res<f64> {
        read_amount(self.theme.as_ref())
    }

    pub fn read_transaction_type(&self, allow_unknown: bool) -> Res<TransactionType> {
        read_transaction_type(self.theme.as_ref(), allow_unknown)
    }

    pub fn read_transaction_type_or_skip(
        &self,
        allow_unknown: bool,
    ) -> Res<Option<TransactionType>> {
        read_transaction_type_or_skip(self.theme.as_ref(), allow_unknown)
    }

    fn create_transaction(&self, transactions: &[Transaction]) -> Res<Transaction> {
        let theme = self.theme.as_ref();
        let date = self.read_date()?;
        let currency = self.read_currency()?;
        let amount = self.read_amount()?;
        let transaction_type = self.read_transaction_type(false)?;
        let note = self.note_wizard.read(theme, transactions)?;

        Ok(Transaction::new(
            date,
            Amount::new(amount, currency),
            None,
            transaction_type,
            Some(note),
        ))
    }
}
