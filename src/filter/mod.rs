use crate::{
    error::{Error, Res},
    transaction::transaction_type::TransactionType,
};
use chrono::{Datelike, NaiveDate};
use std::fmt;

#[derive(Clone, Debug)]
pub struct Request {
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
    pub transaction_type: Option<TransactionType>,

    pub search: Option<String>,
}

impl Request {
    pub fn new(
        from: Option<NaiveDate>,
        to: Option<NaiveDate>,
        transaction_type: Option<TransactionType>,
        search: Option<String>,
    ) -> Self {
        Request {
            from,
            to,
            transaction_type,
            search,
        }
    }

    pub fn empty(&self) -> bool {
        self.from.is_none()
            && self.to.is_none()
            && self.transaction_type.is_none()
            && self.search.is_none()
    }

    pub fn parse_from_date(input: &str) -> Res<NaiveDate> {
        if input.is_empty() {
            return Err(Error::Argument("Date input must not be empty".to_string()));
        }
        let prepared_input = if regex::Regex::new(r"^\d{4}-\d{2}-\d{2}$")
            .unwrap()
            .is_match(input)
        {
            input.to_owned()
        } else if regex::Regex::new(r"^\d{4}-\d{2}$").unwrap().is_match(input) {
            format!("{}-01", input)
        } else if regex::Regex::new(r"^\d{4}$").unwrap().is_match(input) {
            format!("{}-01-01", input)
        } else {
            return Err(Error::Argument(format!(
                "Could not parse date {}. Please use format 'YYYY-MM-DD'",
                input
            )));
        };

        NaiveDate::parse_from_str(&prepared_input, "%Y-%m-%d")
            .map_err(build_date_parsing_error(input))
    }

    pub fn parse_to_date(input: &str) -> Res<NaiveDate> {
        if input.is_empty() {
            return Err(Error::Argument("Date input must not be empty".to_string()));
        }

        let prepared_input = if regex::Regex::new(r"^\d{4}-\d{2}-\d{2}$")
            .unwrap()
            .is_match(input)
        {
            input.to_owned()
        } else if regex::Regex::new(r"^\d{4}-\d{2}$").unwrap().is_match(input) {
            let first_day_of_month =
                NaiveDate::parse_from_str(&format!("{}-01", input), "%Y-%m-%d")
                    .map_err(build_date_parsing_error(input))?;

            return last_day_of_month(first_day_of_month);
        } else if regex::Regex::new(r"^\d{4}$").unwrap().is_match(input) {
            format!("{}-12-31", input)
        } else {
            return Err(Error::Argument(format!(
                "Could not parse date {}. Please use format 'YYYY-MM-DD'",
                input
            )));
        };

        NaiveDate::parse_from_str(&prepared_input, "%Y-%m-%d")
            .map_err(build_date_parsing_error(input))
    }
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();

        if let Some(transaction_type) = self.transaction_type {
            s.push_str("Typ:");
            s.push_str(transaction_type.to_str());
            if self.from.is_some() || self.to.is_some() {
                s.push_str(", ");
            }
        }
        if let Some(from) = self.from {
            s.push_str("Von:");
            s.push_str(&from.to_string());
            if self.from.is_some() || self.to.is_some() {
                s.push_str(", ");
            }
        }
        if let Some(to) = self.to {
            s.push_str("Bis:");
            s.push_str(&to.to_string());
        }
        if let Some(search) = &self.search {
            s.push_str("Enthält:");
            s.push_str(search);
        }

        f.write_str(s.as_str())
    }
}

fn build_date_parsing_error<S: Into<String>>(input: S) -> impl Fn(chrono::ParseError) -> Error {
    let input = input.into();

    move |e| Error::Argument(format!("Could not parse date {}: {}", input, e))
}

fn last_day_of_month(date: NaiveDate) -> Res<NaiveDate> {
    match NaiveDate::from_ymd_opt(date.year(), date.month() + 1, 1)
        .unwrap_or(NaiveDate::from_ymd_opt(date.year() + 1, 1, 1).unwrap())
        .pred_opt()
    {
        Some(d) => Ok(d),
        None => Err(Error::Argument(
            "First representable date reached".to_string(),
        )),
    }
}
