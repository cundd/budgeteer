use crate::currency::Currency;
use crate::file::normalize_file_path;
use crate::filter::Request;
use crate::invoice::invoice_type::InvoiceType;
use crate::invoice::Invoice;
use crate::printer::{Printer, PrinterTrait};
use crate::verbosity::Verbosity;
use crate::wizard::Wizard;
use chrono::Datelike;
use clap::{arg, command, Parser, Subcommand};
use duplicate_check::DuplicateChecker;
use error::Error;
use persistence::InvoiceRepository;
use std::path::PathBuf;

mod calculator;
mod currency;
mod duplicate_check;
mod error;
mod file;
mod filter;
mod import;
mod invoice;
mod month;
mod persistence;
mod printer;
mod verbosity;
mod wizard;

/// Manage information about paid invoices
#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Show information about paid invoices
    #[command(alias("a"))]
    Analyze {
        /// Budget file to use
        #[arg(value_name = "FILE")]
        input: PathBuf,

        /// Show entries from this date
        #[arg(short, long)]
        from: Option<String>,

        /// Show entries up to and including this date
        #[arg(short('x'), long)]
        to: Option<String>,

        /// Filter by type
        #[arg(short, long)]
        r#type: Option<InvoiceType>,

        /// Level of verbosity
        #[arg(short, long, action = clap::ArgAction::Count)]
        verbosity: u8,
    },

    /// Interactive wizard to create new rows
    #[command(alias("w"))]
    Wizard {
        /// Budget file to use
        #[arg(value_name = "FILE")]
        output: PathBuf,

        /// Level of verbosity
        #[arg(short, long, action = clap::ArgAction::Count)]
        verbosity: u8,
    },

    /// Import data from Markdown files into the database
    Import {
        /// Markdown or JSON file to import
        #[arg(value_name = "IMPORT-FILE")]
        input: PathBuf,

        /// Budget file to use
        #[arg(value_name = "DATABASE")]
        output: PathBuf,

        /// Level of verbosity
        #[arg(short, long, action = clap::ArgAction::Count)]
        verbosity: u8,
    },

    /// Display the available types
    ShowTypes {},
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut printer = Printer::new();
    let base_currency = Currency::base();

    match &cli.command {
        Some(Commands::Analyze {
            input,
            from,
            to,
            r#type,
            verbosity,
        }) => {
            let input_file = normalize_file_path(input)?;
            let repository = InvoiceRepository::new(&input_file).await?;

            let from = if let Some(from) = from {
                Some(Request::parse_from_date(from)?)
            } else {
                None
            };
            let to = if let Some(to) = to {
                Some(Request::parse_to_date(to)?)
            } else {
                None
            };

            let filter_request = Request::new(from, to, *r#type);
            let verbosity = Verbosity::from_int(*verbosity);

            if verbosity >= Verbosity::Info {
                printer.print_filter_request(&filter_request);
            }

            let invoices_to_print = repository.fetch_with_request(filter_request).await?;
            printer.print_invoices(&base_currency, &invoices_to_print);

            for month in 1..13 {
                filter_and_print_month_sum(&mut printer, &base_currency, &invoices_to_print, month);
            }

            printer.print_newline();
            printer.print_sum(&base_currency, &invoices_to_print);
        }

        Some(Commands::Import {
            input,
            output,
            verbosity,
        }) => {
            let verbosity = Verbosity::from_int(*verbosity);

            let input_file = normalize_file_path(input)?;
            let output_file = normalize_file_path(output)?;
            let repository = InvoiceRepository::new(&output_file).await?;
            let current_transactions = repository.fetch_all().await?;

            let result = match input_file
                .extension()
                .map(|e| e.to_str().expect("Path is not UTF8"))
            {
                Some("json") => import::json::get_transactions(input_file, |mut transaction| {
                    printer.print_header("Complete the following transaction details");
                    printer.print_invoice(&base_currency, &transaction);

                    let possible_duplicates = DuplicateChecker::get_possible_duplicates(
                        &transaction,
                        &current_transactions,
                    );
                    if !possible_duplicates.is_empty() {
                        printer.print_subheader("Found possible duplicates:");
                        for possible_duplicate in possible_duplicates {
                            printer.print_invoice(&base_currency, possible_duplicate);
                        }
                    }

                    let selected_invoice_type = Wizard::new().read_invoice_type_or_skip(true)?;
                    match selected_invoice_type {
                        Some(i) => transaction.invoice_type = i,
                        None => return Ok(None),
                    }

                    Ok(Some(transaction))
                })?,
                Some("md") => import::markdown::get_transactions(input_file)?,
                Some(e) => {
                    return Err(Box::new(Error::Import(format!(
                        "No parser to import {} files",
                        e
                    ))))?
                }
                None => {
                    return Err(Box::new(Error::Import(format!(
                        "Could not detect the extension of file path {}",
                        input_file.display()
                    ))))?
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
                    printer.print_invoice(&base_currency, transaction);
                    printer.print_newline();

                    failed_imports_counter += 1;
                } else {
                    successful_imports_counter += 1;
                }
            }

            printer.print_header("Imported the following transactions:");
            printer.print_invoices(&base_currency, &transactions);
            printer.println(format!(
                "{} successful imports / {} failures / {} parsing errors",
                successful_imports_counter,
                failed_imports_counter,
                errors.len()
            ));
        }

        Some(Commands::Wizard {
            output,
            verbosity: _,
        }) => {
            let output_file = normalize_file_path(output)?;
            let repository = InvoiceRepository::new(&output_file).await?;

            let invoices_to_print = repository.fetch_all().await?;
            if !invoices_to_print.is_empty() {
                printer.print_header("The output file contains these invoices:");
                printer.print_invoices(&base_currency, &invoices_to_print);
            }

            let wiz = Wizard::new();

            return Ok(wiz
                .run(
                    &mut printer,
                    &base_currency,
                    &repository,
                    &invoices_to_print,
                )
                .await?);
        }
        Some(Commands::ShowTypes {}) => show_types(&mut printer),
        None => {}
    }

    Ok(())
}

fn filter_and_print_month_sum(
    printer: &mut Printer,
    base_currency: &Currency,
    all_invoices: &[Invoice],
    month: u32,
) {
    let invoices: Vec<Invoice> = all_invoices
        .iter()
        .filter(|i| i.date.month() == month)
        .map(Clone::clone)
        .collect();
    printer.print_month_sum(month.into(), base_currency, &invoices);
}

fn show_types(printer: &mut Printer) {
    printer.print_header("Available types:");
    for invoice_type in &InvoiceType::all_known() {
        printer.println(format!("- {}: {}", invoice_type.identifier(), invoice_type));
    }
}
