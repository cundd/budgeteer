use amount_converter::AmountConverter;
use invoice::Invoice;
use error::Error;
use chrono::NaiveDate;
use invoice::invoice_type::InvoiceType;
use invoice::amount::Amount;
use std;
use std::collections::BTreeMap;
use file_reader::LineParts;
use currency::Currency;

pub struct ParserResult {
    pub invoices: BTreeMap<NaiveDate, Invoice>,
    pub errors: Vec<Error>,
}

pub struct InvoiceParser {}

impl InvoiceParser {
    pub fn new() -> Self {
        InvoiceParser {}
    }

    pub fn parse_lines(&self, lines: Vec<LineParts>) -> ParserResult {
        let mut invoices = BTreeMap::new();
        let mut errors = vec![];
        for parts in lines {
            match self.build_from_vec(parts.iter().map(String::as_str).collect()) {
                Ok(invoice) => { invoices.insert(invoice.date, invoice); }
                Err(error) => { errors.push(error); }
            }
        }

        ParserResult { invoices, errors }
    }

    #[allow(dead_code)]
    pub fn build_from_vec_with_converter(&self, amount_converter: &AmountConverter, parts: Vec<&str>) -> Result<Invoice, Error> {
        self.build_from_vec_with_converter_option(Some(amount_converter), parts)
    }

    pub fn build_from_vec(&self, parts: Vec<&str>) -> Result<Invoice, Error> {
        self.build_from_vec_with_converter_option(None, parts)
    }

    fn build_from_vec_with_converter_option(&self, amount_converter: Option<&AmountConverter>, parts: Vec<&str>) -> Result<Invoice, Error> {
        let string_vec: Vec<String> =
            parts.iter()
                .map(|x| { String::from(*x) })
                .collect();

        let date = self.parse_date(&string_vec)?;
        let currency = self.get_vec_part_or_error(
            &string_vec,
            1,
            "Could not read currency from line",
        )?;

        let amount = Amount {
            currency: Currency::from_string(&currency),
            value: self.parse_amount(&string_vec)?,
        };

        let base_amount = match amount_converter {
//            Some(a) => Some(a.convert_to_base(&amount)),
            Some(_) => None,
            None => None
        };

        let invoice_type = InvoiceType::from_str(&string_vec.get(3).unwrap_or(&"".to_string()));
        let comment = self.get_vec_part(&string_vec, 4);

        Ok(Invoice {
            date,
            amount,
            base_amount,
            invoice_type,
            comment,
        })
    }

    fn parse_date(&self, string_vec: &Vec<String>) -> Result<NaiveDate, Error> {
        match self.get_vec_part_or_error(&string_vec, 0, "Could not read date from line") {
            Ok(s) => match NaiveDate::parse_from_str(&s, "%d.%m.%Y") {
                Ok(d) => Ok(d),
                Err(e) => Err(Error::ParseError(format!(
                    "Could not parse date '{}': {}",
                    s,
                    std::error::Error::description(&e)
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

        match amount_string.trim().parse::<f64>() {
            Ok(f) => Ok(f),
            Err(e) => Err(Error::from(e))
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
