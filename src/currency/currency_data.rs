use crate::currency::Currency;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub type CurrencyMap = HashMap<&'static str, Currency>;

lazy_static! {
    static ref CURRENCY_MAP: CurrencyMap = {
        let mut map = HashMap::new();

        map.insert("EUR", Currency::eur());
        map.insert("CHF", Currency::chf());
        map.insert("USD", Currency::usd());

        map
    };
}

pub fn all() -> CurrencyMap {
    CURRENCY_MAP.clone()
}
