use std::fmt;
use currency::Currency;

#[derive(Debug, Clone, PartialEq)]
pub struct Amount {
    pub currency: Currency,
    pub value: f64,
}

impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {:.2}", self.currency, self.value)
    }
}
