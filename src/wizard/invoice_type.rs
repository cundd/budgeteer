use crate::error::Res;
use crate::invoice::invoice_type::InvoiceType;
use dialoguer::theme::Theme;
use dialoguer::Select;

pub fn read_invoice_type(theme: &dyn Theme, allow_unknown: bool) -> Res<InvoiceType> {
    let all = if allow_unknown {
        InvoiceType::all().to_vec()
    } else {
        InvoiceType::all_known().to_vec()
    };
    let i = Select::with_theme(theme)
        .with_prompt("Type")
        .default(0)
        .items(&all[..])
        .interact()?;

    Ok(all[i])
}

pub fn read_invoice_type_or_skip(
    theme: &dyn Theme,
    allow_unknown: bool,
) -> Res<Option<InvoiceType>> {
    let all = if allow_unknown {
        InvoiceType::all().to_vec()
    } else {
        InvoiceType::all_known().to_vec()
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
