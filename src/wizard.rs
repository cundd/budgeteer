use dialoguer::Input;
use error::Res;
use chrono::{NaiveDate, Local};
use currency::Currency;
use invoice::invoice_type::InvoiceType;
use invoice::Invoice;
use invoice::amount::Amount;

pub struct Wizard {}

impl Wizard {
    pub fn run(&self) -> Res<Invoice> {
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
            Err(_) => self.read_currency()
        }
    }
    fn read_amount(&self) -> Res<f64> {
        Ok(Input::<f64>::new()
            .with_prompt("Amount")
            .interact()?)
    }
    fn read_invoice_type(&self) -> Res<InvoiceType> {
        Ok(Input::<InvoiceType>::new()
            .with_prompt("Type")
            .interact()?)
    }
    fn read_note(&self) -> Res<String> {
        Ok(Input::<String>::new()
            .with_prompt("Note")
            .allow_empty(true)
            .interact()?)
    }
}