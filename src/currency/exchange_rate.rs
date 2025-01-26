use super::Currency;

#[derive(Clone, Debug, PartialEq, sqlx::FromRow)]
pub struct ExchangeRate {
    pub year: i32,
    pub month: i64,
    pub day: i64,
    pub base_currency: Currency,
    pub currency: Currency,
    pub rate: f64,
}

// impl FromRow<'_, SqliteRow> for ExchangeRate {
//     fn from_row(row: &SqliteRow) -> sqlx::Result<Self> {
//         let currency = Currency::from_str(row.try_get("currency")?).map_err(|e| {
//             sqlx::Error::ColumnDecode {
//                 index: "currency".to_owned(),
//                 source: Box::new(e),
//             }
//         })?;
//
//         let amount = Amount::new(row.try_get("amount")?, &currency);
//
//         Ok(Self {
//             date: row.try_get("date")?,
//             amount,
//             base_amount: None,
//             transaction_type: row.try_get("type")?,
//             note: row.try_get("note")?,
//         })
//     }
// }
