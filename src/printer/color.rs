use std::env;

use crossterm::style::Color;

use crate::transaction::transaction_type::{TransactionType, NUMBER_OF_TYPES};

pub(super) fn color_for_type(transaction_type: TransactionType, light: bool) -> Color {
    let index = TransactionType::all()
        .iter()
        .position(|t| *t == transaction_type)
        .unwrap();

    if !has_true_color_support() {
        let difference = (229 - 124) / NUMBER_OF_TYPES;
        let value: u8 = (difference * index).try_into().unwrap();

        if light {
            Color::AnsiValue(124 + value)
        } else {
            Color::AnsiValue(value)
        }
    } else {
        let difference = 360 / NUMBER_OF_TYPES;

        Hsl {
            hue: (difference * index) as f32,
            saturation: 0.9,
            luminence: if light { 0.7 } else { 0.4 },
        }
        .to_rgb()
    }
}

pub(super) fn color_for_income() -> Color {
    if !has_true_color_support() {
        Color::Green
    } else {
        Color::Rgb {
            r: 6,
            g: 168,
            b: 59,
        }
    }
}

pub(super) fn color_for_expenses() -> Color {
    if !has_true_color_support() {
        Color::Red
    } else {
        Color::Rgb {
            r: 232,
            g: 53,
            b: 32,
        }
    }
}

fn has_true_color_support() -> bool {
    match env::var("COLORTERM") {
        Ok(v) => v == "truecolor",
        Err(_) => false,
    }
}

#[derive(Clone, Debug)]
struct Hsl {
    hue: f32,
    saturation: f32,
    luminence: f32,
}

impl Hsl {
    fn to_rgb(&self) -> Color {
        let chroma = (1.0 - (2.0 * self.luminence - 1.0).abs()) * self.saturation;
        let hue_as_float = self.hue;
        let x = chroma * (1.0 - ((hue_as_float / 60.0 % 2.0) - 1.0).abs());

        // We add this amount to each channel to account for luminence
        let match_value = self.luminence - (chroma / 2.0);
        let mut red: f32 = match_value;
        let mut green: f32 = match_value;
        let mut blue: f32 = match_value;
        if self.hue >= 0.0 && self.hue < 60.0 || self.hue == 360.0 {
            red += chroma;
            green += x;
        } else if self.hue >= 60.0 && self.hue < 120.0 {
            red += x;
            green += chroma;
        } else if self.hue >= 120.0 && self.hue < 180.0 {
            green += chroma;
            blue += x;
        } else if self.hue >= 180.0 && self.hue < 240.0 {
            green += x;
            blue += chroma;
        } else if self.hue >= 240.0 && self.hue < 300.0 {
            red += x;
            blue += chroma;
        } else if self.hue >= 300.0 && self.hue < 360.0 {
            red += chroma;
            blue += x;
        } else {
            panic!("Out of bounds");
        }

        let red_int: u8 = (255.0 * red).round() as u8;
        let green_int: u8 = (255.0 * green).round() as u8;
        let blue_int: u8 = (255.0 * blue).round() as u8;

        Color::Rgb {
            r: red_int,
            g: green_int,
            b: blue_int,
        }
    }
}
