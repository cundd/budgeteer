use crate::invoice::Invoice;
use crate::invoice::invoice_type::InvoiceType;
use crate::invoice::amount::Amount;

pub struct Calculator {}

impl Calculator {
    pub fn sum(invoices: &[Invoice]) -> f64 {
        let mut sum = 0.0;
        for invoice in invoices {
            if let Some(a) = invoice.base_amount() { sum += a.value() }
        }
        sum
    }

    pub fn major_type(invoices: &[Invoice]) -> Option<InvoiceType> {
        let r = Calculator::rate(invoices);

        let mut major_type: Option<InvoiceType> = None;
        let mut max = 0.0;

        if r.car > max {
            max = r.car;
            major_type = Some(InvoiceType::Car);
        }
        if r.clothes > max {
            max = r.clothes;
            major_type = Some(InvoiceType::Clothes);
        }
        if r.eat > max {
            max = r.eat;
            major_type = Some(InvoiceType::Eat);
        }
        if r.gas > max {
            max = r.gas;
            major_type = Some(InvoiceType::Gas);
        }
        if r.fun > max {
            max = r.fun;
            major_type = Some(InvoiceType::Fun);
        }
        if r.health > max {
            max = r.health;
            major_type = Some(InvoiceType::Health);
        }
        if r.home > max {
            max = r.home;
            major_type = Some(InvoiceType::Home);
        }
        if r.telecommunication > max {
            max = r.telecommunication;
            major_type = Some(InvoiceType::Telecommunication);
        }
        if r.unknown > max {
            major_type = Some(InvoiceType::Unknown);
        }

        major_type
    }

    pub fn sum_for_type(invoices: &[Invoice], invoice_type: InvoiceType) -> f64 {
        let mut sum = 0.0;
        for invoice in invoices {
            if invoice.invoice_type() == invoice_type {
                if let Some(a) = invoice.base_amount() { sum += a.value() }
            }
        }
        sum
    }

    #[allow(unused)]
    pub fn sort(invoices: &Vec<Invoice>) -> Vec<Invoice> {
        let mut clone = invoices.clone();
        clone.sort_by(|a, b| a.partial_cmp(b).map(|s| s.reverse()).unwrap());

        clone
    }

    fn rate(invoices: &[Invoice]) -> InvoiceTypeScore {
        let mut score = InvoiceTypeScore::new();
        for invoice in invoices {
            score.add(invoice.invoice_type(), invoice.base_amount());
        }
        score
    }
}

struct InvoiceTypeScore {
    car: f64,
    clothes: f64,
    eat: f64,
    gas: f64,
    fun: f64,
    health: f64,
    home: f64,
    telecommunication: f64,
    unknown: f64,
}

impl InvoiceTypeScore {
    pub fn new() -> Self {
        InvoiceTypeScore {
            car: 0.0,
            clothes: 0.0,
            eat: 0.0,
            gas: 0.0,
            fun: 0.0,
            health: 0.0,
            home: 0.0,
            telecommunication: 0.0,
            unknown: 0.0,
        }
    }

    fn add(&mut self, invoice_type: InvoiceType, amount: Option<Amount>) {
        let amount = match amount {
            Some(a) => a.value(),
            None => 0.0
        };

        match invoice_type {
            InvoiceType::Car => self.car += amount,
            InvoiceType::Clothes => self.clothes += amount,
            InvoiceType::Eat => self.eat += amount,
            InvoiceType::Gas => self.gas += amount,
            InvoiceType::Fun => self.fun += amount,
            InvoiceType::Health => self.health += amount,
            InvoiceType::Home => self.home += amount,
            InvoiceType::Telecommunication => self.telecommunication += amount,
            InvoiceType::Unknown => self.unknown += amount,
        }
    }
}

