use crate::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, PartialOrd, PartialEq, Copy, Debug)]
pub enum InvoiceType {
    // "A" => Car / Auto
    Car,
    // "C" => Clothes, Body & Cosmetics / Kleidung
    Clothes,
    // "E" => Eat / Essen
    Eat,
    // "T" => Gas / Tanken
    Gas,
    // "F" => Fun / Hobby
    Fun,
    // "G" => Health / Gesundheit
    Health,
    // "H" => Home / Haus
    Home,
    // "I" => Internet / Handy / TV
    Telecommunication,

    // Unknown
    Unknown,
}

impl InvoiceType {
    pub fn from_str(input: &str) -> Self {
        match input.to_uppercase().as_str() {
            "A" | "CAR" | "AUTO" => InvoiceType::Car,
            "C" | "K" | "CLOTHES" | "BODY" | "KLEIDUNG" => InvoiceType::Clothes,
            "E" | "EAT" | "ESSEN" => InvoiceType::Eat,
            "F" | "FUN" | "HOBBY" => InvoiceType::Fun,
            "T" | "GAS" | "TANKEN" => InvoiceType::Gas,
            "G" | "HEALTH" | "GESUNDHEIT" => InvoiceType::Health,
            "H" | "HOME" | "HAUS" => InvoiceType::Home,
            "I" | "TELECOMMUNICATION" | "INTERNET" | "HANDY" | "TV" => {
                InvoiceType::Telecommunication
            }
            _ => InvoiceType::Unknown,
        }
    }

    pub fn identifier(&self) -> char {
        match self {
            InvoiceType::Car => 'A',
            InvoiceType::Clothes => 'C',
            InvoiceType::Eat => 'E',
            InvoiceType::Fun => 'F',
            InvoiceType::Gas => 'T',
            InvoiceType::Health => 'G',
            InvoiceType::Home => 'H',
            InvoiceType::Telecommunication => 'I',
            InvoiceType::Unknown => ' ',
        }
    }

    /// Return all invoice types except `InvoiceType::Unknown`
    pub fn all_known() -> [InvoiceType; 8] {
        [
            InvoiceType::Car,
            InvoiceType::Clothes,
            InvoiceType::Eat,
            InvoiceType::Gas,
            InvoiceType::Fun,
            InvoiceType::Health,
            InvoiceType::Home,
            InvoiceType::Telecommunication,
        ]
    }

    /// Return all invoice types (including `InvoiceType::Unknown`)
    pub fn all() -> [InvoiceType; 8] {
        [
            InvoiceType::Car,
            InvoiceType::Clothes,
            InvoiceType::Eat,
            InvoiceType::Gas,
            InvoiceType::Fun,
            InvoiceType::Health,
            InvoiceType::Home,
            InvoiceType::Telecommunication,
        ]
    }
}

impl fmt::Display for InvoiceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            InvoiceType::Car => "Car / Auto",
            InvoiceType::Clothes => "Clothes / Kleidung",
            InvoiceType::Eat => "Food / Essen",
            InvoiceType::Fun => "Fun / Freunde / Hobby",
            InvoiceType::Gas => "Gas / Tanken",
            InvoiceType::Health => "Health / Gesundheit",
            InvoiceType::Home => "Home / Hause",
            InvoiceType::Telecommunication => "Internet / Handy / TV",
            InvoiceType::Unknown => "Diverse",
        };

        write!(f, "{}", description)
    }
}

impl FromStr for InvoiceType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(InvoiceType::from_str(s))
    }
}
