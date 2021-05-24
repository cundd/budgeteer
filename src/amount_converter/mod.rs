use std::collections::HashMap;

use crate::currency::Currency;
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
            Some(a) => invoice.with_base_amount(a),
            None => invoice.clone(),
        }
    }

    fn convert(&self, invoice: &Invoice, amount: &Amount, to: &Currency) -> Option<Amount> {
        if &amount.currency() == to {
            return Some(Amount::new(amount.value(), to));
        }
        let rate = self
            .rate_map
            .get(&invoice.date().format("%Y-%m-%d").to_string())?
            // .expect(&format!("Currency '{}' not found in map", amount.currency()))
            .rate;
        let factor = if amount.currency() == self.base_currency {
            rate
        } else {
            1.0 / rate
        };

        Some(Amount::new(amount.value() * factor, to))
    }

    fn convert_to_base(&self, invoice: &Invoice, amount: &Amount) -> Option<Amount> {
        self.convert(invoice, amount, &self.base_currency)
    }
}
