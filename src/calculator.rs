use crate::currency::Currency;
use crate::transaction::amount::Amount;
use crate::transaction::transaction_type::{TransactionType, NUMBER_OF_TYPES};
use crate::transaction::Transaction;

pub struct Calculator {}

#[derive(Debug)]
pub struct Totals {
    pub total: f64,
    pub income: f64,
    pub expenses: f64,
}

impl Calculator {
    pub fn totals(transactions: &[Transaction]) -> Totals {
        let mut income = 0.0;
        let mut expenses = 0.0;
        for transaction in transactions {
            if let Some(a) = &transaction.base_amount {
                if a.value > 0.0 {
                    income += a.value
                } else {
                    expenses += a.value
                }
            }
        }

        let total = income + expenses;
        let total = if total != -0.0 { total } else { 0.0 };

        Totals {
            total,
            income,
            expenses,
        }
    }

    pub fn sum(transactions: &[Transaction]) -> f64 {
        let sum = transactions
            .iter()
            .filter_map(|i: &Transaction| i.base_amount.as_ref().map(|a| a.value))
            .sum();

        if sum != -0.0 {
            sum
        } else {
            0.0
        }
    }

    pub fn major_types(transactions: &[Transaction]) -> MajorTypes {
        let r = Calculator::rate(transactions);

        let result: (TransactionType, IncomeAndExpenses) = r
            .clone()
            .into_iter()
            .max_by(|a, b| a.1.income.total_cmp(&b.1.income))
            .unwrap();
        let max_income = MajorTypeEntry::new(result.0, result.1.income);

        let result: (TransactionType, IncomeAndExpenses) = r
            .clone()
            .into_iter()
            .max_by(|a, b| b.1.expenses.total_cmp(&a.1.expenses))
            .unwrap();
        let max_expenses = MajorTypeEntry::new(result.0, result.1.expenses);

        let result: (TransactionType, IncomeAndExpenses) = r
            .clone()
            .into_iter()
            .min_by(|a, b| a.1.income.total_cmp(&b.1.income))
            .unwrap();
        let min_income = MajorTypeEntry::new(result.0, result.1.income);

        let result: (TransactionType, IncomeAndExpenses) = r
            .into_iter()
            .min_by(|a, b| b.1.expenses.total_cmp(&a.1.expenses))
            .unwrap();
        let min_expenses = MajorTypeEntry::new(result.0, result.1.expenses);

        MajorTypes {
            min_income,
            min_expenses,
            max_income,
            max_expenses,
        }
    }

    pub fn totals_for_type(
        transactions: &[Transaction],
        transaction_type: TransactionType,
    ) -> Totals {
        Self::totals(
            transactions
                .iter()
                .filter_map(|t| {
                    if t.transaction_type == transaction_type {
                        Some(t.clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<Transaction>>()
                .as_ref(),
        )
    }

    pub fn totals_for_type_and_currency(
        transactions: &[Transaction],
        transaction_type: TransactionType,
        currency: &Currency,
    ) -> Totals {
        Self::totals(
            transactions
                .iter()
                .filter_map(|t| {
                    if t.transaction_type == transaction_type
                        && t.amount_ref().currency_ref() == currency
                    {
                        Some(t.clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<Transaction>>()
                .as_ref(),
        )
    }

    #[allow(unused)]
    pub fn sort(transactions: &[Transaction]) -> Vec<Transaction> {
        let mut clone = transactions.to_owned();
        clone.sort_by(|a, b| a.partial_cmp(b).map(|s| s.reverse()).unwrap());

        clone
    }

    fn rate(transactions: &[Transaction]) -> TransactionTypeScore {
        let mut score = TransactionTypeScore::new();
        for transaction in transactions {
            score.push(
                transaction.transaction_type(),
                transaction.base_amount.clone(),
            );
        }
        score
    }
}

#[derive(Debug, Clone)]
pub struct MajorTypeEntry {
    pub transaction_type: TransactionType,
    pub value: f64,
}

impl MajorTypeEntry {
    fn new(transaction_type: TransactionType, value: f64) -> Self {
        Self {
            transaction_type,
            value,
        }
    }
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct MajorTypes {
    pub min_income: MajorTypeEntry,
    pub min_expenses: MajorTypeEntry,
    pub max_income: MajorTypeEntry,
    pub max_expenses: MajorTypeEntry,
}

#[derive(Debug, Clone, Default)]
struct IncomeAndExpenses {
    income: f64,
    expenses: f64,
}

impl IncomeAndExpenses {
    fn push(&mut self, amount: &Amount) {
        let value = amount.value;
        if value < 0.0 {
            self.expenses += value
        } else {
            self.income += value
        }
    }
}

#[derive(Debug, Clone, Default)]
struct TransactionTypeScore {
    body: IncomeAndExpenses,
    car: IncomeAndExpenses,
    clothes: IncomeAndExpenses,
    eat: IncomeAndExpenses,
    education: IncomeAndExpenses,
    gas: IncomeAndExpenses,
    fun: IncomeAndExpenses,
    health: IncomeAndExpenses,
    home: IncomeAndExpenses,
    insurance: IncomeAndExpenses,
    telecommunication: IncomeAndExpenses,
    donation: IncomeAndExpenses,
    tax: IncomeAndExpenses,
    unknown: IncomeAndExpenses,
}

impl TransactionTypeScore {
    pub fn new() -> Self {
        TransactionTypeScore::default()
    }

    fn push(&mut self, transaction_type: TransactionType, amount: Option<Amount>) {
        let amount = &match amount {
            Some(a) => a,
            None => return,
        };

        match transaction_type {
            TransactionType::Body => self.body.push(amount),
            TransactionType::Car => self.car.push(amount),
            TransactionType::Clothes => self.clothes.push(amount),
            TransactionType::Eat => self.eat.push(amount),
            TransactionType::Education => self.education.push(amount),
            TransactionType::Gas => self.gas.push(amount),
            TransactionType::Fun => self.fun.push(amount),
            TransactionType::Health => self.health.push(amount),
            TransactionType::Home => self.home.push(amount),
            TransactionType::Insurance => self.insurance.push(amount),
            TransactionType::Telecommunication => self.telecommunication.push(amount),
            TransactionType::Donation => self.donation.push(amount),
            TransactionType::Tax => self.tax.push(amount),
            TransactionType::Unknown => self.unknown.push(amount),
        }
    }
}

impl IntoIterator for TransactionTypeScore {
    type Item = (TransactionType, IncomeAndExpenses);

    type IntoIter = std::array::IntoIter<(TransactionType, IncomeAndExpenses), NUMBER_OF_TYPES>;

    fn into_iter(self) -> Self::IntoIter {
        [
            (TransactionType::Body, self.body),
            (TransactionType::Car, self.car),
            (TransactionType::Clothes, self.clothes),
            (TransactionType::Eat, self.eat),
            (TransactionType::Education, self.education),
            (TransactionType::Gas, self.gas),
            (TransactionType::Fun, self.fun),
            (TransactionType::Health, self.health),
            (TransactionType::Home, self.home),
            (TransactionType::Insurance, self.insurance),
            (TransactionType::Telecommunication, self.telecommunication),
            (TransactionType::Donation, self.donation),
            (TransactionType::Tax, self.tax),
            (TransactionType::Unknown, self.unknown),
        ]
        .into_iter()
    }
}
