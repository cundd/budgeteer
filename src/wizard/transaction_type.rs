use crate::error::Res;
use crate::transaction::transaction_type::TransactionType;
use dialoguer::theme::Theme;
use dialoguer::Select;

pub fn read_transaction_type(theme: &dyn Theme, allow_unknown: bool) -> Res<TransactionType> {
    let all = if allow_unknown {
        TransactionType::all().to_vec()
    } else {
        TransactionType::all_known().to_vec()
    };
    let i = Select::with_theme(theme)
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
    let selection = Select::with_theme(theme)
        .with_prompt("Type (or press ESC to skip)")
        .default(0)
        .items(&all[..])
        .interact_opt()?;

    match selection {
        Some(i) => Ok(Some(all[i])),
        None => Ok(None),
    }
}
