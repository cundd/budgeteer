use error::Error;
use invoice::Invoice;
use currency::Currency;
use filter::Request;
use calculator::Calculator;
use invoice::invoice_type::InvoiceType;
use month::Month;

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

    fn print_sum(&self, base_currency: &Currency, invoices: &Vec<Invoice>) -> ();
    fn print_month_sum(&self, month: Month, base_currency: &Currency, invoices: &Vec<Invoice>) -> ();
}

pub struct Printer {}

impl Printer {
    pub fn new() -> Self {
        Printer {}
    }

    fn print_type_sum(&self, base_currency: &Currency, invoices: &Vec<Invoice>) -> () {
        let types = vec![
            InvoiceType::Car,
            InvoiceType::Clothes,
            InvoiceType::Eat,
            InvoiceType::Fun,
            InvoiceType::Gas,
            InvoiceType::Health,
            InvoiceType::Home,
            InvoiceType::Telecommunication,
            InvoiceType::Unknown,
        ];
        for invoice_type in types {
            let sum = Calculator::sum_for_type(invoices, invoice_type);
            println!(
                "{:width$}: {} {: >8.2}",
                format!("{}", invoice_type),
                base_currency,
                sum,
                width = 22
            );
        }
    }

    fn print_grand_total(&self, base_currency: &Currency, invoices: &Vec<Invoice>) -> () {
        println!("TOTAL: {} {:.2}", base_currency, Calculator::sum(invoices));
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
        let note = match invoice.note() {
            Some(ref c) => c.to_owned(),
            None => "".to_owned()
        };

        println!("Datum:     {}", invoice.date().format("%A %d.%m.%Y"));

        if &invoice.amount().currency() != base_currency {
            match invoice.clone().base_amount() {
                Some(converted_amount) => println!(
                    "Betrag:    {} ({})",
                    invoice.amount(),
                    converted_amount
                ),
                None => println!("Betrag:    {}", invoice.amount())
            }
        } else {
            println!("Betrag:    {}", invoice.amount());
        }
        println!("Typ:       {}", invoice.invoice_type());
        println!("Notiz:     {}", note);
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

    fn print_sum(&self, base_currency: &Currency, invoices: &Vec<Invoice>) -> () {
        self.print_type_sum(base_currency, invoices);
        println!("----------------------------------");
        self.print_grand_total(base_currency, invoices);
    }

    fn print_month_sum(&self, month: Month, base_currency: &Currency, invoices: &Vec<Invoice>) -> () {
        println!(
            "{:width$}: {} {: >8.2}",
            format!("{}", month),
            base_currency,
            Calculator::sum(invoices),
            width = 12
        );
    }
}

