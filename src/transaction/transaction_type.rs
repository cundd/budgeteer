use crate::error::Error;
use std::str::FromStr;
use std::{char, fmt};

pub const NUMBER_OF_TYPES: usize = 15;

#[derive(Clone, PartialOrd, PartialEq, Eq, Copy, Debug, Hash, clap::ValueEnum, sqlx::Type)]
#[repr(u8)]
pub enum TransactionType {
    // "B" => Body / Cosmetics
    Body = b'B',
    // "A" => Car / Auto
    Car = b'A',
    // "C" => Clothes / Kleidung
    Clothes = b'C',
    // "E" => Eat / Essen
    Eat = b'E',
    // "U" => Education / Ausbildung
    Education = b'N',
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
    // "S" => Insurance / Versicherung
    Insurance = b'S',
    // "D" => Spende
    Donation = b'D',
    // "T" => Tax / Steuern
    Tax = b'X',
    // "J" => Banking
    Banking = b'J',

    // Unknown
    Unknown = b'U',
}

impl TransactionType {
    pub fn from_str(input: &str) -> Self {
        match input.to_uppercase().as_str() {
            "A" | "CAR" | "AUTO" => TransactionType::Car,
            "B" | "BODY" | "COSMETICS" => TransactionType::Body,
            "C" | "K" | "CLOTHES" | "KLEIDUNG" => TransactionType::Clothes,
            "E" | "EAT" | "ESSEN" => TransactionType::Eat,
            "N" | "EDUCATION" | "AUSBILDUNG" => TransactionType::Education,
            "F" | "FUN" | "HOBBY" => TransactionType::Fun,
            "T" | "GAS" | "TANKEN" => TransactionType::Gas,
            "G" | "HEALTH" | "GESUNDHEIT" => TransactionType::Health,
            "D" | "DONATION" | "SPENDE" => TransactionType::Donation,
            "I" | "TELECOMMUNICATION" | "INTERNET" | "HANDY" | "TV" => {
                TransactionType::Telecommunication
            }
            "S" | "INSURANCE" | "VERSICHERUNG" => TransactionType::Insurance,
            "X" | "TAX" | "TAXES" | "STEUERN" | "STEUER" => TransactionType::Tax,
            "H" | "HOUSE" | "HOME" => TransactionType::Home,
            "J" | "BANK" | "BANKING" => TransactionType::Banking,
            _ => TransactionType::Unknown,
        }
    }

    pub fn identifier(&self) -> char {
        char::from_u32(*self as isize as u32).unwrap()
    }

    /// Return all transaction types except `TransactionType::Unknown`
    pub fn all_known() -> [TransactionType; NUMBER_OF_TYPES - 1] {
        [
            TransactionType::Body,
            TransactionType::Car,
            TransactionType::Clothes,
            TransactionType::Eat,
            TransactionType::Education,
            TransactionType::Gas,
            TransactionType::Fun,
            TransactionType::Health,
            TransactionType::Home,
            TransactionType::Insurance,
            TransactionType::Telecommunication,
            TransactionType::Donation,
            TransactionType::Tax,
            TransactionType::Banking,
        ]
    }

    /// Return all transaction types (including `TransactionType::Unknown`)
    pub fn all() -> [TransactionType; NUMBER_OF_TYPES] {
        [
            TransactionType::Body,
            TransactionType::Car,
            TransactionType::Clothes,
            TransactionType::Eat,
            TransactionType::Education,
            TransactionType::Gas,
            TransactionType::Fun,
            TransactionType::Health,
            TransactionType::Home,
            TransactionType::Telecommunication,
            TransactionType::Insurance,
            TransactionType::Donation,
            TransactionType::Tax,
            TransactionType::Banking,
            TransactionType::Unknown,
        ]
    }

    pub fn to_str(self) -> &'static str {
        match self {
            TransactionType::Body => "Body / Cosmetics",
            TransactionType::Car => "Car / Auto",
            TransactionType::Clothes => "Clothes / Kleidung",
            TransactionType::Eat => "Food / Essen",
            TransactionType::Education => "Education / Ausbildung",
            TransactionType::Fun => "Fun / Freunde / Hobby",
            TransactionType::Gas => "Gas / Tanken",
            TransactionType::Health => "Health / Gesundheit",
            TransactionType::Home => "Home / Haus",
            TransactionType::Telecommunication => "Internet / Handy / TV",
            TransactionType::Insurance => "Insurance / Versicherung",
            TransactionType::Donation => "Donation / Spende",
            TransactionType::Tax => "Tax / Steuer",
            TransactionType::Banking => "Bank / Banking",
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
