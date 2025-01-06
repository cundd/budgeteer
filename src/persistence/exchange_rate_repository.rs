use super::Database;
use crate::{currency::exchange_rate::ExchangeRate, error::Error};
use std::path::Path;

pub struct ExchangeRateRepository {
    database: Database,
}

impl ExchangeRateRepository {
    pub async fn new(path: &Path) -> Result<Self, Error> {
        let database = Database::new(path).await?;

        Ok(Self { database })
    }

    pub async fn fetch_all(&self) -> Result<Vec<ExchangeRate>, Error> {
        Ok(sqlx::query_as("SELECT * FROM exchange_rates;")
            .fetch_all(&self.database.pool)
            .await?)
    }
}
