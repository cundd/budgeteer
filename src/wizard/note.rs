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
    options: Vec<String>,
}

impl NoteCompletion {
    fn new(invoices: &[Invoice]) -> Self {
        // Get unique notes
        let notes = invoices
            .iter()
            .filter_map(|i| match i.note() {
                Some(note) if note.is_empty() => None,
                Some(note) => Some(note),
                None => None,
            })
            .collect::<HashSet<String>>();

        // Sort the notes
        let mut options = Vec::from_iter(notes.into_iter());
        options.sort();

        Self { options }
    }
}

impl Completion for NoteCompletion {
    /// Simple completion implementation based on substring
    fn get(&self, input: &str) -> Option<String> {
        let input_uppercase = input.to_uppercase();

        self.options
            .iter()
            .find(|option| option.to_uppercase().starts_with(&input_uppercase))
            .cloned()
    }
}
