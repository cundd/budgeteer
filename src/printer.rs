use std::env;

use ansi_term::Colour::RGB;
use ansi_term::{Colour, Style};

use crate::calculator::Calculator;
use crate::currency::{currency_data, Currency};
use crate::error::Error;
use crate::filter::Request;
use crate::invoice::invoice_type::InvoiceType;
use crate::invoice::{contains_invoice_in_currency, Invoice};
use crate::month::Month;

pub trait PrinterTrait {
    fn print_errors(&self, errors: Vec<Error>) {
        for ref error in errors {
            self.print_error(error);
        }
    }

    fn print_error(&self, error: &Error);

    fn print_invoices(&self, base_currency: &Currency, invoices: &[Invoice]) {
        for invoice in invoices {
            self.print_invoice(&base_currency, invoice)
        }
        println!()
    }

    fn print_invoice(&self, base_currency: &Currency, invoice: &Invoice);

    fn print_filter_request(&self, filter_request: &Request);

    fn print_sum(&self, base_currency: &Currency, invoices: &[Invoice]);
    fn print_month_sum(&self, month: Month, base_currency: &Currency, invoices: &[Invoice]);
}

pub struct Printer {}

impl Printer {
    pub fn new() -> Self {
        Printer {}
    }

    fn print_type_sum(&self, base_currency: &Currency, invoices: &[Invoice]) {
        // Skip currencies without any Invoice
        let currencies_to_output: Vec<Currency> = currency_data::all()
            .into_iter()
            .filter_map(|(_, currency)| {
                if contains_invoice_in_currency(invoices, &currency) {
                    Some(currency)
                } else {
                    None
                }
            })
            .collect();
        self.print_type_sum_header(&currencies_to_output);

        for invoice_type in InvoiceType::all().iter() {
            let invoice_type = *invoice_type;
            let sum = Calculator::sum_for_type(invoices, invoice_type);

            print_styled_for_type(
                invoice_type,
                &format!(
                    "{:width$}: {:<4} {: >8.2}  ",
                    format!("{}", invoice_type),
                    base_currency.symbol,
                    sum,
                    width = 22
                ),
                false,
                true,
            );

            for currency in &currencies_to_output {
                let sum = Calculator::sum_for_type_and_currency(invoices, invoice_type, currency);
                print_styled_for_type(
                    invoice_type,
                    &format!(" | {:<4} {: >8.2}", currency.symbol, sum,),
                    false,
                    true,
                );
            }

            print_styled_for_type(
                invoice_type,
                &invoice_type.identifier().to_string(),
                true,
                true,
            );
            println!()
        }
    }

    fn print_type_sum_header(&self, currencies_to_output: &[Currency]) {
        print_styled_header(format!(
            "{:width$}: {}",
            "Typ",
            "∑ Basis Währung",
            width = 22
        ));

        for currency in currencies_to_output {
            print_styled_header(format!(" | ∑ {:<4}       ", currency.symbol));
        }

        print_styled_header(" ");
        println!()
    }

    fn print_grand_total(&self, base_currency: &Currency, invoices: &[Invoice]) {
        println!("TOTAL: {} {:.2}", base_currency, Calculator::sum(invoices));
    }
}

impl PrinterTrait for Printer {
    fn print_error(&self, error: &Error) {
        match error {
            Error::LineComment => {}
            Error::LineEmpty => {}
            Error::LineSeparator => {}
            _ => eprintln!("Invoice error {}", error),
        }
    }

    fn print_invoice(&self, base_currency: &Currency, invoice: &Invoice) {
        let note = get_prepared_note(invoice);

        let amount_string = if &invoice.amount().currency() != base_currency {
            match invoice.base_amount() {
                Some(converted_amount) => format!("{} ({})", invoice.amount(), converted_amount),
                None => format!("{}", invoice.amount()),
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
                date, amount_string, invoice_type, note,
            );
        }
    }

    fn print_filter_request(&self, filter_request: &Request) {
        println!("Filter:");
        if filter_request.empty() {
            println!("Keine");
        } else {
            println!("{}", filter_request);
        }
        println!();
    }

    fn print_sum(&self, base_currency: &Currency, invoices: &[Invoice]) {
        self.print_type_sum(base_currency, invoices);
        println!("-----------------------------------------");
        self.print_grand_total(base_currency, invoices);
    }

    fn print_month_sum(&self, month: Month, base_currency: &Currency, invoices: &[Invoice]) {
        if !invoices.is_empty() {
            if let Some(max_type) = Calculator::major_type(invoices) {
                println!(
                    "{:width$}: {} {: >8.2} {}",
                    format!("{}", month),
                    base_currency,
                    Calculator::sum(invoices),
                    style_for_type(max_type, &max_type.identifier().to_string(), true, true),
                    width = 12
                );
                return;
            }
        }

        println!(
            "{:width$}: {} {: >8.2}",
            format!("{}", month),
            base_currency,
            0,
            width = 12
        )
    }
}

fn style_for_type<T: AsRef<str>>(invoice_type: InvoiceType, text: T, fg: bool, bg: bool) -> String {
    if !has_true_color_support() {
        return text.as_ref().to_owned();
    }

    let prepared_multi_line = text
        .as_ref()
        .lines()
        .map(|l| {
            if !l.is_empty() {
                format!(" {} ", l)
            } else {
                "".to_owned()
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

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

fn print_styled_for_type<T: AsRef<str>>(invoice_type: InvoiceType, text: T, fg: bool, bg: bool) {
    print!("{}", style_for_type(invoice_type, text, fg, bg))
}

fn style_header<T: AsRef<str>>(text: T) -> String {
    if !has_true_color_support() {
        return text.as_ref().to_owned();
    }

    let prepared_multi_line = text
        .as_ref()
        .lines()
        .map(|l| {
            if !l.is_empty() {
                format!(" {} ", l)
            } else {
                "".to_owned()
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

    Style::new()
        .fg(Colour::White)
        .on(Colour::Black)
        .paint(prepared_multi_line)
        .to_string()
}

fn print_styled_header<T: AsRef<str>>(text: T) {
    print!("{}", style_header(text))
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
            if buffer.is_empty() {
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
