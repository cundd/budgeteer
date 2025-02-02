use super::{Database, ExchangeRateRepository};
use crate::{
    currency::{
        amount_converter::AmountConverter, exchange_rate_provider::ExchangeRateProvider, Currency,
    },
    error::Error,
    filter::Request,
    transaction::Transaction,
};
use chrono::{NaiveDate, Utc};
use std::path::Path;

pub struct TransactionRepository {
    database: Database,
    exchange_rate_provider: ExchangeRateProvider,
}

impl TransactionRepository {
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

    pub async fn add(&self, transaction: &Transaction) -> Result<i64, Error> {
        let date = transaction.date();
        let currency = transaction.amount().currency().iso;
        let value = transaction.amount().value();
        let transaction_type = transaction.transaction_type();
        let note = transaction.note();

        // Insert the spending, then obtain the ID of this row
        let id = sqlx::query!(
            r#"
INSERT INTO transactions ( date, currency, amount, type, note )
VALUES ( ?1, ?2, ?3, ?4, ?5 )
        "#,
            date,
            currency,
            value,
            transaction_type,
            note,
        )
        .execute(&self.database.pool)
        .await?
        .last_insert_rowid();

        Ok(id)
    }

    pub async fn fetch_all(&self) -> Result<Vec<Transaction>, Error> {
        let transactions: Vec<Transaction> = sqlx::query_as(r#"SELECT * FROM transactions;"#)
            .fetch_all(&self.database.pool)
            .await?;

        Ok(transactions
            .into_iter()
            .map(|i| self.prepare_base_amount(i))
            .collect())
    }

    pub async fn fetch_with_request(
        &self,
        filter_request: Request,
    ) -> Result<Vec<Transaction>, Error> {
        let from = filter_request
            .from
            .unwrap_or(NaiveDate::from_ymd_opt(1900, 1, 1).unwrap());
        let to = filter_request.to.unwrap_or(Utc::now().naive_utc().date());
        let search = filter_request
            .search
            .map_or("%".to_string(), |s| format!("%{s}%"));

        let exclude = filter_request.exclude.map_or(
            "a string that should not exist in the text <>< -##".to_string(),
            |s| format!("%{s}%"),
        );

        let query = if let Some(transaction_type) = filter_request.transaction_type {
            sqlx::query_as(
                r#"SELECT * FROM transactions WHERE (date > ? AND date <= ?) AND type = ? AND note LIKE ? AND note NOT LIKE ?;"#,
            )
            .bind(from)
            .bind(to)
            .bind(transaction_type)
            .bind(search)
            .bind(exclude)
        } else {
            sqlx::query_as(
                r#"SELECT * FROM transactions WHERE (date > ? AND date <= ?) AND note LIKE ? AND note NOT LIKE ?;"#,
            )
            .bind(from)
            .bind(to)
            .bind(search)
            .bind(exclude)
        };

        let transactions: Vec<Transaction> = query.fetch_all(&self.database.pool).await?;

        Ok(transactions
            .into_iter()
            .map(|i| self.prepare_base_amount(i))
            .collect())
    }

    fn prepare_base_amount(&self, transaction: Transaction) -> Transaction {
        if transaction.amount.currency == Currency::base() {
            return transaction.with_base_amount(transaction.amount.clone());
        }

        let exchange_rate = self.exchange_rate_provider.find_exchange_rate(&transaction);

        match exchange_rate {
            Some(exchange_rate) => AmountConverter::convert_to_base(transaction, exchange_rate),
            None => transaction,
        }
    }
}
