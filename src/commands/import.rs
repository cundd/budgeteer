use crate::{
    currency::Currency,
    duplicate_check::DuplicateChecker,
    error::{Error, Res},
    file::normalize_file_path,
    import,
    persistence::TransactionRepository,
    printer::PrinterTrait,
    verbosity::Verbosity,
    wizard::Wizard,
};
use std::path::PathBuf;

pub async fn import<P: PrinterTrait>(
    printer: &mut P,
    base_currency: Currency,
    input: &PathBuf,
    output: &PathBuf,
    no_interaction: bool,
    verbosity: Verbosity,
) -> Res<()> {
    let input_file = normalize_file_path(input)?;
    let output_file = normalize_file_path(output)?;
    let repository = TransactionRepository::new(&output_file).await?;
    let current_transactions = repository.fetch_all().await?;

    let result = match input_file
        .extension()
        .map(|e| e.to_str().expect("Path is not UTF8"))
    {
        Some("json") => import::json::get_transactions(input_file, |mut transaction| {
            if !no_interaction {
                printer.print_header("Complete the following transaction details");
                printer.print_transaction(&base_currency, &transaction);

                let possible_duplicates =
                    DuplicateChecker::get_possible_duplicates(&transaction, &current_transactions);
                if !possible_duplicates.is_empty() {
                    printer.print_warning("⚠︎ Found possible duplicates:");
                    for possible_duplicate in possible_duplicates {
                        printer.print_transaction(&base_currency, possible_duplicate);
                    }
                }

                let selected_transaction_type =
                    Wizard::new().read_transaction_type_or_skip(true)?;
                match selected_transaction_type {
                    Some(i) => transaction.transaction_type = i,
                    None => return Ok(None),
                }
            }

            Ok(Some(transaction))
        })?,
        Some("md") => import::markdown::get_transactions(input_file)?,
        Some(e) => return Err(Error::Import(format!("No parser to import {} files", e)))?,
        None => {
            return Err(Error::Import(format!(
                "Could not detect the extension of file path {}",
                input_file.display()
            )))?
        }
    };

    let transactions = result.transactions;
    let errors = result.errors;

    for error in &errors {
        eprintln!("Error during import: {}", error);
    }
    let mut successful_imports_counter = 0;
    let mut failed_imports_counter = 0;
    for transaction in &transactions {
        if verbosity >= Verbosity::Debug {
            printer.println(format!("Try to import transaction: {:#?}", transaction));
        }
        if let Err(e) = repository.add(transaction).await {
            eprintln!("Error during import of transaction: {}", e);
            printer.print_transaction(&base_currency, transaction);
            printer.print_newline();

            failed_imports_counter += 1;
        } else {
            successful_imports_counter += 1;
        }
    }

    printer.print_header("Imported the following transactions:");
    printer.print_transactions(&base_currency, &transactions);
    printer.println(format!(
        "{} successful imports / {} failures / {} parsing errors",
        successful_imports_counter,
        failed_imports_counter,
        errors.len()
    ));

    Ok(())
}
