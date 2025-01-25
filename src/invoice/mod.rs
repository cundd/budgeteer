use crate::currency::Currency;
use crate::invoice::amount::Amount;
use crate::invoice::invoice_type::InvoiceType;
use chrono::prelude::*;
use main_transaction_data::MainTransactionData;
use sqlx::prelude::FromRow;
use sqlx::sqlite::SqliteRow;
use sqlx::Row;
use std::cmp::Ordering;
use std::fmt;

pub mod amount;
pub mod invoice_type;
pub mod main_transaction_data;

#[derive(Clone, Debug, PartialEq)]
pub struct Invoice {
    pub date: NaiveDate,
    pub amount: Amount,
    pub base_amount: Option<Amount>,
    pub invoice_type: InvoiceType,
    pub note: Option<String>,
}

impl Invoice {
    pub fn new(
        date: NaiveDate,
        amount: Amount,
        base_amount: Option<Amount>,
        invoice_type: InvoiceType,
        note: Option<String>,
    ) -> Self {
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
            None => "".to_owned(),
        };
        write!(
            f,
            "\
Datum:     {}
Betrag:    {}
Typ:       {}
Notiz:     {}\
",
            self.date, self.amount, self.invoice_type, note
        )
    }
}

impl PartialOrd for Invoice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let a_base_amount = self.base_amount();
        let b_base_amount = other.base_amount();
        match a_base_amount {
            Some(a_amount) => match b_base_amount {
                Some(b_amount) => a_amount.partial_cmp(&b_amount),
                None => Some(Ordering::Greater),
            },
            None => {
                if b_base_amount.is_some() {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Equal)
                }
            }
        }
    }
}

impl MainTransactionData for Invoice {
    fn amount(&self) -> &Amount {
        &self.amount
    }

    fn date(&self) -> NaiveDate {
        self.date
    }
}

impl MainTransactionData for &Invoice {
    fn amount(&self) -> &Amount {
        &self.amount
    }

    fn date(&self) -> NaiveDate {
        self.date
    }
}

pub fn contains_invoice_in_currency(invoices: &[Invoice], currency: &Currency) -> bool {
    invoices
        .iter()
        .any(|invoice| invoice.amount_ref().currency_ref() == currency)
}

impl FromRow<'_, SqliteRow> for Invoice {
    fn from_row(row: &SqliteRow) -> sqlx::Result<Self> {
        let currency: Currency = row.try_get("currency")?;
        let amount = Amount::new(row.try_get("amount")?, currency);

        Ok(Self {
            date: row.try_get("date")?,
            amount,
            base_amount: None,
            invoice_type: row.try_get("type")?,
            note: row.try_get("note")?,
        })
    }
}
