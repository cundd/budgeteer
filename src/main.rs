#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate clap;
extern crate chrono;
extern crate serde_json;
extern crate core;

mod error;
mod file_reader;
mod invoice;
mod amount_converter;
mod rate_provider;
mod printer;
mod filter;
mod currency;
mod calculator;
mod verbosity;

use clap::{Arg, App, ArgMatches};
use rate_provider::RateProvider;
use amount_converter::AmountConverter;
use invoice::Invoice;
use error::Error;
use filter::{Filter, Request};
use calculator::Calculator;
use file_reader::FileReader;
use invoice::invoice_parser::InvoiceParser;
use currency::Currency;
use std::collections::HashSet;
use verbosity::Verbosity;
use printer::{Printer, PrinterTrait};

fn main() {
    let matches = App::new("Budgeteer")
        .version("0.1.0")
        .author("Daniel Corn <info@corn.rest>")
        .about("Show information about paid invoices")
        .arg(Arg::with_name("input")
            .help("Budget file to use")
            .required(true)
            .index(1))
//        .arg(Arg::with_name("rate")
//            .long("rate")
//            .short("r")
//            .takes_value(true)
//            .help("Currency change rate (CHF per €)"))
        .arg(Arg::with_name("year")
            .long("year")
            .short("y")
            .takes_value(true)
            .help("Filter by year"))
        .arg(Arg::with_name("type")
            .long("type")
            .short("t")
            .takes_value(true)
            .help("Filter by type"))
        .arg(Arg::with_name("month")
            .long("month")
            .short("m")
            .takes_value(true)
            .help("Filter by month"))
        .arg(Arg::with_name("day")
            .long("day")
            .short("d")
            .takes_value(true)
            .help("Filter by day"))
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Level of verbosity"))
        .get_matches();

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
//    match matches.occurrences_of("v") {
//        0 => println!("No verbose info"),
//        1 => println!("Some verbose info"),
//        2 => println!("Tons of verbose info"),
//        3 | _ => println!("Don't be crazy"),
//    }

    match execute(matches) {
        Err(e) => eprintln!("{}", e),
        Ok(_) => {}
    }
}

fn execute(matches: ArgMatches) -> Result<(), Error> {
    let input_file = matches.value_of("input").unwrap();
//    let rate_string = matches.value_of("rate").unwrap();
//    let rate = get_rate(rate_string)?;
    let filter_request = Request::from_strings(
        matches.value_of("year"),
        matches.value_of("month"),
        matches.value_of("day"),
        matches.value_of("type"),
    )?;

    let printer = Printer::new();

    let base_currency = Currency::from_string("€");
    let parser = InvoiceParser::new();

    let verbosity = Verbosity::from_int(matches.occurrences_of("v"));
    if verbosity >= Verbosity::Info {
        printer.print_filter_request(&filter_request);
    }
    let invoices = get_filtered_invoices(
        input_file,
        filter_request,
        &parser,
        &base_currency,
        &printer,
    )?;


    printer.print_invoices(&base_currency, &invoices);

    println!("TOTAL: {} {:.2}", base_currency, Calculator::sum(invoices));

    Ok(())
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

fn get_filtered_invoices(
    input_file: &str,
    filter_request: Request,
    parser: &InvoiceParser,
    base_currency: &Currency,
    printer: &Printer,
) -> Result<Vec<Invoice>, Error> {
    let lines = FileReader::read(input_file)?;
    let result = parser.parse_lines(lines.lines);
    let invoices: Vec<Invoice> = result.invoices.into_iter().map(|(_, i)| i).collect();

    printer.print_errors(result.errors);

    if invoices.len() == 0 {
        return Ok(vec![]);
    }
    let rate_map = RateProvider::fetch_rates(
        invoices.first().unwrap().date,
        invoices.last().unwrap().date,
        collect_currencies(&invoices))?;

    let amount_converter = AmountConverter::new(base_currency.to_owned(), rate_map);

    if filter_request.empty() {
        if invoices.len() == 0 {
            println!("No invoices found");
            Ok(vec![])
        } else {
            Ok(invoices.into_iter().map(|i| amount_converter.invoice_with_base_amount(&i)).collect())
        }
    } else {
        let invoices_filtered = Filter::filter(invoices, &filter_request);
        if invoices_filtered.len() == 0 {
            println!("No invoices found for filter `{}`", filter_request);
            Ok(vec![])
        } else {
            Ok(invoices_filtered.into_iter().map(|i| amount_converter.invoice_with_base_amount(&i)).collect())
        }
    }
}

fn collect_currencies(invoices: &Vec<Invoice>) -> Vec<&str> {
    let mut currencies: HashSet<_> = HashSet::new();
    for invoice in invoices {
        if invoice.amount.currency.iso != "EUR" {
            currencies.insert(invoice.amount.currency.iso.as_str());
        }
    }

    currencies.into_iter().collect()
}
