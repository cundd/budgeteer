use crate::amount_converter::AmountConverter;
use crate::currency::Currency;
use crate::error::{Error, Res};
use crate::file::{normalize_file_path, FileReader, FileWriter};
use crate::filter::{Filter, Request};
use crate::invoice::invoice_parser::InvoiceParser;
use crate::invoice::invoice_type::InvoiceType;
use crate::invoice::Invoice;
use crate::printer::{Printer, PrinterTrait};
use crate::rate_provider::RateProvider;
use crate::verbosity::Verbosity;
use crate::wizard::Wizard;
use chrono::{Datelike, Local};
use clap::{arg, command, Parser, Subcommand};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::path::PathBuf;
use std::vec;

mod amount_converter;
mod calculator;
mod currency;
mod error;
mod file;
mod filter;
mod invoice;
mod month;
mod printer;
mod rate_provider;
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

    /// Display the available types
    ShowTypes {},
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Analyze {
            input,
            year,
            month,
            day,
            r#type,
            verbosity,
        }) => {
            let printer = Printer::new();
            let base_currency = Currency::eur();

            let input_file = normalize_file_path(input)?;

            let filter_request = Request::new(*year, *month, *day, *r#type);
            let verbosity = Verbosity::from_int(*verbosity);

            if verbosity >= Verbosity::Info {
                printer.print_filter_request(&filter_request);
            }

            let invoices_to_print = load_and_display_invoices(
                &printer,
                &base_currency,
                input_file,
                Some(&filter_request),
                verbosity,
            )?;

            // Print an overview of the months, if there is **no** filter for the month
            if filter_request.month().is_none() {
                for month in 1..13 {
                    filter_and_print_month_sum(
                        &filter_request,
                        &printer,
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
        Some(Commands::Wizard { output, verbosity }) => {
            let printer = Printer::new();
            let base_currency = Currency::eur();

            let output_file = normalize_file_path(output)?;
            FileWriter::check_output_path(&output_file)?;
            let invoices_to_print = if output_file.exists() {
                let verbosity = Verbosity::from_int(*verbosity);
                println!("The output file contains these invoices:");
                load_and_display_invoices(&printer, &base_currency, &output_file, None, verbosity)
                    .unwrap_or_default()
            } else {
                vec![]
            };

            let wiz = Wizard::new();

            return Ok(wiz.run(&printer, &base_currency, &output_file, &invoices_to_print)?);
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

fn load_and_display_invoices<P: AsRef<Path>>(
    printer: &Printer,
    base_currency: &Currency,
    input_file: P,
    filter_request: Option<&Request>,
    verbosity: Verbosity,
) -> Res<Vec<Invoice>> {
    let invoices_to_print = get_invoices(
        input_file,
        filter_request,
        base_currency,
        printer,
        verbosity,
    )?;

    printer.print_invoices(base_currency, &invoices_to_print);

    Ok(invoices_to_print)
}

fn filter_and_print_month_sum(
    filter_request: &Request,
    printer: &Printer,
    base_currency: &Currency,
    all_invoices: &[Invoice],
    month: u32,
) {
    let filter_request_for_month = filter_request.with_month(month);
    let invoices = Filter::filter(all_invoices, &filter_request_for_month);
    printer.print_month_sum(month.into(), base_currency, &invoices);
}

fn get_invoices<P: AsRef<Path>>(
    input_file: P,
    filter_request: Option<&Request>,
    base_currency: &Currency,
    printer: &Printer,
    verbosity: Verbosity,
) -> Result<Vec<Invoice>, Error> {
    let lines = FileReader::read(input_file)?;
    let parser = InvoiceParser::new();
    let result = parser.parse_lines(lines.lines);
    let all_invoices = result.invoices;
    let invoices = match filter_request {
        Some(filter) if !filter.empty() => Filter::filter(&all_invoices, filter_request.unwrap()),
        _ => all_invoices,
    };

    printer.print_errors(result.errors);

    if invoices.is_empty() {
        return Ok(vec![]);
    }

    let rate_map = match RateProvider::fetch_rates(
        invoices.first().unwrap().date(),
        invoices.last().unwrap().date(),
        collect_currencies(&invoices, base_currency),
    ) {
        Ok(rate_map) => rate_map,
        Err(e) => {
            if verbosity >= Verbosity::Debug {
                eprintln!("{}", e)
            }
            HashMap::new()
        }
    };

    let amount_converter = AmountConverter::new(base_currency.to_owned(), rate_map);
    Ok(invoices
        .into_iter()
        .map(|i| amount_converter.invoice_with_base_amount(&i))
        .collect())
}

fn collect_currencies<'a>(invoices: &'a [Invoice], base_currency: &Currency) -> Vec<&'a str> {
    let mut currencies: HashSet<_> = HashSet::new();
    for invoice in invoices {
        if &invoice.amount().currency() != base_currency {
            currencies.insert(invoice.amount_ref().currency_ref().iso.as_str());
        }
    }

    currencies.into_iter().collect()
}

fn show_types() {
    println!("Available types:");
    for invoice_type in &InvoiceType::all_known() {
        println!("- {}: {}", invoice_type.identifier(), invoice_type);
    }
}
