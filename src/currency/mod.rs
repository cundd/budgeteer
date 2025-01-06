use crate::error::Error;
use sqlx::sqlite::SqliteTypeInfo;
use sqlx::{Database, Decode, Sqlite, Type};
use std::fmt;
use std::str::FromStr;

pub mod amount_converter;
pub mod currency_data;
pub mod exchange_rate;
pub mod exchange_rate_provider;

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

    /// Return the systems current base amount
    pub fn base() -> Self {
        Self::eur()
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
}

impl FromStr for Currency {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let all_currencies = currency_data::all();
        match all_currencies.get(input) {
            Some(c) => Ok(c.clone()),
            None => match all_currencies.iter().find(|(_, c)| c.symbol == input) {
                Some((_, c)) => Ok(c.to_owned()),
                None => Err(Error::Parse(format!("Currency '{}' not found", input))),
            },
        }
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.symbol)
    }
}

impl Type<Sqlite> for Currency {
    fn type_info() -> SqliteTypeInfo {
        <str as Type<Sqlite>>::type_info()
    }
}

// DB is the database driver
// `'r` is the lifetime of the `Row` being decoded
impl<'r, DB: Database> Decode<'r, DB> for Currency
where
    // we want to delegate some of the work to string decoding so let's make sure strings
    // are supported by the database
    &'r str: Decode<'r, DB>,
{
    fn decode(
        value: <DB as Database>::ValueRef<'r>,
    ) -> Result<Currency, Box<dyn std::error::Error + 'static + Send + Sync>> {
        // the interface of ValueRef is largely unstable at the moment
        // so this is not directly implementable

        // however, you can delegate to a type that matches the format of the type you want
        // to decode (such as a UTF-8 string)

        let value = <&str as Decode<DB>>::decode(value)?;

        // now you can parse this into your type (assuming there is a `FromStr`)

        Ok(value.parse()?)
    }
}
