use crate::currency::Currency;
use crate::transaction::amount::Amount;
use crate::transaction::transaction_type::TransactionType;
use crate::transaction::Transaction;

pub struct Calculator {}

impl Calculator {
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

    pub fn sum_for_type(transactions: &[Transaction], transaction_type: TransactionType) -> f64 {
        let sum = transactions
            .iter()
            .filter_map(|i| {
                if i.transaction_type() == transaction_type {
                    i.base_amount().map(|a| a.value())
                } else {
                    None
                }
            })
            .sum();

        if sum != -0.0 {
            sum
        } else {
            0.0
        }
    }

    pub fn sum_for_type_and_currency(
        transactions: &[Transaction],
        transaction_type: TransactionType,
        currency: &Currency,
    ) -> f64 {
        let sum = transactions
            .iter()
            .filter(|i| {
                i.transaction_type() == transaction_type
                    && i.amount_ref().currency_ref() == currency
            })
            .map(|i| i.amount_ref().value())
            .sum();

        if sum != -0.0 {
            sum
        } else {
            0.0
        }
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

#[derive(Debug, Clone)]
struct TransactionTypeScore {
    car: IncomeAndExpenses,
    clothes: IncomeAndExpenses,
    eat: IncomeAndExpenses,
    gas: IncomeAndExpenses,
    fun: IncomeAndExpenses,
    health: IncomeAndExpenses,
    home: IncomeAndExpenses,
    telecommunication: IncomeAndExpenses,
    unknown: IncomeAndExpenses,
}

impl TransactionTypeScore {
    pub fn new() -> Self {
        TransactionTypeScore {
            car: IncomeAndExpenses::default(),
            clothes: IncomeAndExpenses::default(),
            eat: IncomeAndExpenses::default(),
            gas: IncomeAndExpenses::default(),
            fun: IncomeAndExpenses::default(),
            health: IncomeAndExpenses::default(),
            home: IncomeAndExpenses::default(),
            telecommunication: IncomeAndExpenses::default(),
            unknown: IncomeAndExpenses::default(),
        }
    }

    fn push(&mut self, transaction_type: TransactionType, amount: Option<Amount>) {
        let amount = &match amount {
            Some(a) => a,
            None => return,
        };

        match transaction_type {
            TransactionType::Car => self.car.push(amount),
            TransactionType::Clothes => self.clothes.push(amount),
            TransactionType::Eat => self.eat.push(amount),
            TransactionType::Gas => self.gas.push(amount),
            TransactionType::Fun => self.fun.push(amount),
            TransactionType::Health => self.health.push(amount),
            TransactionType::Home => self.home.push(amount),
            TransactionType::Telecommunication => self.telecommunication.push(amount),
            TransactionType::Unknown => self.unknown.push(amount),
        }
    }
}

impl IntoIterator for TransactionTypeScore {
    type Item = (TransactionType, IncomeAndExpenses);

    type IntoIter = std::array::IntoIter<(TransactionType, IncomeAndExpenses), 9>;

    fn into_iter(self) -> Self::IntoIter {
        [
            (TransactionType::Car, self.car),
            (TransactionType::Clothes, self.clothes),
            (TransactionType::Eat, self.eat),
            (TransactionType::Gas, self.gas),
            (TransactionType::Fun, self.fun),
            (TransactionType::Health, self.health),
            (TransactionType::Home, self.home),
            (TransactionType::Telecommunication, self.telecommunication),
            (TransactionType::Unknown, self.unknown),
        ]
        .into_iter()
    }
}
