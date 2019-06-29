use std::collections::HashMap;
use currency::Currency;

pub fn all() -> HashMap<&'static str, Currency> {
    let mut map = HashMap::new();

    map.insert("EUR", Currency::new("EUR", "€"));
    map.insert("CHF", Currency::new("CHF", "CHF"));


    map
}