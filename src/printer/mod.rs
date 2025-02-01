mod chart;

use crate::calculator::Calculator;
use crate::currency::{currency_data, Currency};
use crate::filter::Request;
use crate::month::Month;
use crate::transaction::transaction_type::TransactionType;
use crate::transaction::{contains_transaction_in_currency, Transaction};
use chart::print_bar_chart;
use crossterm::style::Color;
use crossterm::style::Stylize;
use std::env;
use std::io::{stdout, Write};

static STDOUT_WRITE_ERROR: &str = "Could not write to stdout";

pub trait PrinterTrait {
    fn print_transactions(&mut self, base_currency: &Currency, transactions: &[Transaction]) {
        for transaction in transactions {
            self.print_transaction(base_currency, transaction)
        }
        self.print_newline()
    }

    fn print_transaction(&mut self, base_currency: &Currency, transaction: &Transaction);

    fn print_filter_request(&mut self, filter_request: &Request);

    fn print_sum(&mut self, base_currency: &Currency, transactions: &[Transaction]);
    fn print_month_sum(
        &mut self,
        month: Month,
        base_currency: &Currency,
        transactions: &[Transaction],
    );
    fn print_header<S: AsRef<str>>(&mut self, text: S);
    fn print_subheader<S: AsRef<str>>(&mut self, text: S);
    fn print_warning<S: AsRef<str>>(&mut self, text: S);
    fn print_newline(&mut self);
    fn print<S: AsRef<str>>(&mut self, text: S);
    fn println<S: AsRef<str>>(&mut self, text: S);
}

pub struct Printer {
    output: std::io::Stdout,
}

impl Printer {
    pub fn new() -> Self {
        Printer { output: stdout() }
    }
    fn print_type_sum(&mut self, base_currency: &Currency, transactions: &[Transaction]) {
        // Skip currencies without any Transaction
        let currencies_to_output: Vec<Currency> = currency_data::all()
            .into_iter()
            .filter_map(|(_, currency)| {
                if contains_transaction_in_currency(transactions, &currency) {
                    Some(currency)
                } else {
                    None
                }
            })
            .collect();

        self.print_type_sum_header(&currencies_to_output);

        for transaction_type in TransactionType::all() {
            let sum = Calculator::sum_for_type(transactions, transaction_type);

            self.print(style_for_type(
                transaction_type,
                format!(
                    " {:width$}│ {:<4} {: >10.2} ",
                    format!("{}", transaction_type),
                    base_currency.symbol,
                    sum,
                    width = 22
                ),
                false,
                true,
            ));

            for currency in &currencies_to_output {
                let sum =
                    Calculator::sum_for_type_and_currency(transactions, transaction_type, currency);
                self.print(style_for_type(
                    transaction_type,
                    format!("│ {:<3} {: >9.2} ", currency.symbol, sum),
                    false,
                    true,
                ));
            }

            self.print(style_for_type(
                transaction_type,
                format!(" {} ", transaction_type.identifier()),
                true,
                true,
            ));
            self.print_newline();
        }
    }

    fn print_type_sum_header(&mut self, currencies_to_output: &[Currency]) {
        self.print(style_header(format!(
            " {:width$}│ {} ",
            "Typ",
            "∑ Basis Währung",
            width = 22
        )));

        for currency in currencies_to_output {
            self.print(style_header(format!("│ ∑ {:<12}", currency.symbol)));
        }

        self.print(style_header("   "));
        self.print_newline();
    }

    fn print_grand_total(&mut self, base_currency: &Currency, transactions: &[Transaction]) {
        let totals = Calculator::totals(transactions);

        self.println(
            format!("Income:   {} {: >10.2}", base_currency, totals.income)
                .with(color_for_income())
                .to_string(),
        );
        self.println(
            format!("Expenses: {} {: >10.2}", base_currency, totals.expenses)
                .with(color_for_expenses())
                .to_string(),
        );

        let total_formatted =
            format!("{} {: >10.2}", base_currency, totals.total).with(if totals.total > 0.0 {
                color_for_income()
            } else {
                color_for_expenses()
            });
        self.print_header(format!("TOTAL:    {}", total_formatted));
    }

    fn terminal_width(&self) -> usize {
        crossterm::terminal::size().map_or(50, |s| s.0) as usize
    }
}

