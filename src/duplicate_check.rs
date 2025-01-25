use chrono::Days;

use crate::invoice::{main_transaction_data::MainTransactionData, Invoice};

pub struct DuplicateChecker {}

impl DuplicateChecker {
    pub fn get_possible_duplicates<'a, T: MainTransactionData>(
        transaction: &T,
        transactions: &'a [Invoice],
    ) -> Vec<&'a Invoice> {
        transactions
            .iter()
            .filter(|item| is_possible_duplicate(item, transaction))
            .collect()
    }
}

fn is_possible_duplicate<A: MainTransactionData, B: MainTransactionData>(a: &A, b: &B) -> bool {
    if a.amount() != b.amount() {
        return false;
    }
    if a.date() == b.date() {
        return true;
    }
    let three_days_before = b
        .date()
        .checked_sub_days(Days::new(3))
        .expect("Date is out of range");

    let three_days_after = b
        .date()
        .checked_add_days(Days::new(3))
        .expect("Date is out of range");

    a.date() >= three_days_before && a.date() <= three_days_after
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        currency::Currency,
        invoice::{amount::Amount, main_transaction_data::MainTransactionData},
    };
    use chrono::{NaiveDate, Utc};

    #[derive(Debug, PartialEq)]
    struct DummyTransaction {
        amount: Amount,
        date: NaiveDate,
    }

    impl MainTransactionData for DummyTransaction {
        fn amount(&self) -> &Amount {
            &self.amount
        }

        fn date(&self) -> NaiveDate {
            self.date
        }
    }

    fn x_eur(value: f64) -> Amount {
        Amount::new(value, Currency::eur())
    }

    #[test]
    fn is_possible_duplicate_test_amount() {
        assert!(is_possible_duplicate(
            &DummyTransaction {
                amount: x_eur(102.19),
                date: Utc::now().date_naive()
            },
            &DummyTransaction {
                amount: x_eur(102.19),
                date: Utc::now().date_naive()
            }
        ));
        assert!(!is_possible_duplicate(
            &DummyTransaction {
                amount: x_eur(-102.19),
                date: Utc::now().date_naive()
            },
            &DummyTransaction {
                amount: x_eur(102.19),
                date: Utc::now().date_naive()
            }
        ));
        assert!(!is_possible_duplicate(
            &DummyTransaction {
                amount: x_eur(1.1),
                date: Utc::now().date_naive()
            },
            &DummyTransaction {
                amount: x_eur(2.2),
                date: Utc::now().date_naive()
            }
        ));
    }

    #[test]
    fn is_possible_duplicate_test_date() {
        assert!(is_possible_duplicate(
            &DummyTransaction {
                amount: x_eur(10.20),
                date: Utc::now().date_naive()
            },
            &DummyTransaction {
                amount: x_eur(10.20),
                date: Utc::now().date_naive()
            },
        ));

        // Within +3 days
        assert!(is_possible_duplicate(
            &DummyTransaction {
                amount: x_eur(10.20),
                date: NaiveDate::from_ymd_opt(2024, 1, 24).unwrap()
            },
            &DummyTransaction {
                amount: x_eur(10.20),
                date: NaiveDate::from_ymd_opt(2024, 1, 27).unwrap()
            },
        ));
        // Within -3 days
        assert!(is_possible_duplicate(
            &DummyTransaction {
                amount: x_eur(10.20),
                date: NaiveDate::from_ymd_opt(2024, 1, 24).unwrap()
            },
            &DummyTransaction {
                amount: x_eur(10.20),
                date: NaiveDate::from_ymd_opt(2024, 1, 21).unwrap()
            },
        ));
    }
}
