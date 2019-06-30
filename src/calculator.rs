use invoice::Invoice;
use invoice::invoice_type::InvoiceType;

pub struct Calculator {}

impl Calculator {
    pub fn sum(invoices: &Vec<Invoice>) -> f64 {
        let mut sum = 0.0;
        for invoice in invoices {
            match invoice.base_amount() {
                Some(a) => sum += a.value(),
                None => {}
            }
        }
        sum
    }

    pub fn sum_for_type(invoices: &Vec<Invoice>, invoice_type: InvoiceType) -> f64 {
        let mut sum = 0.0;
        for invoice in invoices {
            if invoice.invoice_type() == invoice_type {
                match invoice.base_amount() {
                    Some(a) => sum += a.value(),
                    None => {}
                }
            }
        }
        sum
    }
}
