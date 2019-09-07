use error::Error;
use invoice::Invoice;
use currency::Currency;
use filter::Request;
use calculator::Calculator;
use invoice::invoice_type::InvoiceType;
use month::Month;
use ansi_term::Colour::RGB;
use ansi_term::{Style, Colour};
use std::env;

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
        println!()
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
                "{}{}",
                style_for_type(
                    invoice_type,
                    &format!(
                        "{:width$}: {} {: >8.2}  ",
                        format!("{}", invoice_type),
                        base_currency,
                        sum,
                        width = 22
                    ),
                    false,
                    true,
                ),
                style_for_type(
                    invoice_type,
                    &invoice_type.identifier().to_string(),
                    true,
                    true,
                ),
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
        let note = get_prepared_note(invoice);

        let amount_string = if &invoice.amount().currency() != base_currency {
            match invoice.clone().base_amount() {
                Some(converted_amount) => format!(
                    "{} ({})",
                    invoice.amount(),
                    converted_amount
                ),
                None => format!("{}", invoice.amount())
            }
        } else {
            format!("{}", invoice.amount())
        };

        let invoice_type = invoice.invoice_type();
        let date = invoice.date().format("%A %d.%m.%Y");

        if has_true_color_support() {
            println!(
                r#"
{ } Datum   : {}
Betrag      : {}
Typ         : {}
Notiz       : {}"#,
                style_for_type(invoice_type, " ", false, true),
                date,
                amount_string,
                invoice_type,
                note,
            );
        } else {
            println!(
                r#"
Datum       : {}
Betrag      : {}
Typ         : {}
Notiz       : {}"#,
                date,
                amount_string,
                invoice_type,
                note,
            );
        }
//        println!("{}", style_for_type(invoice_type, &format!(
//            r#"
//Datum      : {}
//Betrag     : {}
//Typ        : {}
//Notiz      : {}"#,
//            date,
//            amount_string,
//            invoice_type,
//            note,
//        )));
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
        println!("-----------------------------------------");
        self.print_grand_total(base_currency, invoices);
    }

    fn print_month_sum(&self, month: Month, base_currency: &Currency, invoices: &Vec<Invoice>) -> () {
        if invoices.len() > 0 {
            let max_type = Calculator::major_type(invoices).unwrap();

            println!(
                "{:width$}: {} {: >8.2} {}",
                format!("{}", month),
                base_currency,
                Calculator::sum(invoices),
                style_for_type(max_type, &max_type.identifier().to_string(), true, true),
                width = 12
            );
        } else {
            println!(
                "{:width$}: {} {: >8.2}",
                format!("{}", month),
                base_currency,
                0,
                width = 12
            )
        }
    }
}

fn style_for_type(invoice_type: InvoiceType, text: &str, fg: bool, bg: bool) -> String {
    if !has_true_color_support() {
        return text.to_owned();
    }
    let prepared_multi_line = text.lines().map(
        |l| {
            if l.len() > 0 {
                format!(" {} ", l)
            } else {
                "".to_owned()
            }
        }
    ).collect::<Vec<String>>().join("\n");

    if !fg && !bg {
        return prepared_multi_line;
    }

    let style = if fg && bg {
        Style::new()
            .fg(color_for_type(invoice_type, false))
            .on(color_for_type(invoice_type, true))
    } else if fg {
        Style::new().fg(color_for_type(invoice_type, false))
    } else {
        Style::new().on(color_for_type(invoice_type, true))
    };

    style.paint(prepared_multi_line).to_string()
}

fn color_for_type(invoice_type: InvoiceType, light: bool) -> Colour {
    if light {
        match invoice_type {
            InvoiceType::Car => RGB(112, 255, 81),
            InvoiceType::Clothes => RGB(177, 255, 79),
            InvoiceType::Eat => RGB(225, 255, 79),
            InvoiceType::Fun => RGB(255, 237, 61),
            InvoiceType::Gas => RGB(255, 200, 53),
            InvoiceType::Health => RGB(255, 173, 45),
            InvoiceType::Home => RGB(255, 136, 126),
            InvoiceType::Telecommunication => RGB(255, 120, 186),
            InvoiceType::Unknown => RGB(215, 151, 255),
        }
    } else {
        match invoice_type {
            InvoiceType::Car => RGB(84, 189, 60),
            InvoiceType::Clothes => RGB(132, 189, 58),
            InvoiceType::Eat => RGB(167, 189, 58),
            InvoiceType::Fun => RGB(189, 174, 45),
            InvoiceType::Gas => RGB(189, 146, 40),
            InvoiceType::Health => RGB(189, 127, 34),
            InvoiceType::Home => RGB(189, 101, 94),
            InvoiceType::Telecommunication => RGB(189, 91, 140),
            InvoiceType::Unknown => RGB(159, 113, 189),
        }
    }
}

fn has_true_color_support() -> bool {
    match env::var("COLORTERM") {
        Ok(v) => v == "truecolor",
        Err(_) => false,
    }
}

fn get_prepared_note(invoice: &Invoice) -> String {
    if let Some(note) = invoice.note() {
        let mut buffer: Vec<String> = vec![];

        for l in note.split("<br />") {
            if buffer.len() == 0 {
                buffer.push(l.to_owned())
            } else {
                buffer.push(format!("              {}", l))
            }
        }

        buffer.join("\n")
    } else {
        "".to_owned()
    }
}
