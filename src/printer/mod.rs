mod chart;
mod color;

use crate::calculator::{Calculator, Totals};
use crate::currency::{currency_data, Currency};
use crate::filter::Request;
use crate::month::Month;
use crate::transaction::transaction_type::TransactionType;
use crate::transaction::{contains_transaction_in_currency, Transaction};
use chart::print_bar_chart;
use color::{color_for_expenses, color_for_income, color_for_type};
use crossterm::style::Color;
use crossterm::style::Stylize;
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
            let Totals { expenses, .. } =
                Calculator::totals_for_type(transactions, transaction_type);

            self.print(style_for_type(
                transaction_type,
                format!(
                    " {:width$}│ {:<4} {: >10.2} ",
                    format!("{}", transaction_type),
                    base_currency.symbol,
                    expenses,
                    width = 25
                ),
                false,
                true,
            ));

            for currency in &currencies_to_output {
                let Totals { expenses, .. } = Calculator::totals_for_type_and_currency(
                    transactions,
                    transaction_type,
                    currency,
                );
                self.print(style_for_type(
                    transaction_type,
                    format!("│ {:<3} {: >9.2} ", currency.symbol, expenses),
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
        self.print_header("Expenses per type");
        self.print_newline();
        self.print(style_header(format!(
            " {:width$}│ {} ",
            "Typ",
            "∑ Basis Währung",
            width = 25
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
                "{:width$}: {} {: >9.2} {}",
                month.to_string(),
                base_currency,
                Calculator::sum(transactions),
                style_for_type(
                    major_types.max_expenses.transaction_type,
                    format!(
                        " {} {: >9.2} ",
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
            month.to_string(),
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
