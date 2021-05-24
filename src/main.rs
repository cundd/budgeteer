#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

use std::collections::{HashMap, HashSet};
use std::path::Path;

use chrono::{Datelike, Local};
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

use crate::amount_converter::AmountConverter;
use crate::currency::Currency;
use crate::error::{Error, Res};
use crate::file::FileWriter;
use crate::file::{normalize_file_path, FileReader};
use crate::filter::{Filter, Request};
use crate::invoice::invoice_parser::InvoiceParser;
use crate::invoice::invoice_type::InvoiceType;
use crate::invoice::Invoice;
use crate::printer::{Printer, PrinterTrait};
use crate::rate_provider::RateProvider;
use crate::verbosity::Verbosity;
use crate::wizard::Wizard;

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

fn main() {
    let matches = App::new("Budgeteer")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Daniel Corn <info@corn.rest>")
        .about("Manage information about paid invoices")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("analyze")
                .alias("a")
                .about("Show information about paid invoices")
                .arg(
                    Arg::with_name("input")
                        .help("Budget file to use")
                        .required(true)
                        .index(1),
                )
                //        .arg(Arg::with_name("rate")
                //            .long("rate")
                //            .short("r")
                //            .takes_value(true)
                //            .help("Currency change rate (CHF per €)"))
                .arg(
                    Arg::with_name("year")
                        .long("year")
                        .short("y")
                        .takes_value(true)
                        .help("Filter by year (default is the current year)"),
                )
                .arg(
                    Arg::with_name("type")
                        .long("type")
                        .short("t")
                        .takes_value(true)
                        .help("Filter by type"),
                )
                .arg(
                    Arg::with_name("month")
                        .long("month")
                        .short("m")
                        .takes_value(true)
                        .help("Filter by month"),
                )
                .arg(
                    Arg::with_name("day")
                        .long("day")
                        .short("d")
                        .takes_value(true)
                        .help("Filter by day"),
                )
                .arg(
                    Arg::with_name("v")
                        .short("v")
                        .multiple(true)
                        .help("Level of verbosity"),
                ),
        )
        .subcommand(
            SubCommand::with_name("wizard")
                .alias("w")
                .about("Interactive wizard to create new rows")
                .arg(
                    Arg::with_name("output")
                        .help("Budget file to write")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("debug")
                        .short("d")
                        .help("print debug information verbosely"),
                ),
        )
        .subcommand(SubCommand::with_name("show-types").about("Display the available types"))
        .get_matches();

    if let Err(e) = execute(matches) {
        eprintln!("{}", e)
    }
}

fn execute(root_matches: ArgMatches) -> Res<()> {
    if root_matches.subcommand_matches("show-types").is_some() {
        show_types();
        return Ok(());
    }

    let printer = Printer::new();
    let base_currency = Currency::eur();

    if let Some(matches) = root_matches.subcommand_matches("wizard") {
        let output_file = normalize_file_path(matches.value_of("output").unwrap())?;
        FileWriter::check_output_path(&output_file)?;
        if output_file.exists() {
            println!("The output file contains these invoices:");
            let _ = load_and_display_invoices(&printer, &base_currency, &output_file, None);
        }

        let wiz = Wizard::new();

        return wiz.run(&printer, &base_currency, &output_file);
    }
    if let Some(matches) = root_matches.subcommand_matches("analyze") {
        let input_file = normalize_file_path(matches.value_of("input").unwrap())?;

        //    let rate_string = matches.value_of("rate").unwrap();
        //    let rate = get_rate(rate_string)?;
        let filter_request = build_filter_request(&matches)?;
        let verbosity = Verbosity::from_int(matches.occurrences_of("v"));

        if verbosity >= Verbosity::Info {
            printer.print_filter_request(&filter_request);
        }

        let invoices_to_print =
            load_and_display_invoices(&printer, &base_currency, input_file, Some(&filter_request))?;

        // Print an overview of the months, if there is NO filter for the month
        if filter_request.month().is_none() {
            for month in 1..13 {
                filter_and_print_month_sum(
                    &matches,
                    &printer,
                    &base_currency,
                    &invoices_to_print,
                    month,
                );
            }
            println!()
        }
        printer.print_sum(&base_currency, &invoices_to_print);
    }

    Ok(())
}

