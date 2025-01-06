mod file_reader;
mod invoice_parser;

pub use self::file_reader::FileReader;
pub use self::invoice_parser::InvoiceParser;
use crate::error::Error;
use invoice_parser::ParserResult;
use std::path::Path;

pub fn get_invoices<P: AsRef<Path>>(input_file: P) -> Result<ParserResult, Error> {
    let lines = FileReader::read(input_file)?;
    let parser = InvoiceParser::new();
    Ok(parser.parse_lines(lines.lines))
}
