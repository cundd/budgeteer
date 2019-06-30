use std::fmt;
use currency::Currency;

#[derive(Debug, Clone, PartialEq)]
pub struct Amount {
    currency: Currency,
    value: f64,
}

impl Amount {
    pub fn new(value: f64, currency: &Currency) -> Self {
        Amount {
            currency: currency.to_owned(),
            value,
        }
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
