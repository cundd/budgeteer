use crate::transaction::Transaction;
use chrono::Datelike;

use super::exchange_rate::ExchangeRate;

pub struct ExchangeRateProvider {
    exchange_rates: Vec<ExchangeRate>,
}

impl ExchangeRateProvider {
    pub fn new(exchange_rates: Vec<ExchangeRate>) -> Self {
        Self { exchange_rates }
    }

    pub fn find_exchange_rate(&self, transaction: &Transaction) -> Option<ExchangeRate> {
        let date = transaction.date;
        let currency = &transaction.amount.currency.iso;
        let exchange_rate = self.find_by_date_configuration(
            currency,
            date.year(),
            date.month() as i64,
            date.day() as i64,
        );
        if exchange_rate.is_some() {
            return exchange_rate;
        }

        let exchange_rate =
            self.find_by_date_configuration(currency, date.year(), date.month() as i64, -1);
        if exchange_rate.is_some() {
            return exchange_rate;
        }

        let exchange_rate = self.find_by_date_configuration(currency, date.year(), -1, -1);
        if exchange_rate.is_some() {
            return exchange_rate;
        }

        None
    }

    fn find_by_date_configuration(
        &self,
        currency: &str,
        year: i32,
        month: i64,
        day: i64,
    ) -> Option<ExchangeRate> {
        self.exchange_rates
            .iter()
            .find(|rate| {
                rate.currency.iso == currency
                    && rate.year == year
                    && rate.month == month
                    && rate.day == day
            })
            .cloned()
    }
}
