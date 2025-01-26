mod exchange_rate_repository;
mod transaction_repository;

use crate::error::Error;
pub use exchange_rate_repository::ExchangeRateRepository;
use sqlx::SqlitePool;
use std::path::Path;
pub use transaction_repository::TransactionRepository;

pub struct Database {
    pub pool: SqlitePool,
}

impl Database {
    pub async fn new(path: &Path) -> Result<Self, Error> {
        // Make sure the file exists
        let _file = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(path);

        let pool = match SqlitePool::connect(&format!("sqlite:{}", path.display())).await {
            Ok(p) => p,
            Err(e) => {
                eprintln!("{:?}", e);
                return Err(e.into());
            }
        };

        Self::prepare_database(&pool, path).await?;

        Ok(Self { pool })
    }

    async fn prepare_database(pool: &SqlitePool, path: &Path) -> Result<(), Error> {
        sqlx::query(include_str!("../../migrations/01-create-tables.sql"))
            .execute(pool)
            .await
            .map_err(|e| {
                // Check for an "attempt to write a readonly database"-error
                if let sqlx::error::Error::Database(ref inner) = e {
                    if matches!(inner.code(), Some(c) if &c  == "8") {
                        return Error::FileIO(format!(
                            "Attempt to write a readonly database {}",
                            path.display()
                        ));
                    }
                }
                Error::Persistence(format!(
                    "Error during database migration #create-tables: {}",
                    e
                ))
            })?;

        sqlx::query(include_str!("../../migrations/02-prefill-tables.sql"))
            .execute(pool)
            .await
            .map_err(|e| {
                Error::Persistence(format!("Error during database migration #prefill: {}", e))
            })?;

        Ok(())
    }
}
