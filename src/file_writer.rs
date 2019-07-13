use invoice::Invoice;
use error::{Res, Error};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::fs;

pub struct FileWriter {}

impl FileWriter {
    pub fn write_invoice(path: &str, invoice: &Invoice) -> Res<()> {
        let mut file = OpenOptions::new().create(true).append(true).open(path)?;

        let line = format!(
            "| {} | {} | {} | {} | {} | \n",
            invoice.date().format("%d.%m.%Y"),
            invoice.amount().currency(),
            invoice.amount().value(),
            invoice.invoice_type().identifier(),
            invoice.note().unwrap_or("".to_owned())
        );

        Ok(file.write_all(line.as_bytes())?)
    }

    pub fn check_output_path(path_str: &str) -> Res<()> {
        let path = fs::canonicalize(path_str)?;
        if path.is_dir() {
            return Err(Error::FileIO("Output path must not be a directory".to_owned()));
        }

        if let Some(parent) = path.parent() {
            if !parent.exists() {
                return Err(Error::FileIO("Output path parent directory does not exist".to_owned()));
            }
        }
        Ok(())
    }
}
