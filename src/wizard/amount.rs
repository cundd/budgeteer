use crate::error::Res;
use dialoguer::theme::Theme;
use dialoguer::Input;

pub fn read_amount(theme: &dyn Theme) -> Res<f64> {
    let raw_amount = Input::<String>::with_theme(theme)
        .with_prompt("Amount")
        .interact_text()?;

    let raw_amount_normalized = if raw_amount.contains(',') {
        raw_amount.replace(',', ".")
    } else {
        raw_amount
    };

    match raw_amount_normalized.parse::<f64>() {
        Ok(c) => Ok(c),
        Err(_) => {
            println!("Please enter a valid amount");
            read_amount(theme)
        }
    }
}
