use crate::{error::Error, invoice::Invoice};

pub mod json;
pub mod markdown;

pub struct ImportResult {
    pub transactions: Vec<Invoice>,
    pub errors: Vec<Error>,
}
