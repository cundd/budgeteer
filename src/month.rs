use std::fmt;
use std::convert::From;

#[allow(dead_code)]
pub enum Month {
    January = 1,
    February = 2,
    March = 3,
    April = 4,
    May = 5,
    June = 6,
    July = 7,
    August = 8,
    September = 9,
    October = 10,
    November = 11,
    December = 12,
}

impl Month {
    pub fn name_translate(&self) -> &str {
        match self {
            Month::January => "Jänner",
            Month::February => "Februar",
            Month::March => "März",
            Month::April => "April",
            Month::May => "Mai",
            Month::June => "Juni",
            Month::July => "Juli",
            Month::August => "August",
            Month::September => "September",
            Month::October => "Oktober",
            Month::November => "November",
            Month::December => "Dezember",
        }
    }

    fn from_int(i: u64) -> Self {
        match i {
            1 => Month::January,
            2 => Month::February,
            3 => Month::March,
            4 => Month::April,
            5 => Month::May,
            6 => Month::June,
            7 => Month::July,
            8 => Month::August,
            9 => Month::September,
            10 => Month::October,
            11 => Month::November,
            12 => Month::December,
            _ => panic!("Month {} out of bound", i)
        }
    }
}

impl From<u8> for Month {
    fn from(item: u8) -> Self {
        Month::from_int(item as u64)
    }
}

impl From<u32> for Month {
    fn from(item: u32) -> Self {
        Month::from_int(item as u64)
    }
}

impl From<u64> for Month {
    fn from(item: u64) -> Self {
        Month::from_int(item)
    }
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name_translate())
    }
}
