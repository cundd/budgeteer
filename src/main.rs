use crate::currency::Currency;
use crate::error::{Error, Res};
use crate::file::normalize_file_path;
use crate::filter::{Filter, Request};
use crate::invoice::invoice_type::InvoiceType;
use crate::invoice::Invoice;
use crate::printer::{Printer, PrinterTrait};
use crate::verbosity::Verbosity;
use crate::wizard::Wizard;
use chrono::{Datelike, Local};
use clap::{arg, command, Parser, Subcommand};
use persistence::InvoiceRepository;
use std::path::PathBuf;

mod calculator;
mod currency;
mod error;
mod file;
mod filter;
mod invoice;
mod markdown;
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

        /// Filter by year (default is the current year)
        #[arg(short, long, default_value = "now", value_parser = year_argument_parser)]
        year: Option<i32>,

        /// Filter by month
        #[arg(short, long)]
        month: Option<u32>,

        /// Filter by day
        #[arg(short, long)]
        day: Option<u32>,

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
        /// Markdown file to import
        #[arg(value_name = "MARKDOWN-FILE")]
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
            year,
            month,
            day,
            r#type,
            verbosity,
        }) => {
            let input_file = normalize_file_path(input)?;
            let repository = InvoiceRepository::new(&input_file).await?;

            let filter_request = Request::new(*year, *month, *day, *r#type);
            let verbosity = Verbosity::from_int(*verbosity);

            if verbosity >= Verbosity::Info {
                printer.print_filter_request(&filter_request);
            }

            let invoices_to_print = load_and_display_invoices(
                &repository,
                &mut printer,
                &base_currency,
                Some(&filter_request),
            )
            .await?;

            // Print an overview of the months, if there is **no** filter for the month
            if filter_request.month().is_none() {
                for month in 1..13 {
                    filter_and_print_month_sum(
                        &filter_request,
                        &mut printer,
                        &base_currency,
                        &invoices_to_print,
                        month,
                    );
                }
                println!()
            }

            printer.print_sum(&base_currency, &invoices_to_print);
            if filter_request.year().is_none() {
                println!();
                println!("⚠︎ Achtung: Die Werte der Basis Währung können nicht korrekt berechnet werden wenn mehr als 365 Tage ausgegeben werden")
            }
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
            let current_spendings = repository.fetch_all().await?;

            let result = markdown::get_invoices(input_file)?;
            let invoices = result.invoices;
            let parsing_errors = result.errors;

            for error in &parsing_errors {
                eprintln!("Error during parsing Markdown file: {}", error);
            }
            let mut successful_imports_counter = 0;
            let mut failed_imports_counter = 0;
            let mut duplicate_imports_counter = 0;
            for invoice in &invoices {
                if verbosity >= Verbosity::Debug {
                    println!("Try to import invoice: {:#?}", invoice);
                }
                let is_already_present = current_spendings.iter().any(|i: &Invoice| {
                    i.date() == invoice.date()
                        && i.invoice_type() == invoice.invoice_type()
                        && i.amount() == invoice.amount()
                });
                if !is_already_present {
                    if let Err(e) = repository.add(invoice).await {
                        eprintln!("Error during import of invoice: {}", e);

                        failed_imports_counter += 1;
                    } else {
                        successful_imports_counter += 1;
                    }
                } else {
                    duplicate_imports_counter += 1;
                }
            }

            println!("Imported the following invoices:");
            printer.print_invoices(&base_currency, &invoices);
            println!(
                "{} successful imports / {} duplicates / {} failures / {} parsing errors",
                successful_imports_counter,
                duplicate_imports_counter,
                failed_imports_counter,
                parsing_errors.len()
            );
        }

        Some(Commands::Wizard {
            output,
            verbosity: _,
        }) => {
            let output_file = normalize_file_path(output)?;
            let repository = InvoiceRepository::new(&output_file).await?;

            let invoices_to_print = get_invoices(&repository, None).await?;

            if !invoices_to_print.is_empty() {
                println!("The output file contains these invoices:");
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
        Some(Commands::ShowTypes {}) => show_types(),
        None => {}
    }

    Ok(())
}

fn year_argument_parser(input: &str) -> Result<i32, String> {
    if input == "now" {
        return Ok(Local::now().year());
    }

    // if input == "all" {
    //     return Ok(None);
    // }

    input
        .parse::<i32>()
        .map_err(|_| format!("`{input}` isn't a valid year"))
}

async fn load_and_display_invoices(
    repository: &InvoiceRepository,
    printer: &mut Printer,
    base_currency: &Currency,
    filter_request: Option<&Request>,
) -> Res<Vec<Invoice>> {
    let invoices_to_print = get_invoices(repository, filter_request).await?;

    printer.print_invoices(base_currency, &invoices_to_print);

    Ok(invoices_to_print)
}

fn filter_and_print_month_sum(
    filter_request: &Request,
    printer: &mut Printer,
    base_currency: &Currency,
    all_invoices: &[Invoice],
    month: u32,
) {
    let filter_request_for_month = filter_request.with_month(month);
    let invoices = Filter::filter(all_invoices, &filter_request_for_month);
    printer.print_month_sum(month.into(), base_currency, &invoices);
}

async fn get_invoices(
    repository: &InvoiceRepository,
    filter_request: Option<&Request>,
) -> Result<Vec<Invoice>, Error> {
    let all_invoices = repository.fetch_all().await?;

    Ok(match filter_request {
        Some(filter) if !filter.empty() => Filter::filter(&all_invoices, filter_request.unwrap()),
        _ => all_invoices,
    })
}

fn show_types() {
    println!("Available types:");
    for invoice_type in &InvoiceType::all_known() {
        println!("- {}: {}", invoice_type.identifier(), invoice_type);
    }
}
