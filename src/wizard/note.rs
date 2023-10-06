use dialoguer::Completion;

use crate::error::Res;
use dialoguer::theme::Theme;
use dialoguer::Input;

pub fn read_note(theme: &dyn Theme) -> Res<String> {
    let completion = NoteCompletion::default();

    Ok(Input::<String>::with_theme(theme)
        .with_prompt("Note")
        .completion_with(&completion)
        .allow_empty(true)
        .interact_text()?)
}

struct NoteCompletion {
    options: Vec<String>,
}

impl Default for NoteCompletion {
    fn default() -> Self {
        NoteCompletion {
            options: vec![
                "Hover".to_string(),
                "Migros".to_string(),
                "Interspar".to_string(),
            ],
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
            .filter(|option| option.starts_with(&input_uppercase))
            .collect::<Vec<_>>();

        if matches.len() == 1 {
            Some(matches[0].to_string())
        } else {
            None
        }
    }
}
