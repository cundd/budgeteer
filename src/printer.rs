use crate::calculator::Calculator;
use crate::currency::{currency_data, Currency};
use crate::filter::Request;
use crate::invoice::invoice_type::InvoiceType;
use crate::invoice::{contains_invoice_in_currency, Invoice};
use crate::month::Month;
use crossterm::style::Color;
use crossterm::style::Stylize;
use std::collections::HashMap;
use std::env;
use std::io::{stdout, Write};

static STDOUT_WRITE_ERROR: &str = "Could not write to stdout";

pub trait PrinterTrait {
    fn print_invoices(&mut self, base_currency: &Currency, invoices: &[Invoice]) {
        for invoice in invoices {
            self.print_invoice(base_currency, invoice)
        }
        self.print_newline()
    }

    fn print_invoice(&mut self, base_currency: &Currency, invoice: &Invoice);

    fn print_filter_request(&mut self, filter_request: &Request);

    fn print_sum(&mut self, base_currency: &Currency, invoices: &[Invoice]);
    fn print_month_sum(&mut self, month: Month, base_currency: &Currency, invoices: &[Invoice]);
    fn print_newline(&mut self);
}

pub struct Printer {
    output: std::io::Stdout,
}

impl Printer {
    pub fn new() -> Self {
        Printer { output: stdout() }
    }

    fn print<S: AsRef<str>>(&mut self, text: S) {
        write!(self.output, "{}", text.as_ref()).expect(STDOUT_WRITE_ERROR)
    }

    fn println<S: AsRef<str>>(&mut self, text: S) {
        writeln!(self.output, "{}", text.as_ref()).expect(STDOUT_WRITE_ERROR)
    }
    fn print_type_sum(&mut self, base_currency: &Currency, invoices: &[Invoice]) {
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

        for invoice_type in InvoiceType::all() {
            let sum = Calculator::sum_for_type(invoices, invoice_type);

            self.print(style_for_type(
                invoice_type,
                format!(
                    "{:width$}│ {:<4} {: >9.2}",
                    format!("{}", invoice_type),
                    base_currency.symbol,
                    sum,
                    width = 22
                ),
                false,
                true,
                true,
            ));

            for currency in &currencies_to_output {
                let sum = Calculator::sum_for_type_and_currency(invoices, invoice_type, currency);
                self.print(style_for_type(
                    invoice_type,
                    format!("│ {:<2} {: >8.2}", currency.symbol, sum),
                    false,
                    true,
                    true,
                ));
            }

            self.print(style_for_type(
                invoice_type,
                invoice_type.identifier().to_string(),
                true,
                true,
                true,
            ));
            self.print_newline();
        }

        self.print_newline();
        self.println(style_header(format!("{:<50}", "Chart")));
        self.print_bar_chart(base_currency, invoices);
    }

    /// Print the "bar chart"
    fn print_bar_chart(&mut self, _base_currency: &Currency, invoices: &[Invoice]) {
        let mut sum_map = HashMap::new();
        for invoice_type in InvoiceType::all() {
            let sum = Calculator::sum_for_type(invoices, invoice_type);
            sum_map.insert(invoice_type, sum);
        }

        let total: f64 = Calculator::sum(invoices);

        // Use `InvoiceType::all()` again, to maintain the sorting
        for invoice_type in InvoiceType::all() {
            let sum = sum_map[&invoice_type];
            let percent = 100.0 * sum / total;

            let width = (percent.ceil() / 2.0) as usize;
            let text = format!("{}: {:.2}%", invoice_type, percent);
            if text.len() <= width {
                self.print(style_for_type(
                    invoice_type,
                    format!(" {:<width$}", text),
                    false,
                    true,
                    false,
                ));
            } else {
                self.print(style_for_type(
                    invoice_type,
                    format!(" {}", &text[..width]),
                    false,
                    true,
                    false,
                ));
                write!(self.output, "{}", &text[width..]).expect(STDOUT_WRITE_ERROR);
            }

            self.print_newline();
        }
    }

    fn print_type_sum_header(&mut self, currencies_to_output: &[Currency]) {
        self.print(style_header(format!(
            "{:width$}│ {}",
            "Typ",
            "∑ Basis Währung",
            width = 22
        )));

        for currency in currencies_to_output {
            self.print(style_header(format!("│ ∑ {:<5}     ", currency.symbol)));
        }

        self.print(style_header("   "));
        self.print_newline();
    }

    fn print_grand_total(&mut self, base_currency: &Currency, invoices: &[Invoice]) {
        writeln!(
            self.output,
            "TOTAL: {} {:.2}",
            base_currency,
            Calculator::sum(invoices)
        )
        .expect(STDOUT_WRITE_ERROR);
    }
}

impl PrinterTrait for Printer {
    fn print_invoice(&mut self, base_currency: &Currency, invoice: &Invoice) {
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

        writeln!(
            self.output,
            r#"
{ } Datum   : {}
Betrag      : {}
Typ         : {}
Notiz       : {}"#,
            style_for_type(invoice_type, " ", false, true, true),
            date,
            amount_string,
            invoice_type,
            note,
        )
        .expect(STDOUT_WRITE_ERROR);
    }

    fn print_filter_request(&mut self, filter_request: &Request) {
        self.println("Filter:");
        if filter_request.empty() {
            self.println("Keine");
        } else {
            self.println(filter_request.to_string());
        }
        self.print_newline();
    }

    fn print_sum(&mut self, base_currency: &Currency, invoices: &[Invoice]) {
        self.print_type_sum(base_currency, invoices);
        self.println("-----------------------------------------");
        self.print_grand_total(base_currency, invoices);
    }

