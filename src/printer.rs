use error::Error;
use invoice::Invoice;
use currency::Currency;
use filter::Request;

pub trait PrinterTrait {
    fn print_errors(&self, errors: Vec<Error>) {
        for ref error in errors {
            self.print_error(error);
        }
    }

    fn print_error(&self, error: &Error);

    fn print_invoices(&self, base_currency: &Currency, invoices: &Vec<Invoice>) -> () {
        for invoice in invoices {
            self.print_invoice(&base_currency, invoice)
        }
    }

    fn print_invoice(&self, base_currency: &Currency, invoice: &Invoice) -> ();

    fn print_filter_request(&self, filter_request: &Request) -> ();
}

pub struct Printer {}

impl Printer {
    pub fn new() -> Self {
        Printer {}
    }
}

impl PrinterTrait for Printer {
    fn print_error(&self, error: &Error) {
        match error {
            Error::LineComment => {}
            Error::LineEmpty => {}
            Error::LineSeparator => {}
            _ => eprintln!("Invoice error {}", error)
        }
    }

    #[allow(dead_code)]
    fn print_invoice(&self, base_currency: &Currency, invoice: &Invoice) -> () {
        let comment = match invoice.comment {
            Some(ref c) => c.to_owned(),
            None => "".to_owned()
        };

        println!("Datum:     {}", invoice.date.format("%A %d.%m.%Y"));

        if &invoice.amount.currency != base_currency {
            match invoice.clone().base_amount {
                Some(converted_amount) => println!(
                    "Betrag:    {} ({})",
                    invoice.amount,
                    converted_amount
                ),
                None => println!("Betrag:    {}", invoice.amount)
            }
        } else {
            println!("Betrag:    {}", invoice.amount);
        }
        println!("Typ:       {}", invoice.invoice_type);
        println!("Notiz:     {}", comment);
        println!();
    }

    fn print_filter_request(&self, filter_request: &Request) -> () {
        println!("Filter:");
        if filter_request.empty() {
            println!("Keine");
        } else {
            println!("{}", filter_request);
        }
        println!();
    }
}

