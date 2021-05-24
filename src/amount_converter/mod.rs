//mod exchange_rates;
use crate::invoice::amount::Amount;
use std::collections::HashMap;
use crate::rate_provider::Rate;
use crate::invoice::Invoice;
use crate::currency::Currency;

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
        invoice.with_base_amount(self.convert_to_base(invoice, &invoice.amount()))
    }

    pub fn convert(&self, invoice: &Invoice, amount: &Amount, to: &Currency) -> Amount {
        if &amount.currency() == to {
            return Amount::new(amount.value(), to);
        }
        let rate = self.rate_map.get(&invoice.date().format("%Y-%m-%d").to_string())
            .expect(&format!("Currency '{}' not found in map", amount.currency()))
            .rate;
        let factor = if amount.currency() == self.base_currency {
            rate
        } else {
            1.0 / rate
        };

        Amount::new(amount.value() * factor, to)
    }
    pub fn convert_to_base(&self, invoice: &Invoice, amount: &Amount) -> Amount {
        self.convert(invoice, amount, &self.base_currency)
    }
}