impl PrinterTrait for Printer {
    fn print_transaction(&mut self, base_currency: &Currency, transaction: &Transaction) {
        let note = get_prepared_note(transaction);

        let amount_string = if &transaction.amount().currency() != base_currency {
            match transaction.base_amount() {
                Some(converted_amount) => {
                    format!("{} ({})", transaction.amount(), converted_amount)
                }
                None => format!("{}", transaction.amount()),
            }
        } else {
            format!("{}", transaction.amount())
        };

        let transaction_type = transaction.transaction_type();
        let date = transaction.date().format("%A %d.%m.%Y");

        writeln!(
            self.output,
            r#"{ } Datum   : {}
Betrag      : {}
Typ         : {}
Notiz       : {}
"#,
            style_for_type(transaction_type, "   ", false, true),
            date,
            amount_string,
            transaction_type,
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

    fn print_sum(&mut self, base_currency: &Currency, transactions: &[Transaction]) {
        let terminal_width = self.terminal_width();
        let header_width = terminal_width - 1;

        self.print_type_sum(base_currency, transactions);
        self.print_newline();
        self.print_grand_total(base_currency, transactions);
        self.println("─".repeat(terminal_width));
        self.print_newline();

        self.println(style_header(format!(" {:<header_width$}", "Chart")));
        let _ = print_bar_chart(self, base_currency, transactions);
    }

    fn print_month_sum(
        &mut self,
        month: Month,
        base_currency: &Currency,
        transactions: &[Transaction],
    ) {
        if !transactions.is_empty() {
            let major_types = Calculator::major_types(transactions);
            writeln!(
                self.output,
                "{:width$}: {} {: >8.2} {}",
                format!("{}", month),
                base_currency,
                Calculator::sum(transactions),
                style_for_type(
                    major_types.max_expenses.transaction_type,
                    format!(
                        " {} {: >8.2} ",
                        major_types.max_expenses.transaction_type.identifier(),
                        major_types.max_expenses.value
                    ),
                    true,
                    true,
                ),
                width = 12
            )
            .expect(STDOUT_WRITE_ERROR);
            return;
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

    fn print<S: AsRef<str>>(&mut self, text: S) {
        write!(self.output, "{}", text.as_ref()).expect(STDOUT_WRITE_ERROR)
    }

    fn println<S: AsRef<str>>(&mut self, text: S) {
        writeln!(self.output, "{}", text.as_ref()).expect(STDOUT_WRITE_ERROR)
    }

    fn print_header<S: AsRef<str>>(&mut self, text: S) {
        let terminal_width = self.terminal_width();
        self.println(style_header(format!("{:<terminal_width$}", text.as_ref())))
    }

    fn print_subheader<S: AsRef<str>>(&mut self, text: S) {
        self.println(text.as_ref())
    }

    fn print_warning<S: AsRef<str>>(&mut self, text: S) {
        let styled_text = text.as_ref().with(Color::Yellow).to_string();
        self.println(styled_text)
    }
}

fn style_for_type<T: Into<String>>(
    transaction_type: TransactionType,
    text: T,
    fg: bool,
    bg: bool,
) -> String {
    let text = text.into();

    if !fg && !bg {
        return text;
    }

    if fg && bg {
        text.with(color_for_type(transaction_type, false))
            .on(color_for_type(transaction_type, true))
    } else if fg {
        text.with(color_for_type(transaction_type, false))
    } else {
        text.with(Color::Rgb { r: 0, g: 0, b: 0 })
            .on(color_for_type(transaction_type, true))
    }
    .to_string()
}

fn style_header<T: Into<String>>(text: T) -> String {
    text.into().with(Color::White).on(Color::Black).to_string()
}

fn color_for_type(transaction_type: TransactionType, light: bool) -> Color {
    if !has_true_color_support() {
        return if light {
            match transaction_type {
                TransactionType::Body => Color::AnsiValue(81),
                TransactionType::Car => Color::AnsiValue(9),
                TransactionType::Clothes => Color::AnsiValue(10),
                TransactionType::Eat => Color::AnsiValue(11),
                TransactionType::Gas => Color::AnsiValue(12),
                TransactionType::Fun => Color::AnsiValue(13),
                TransactionType::Health => Color::AnsiValue(14),
                TransactionType::Home => Color::AnsiValue(73),
                TransactionType::Telecommunication => Color::AnsiValue(27),
                TransactionType::Donation => Color::AnsiValue(207),
                TransactionType::Unknown => Color::AnsiValue(57),
            }
        } else {
            match transaction_type {
                TransactionType::Body => Color::AnsiValue(39),
                TransactionType::Car => Color::AnsiValue(1),
                TransactionType::Clothes => Color::AnsiValue(2),
                TransactionType::Eat => Color::AnsiValue(3),
                TransactionType::Gas => Color::AnsiValue(4),
                TransactionType::Fun => Color::AnsiValue(5),
                TransactionType::Health => Color::AnsiValue(6),
                TransactionType::Home => Color::AnsiValue(17),
                TransactionType::Telecommunication => Color::AnsiValue(17),
                TransactionType::Donation => Color::AnsiValue(171),
                TransactionType::Unknown => Color::AnsiValue(53),
            }
        };
    }
    if light {
        match transaction_type {
            TransactionType::Body => Color::Rgb {
                r: 56,
                g: 255,
                b: 219,
            },
            TransactionType::Car => Color::Rgb {
                r: 112,
                g: 255,
                b: 81,
            },
            TransactionType::Clothes => Color::Rgb {
                r: 177,
                g: 255,
                b: 79,
            },
            TransactionType::Eat => Color::Rgb {
                r: 225,
                g: 255,
                b: 79,
            },
            TransactionType::Gas => Color::Rgb {
                r: 255,
                g: 237,
                b: 61,
            },
            TransactionType::Fun => Color::Rgb {
                r: 255,
                g: 200,
                b: 53,
            },
            TransactionType::Health => Color::Rgb {
                r: 255,
                g: 173,
                b: 45,
            },
            TransactionType::Home => Color::Rgb {
                r: 255,
                g: 136,
                b: 126,
            },
            TransactionType::Telecommunication => Color::Rgb {
                r: 255,
                g: 120,
                b: 186,
            },
            TransactionType::Donation => Color::Rgb {
                r: 215,
                g: 151,
                b: 255,
            },
            TransactionType::Unknown => Color::Rgb {
                r: 94,
                g: 228,
                b: 255,
            },
        }
    } else {
        match transaction_type {
            TransactionType::Body => Color::Rgb {
                r: 17,
                g: 204,
                b: 170,
            },
            TransactionType::Car => Color::Rgb {
                r: 84,
                g: 189,
                b: 60,
            },
            TransactionType::Clothes => Color::Rgb {
                r: 132,
                g: 189,
                b: 58,
            },
            TransactionType::Eat => Color::Rgb {
                r: 167,
                g: 189,
                b: 58,
            },
            TransactionType::Gas => Color::Rgb {
                r: 189,
                g: 174,
                b: 45,
            },
            TransactionType::Fun => Color::Rgb {
                r: 189,
                g: 146,
                b: 40,
            },
            TransactionType::Health => Color::Rgb {
                r: 189,
                g: 127,
                b: 34,
            },
            TransactionType::Home => Color::Rgb {
                r: 189,
                g: 101,
                b: 94,
            },
            TransactionType::Telecommunication => Color::Rgb {
                r: 189,
                g: 91,
                b: 140,
            },
            TransactionType::Donation => Color::Rgb {
                r: 159,
                g: 113,
                b: 189,
            },
            TransactionType::Unknown => Color::Rgb {
                r: 12,
                g: 170,
                b: 201,
            },
        }
    }
}
fn color_for_income() -> Color {
    if !has_true_color_support() {
        Color::Green
    } else {
        Color::Rgb {
            r: 6,
            g: 168,
            b: 59,
        }
    }
}
fn color_for_expenses() -> Color {
    if !has_true_color_support() {
        Color::Red
    } else {
        Color::Rgb {
            r: 232,
            g: 53,
            b: 32,
        }
    }
}

fn has_true_color_support() -> bool {
    match env::var("COLORTERM") {
        Ok(v) => v == "truecolor",
        Err(_) => false,
    }
}

fn get_prepared_note(transaction: &Transaction) -> String {
    if let Some(note) = transaction.note() {
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
