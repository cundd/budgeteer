use super::WizardTrait;
use crate::error::Res;
use crate::invoice::Invoice;
use dialoguer::theme::Theme;
use dialoguer::Completion;
use dialoguer::Input;
use std::collections::HashSet;

#[derive(Default)]
pub struct NoteWizard {}

impl WizardTrait<String> for NoteWizard {
    fn read(&self, theme: &dyn Theme, invoices: &[Invoice]) -> Res<String> {
        let completion = NoteCompletion::new(invoices);

        Ok(Input::<String>::with_theme(theme)
            .with_prompt("Note")
            .completion_with(&completion)
            .allow_empty(true)
            .interact_text()?)
    }
}

struct NoteCompletion {
    options: HashSet<String>,
}

impl NoteCompletion {
    fn new(invoices: &[Invoice]) -> Self {
        Self {
            options: invoices.iter().filter_map(|i| i.note()).collect(),
        }
    }
}

impl Completion for NoteCompletion {
    /// Simple completion implementation based on substring
    fn get(&self, input: &str) -> Option<String> {
        let input_uppercase = input.to_uppercase();
        let matches = self
            .options
            .iter()
            .filter(|option| option.to_uppercase().starts_with(&input_uppercase))
            .collect::<Vec<_>>();

        if !matches.is_empty() {
            Some(matches[0].to_string())
        } else {
            None
        }
    }
}
