mod date;
use std::path::Path;

use chrono::NaiveDate;
use dialoguer::console::{Style, Term};
use dialoguer::theme::{ColorfulTheme, Theme};
use dialoguer::{Confirm, Input, Select};

use crate::currency::Currency;
use crate::error::Res;
use crate::file::FileWriter;
use crate::invoice::amount::Amount;
use crate::invoice::invoice_type::InvoiceType;
use crate::invoice::Invoice;
use crate::printer::{Printer, PrinterTrait};

use self::date::read_date;

pub struct Wizard {
    theme: Box<dyn Theme>,
    #[allow(dead_code)]
    term: Term,
}

impl Wizard {
    pub fn new() -> Wizard {
        let mut theme = ColorfulTheme::default();
        theme.defaults_style = Style::new().blink();

        Wizard {
            theme: Box::new(theme),
            term: Term::stdout(),
        }
    }

    pub fn run<P: AsRef<Path>>(
        &self,
        printer: &Printer,
        base_currency: &Currency,
        output_file: P,
    ) -> Res<()> {
        println!("Welcome to the invoice wizard");

        println!("Answer the following questions to insert a new invoice");
        println!("(Press ctrl+c to exit)");

        self.run_inner(printer, base_currency, output_file)
    }

    fn run_inner<P: AsRef<Path>>(
        &self,
        printer: &Printer,
        base_currency: &Currency,
        output_file: P,
    ) -> Res<()> {
        let invoice = self.create_invoice()?;

        println!();
        println!("Read the following invoice:");
        printer.print_invoice(&base_currency, &invoice);

        let mut confirm = Confirm::with_theme(self.theme.as_ref());
        if confirm.with_prompt("Save this invoice?").interact()? {
            FileWriter::write_invoice(&output_file, &invoice)?;
            println!("Saved the new invoice");

            if confirm
                .with_prompt("Do you want to insert another invoice?")
                .interact()?
            {
                self.run_inner(printer, base_currency, output_file)
            } else {
                Ok(())
            }
        } else {
            println!();
            println!("Build another invoice instead");

            self.run_inner(printer, base_currency, output_file)
        }
    }

    fn create_invoice(&self) -> Res<Invoice> {
        let date = self.read_date()?;
        let currency = self.read_currency()?;
        let amount = self.read_amount()?;
        let invoice_type = self.read_invoice_type()?;
        let note = self.read_note()?;
        Ok(Invoice::new(
            date,
            Amount::new(amount, &currency),
            None,
            invoice_type,
            Some(note),
        ))
    }

    fn read_date(&self) -> Res<NaiveDate> {
        read_date(self.theme.as_ref())
    }

    fn read_currency(&self) -> Res<Currency> {
        // self.prompt("Currency");
        let raw_currency = Input::<String>::with_theme(self.theme.as_ref())
            .with_prompt("Currency")
            .default("€".to_owned())
            .interact()?
            .to_uppercase();

        match Currency::from_string(&raw_currency) {
            Ok(c) => Ok(c),
            Err(_) => {
                println!("Please enter a valid currency");
                self.read_currency()
            }
        }
    }

    fn read_amount(&self) -> Res<f64> {
        // self.prompt("Amount");
        let raw_amount = Input::<String>::with_theme(self.theme.as_ref())
            .with_prompt("Amount")
            .interact()?;

        let raw_amount_normalized = if raw_amount.contains(',') {
            raw_amount.replace(',', ".")
        } else {
            raw_amount
        };

        match raw_amount_normalized.parse::<f64>() {
            Ok(c) => Ok(c),
            Err(_) => {
                println!("Please enter a valid amount");
                self.read_amount()
            }
        }
    }

    fn read_invoice_type(&self) -> Res<InvoiceType> {
        let all = InvoiceType::all_known();
        // self.prompt("Type");
        let i = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Type")
            .default(0)
            .items(&all[..])
            .interact()?;

        Ok(all[i])
    }

    fn read_note(&self) -> Res<String> {
        // self.prompt("Note");
        Ok(Input::<String>::with_theme(self.theme.as_ref())
            .with_prompt("Note")
            .allow_empty(true)
            .interact()?)
    }

    #[allow(dead_code)]
    fn prompt<'a, S>(&self, prompt: S) -> Res<()>
    where
        S: Into<&'a str>,
    {
        let style = Style::new().yellow();

        Ok(self
            .term
            .write_str(&format!("{}", style.apply_to(prompt.into())))?)
    }
}
