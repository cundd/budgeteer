use crate::currency::Currency;
use crate::transaction::amount::Amount;
use crate::transaction::transaction_type::TransactionType;
use chrono::prelude::*;
use main_transaction_data::MainTransactionData;
use sqlx::prelude::FromRow;
use sqlx::sqlite::SqliteRow;
use sqlx::Row;
use std::cmp::Ordering;
use std::fmt;

pub mod amount;
pub mod main_transaction_data;
pub mod transaction_type;

#[derive(Clone, Debug, PartialEq)]
pub struct Transaction {
    pub date: NaiveDate,
    pub amount: Amount,
    pub base_amount: Option<Amount>,
    pub transaction_type: TransactionType,
    pub note: Option<String>,
}

impl Transaction {
    pub fn new(
        date: NaiveDate,
        amount: Amount,
        base_amount: Option<Amount>,
        transaction_type: TransactionType,
        note: Option<String>,
    ) -> Self {
        Transaction {
            date,
            amount,
            base_amount,
            transaction_type,
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

    pub fn transaction_type(&self) -> TransactionType {
        self.transaction_type
    }

    pub fn note(&self) -> Option<String> {
        self.note.clone()
    }

    pub fn with_base_amount(&self, base_amount: Amount) -> Transaction {
        let mut clone = self.clone();

        clone.base_amount = Some(base_amount);

        clone
    }
}

impl fmt::Display for Transaction {
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
            self.date, self.amount, self.transaction_type, note
        )
    }
}

impl PartialOrd for Transaction {
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

impl MainTransactionData for Transaction {
    fn amount(&self) -> &Amount {
        &self.amount
    }

    fn date(&self) -> NaiveDate {
        self.date
    }
}

impl MainTransactionData for &Transaction {
    fn amount(&self) -> &Amount {
        &self.amount
    }

    fn date(&self) -> NaiveDate {
        self.date
    }
}

pub fn contains_transaction_in_currency(transactions: &[Transaction], currency: &Currency) -> bool {
    transactions
        .iter()
        .any(|transaction| transaction.amount_ref().currency_ref() == currency)
}

impl FromRow<'_, SqliteRow> for Transaction {
    fn from_row(row: &SqliteRow) -> sqlx::Result<Self> {
        let currency: Currency = row.try_get("currency")?;
        let amount = Amount::new(row.try_get("amount")?, currency);

        Ok(Self {
            date: row.try_get("date")?,
            amount,
            base_amount: None,
            transaction_type: row.try_get("type")?,
            note: row.try_get("note")?,
        })
    }
}
