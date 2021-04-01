use crate::invoice::Invoice;
use crate::error::Error;
use chrono::NaiveDate;
use crate::invoice::invoice_type::InvoiceType;
use crate::invoice::amount::Amount;
use crate::file::LineParts;
use crate::currency::Currency;
use std::cmp::Ordering;

pub struct ParserResult {
    pub invoices: Vec<Invoice>,
    pub errors: Vec<Error>,
}

pub struct InvoiceParser {}

impl InvoiceParser {
    pub fn new() -> Self {
        InvoiceParser {}
    }

    pub fn parse_lines(&self, lines: Vec<LineParts>) -> ParserResult {
        let mut invoices = vec![];
        let mut errors = vec![];
        for parts in lines {
            match self.build_from_vec(parts.iter().map(String::as_str).collect()) {
                Ok(invoice) => { invoices.push(invoice) }
                Err(error) => { errors.push(error) }
            }
        }
        invoices.sort_by(|a, b| {
            if a.date > b.date {
                Ordering::Greater
            } else if a.date < b.date {
                Ordering::Less
            } else if a.amount.value() > b.amount.value() {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });

        ParserResult {
            invoices,
            errors,
        }
    }

    pub fn build_from_vec(&self, parts: Vec<&str>) -> Result<Invoice, Error> {
        let string_vec: Vec<String> =
            parts.into_iter()
                .map(String::from)
                .collect();

        let date = self.parse_date(&string_vec)?;
        let raw_currency = self.get_vec_part_or_error(
            &string_vec,
            1,
            "Could not read currency from line",
        )?;
        let currency = Currency::from_string(&raw_currency)?;
        let amount = Amount::new(
            self.parse_amount(&string_vec)?,
            &currency,
        );

        let invoice_type = InvoiceType::from_str(&string_vec.get(3).unwrap_or(&"".to_string()));
        let note = self.get_vec_part(&string_vec, 4);
        let base_amount = None;

        Ok(Invoice::new(date, amount, base_amount, invoice_type, note))
    }

    fn parse_date(&self, string_vec: &Vec<String>) -> Result<NaiveDate, Error> {
        match self.get_vec_part_or_error(&string_vec, 0, "Could not read date from line") {
            Ok(s) => match NaiveDate::parse_from_str(&s, "%d.%m.%Y") {
                Ok(d) => Ok(d),
                Err(e) => Err(Error::ParseError(format!(
                    "Could not parse date '{}': {}",
                    s,
                    &e
                )))
            }
            Err(e) => Err(e)
        }
    }

    fn parse_amount(&self, string_vec: &Vec<String>) -> Result<f64, Error> {
        let amount_string = self.get_vec_part_or_error(
            &string_vec,
            2,
            "Could not read amount from line",
        )?;

        match amount_string
            .trim()
            .replace(',', ".") // Replace ',' with '.'
            .parse::<f64>() {
            Ok(f) => Ok(f),
            Err(e) => Err(Error::ParseError(format!("Could not parse amount '{}': {}", amount_string, e)))
        }
    }

    fn get_vec_part(&self, string_vec: &Vec<String>, index: usize) -> Option<String> {
        match string_vec.get(index) {
            Some(s) => Some(s.to_owned()),
            None => None,
        }
    }

    fn get_vec_part_or_error(&self, string_vec: &Vec<String>, index: usize, msg: &str) -> Result<String, Error> {
        match string_vec.get(index) {
            Some(s) => Ok(s.trim().to_owned()),
            None => Err(Error::ParseError(msg.to_owned())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_from_vec() {
        let invoice_parser = InvoiceParser::new();
        let result = invoice_parser.build_from_vec(vec!["15.02.2019", "€", "66.60", "T", "Gas station"]);
        match result {
            Ok(i) => {
                assert_eq!(i.invoice_type, InvoiceType::Gas);
                assert_eq!(i.amount, Amount::new(66.6, &Currency::eur()));
                assert_eq!(i.date, NaiveDate::from_ymd(2019, 02, 15));
                assert!(i.note.is_some());
                assert_eq!(i.note.unwrap(), "Gas station");
            }
            Err(e) => panic!("{}", e)
        };
    }
}
