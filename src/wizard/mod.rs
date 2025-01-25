mod amount;
mod currency;
mod date;
mod invoice_type;
mod note;

use self::amount::read_amount;
use self::currency::read_currency;
use self::date::read_date;
use self::invoice_type::read_invoice_type;
use self::invoice_type::read_invoice_type_or_skip;
use self::note::NoteWizard;
use crate::currency::Currency;
use crate::error::Res;
use crate::invoice::amount::Amount;
use crate::invoice::invoice_type::InvoiceType;
use crate::invoice::Invoice;
use crate::persistence::InvoiceRepository;
use crate::printer::{Printer, PrinterTrait};
use chrono::NaiveDate;
use dialoguer::console::Style;
use dialoguer::theme::{ColorfulTheme, Theme};
use dialoguer::Confirm;

// Trait for "sub"-wizards
trait WizardTrait<T> {
    fn read(&self, theme: &dyn Theme, invoices: &[Invoice]) -> Res<T>;
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

    pub async fn run(
        &self,
        printer: &mut Printer,
        base_currency: &Currency,
        repository: &InvoiceRepository,
        invoices: &[Invoice],
    ) -> Res<()> {
        println!("Welcome to the invoice wizard");

        println!("Answer the following questions to insert a new invoice");
        println!("(Press ctrl+c to exit)");

        loop {
            let invoice = self.create_invoice(invoices)?;

            println!();
            println!("Read the following invoice:");
            printer.print_invoice(base_currency, &invoice);

            let confirm = Confirm::with_theme(self.theme.as_ref());
            if confirm
                .clone()
                .with_prompt("Save this invoice?")
                .default(true)
                .interact()?
            {
                match repository.add(&invoice).await {
                    Ok(id) => println!("Saved the new invoice #{}", id),
                    Err(_) => eprintln!("Could not store the invoice"),
                }

                if !confirm
                    .with_prompt("Do you want to insert another invoice?")
                    .default(true)
                    .interact()?
                {
                    return Ok(());
                }
            } else {
                println!();
                println!("Build another invoice instead");
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

    pub fn read_invoice_type(&self, allow_unknown: bool) -> Res<InvoiceType> {
        read_invoice_type(self.theme.as_ref(), allow_unknown)
    }

    pub fn read_invoice_type_or_skip(&self, allow_unknown: bool) -> Res<Option<InvoiceType>> {
        read_invoice_type_or_skip(self.theme.as_ref(), allow_unknown)
    }

    fn create_invoice(&self, invoices: &[Invoice]) -> Res<Invoice> {
        let theme = self.theme.as_ref();
        let date = self.read_date()?;
        let currency = self.read_currency()?;
        let amount = self.read_amount()?;
        let invoice_type = self.read_invoice_type(false)?;
        let note = self.note_wizard.read(theme, invoices)?;

        Ok(Invoice::new(
            date,
            Amount::new(amount, currency),
            None,
            invoice_type,
            Some(note),
        ))
    }
}
