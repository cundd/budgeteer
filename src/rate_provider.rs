use crate::error::Error;
use crate::error::Res;
use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;
use serde_json::from_str;
use serde_json::{from_reader, Value};
use std::collections::HashMap;

pub struct RateProvider {}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Rate {
    pub date: NaiveDate,
    pub currency: String,
    pub rate: f64,
}

impl Rate {
    fn new(date: NaiveDate, currency: &str, rate: f64) -> Self {
        Rate {
            currency: currency.to_owned(),
            rate,
            date,
        }
    }
}

const DATE_FORMAT: &str = "%Y-%m-%d";

#[derive(Serialize, Deserialize, Debug)]
struct RawRates {
    rates: HashMap<String, Value>,
}

impl RateProvider {
    pub async fn fetch_rates(
        start: NaiveDate,
        end: NaiveDate,
        symbols: Vec<&str>,
    ) -> Res<HashMap<String, Rate>> {
        let request_url = RateProvider::build_request_url(start, end, &symbols);

        let body = reqwest::get(&request_url).await?.text().await?;
        let raw_rates: RawRates = match from_str(&body) {
            Ok(r) => r,
            Err(e) => {
                return Err(Error::Rate(format!("{} for body '{}'", e, body)));
            }
        };
        let rates = raw_rates
            .rates
            .iter()
            .flat_map(|(raw_date, raw_rate)| RateProvider::build_rate(&symbols, raw_date, raw_rate))
            .collect::<Vec<Rate>>();

        let mut result_map = HashMap::new();
        for rate in rates {
            result_map.insert(rate.date.format(DATE_FORMAT).to_string(), rate);
        }

        Ok(result_map)
    }

    fn build_rate(symbols: &[&str], raw_date: &str, raw_rate: &Value) -> Vec<Rate> {
        let date = match NaiveDate::parse_from_str(raw_date, DATE_FORMAT) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("{}", e);
                return vec![];
            }
        };

        let mut rates = vec![];
        for currency_symbol in symbols.iter() {
            if let Some(r) = raw_rate.get(currency_symbol) {
                rates.push(Rate::new(date, currency_symbol, r.as_f64().unwrap()))
            }
        }

        rates
    }

    fn build_request_url(start: NaiveDate, end: NaiveDate, symbols: &[&str]) -> String {
        format!(
            "https://api.exchangerate.host/timeseries?start_date={start}&end_date={end}&symbols={symbols}",
            start = start.format(DATE_FORMAT),
            end = end.format(DATE_FORMAT),
            symbols = symbols.join(",")
        )
    }
}
