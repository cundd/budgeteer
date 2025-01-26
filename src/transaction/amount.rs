use crate::currency::Currency;
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Amount {
    pub currency: Currency,
    pub value: f64,
}

impl Amount {
    pub fn new(value: f64, currency: Currency) -> Self {
        Amount { currency, value }
    }

    pub fn currency(&self) -> Currency {
        self.currency.clone()
    }

    pub fn currency_ref(&self) -> &Currency {
        &self.currency
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {:.2}", self.currency, self.value)
    }
}

impl PartialOrd for Amount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}
