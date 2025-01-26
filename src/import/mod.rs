use crate::{error::Error, transaction::Transaction};

pub mod json;
pub mod markdown;

pub struct ImportResult {
    pub transactions: Vec<Transaction>,
    pub errors: Vec<Error>,
}
