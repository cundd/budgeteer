use crossterm::tty::IsTty;
use crossterm::QueueableCommand;

use super::{style_for_type, Printer, PrinterTrait};
use crate::calculator::Calculator;
use crate::currency::Currency;
use crate::transaction::transaction_type::{TransactionType, NUMBER_OF_TYPES};
use crate::transaction::Transaction;
use std::collections::HashMap;
use std::io::Write;
use std::thread;
use std::time::Duration;

/// Print the "bar chart"
pub(super) fn print_bar_chart(
    printer: &mut Printer,
    _base_currency: &Currency,
    transactions: &[Transaction],
) -> Result<(), std::io::Error> {
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
        let width = (percent.round() as usize) / (100 / CHART_WIDTH);

        printer.print(style_for_type(
            transaction_type,
            " ".repeat(width),
            false,
            true,
        ));
    }
    printer.print_newline();

    if printer.output.is_tty() {
        let mut scale = 0.0;
        loop {
            render_bars(printer, sum_map.clone(), scale);
            scale += 0.1;
            thread::sleep(Duration::from_millis(100));

            printer.output.queue(crossterm::cursor::MoveToColumn(0))?;
            printer
                .output
                .queue(crossterm::cursor::MoveUp(NUMBER_OF_TYPES as u16))?;
            printer.output.flush()?;
            if scale > 1.0 {
                break;
            }
        }
    }

    render_bars(printer, sum_map, 1.0);
    Ok(())
}

type PercentMap = HashMap<TransactionType, f64>;
const CHART_WIDTH: usize = 50;

fn render_bars(printer: &mut Printer, sum_map: PercentMap, scale: f64) {
    for transaction_type in TransactionType::all() {
        let percent = sum_map[&transaction_type];

        let width = ((scale * percent).ceil() as usize) / (100 / CHART_WIDTH);
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
