use super::{exchange_rate::ExchangeRate, Currency};
use crate::invoice::{amount::Amount, Invoice};

pub struct AmountConverter {}

impl AmountConverter {
    pub fn convert_to_base(invoice: Invoice, exchange_rate: ExchangeRate) -> Invoice {
        let converted_amount = invoice.amount.value() * (exchange_rate.rate);
        invoice.with_base_amount(Amount::new(converted_amount, Currency::base()))
    }
}
