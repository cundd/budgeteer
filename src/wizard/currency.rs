use dialoguer::Completion;

use crate::currency::currency_data::all;
use crate::currency::Currency;

use crate::error::Res;
use dialoguer::theme::Theme;
use dialoguer::Input;

pub fn read_currency(theme: &dyn Theme) -> Res<Currency> {
    let completion = CurrentCompletion::default();
    let raw_currency = Input::<String>::with_theme(theme)
        .with_prompt("Currency")
        .default("€".to_owned())
        .completion_with(&completion)
        .interact_text()?
        .to_uppercase();

    match Currency::from_string(&raw_currency) {
        Ok(c) => Ok(c),
        Err(_) => {
            println!("Please enter a valid currency");
            read_currency(theme)
        }
    }
}

struct CurrentCompletion {
    options: Vec<String>,
}

impl Default for CurrentCompletion {
    fn default() -> Self {
        CurrentCompletion {
            options: all().keys().map(|k| str::to_string(k)).collect(),
        }
    }
}

impl Completion for CurrentCompletion {
    /// Simple completion implementation based on substring
    fn get(&self, input: &str) -> Option<String> {
        let input_uppercase = input.to_uppercase();
        let matches = self
            .options
            .iter()
            .filter(|option| option.starts_with(&input_uppercase))
            .collect::<Vec<_>>();

        if matches.len() == 1 {
            Some(matches[0].to_string())
        } else {
            None
        }
    }
}
