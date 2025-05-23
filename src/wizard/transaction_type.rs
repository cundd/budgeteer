use crate::error::Res;
use crate::transaction::transaction_type::TransactionType;
use dialoguer::theme::Theme;
use dialoguer::FuzzySelect;

pub fn read_transaction_type(theme: &dyn Theme, allow_unknown: bool) -> Res<TransactionType> {
    let all = if allow_unknown {
        TransactionType::all().to_vec()
    } else {
        TransactionType::all_known().to_vec()
    };
    let i = FuzzySelect::with_theme(theme)
        .with_prompt("Type")
        .default(0)
        .items(&all[..])
        .interact()?;

    Ok(all[i])
}

pub fn read_transaction_type_or_skip(
    theme: &dyn Theme,
    allow_unknown: bool,
) -> Res<Option<TransactionType>> {
    let all = if allow_unknown {
        TransactionType::all().to_vec()
    } else {
        TransactionType::all_known().to_vec()
    };
    let selection = FuzzySelect::with_theme(theme)
        .with_prompt("Select type (or press ESC to ignore this transaction)")
        .default(0)
        .items(&all[..])
        .interact_opt()?;

    match selection {
        Some(i) => Ok(Some(all[i])),
        None => Ok(None),
    }
}
