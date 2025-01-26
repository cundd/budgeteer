use crate::{
    printer::Printer, printer::PrinterTrait, transaction::transaction_type::TransactionType,
};

pub fn show_types(printer: &mut Printer) {
    printer.print_header("Available types:");
    for transaction_type in &TransactionType::all_known() {
        printer.println(format!(
            "- {}: {}",
            transaction_type.identifier(),
            transaction_type
        ));
    }
}
