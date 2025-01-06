use crate::error::Error;
use std::str::FromStr;
use std::{char, fmt};

#[derive(Clone, PartialOrd, PartialEq, Eq, Copy, Debug, Hash, clap::ValueEnum, sqlx::Type)]
#[repr(u8)]
pub enum InvoiceType {
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
        char::from_u32(*self as isize as u32).unwrap()
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
    pub fn all() -> [InvoiceType; 9] {
        [
            InvoiceType::Car,
            InvoiceType::Clothes,
            InvoiceType::Eat,
            InvoiceType::Gas,
            InvoiceType::Fun,
            InvoiceType::Health,
            InvoiceType::Home,
            InvoiceType::Telecommunication,
            InvoiceType::Unknown,
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

        f.write_str(description)
    }
}

impl FromStr for InvoiceType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(InvoiceType::from_str(s))
    }
}
