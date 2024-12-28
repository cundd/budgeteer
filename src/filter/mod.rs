mod request;

pub use self::request::*;

use crate::invoice::Invoice;
use chrono::Datelike;

pub struct Filter {}

impl Filter {
    pub fn filter(invoices: &[Invoice], request: &Request) -> Vec<Invoice> {
        invoices
            .iter()
            .filter(|invoice| Filter::matches_filter(invoice, request))
            .map(Clone::clone)
            .collect()
    }

    fn matches_filter(invoice: &Invoice, request: &Request) -> bool {
        if request.year().is_some() && invoice.date().year() != request.year().unwrap() {
            return false;
        }
        if request.month().is_some() && invoice.date().month() != request.month().unwrap() {
            return false;
        }

        if request.day().is_some() && invoice.date().day() != request.day().unwrap() {
            return false;
        }

        if request.invoice_type().is_some()
            && invoice.invoice_type() != request.invoice_type().unwrap()
        {
            return false;
        }

        true
    }
}
