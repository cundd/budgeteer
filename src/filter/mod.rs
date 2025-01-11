pub use self::request::*;
use crate::invoice::Invoice;

mod request;

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
        if let Some(from) = request.from {
            if invoice.date() < from {
                return false;
            }
        }

        if let Some(to) = request.to {
            if invoice.date() > to {
                return false;
            }
        }

        if let Some(invoice_type) = request.invoice_type() {
            if invoice.invoice_type() != invoice_type {
                return false;
            }
        }

        true
    }
}
