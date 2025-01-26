use crate::{
    currency::Currency, error::Res, file::normalize_file_path, persistence::TransactionRepository,
    printer::PrinterTrait, wizard::Wizard,
};
use std::path::PathBuf;

pub async fn wizard<P: PrinterTrait>(
    printer: &mut P,
    base_currency: Currency,
    output: &PathBuf,
) -> Res<()> {
    let output_file = normalize_file_path(output)?;
    let repository = TransactionRepository::new(&output_file).await?;

    let transactions_to_print = repository.fetch_all().await?;
    if !transactions_to_print.is_empty() {
        printer.print_header("The output file contains these transactions:");
        printer.print_transactions(&base_currency, &transactions_to_print);
    }

    let wiz = Wizard::new();

    wiz.run(printer, &base_currency, &repository, &transactions_to_print)
        .await
}