    fn print_month_sum(&mut self, month: Month, base_currency: &Currency, invoices: &[Invoice]) {
        if !invoices.is_empty() {
            if let Some(max_type) = Calculator::major_type(invoices) {
                writeln!(
                    self.output,
                    "{:width$}: {} {: >8.2} {}",
                    format!("{}", month),
                    base_currency,
                    Calculator::sum(invoices),
                    style_for_type(
                        max_type,
                        max_type.identifier().to_string(),
                        true,
                        true,
                        true
                    ),
                    width = 12
                )
                .expect(STDOUT_WRITE_ERROR);
                return;
            }
        }

        writeln!(
            self.output,
            "{:width$}: {} {: >8.2}",
            format!("{}", month),
            base_currency,
            0,
            width = 12
        )
        .expect(STDOUT_WRITE_ERROR);
    }

    fn print_newline(&mut self) {
        self.println("")
    }
}

fn style_for_type<T: AsRef<str>>(
    invoice_type: InvoiceType,
    text: T,
    fg: bool,
    bg: bool,
    add_space: bool,
) -> String {
    let prepared_multi_line = text
        .as_ref()
        .lines()
        .map(|l| {
            if l.is_empty() {
                "".to_owned()
            } else if add_space {
                format!(" {} ", l)
            } else {
                l.to_owned()
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

    if !fg && !bg {
        return prepared_multi_line;
    }

    if fg && bg {
        prepared_multi_line
            .with(color_for_type(invoice_type, false))
            .on(color_for_type(invoice_type, true))
    } else if fg {
        prepared_multi_line.with(color_for_type(invoice_type, false))
    } else {
        prepared_multi_line.on(color_for_type(invoice_type, true))
    }
    .to_string()
}

fn style_header<T: AsRef<str>>(text: T) -> String {
    let prepared_multi_line = text
        .as_ref()
        .lines()
        .map(|l| {
            if !l.is_empty() {
                format!(" {}", l)
            } else {
                "".to_owned()
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

    prepared_multi_line
        .with(Color::White)
        .on(Color::Black)
        .to_string()
}

fn color_for_type(invoice_type: InvoiceType, light: bool) -> Color {
    if !has_true_color_support() {
        return if light {
            match invoice_type {
                InvoiceType::Car => Color::AnsiValue(9),
                InvoiceType::Clothes => Color::AnsiValue(10),
                InvoiceType::Eat => Color::AnsiValue(11),
                InvoiceType::Gas => Color::AnsiValue(12),
                InvoiceType::Fun => Color::AnsiValue(13),
                InvoiceType::Health => Color::AnsiValue(14),
                InvoiceType::Home => Color::AnsiValue(73),
                InvoiceType::Telecommunication => Color::AnsiValue(27),
                InvoiceType::Unknown => Color::AnsiValue(57),
            }
        } else {
            match invoice_type {
                InvoiceType::Car => Color::AnsiValue(1),
                InvoiceType::Clothes => Color::AnsiValue(2),
                InvoiceType::Eat => Color::AnsiValue(3),
                InvoiceType::Gas => Color::AnsiValue(4),
                InvoiceType::Fun => Color::AnsiValue(5),
                InvoiceType::Health => Color::AnsiValue(6),
                InvoiceType::Home => Color::AnsiValue(17),
                InvoiceType::Telecommunication => Color::AnsiValue(17),
                InvoiceType::Unknown => Color::AnsiValue(53),
            }
        };
    }
    if light {
        match invoice_type {
            InvoiceType::Car => Color::Rgb {
                r: 112,
                g: 255,
                b: 81,
            },
            InvoiceType::Clothes => Color::Rgb {
                r: 177,
                g: 255,
                b: 79,
            },
            InvoiceType::Eat => Color::Rgb {
                r: 225,
                g: 255,
                b: 79,
            },
            InvoiceType::Gas => Color::Rgb {
                r: 255,
                g: 237,
                b: 61,
            },
            InvoiceType::Fun => Color::Rgb {
                r: 255,
                g: 200,
                b: 53,
            },
            InvoiceType::Health => Color::Rgb {
                r: 255,
                g: 173,
                b: 45,
            },
            InvoiceType::Home => Color::Rgb {
                r: 255,
                g: 136,
                b: 126,
            },
            InvoiceType::Telecommunication => Color::Rgb {
                r: 255,
                g: 120,
                b: 186,
            },
            InvoiceType::Unknown => Color::Rgb {
                r: 215,
                g: 151,
                b: 255,
            },
        }
    } else {
        match invoice_type {
            InvoiceType::Car => Color::Rgb {
                r: 84,
                g: 189,
                b: 60,
            },
            InvoiceType::Clothes => Color::Rgb {
                r: 132,
                g: 189,
                b: 58,
            },
            InvoiceType::Eat => Color::Rgb {
                r: 167,
                g: 189,
                b: 58,
            },
            InvoiceType::Gas => Color::Rgb {
                r: 189,
                g: 174,
                b: 45,
            },
            InvoiceType::Fun => Color::Rgb {
                r: 189,
                g: 146,
                b: 40,
            },
            InvoiceType::Health => Color::Rgb {
                r: 189,
                g: 127,
                b: 34,
            },
            InvoiceType::Home => Color::Rgb {
                r: 189,
                g: 101,
                b: 94,
            },
            InvoiceType::Telecommunication => Color::Rgb {
                r: 189,
                g: 91,
                b: 140,
            },
            InvoiceType::Unknown => Color::Rgb {
                r: 159,
                g: 113,
                b: 189,
            },
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