fn load_and_display_invoices<P: AsRef<Path>>(
    printer: &Printer,
    base_currency: &Currency,
    input_file: P,
    filter_request: Option<&Request>,
) -> Res<Vec<Invoice>> {
    let parser = InvoiceParser::new();
    let all_invoices = get_invoices(input_file, &parser, &base_currency, Some(&printer))?;
    let invoices_to_print = if filter_request.is_none() || filter_request.unwrap().empty() {
        all_invoices
    } else {
        Filter::filter(&all_invoices, &filter_request.unwrap())
    };
    printer.print_invoices(&base_currency, &invoices_to_print);

    Ok(invoices_to_print)
}

fn filter_and_print_month_sum(
    matches: &ArgMatches,
    printer: &Printer,
    base_currency: &Currency,
    all_invoices: &[Invoice],
    month: u32,
) {
    if let Ok(filter_request) = build_month_filter_request(&matches, month) {
        let invoices = Filter::filter(&all_invoices, &filter_request);
        printer.print_month_sum(month.into(), &base_currency, &invoices);
    }
}

fn build_filter_request(matches: &ArgMatches) -> Res<Request> {
    let year = matches.value_of("year");
    match year {
        // No year argument was given => Default to the current year
        None => Request::from_year_and_strings(
            Local::now().year(),
            matches.value_of("month"),
            matches.value_of("day"),
            matches.value_of("type"),
        ),
        // Year argument is "all" => Do **not** filter
        Some(y) if y == "all" => Request::from_strings(
            None,
            matches.value_of("month"),
            matches.value_of("day"),
            matches.value_of("type"),
        ),
        // Parse the year argument
        Some(_) => Request::from_strings(
            year,
            matches.value_of("month"),
            matches.value_of("day"),
            matches.value_of("type"),
        ),
    }
}

fn build_month_filter_request(matches: &ArgMatches, month: u32) -> Res<Request> {
    Ok(build_filter_request(matches)?.with_month(month))
}

//fn get_rate(rate_string: &str) -> Result<f32, Error> {
//    if rate_string.trim() == "" {
//        return Ok(1.0);
//    }
//    match rate_string.trim().parse::<f32>() {
//        Ok(r) => Ok(r),
//        Err(_) => return Err(Error::General("Could not parse rate as number".to_owned()))
//    }
//}

fn get_invoices<P: AsRef<Path>>(
    input_file: P,
    parser: &InvoiceParser,
    base_currency: &Currency,
    printer: Option<&Printer>,
) -> Result<Vec<Invoice>, Error> {
    let lines = FileReader::read(input_file)?;
    let result = parser.parse_lines(lines.lines);
    let invoices: Vec<Invoice> = result.invoices;

    if let Some(printer) = printer {
        printer.print_errors(result.errors);
    }

    if invoices.is_empty() {
        return Ok(vec![]);
    }
    // let rate_map = RateProvider::fetch_rates(
    //     invoices.first().unwrap().date(),
    //     invoices.last().unwrap().date(),
    //     collect_currencies(&invoices, &base_currency),
    // )?;
    let rate_map = match RateProvider::fetch_rates(
        invoices.first().unwrap().date(),
        invoices.last().unwrap().date(),
        collect_currencies(&invoices, &base_currency),
    ) {
        Ok(r) => r,
        Err(_) => HashMap::new(),
    };

    let amount_converter = AmountConverter::new(base_currency.to_owned(), rate_map);
    Ok(invoices
        .into_iter()
        .map(|i| amount_converter.invoice_with_base_amount(&i))
        .collect())
}

fn collect_currencies<'a, 'b>(
    invoices: &'a [Invoice],
    base_currency: &'b Currency,
) -> Vec<&'a str> {
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
