use crate::{
    currency::Currency,
    error::Res,
    file::normalize_file_path,
    filter::Request,
    invoice::{invoice_type::InvoiceType, Invoice},
    persistence::InvoiceRepository,
    printer::PrinterTrait,
    verbosity::Verbosity,
};
use chrono::prelude::*;
use std::path::PathBuf;

pub async fn analyze<P: PrinterTrait>(
    printer: &mut P,
    base_currency: Currency,
    input: &PathBuf,
    from: Option<String>,
    to: Option<String>,
    transaction_type: Option<InvoiceType>,
    verbosity: Verbosity,
) -> Res<()> {
    let input_file = normalize_file_path(input)?;
    let repository = InvoiceRepository::new(&input_file).await?;

    let from = if let Some(from) = from {
        Some(Request::parse_from_date(&from)?)
    } else {
        None
    };
    let to = if let Some(to) = to {
        Some(Request::parse_to_date(&to)?)
    } else {
        None
    };

    let filter_request = Request::new(from, to, transaction_type);

    if verbosity >= Verbosity::Info {
        printer.print_filter_request(&filter_request);
    }

    let invoices_to_print = repository.fetch_with_request(filter_request).await?;
    printer.print_invoices(&base_currency, &invoices_to_print);

    for month in 1..13 {
        filter_and_print_month_sum(printer, &base_currency, &invoices_to_print, month);
    }

    printer.print_newline();
    printer.print_sum(&base_currency, &invoices_to_print);
    Ok(())
}

fn filter_and_print_month_sum<P: PrinterTrait>(
    printer: &mut P,
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
