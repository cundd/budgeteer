use crate::currency::Currency;
use crate::error::Error;
use crate::import::markdown::file_reader::LineParts;
use crate::import::ImportResult;
use crate::transaction::amount::Amount;
use crate::transaction::transaction_type::TransactionType;
use crate::transaction::Transaction;
use chrono::NaiveDate;
use std::cmp::Ordering;
use std::str::FromStr;

pub struct TransactionParser {}

impl TransactionParser {
    pub fn new() -> Self {
        TransactionParser {}
    }

    pub fn parse_lines(&self, lines: Vec<LineParts>) -> ImportResult {
        let mut transactions = vec![];
        let mut errors = vec![];
        for parts in lines {
            match self.build_from_vec(parts.iter().map(String::as_str).collect()) {
                Ok(transaction) => transactions.push(transaction),
                Err(error) => errors.push(error),
            }
        }
        transactions.sort_by(|a, b| {
            if a.date() > b.date() {
                Ordering::Greater
            } else if a.date() < b.date() {
                Ordering::Less
            } else if a.amount().value() > b.amount().value() {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });

        ImportResult {
            transactions,
            errors,
        }
    }

    pub fn build_from_vec(&self, parts: Vec<&str>) -> Result<Transaction, Error> {
        let string_vec: Vec<String> = parts.into_iter().map(String::from).collect();

        let date = self.parse_date(&string_vec)?;
        let raw_currency =
            self.get_vec_part_or_error(&string_vec, 1, "Could not read currency from line")?;
        let currency = Currency::from_str(&raw_currency)?;
        let amount = Amount::new(-1.0 * self.parse_amount(&string_vec)?, currency);

        let transaction_type =
            TransactionType::from_str(string_vec.get(3).unwrap_or(&"".to_string()));
        let note = self.get_vec_part(&string_vec, 4);
        let base_amount = None;

        Ok(Transaction::new(
            date,
            amount,
            base_amount,
            transaction_type,
            note,
        ))
    }

    fn parse_date(&self, string_vec: &[String]) -> Result<NaiveDate, Error> {
        match self.get_vec_part_or_error(string_vec, 0, "Could not read date from line") {
            Ok(s) => match NaiveDate::parse_from_str(&s, "%d.%m.%Y") {
                Ok(d) => Ok(d),
                Err(e) => Err(Error::Parse(format!(
                    "Could not parse date '{}': {}",
                    s, &e
                ))),
            },
            Err(e) => Err(e),
        }
    }

    fn parse_amount(&self, string_vec: &[String]) -> Result<f64, Error> {
        let amount_string =
            self.get_vec_part_or_error(string_vec, 2, "Could not read amount from line")?;

        match amount_string
            .trim()
            .replace(',', ".") // Replace ',' with '.'
            .parse::<f64>()
        {
            Ok(f) => Ok(f),
            Err(e) => Err(Error::Parse(format!(
                "Could not parse amount '{}': {}",
                amount_string, e
            ))),
        }
    }

    fn get_vec_part(&self, string_vec: &[String], index: usize) -> Option<String> {
        string_vec.get(index).map(|s| s.to_owned())
    }

    fn get_vec_part_or_error(
        &self,
        string_vec: &[String],
        index: usize,
        msg: &str,
    ) -> Result<String, Error> {
        match string_vec.get(index) {
            Some(s) => Ok(s.trim().to_owned()),
            None => Err(Error::Parse(msg.to_owned())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_from_vec() {
        let transaction_parser = TransactionParser::new();
        let result =
            transaction_parser.build_from_vec(vec!["15.02.2019", "€", "66.60", "T", "Gas station"]);
        match result {
            Ok(i) => {
                assert_eq!(i.transaction_type(), TransactionType::Gas);
                assert_eq!(i.amount(), Amount::new(-66.6, Currency::eur()));
                assert_eq!(i.date(), NaiveDate::from_ymd_opt(2019, 2, 15).unwrap());
                assert!(i.note().is_some());
                assert_eq!(i.note().unwrap(), "Gas station");
            }
            Err(e) => panic!("{}", e),
        };
    }
}
