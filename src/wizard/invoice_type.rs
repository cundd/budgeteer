use crate::error::Res;
use crate::invoice::invoice_type::InvoiceType;
use dialoguer::theme::Theme;
use dialoguer::Select;

pub fn read_invoice_type(theme: &dyn Theme) -> Res<InvoiceType> {
    let all = InvoiceType::all_known();
    let i = Select::with_theme(theme)
        .with_prompt("Type")
        .default(0)
        .items(&all[..])
        .interact()?;

    Ok(all[i])
}
