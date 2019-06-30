mod currency_data;

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Currency {
    pub iso: String,
    pub symbol: String,
}

impl Currency {
    pub fn new<'a>(iso: &'a str, symbol: &'a str) -> Self {
        Currency {
            iso: iso.to_owned(),
            symbol: symbol.to_owned(),
        }
    }

    pub fn eur() -> Self {
        Currency::new("EUR", "€")
    }

    pub fn chf() -> Self {
        Currency::new("CHF", "CHF")
    }

    pub fn usd() -> Self {
        Currency::new("USD", "$")
    }

    pub fn from_string(input: &str) -> Self {
        let all_currencies = currency_data::all();
        match all_currencies.get(input) {
            Some(c) => c.clone(),
            None => {
                match all_currencies.iter().find(|(_, c)| &c.symbol == input) {
                    Some((_, c)) => c.to_owned(),
                    None => panic!("Currency '{}' not found", input),
                }
            }
        }
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.symbol)
    }
}
