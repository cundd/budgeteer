use crate::{
    currency::Currency,
    error::Res,
    file::normalize_file_path,
    filter::Request,
    persistence::TransactionRepository,
    printer::PrinterTrait,
    transaction::{transaction_type::TransactionType, Transaction},
    verbosity::Verbosity,
};
use chrono::prelude::*;
use std::path::PathBuf;

#[allow(clippy::too_many_arguments)]
pub async fn analyze<P: PrinterTrait>(
    printer: &mut P,
    base_currency: Currency,
    input: &PathBuf,
    from: Option<String>,
    to: Option<String>,
    search: Option<String>,
    exclude: Option<String>,
    transaction_type: Option<TransactionType>,
    verbosity: Verbosity,
) -> Res<()> {
    let input_file = normalize_file_path(input)?;
    let repository = TransactionRepository::new(&input_file).await?;
    let filter_request = Request::from_arguments(from, to, transaction_type, search, exclude)?;

    if verbosity >= Verbosity::Info {
        printer.print_filter_request(&filter_request);
    }

    let transactions_to_print = repository.fetch_with_request(filter_request).await?;
    printer.print_transactions(&base_currency, &transactions_to_print);

    for month in 1..13 {
        filter_and_print_month_sum(printer, &base_currency, &transactions_to_print, month);
    }

    printer.print_newline();
    printer.print_sum(&base_currency, &transactions_to_print);
    Ok(())
}

fn filter_and_print_month_sum<P: PrinterTrait>(
    printer: &mut P,
    base_currency: &Currency,
    all_transactions: &[Transaction],
    month: u32,
) {
    let transactions: Vec<Transaction> = all_transactions
        .iter()
        .filter(|i| i.date.month() == month)
        .map(Clone::clone)
        .collect();
    printer.print_month_sum(month.into(), base_currency, &transactions);
}
