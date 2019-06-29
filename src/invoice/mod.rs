use std::fmt;
use invoice::invoice_type::InvoiceType;
use invoice::amount::Amount;
use chrono::prelude::*;

pub mod invoice_type;
pub mod invoice_parser;
pub mod amount;

#[derive(Clone)]
pub struct Invoice {
    pub date: NaiveDate,
    pub amount: Amount,
    pub base_amount: Option<Amount>,
    pub invoice_type: InvoiceType,
    pub comment: Option<String>,
}

impl fmt::Display for Invoice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let comment = match self.comment {
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
            comment
        )
    }
}
