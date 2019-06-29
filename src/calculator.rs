use invoice::Invoice;

pub struct Calculator {}

impl Calculator {
    pub fn sum(invoices: Vec<Invoice>) -> f64 {
        let mut sum = 0.0;
        for invoice in invoices {
            match invoice.base_amount {
                Some(a) => sum += a.value,
                None => {}
            }
        }
        sum
    }
}
