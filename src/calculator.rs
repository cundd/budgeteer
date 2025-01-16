use crate::currency::Currency;
use crate::invoice::amount::Amount;
use crate::invoice::invoice_type::InvoiceType;
use crate::invoice::Invoice;

pub struct Calculator {}

impl Calculator {
    pub fn sum(invoices: &[Invoice]) -> f64 {
        let sum = invoices
            .iter()
            .filter_map(|i: &Invoice| i.base_amount.as_ref().map(|a| a.value))
            .sum();

        if sum != -0.0 {
            sum
        } else {
            0.0
        }
    }

    pub fn major_types(invoices: &[Invoice]) -> MajorTypes {
        let r = Calculator::rate(invoices);

        let result: (InvoiceType, IncomeAndExpenses) = r
            .clone()
            .into_iter()
            .max_by(|a, b| a.1.income.total_cmp(&b.1.income))
            .unwrap();
        let max_income = MajorTypeEntry::new(result.0, result.1.income);

        let result: (InvoiceType, IncomeAndExpenses) = r
            .clone()
            .into_iter()
            .max_by(|a, b| b.1.expenses.total_cmp(&a.1.expenses))
            .unwrap();
        let max_expenses = MajorTypeEntry::new(result.0, result.1.expenses);

        let result: (InvoiceType, IncomeAndExpenses) = r
            .clone()
            .into_iter()
            .min_by(|a, b| a.1.income.total_cmp(&b.1.income))
            .unwrap();
        let min_income = MajorTypeEntry::new(result.0, result.1.income);

        let result: (InvoiceType, IncomeAndExpenses) = r
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

    pub fn sum_for_type(invoices: &[Invoice], invoice_type: InvoiceType) -> f64 {
        let sum = invoices
            .iter()
            .filter_map(|i| {
                if i.invoice_type() == invoice_type {
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
        invoices: &[Invoice],
        invoice_type: InvoiceType,
        currency: &Currency,
    ) -> f64 {
        let sum = invoices
            .iter()
            .filter(|i| {
                i.invoice_type() == invoice_type && i.amount_ref().currency_ref() == currency
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
    pub fn sort(invoices: &[Invoice]) -> Vec<Invoice> {
        let mut clone = invoices.to_owned();
        clone.sort_by(|a, b| a.partial_cmp(b).map(|s| s.reverse()).unwrap());

        clone
    }

    fn rate(invoices: &[Invoice]) -> InvoiceTypeScore {
        let mut score = InvoiceTypeScore::new();
        for invoice in invoices {
            score.push(invoice.invoice_type(), invoice.base_amount.clone());
        }
        score
    }
}

#[derive(Debug, Clone)]
pub struct MajorTypeEntry {
    pub invoice_type: InvoiceType,
    pub value: f64,
}

impl MajorTypeEntry {
    fn new(invoice_type: InvoiceType, value: f64) -> Self {
        Self {
            invoice_type,
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
struct InvoiceTypeScore {
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

impl InvoiceTypeScore {
    pub fn new() -> Self {
        InvoiceTypeScore {
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

    fn push(&mut self, invoice_type: InvoiceType, amount: Option<Amount>) {
        let amount = &match amount {
            Some(a) => a,
            None => return,
        };

        match invoice_type {
            InvoiceType::Car => self.car.push(amount),
            InvoiceType::Clothes => self.clothes.push(amount),
            InvoiceType::Eat => self.eat.push(amount),
            InvoiceType::Gas => self.gas.push(amount),
            InvoiceType::Fun => self.fun.push(amount),
            InvoiceType::Health => self.health.push(amount),
            InvoiceType::Home => self.home.push(amount),
            InvoiceType::Telecommunication => self.telecommunication.push(amount),
            InvoiceType::Unknown => self.unknown.push(amount),
        }
    }
}

impl IntoIterator for InvoiceTypeScore {
    type Item = (InvoiceType, IncomeAndExpenses);

    type IntoIter = std::array::IntoIter<(InvoiceType, IncomeAndExpenses), 9>;

    fn into_iter(self) -> Self::IntoIter {
        [
            (InvoiceType::Car, self.car),
            (InvoiceType::Clothes, self.clothes),
            (InvoiceType::Eat, self.eat),
            (InvoiceType::Gas, self.gas),
            (InvoiceType::Fun, self.fun),
            (InvoiceType::Health, self.health),
            (InvoiceType::Home, self.home),
            (InvoiceType::Telecommunication, self.telecommunication),
            (InvoiceType::Unknown, self.unknown),
        ]
        .into_iter()
    }
}
