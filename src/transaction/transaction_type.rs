use crate::error::Error;
use std::str::FromStr;
use std::{char, fmt};

#[derive(Clone, PartialOrd, PartialEq, Eq, Copy, Debug, Hash, clap::ValueEnum, sqlx::Type)]
#[repr(u8)]
pub enum TransactionType {
    // "A" => Car / Auto
    Car = b'A',
    // "C" => Clothes, Body & Cosmetics / Kleidung
    Clothes = b'C',
    // "E" => Eat / Essen
    Eat = b'E',
    // "T" => Gas / Tanken
    Gas = b'T',
    // "F" => Fun / Hobby
    Fun = b'F',
    // "G" => Health / Gesundheit
    Health = b'G',
    // "H" => Home / Haus
    Home = b'H',
    // "I" => Internet / Handy / TV
    Telecommunication = b'I',

    // Unknown
    Unknown = b'U',
}

impl TransactionType {
    pub fn from_str(input: &str) -> Self {
        match input.to_uppercase().as_str() {
            "A" | "CAR" | "AUTO" => TransactionType::Car,
            "C" | "K" | "CLOTHES" | "BODY" | "KLEIDUNG" => TransactionType::Clothes,
            "E" | "EAT" | "ESSEN" => TransactionType::Eat,
            "F" | "FUN" | "HOBBY" => TransactionType::Fun,
            "T" | "GAS" | "TANKEN" => TransactionType::Gas,
            "G" | "HEALTH" | "GESUNDHEIT" => TransactionType::Health,
            "H" | "HOME" | "HAUS" => TransactionType::Home,
            "I" | "TELECOMMUNICATION" | "INTERNET" | "HANDY" | "TV" => {
                TransactionType::Telecommunication
            }
            _ => TransactionType::Unknown,
        }
    }

    pub fn identifier(&self) -> char {
        char::from_u32(*self as isize as u32).unwrap()
    }

    /// Return all transaction types except `TransactionType::Unknown`
    pub fn all_known() -> [TransactionType; 8] {
        [
            TransactionType::Car,
            TransactionType::Clothes,
            TransactionType::Eat,
            TransactionType::Gas,
            TransactionType::Fun,
            TransactionType::Health,
            TransactionType::Home,
            TransactionType::Telecommunication,
        ]
    }

    /// Return all transaction types (including `TransactionType::Unknown`)
    pub fn all() -> [TransactionType; 9] {
        [
            TransactionType::Car,
            TransactionType::Clothes,
            TransactionType::Eat,
            TransactionType::Gas,
            TransactionType::Fun,
            TransactionType::Health,
            TransactionType::Home,
            TransactionType::Telecommunication,
            TransactionType::Unknown,
        ]
    }

    pub fn to_str(self) -> &'static str {
        match self {
            TransactionType::Car => "Car / Auto",
            TransactionType::Clothes => "Clothes / Kleidung",
            TransactionType::Eat => "Food / Essen",
            TransactionType::Fun => "Fun / Freunde / Hobby",
            TransactionType::Gas => "Gas / Tanken",
            TransactionType::Health => "Health / Gesundheit",
            TransactionType::Home => "Home / Hause",
            TransactionType::Telecommunication => "Internet / Handy / TV",
            TransactionType::Unknown => "Diverse",
        }
    }
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.to_str())
    }
}

impl FromStr for TransactionType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TransactionType::from_str(s))
    }
}
