use super::{exchange_rate::ExchangeRate, Currency};
use crate::transaction::{amount::Amount, Transaction};

pub struct AmountConverter {}

impl AmountConverter {
    pub fn convert_to_base(transaction: Transaction, exchange_rate: ExchangeRate) -> Transaction {
        let converted_amount = transaction.amount.value() * (exchange_rate.rate);
        transaction.with_base_amount(Amount::new(converted_amount, Currency::base()))
    }
}
