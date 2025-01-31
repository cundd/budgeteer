use super::{style_for_type, PrinterTrait};
use crate::calculator::Calculator;
use crate::currency::Currency;
use crate::transaction::transaction_type::TransactionType;
use crate::transaction::Transaction;
use std::collections::HashMap;

/// Print the "bar chart"
pub(super) fn print_bar_chart<P: PrinterTrait>(
    printer: &mut P,
    _base_currency: &Currency,
    transactions: &[Transaction],
) {
    let chart_width = 50;
    let total: f64 = Calculator::sum(transactions);

    let mut sum_map = HashMap::new();
    for transaction_type in TransactionType::all() {
        let sum = Calculator::sum_for_type(transactions, transaction_type);

        let percent = if total != 0.0 {
            100.0 * sum / total
        } else {
            0.0
        };

        sum_map.insert(transaction_type, percent);
    }

    // Use `TransactionType::all()` again, to maintain the sorting
    for transaction_type in TransactionType::all() {
        let percent = sum_map[&transaction_type];
        let width = (percent.round() as usize) / (100 / chart_width);

        printer.print(style_for_type(
            transaction_type,
            " ".repeat(width),
            false,
            true,
        ));
    }
    printer.print_newline();

    // Use `TransactionType::all()` again, to maintain the sorting
    for transaction_type in TransactionType::all() {
        let percent = sum_map[&transaction_type];

        let width = (percent.ceil() as usize) / (100 / chart_width);
        let percent_formatted = if percent != 0.0 {
            format!("{:.2}%", percent)
        } else {
            "0%".to_string()
        };
        let text = format!("{}: {}", transaction_type, percent_formatted);
        if text.len() <= width {
            printer.print(style_for_type(
                transaction_type,
                format!(" {:<width$}", text),
                false,
                true,
            ));
        } else {
            printer.print(style_for_type(
                transaction_type,
                format!(" {}", &text[..width]),
                false,
                true,
            ));
            printer.print(&text[width..]);
        }

        printer.print_newline();
    }
}
