use dialoguer::{Input, Select, Confirmation};
use error::Res;
use chrono::{NaiveDate, Local};
use currency::Currency;
use invoice::invoice_type::InvoiceType;
use invoice::Invoice;
use invoice::amount::Amount;
use dialoguer::theme::ColorfulTheme;
use printer::{Printer, PrinterTrait};
use file::FileWriter;
use std::path::Path;

pub struct Wizard {}

impl Wizard {
    pub fn run<P: AsRef<Path>>(&self, printer: &Printer, base_currency: &Currency, output_file: P) -> Res<()> {
        println!("Welcome to the invoice wizard");

        println!("Answer the following questions to insert a new invoice");
        println!("(Press ctrl+c to exit)");

        self.run_inner(printer, base_currency, output_file)
    }
    fn run_inner<P: AsRef<Path>>(&self, printer: &Printer, base_currency: &Currency, output_file: P) -> Res<()> {
        let invoice = self.create_invoice()?;

        println!();
        println!("Read the following invoice:");
        printer.print_invoice(&base_currency, &invoice);

        if Confirmation::new().with_text("Save this invoice?").interact()? {
            FileWriter::write_invoice(&output_file, &invoice)?;
            println!("Saved the new invoice");

            if Confirmation::new().with_text("Do you want to insert another invoice?").interact()? {
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
        let raw_date = Input::<String>::new()
            .with_prompt("Date (dd.mm.yyyy)")
            .default(Local::now().format("%d.%m.%Y").to_string())
            .interact()?;

        match NaiveDate::parse_from_str(&raw_date, "%d.%m.%Y") {
            Ok(d) => Ok(d),
            Err(_) => self.read_date()
        }
    }

    fn read_currency(&self) -> Res<Currency> {
        let raw_currency = Input::<String>::new()
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
        let raw_amount = Input::<String>::new()
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
        let all = InvoiceType::all();
        let i = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Type")
            .default(0)
            .items(&all[..])
            .interact()?;

        Ok(all[i])

//        Ok(Input::<InvoiceType>::new()
//            .with_prompt("Type")
//            .interact()?)
    }
    fn read_note(&self) -> Res<String> {
        Ok(Input::<String>::new()
            .with_prompt("Note")
            .allow_empty(true)
            .interact()?)
    }
}