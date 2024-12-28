use crate::invoice::invoice_type::InvoiceType;
use std::fmt;

#[derive(Clone)]
pub struct Request {
    year: Option<i32>,
    month: Option<u32>,
    day: Option<u32>,
    invoice_type: Option<InvoiceType>,
}

impl Request {
    pub fn new(
        year: Option<i32>,
        month: Option<u32>,
        day: Option<u32>,
        invoice_type: Option<InvoiceType>,
    ) -> Self {
        Request {
            year,
            month,
            day,
            invoice_type,
        }
    }

    pub fn with_month(&self, month: u32) -> Self {
        let mut clone = self.clone();
        clone.month = Some(month);

        clone
    }

    pub fn empty(&self) -> bool {
        self.year.is_none()
            && self.month.is_none()
            && self.day.is_none()
            && self.invoice_type.is_none()
    }

    pub fn year(&self) -> Option<i32> {
        self.year
    }

    pub fn month(&self) -> Option<u32> {
        self.month
    }

    pub fn day(&self) -> Option<u32> {
        self.day
    }

    pub fn invoice_type(&self) -> Option<InvoiceType> {
        self.invoice_type
    }
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let has_type = self.invoice_type.is_some();
        let has_year = self.year.is_some();
        let has_month = self.month.is_some();
        let has_day = self.day.is_some();

        let mut s = String::new();
        if has_type {
            s.push_str(&format!("Typ: {}", self.invoice_type.unwrap()));
            if has_year || has_month || has_day {
                s.push_str(", ");
            }
        }
        if has_year {
            s.push_str(&format!("Jahr: {}", self.year.unwrap()));
            if has_month || has_day {
                s.push_str(", ");
            }
        }
        if has_month {
            s.push_str(&format!("Monat: {}", self.month.unwrap()));
            if has_day {
                s.push_str(" & ");
            }
        }
        if has_day {
            s.push_str(&format!("Tag: {}", self.day.unwrap()));
        }

        f.write_str(s.as_str())
    }
}
