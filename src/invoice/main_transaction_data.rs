use chrono::NaiveDate;

use super::amount::Amount;

pub trait MainTransactionData {
    fn amount(&self) -> &Amount;
    fn date(&self) -> NaiveDate;
}
