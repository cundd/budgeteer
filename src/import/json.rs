use chrono::{DateTime, FixedOffset};

use super::ImportResult;
use crate::{
    currency::Currency,
    error::{Error, Res},
    invoice::{amount::Amount, invoice_type::InvoiceType, Invoice},
};
use std::{fs::File, io::BufReader, path::Path, str::FromStr};

#[derive(Debug, serde::Deserialize)]
struct TransactionJson {
    amount: f64,
    date: DateTime<FixedOffset>,
    currency: String,
    note: String,
}

impl TransactionJson {
    fn into_transaction<P>(self, mut prepare_transaction: P) -> Res<Option<Invoice>>
    where
        P: FnMut(Invoice) -> Res<Option<Invoice>>,
    {
        let currency = Currency::from_str(&self.currency)?;

        prepare_transaction(Invoice::new(
            self.date.date_naive(),
            Amount::new(self.amount, currency),
            None,
            InvoiceType::Unknown,
            Some(self.note),
        ))
    }
}

pub fn get_transactions<T, P: AsRef<Path>>(
    input_file: P,
    mut prepare_transaction: T,
) -> Result<ImportResult, Error>
where
    T: FnMut(Invoice) -> Res<Option<Invoice>>,
{
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let transaction_json: Vec<TransactionJson> = serde_json::from_reader(reader)?;

    let (transactions, errors) = partition_and_unpack(
        transaction_json
            .into_iter()
            .map(|t| t.into_transaction(&mut prepare_transaction)),
    );

    Ok(ImportResult {
        transactions,
        errors,
    })
}

fn partition_and_unpack<T: std::fmt::Debug, E: std::fmt::Debug>(
    input: impl Iterator<Item = Result<Option<T>, E>>,
) -> (Vec<T>, Vec<E>) {
    let (oks, errors): (Vec<Result<Option<T>, _>>, Vec<Result<_, E>>) =
        input.into_iter().partition(Result::is_ok);

    (
        oks.into_iter().filter_map(Result::unwrap).collect(),
        errors.into_iter().map(Result::unwrap_err).collect(),
    )
}
