mod file_reader;
mod transaction_parser;

pub use self::file_reader::FileReader;
pub use self::transaction_parser::TransactionParser;
use super::ImportResult;
use crate::error::Error;
use std::path::Path;

pub fn get_transactions<P: AsRef<Path>>(input_file: P) -> Result<ImportResult, Error> {
    let lines = FileReader::read(input_file)?;
    let parser = TransactionParser::new();
    Ok(parser.parse_lines(lines.lines))
}
