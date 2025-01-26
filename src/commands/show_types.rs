use crate::{invoice::invoice_type::InvoiceType, printer::Printer, printer::PrinterTrait};

pub fn show_types(printer: &mut Printer) {
    printer.print_header("Available types:");
    for invoice_type in &InvoiceType::all_known() {
        printer.println(format!("- {}: {}", invoice_type.identifier(), invoice_type));
    }
}
