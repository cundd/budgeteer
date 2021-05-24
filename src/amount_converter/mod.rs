use std::collections::HashMap;

use crate::currency::Currency;
use crate::error::{Error, Res};
use crate::invoice::amount::Amount;
use crate::invoice::Invoice;
use crate::rate_provider::Rate;

pub struct AmountConverter {
    base_currency: Currency,
    rate_map: HashMap<String, Rate>,
}

impl AmountConverter {
    pub fn new(base_currency: Currency, rate_map: HashMap<String, Rate>) -> AmountConverter {
        AmountConverter {
            base_currency,
            rate_map,
        }
    }

    pub fn invoice_with_base_amount(&self, invoice: &Invoice) -> Invoice {
        let base_amount = self.convert_to_base(invoice, &invoice.amount());
        match base_amount {
            Ok(a) => invoice.with_base_amount(a),
            Err(_) => invoice.clone(),
        }
    }

    fn convert(&self, invoice: &Invoice, amount: &Amount, to: &Currency) -> Res<Amount> {
        if &amount.currency() == to {
            return Ok(Amount::new(amount.value(), to));
        }
        let invoice_date_formatted = invoice.date().format("%Y-%m-%d");
        let rate = match self.rate_map.get(&invoice_date_formatted.to_string()) {
            Some(f) => f.rate,
            None => {
                return Err(Error::RateError(format!(
                    "Currency '{}' not found in map for date {}",
                    amount.currency(),
                    invoice_date_formatted
                )))
            }
        };
        let factor = if amount.currency() == self.base_currency {
            rate
        } else {
            1.0 / rate
        };

        Ok(Amount::new(amount.value() * factor, to))
    }

    fn convert_to_base(&self, invoice: &Invoice, amount: &Amount) -> Res<Amount> {
        self.convert(invoice, amount, &self.base_currency)
    }
}
