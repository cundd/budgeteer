use super::{Database, ExchangeRateRepository};
use crate::{
    currency::{
        amount_converter::AmountConverter, exchange_rate_provider::ExchangeRateProvider, Currency,
    },
    error::Error,
    filter::Request,
    invoice::Invoice,
};
use chrono::NaiveDate;
use std::path::Path;

pub struct InvoiceRepository {
    database: Database,
    exchange_rate_provider: ExchangeRateProvider,
}

impl InvoiceRepository {
    pub async fn new(path: &Path) -> Result<Self, Error> {
        let database = Database::new(path).await?;
        let exchange_rate_repository = ExchangeRateRepository::new(path).await?;
        let exchange_rates = exchange_rate_repository.fetch_all().await?;
        let exchange_rate_provider = ExchangeRateProvider::new(exchange_rates);

        Ok(Self {
            database,
            exchange_rate_provider,
        })
    }

    pub async fn add(&self, invoice: &Invoice) -> Result<i64, Error> {
        let date = invoice.date();
        let currency = invoice.amount().currency().iso;
        let value = invoice.amount().value();
        let invoice_type = invoice.invoice_type();
        let note = invoice.note();

        // Insert the spending, then obtain the ID of this row
        let id = sqlx::query!(
            r#"
INSERT INTO transactions ( date, currency, amount, type, note )
VALUES ( ?1, ?2, ?3, ?4, ?5 )
        "#,
            date,
            currency,
            value,
            invoice_type,
            note,
        )
        .execute(&self.database.pool)
        .await?
        .last_insert_rowid();

        Ok(id)
    }

    pub async fn fetch_all(&self) -> Result<Vec<Invoice>, Error> {
        let transactions: Vec<Invoice> = sqlx::query_as(r#"SELECT * FROM transactions;"#)
            .fetch_all(&self.database.pool)
            .await?;

        Ok(transactions
            .into_iter()
            .map(|i| self.prepare_base_amount(i))
            .collect())
    }

    pub async fn fetch_with_request(&self, filter_request: Request) -> Result<Vec<Invoice>, Error> {
        let query = if let Some(invoice_type) = filter_request.invoice_type {
            sqlx::query_as(
                r#"SELECT * FROM transactions WHERE (date > ? AND date <= ?) AND type = ?;"#,
            )
            .bind(filter_request.from.unwrap_or(NaiveDate::MIN))
            .bind(filter_request.to.unwrap_or(NaiveDate::MAX))
            .bind(invoice_type)
        } else {
            sqlx::query_as(r#"SELECT * FROM transactions WHERE (date > ? AND date <= ?)"#)
                .bind(filter_request.from.unwrap_or(NaiveDate::MIN))
                .bind(filter_request.to.unwrap_or(NaiveDate::MAX))
        };

        let transactions: Vec<Invoice> = query.fetch_all(&self.database.pool).await?;

        Ok(transactions
            .into_iter()
            .map(|i| self.prepare_base_amount(i))
            .collect())
    }

    fn prepare_base_amount(&self, invoice: Invoice) -> Invoice {
        if invoice.amount.currency == Currency::base() {
            return invoice.with_base_amount(invoice.amount.clone());
        }

        let exchange_rate = self.exchange_rate_provider.find_exchange_rate(&invoice);

        match exchange_rate {
            Some(exchange_rate) => AmountConverter::convert_to_base(invoice, exchange_rate),
            None => invoice,
        }
    }
}
