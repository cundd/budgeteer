use std::fmt;

#[derive(Clone, PartialOrd, PartialEq, Copy, Debug)]
pub enum InvoiceType {
    // "A" => Car / Auto
    Car,
    // "C" => Clothes & Cosmetics / Kleidung
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
            "C" | "K" | "CLOTHES" | "KLEIDUNG" => InvoiceType::Clothes,
            "E" | "EAT" | "ESSEN" => InvoiceType::Eat,
            "F" | "FUN" | "HOBBY" => InvoiceType::Fun,
            "T" | "GAS" | "TANKEN" => InvoiceType::Gas,
            "G" | "HEALTH" | "GESUNDHEIT" => InvoiceType::Health,
            "H" | "HOME" | "HAUS" => InvoiceType::Home,
            "I" | "TELECOMMUNICATION" | "INTERNET" | "HANDY" | "TV" => InvoiceType::Telecommunication,
            _ => InvoiceType::Unknown
        }
    }
}


// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for InvoiceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match self {
            &InvoiceType::Car => "Car / Auto",
            &InvoiceType::Clothes => "Clothes / Kleidung",
            &InvoiceType::Eat => "Food / Essen",
            &InvoiceType::Fun => "Fun / Freunde / Hobby",
            &InvoiceType::Gas => "Gas / Tanken",
            &InvoiceType::Health => "Health / Gesundheit",
            &InvoiceType::Home => "Home / Hause",
            &InvoiceType::Telecommunication => "Internet / Handy / TV",
            &InvoiceType::Unknown => "Diverse",
        };

        write!(f, "{}", description)
    }
}
