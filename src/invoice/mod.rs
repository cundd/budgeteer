use std::fmt;
use invoice::invoice_type::InvoiceType;
use invoice::amount::Amount;
use chrono::prelude::*;

pub mod invoice_type;
pub mod invoice_parser;
pub mod amount;

#[derive(Clone)]
pub struct Invoice {
    date: NaiveDate,
    amount: Amount,
    base_amount: Option<Amount>,
    invoice_type: InvoiceType,
    note: Option<String>,
}

impl Invoice {
    pub fn new(date: NaiveDate, amount: Amount, base_amount: Option<Amount>, invoice_type: InvoiceType, note: Option<String>) -> Self {
        Invoice {
            date,
            amount,
            base_amount,
            invoice_type,
            note,
        }
    }
    pub fn date(&self) -> NaiveDate {
        self.date
    }
    pub fn amount(&self) -> Amount {
        self.amount.clone()
    }
    pub fn amount_ref(&self) -> &Amount {
        &self.amount
    }
    pub fn base_amount(&self) -> Option<Amount> {
        self.base_amount.clone()
    }
    pub fn invoice_type(&self) -> InvoiceType {
        self.invoice_type
    }
    pub fn note(&self) -> Option<String> {
        self.note.clone()
    }

    pub fn with_base_amount(&self, base_amount: Amount) -> Invoice {
        let mut clone = self.clone();

        clone.base_amount = Some(base_amount);

        clone
    }
}

impl fmt::Display for Invoice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let note = match self.note {
            Some(ref c) => c.to_owned(),
            None => "".to_owned()
        };
        write!(
            f,
            "\
Datum:     {}
Betrag:    {}
Typ:       {}
Notiz:     {}\
",
            self.date,
            self.amount,
            self.invoice_type,
            note
        )
    }
}
