use chrono::{Days, Utc};

use crate::{
    currency::Currency, error::Res, file::normalize_file_path, persistence::TransactionRepository,
    printer::PrinterTrait, transaction::Transaction, wizard::Wizard,
};
use std::path::PathBuf;

pub async fn wizard<P: PrinterTrait>(
    printer: &mut P,
    base_currency: Currency,
    output: &PathBuf,
) -> Res<()> {
    let output_file = normalize_file_path(output)?;
    let repository = TransactionRepository::new(&output_file).await?;

    let current_transactions = repository.fetch_all().await?;
    let transactions_to_print = get_transactions_in_last_n_days(&current_transactions, 31);
    if !transactions_to_print.is_empty() {
        printer.print_header("The output file contains these transactions:");
        printer.print_transactions(&base_currency, &transactions_to_print);
    }

    let wiz = Wizard::new();

    wiz.run(printer, &base_currency, &repository, &current_transactions)
        .await
}

fn get_transactions_in_last_n_days(transactions: &[Transaction], days: u64) -> Vec<Transaction> {
    let reference_date = Utc::now()
        .date_naive()
        .checked_sub_days(Days::new(days))
        .expect("Date out of range");

    transactions
        .iter()
        .rev()
        .filter_map(|t| {
            if t.date >= reference_date {
                Some(t.clone())
            } else {
                None
            }
        })
        .rev()
        .collect()
}
