use std::collections::HashMap;
use currency::Currency;

pub fn all() -> HashMap<&'static str, Currency> {
    let mut map = HashMap::new();

    map.insert("EUR", Currency::eur());
    map.insert("CHF", Currency::chf());
    map.insert("USD", Currency::usd());

    map
}