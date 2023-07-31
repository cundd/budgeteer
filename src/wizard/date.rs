use crate::error::Res;
use chrono::{Datelike, Local, NaiveDate};
use dialoguer::theme::Theme;
use dialoguer::Input;

pub fn read_date(theme: &dyn Theme) -> Res<NaiveDate> {
    let raw_date = Input::<String>::with_theme(theme)
        .with_prompt("Date (dd.mm.yyyy)")
        .default(Local::now().format("%d.%m.%Y").to_string())
        .interact()?;

    let prepared_raw_date = prepare_raw_date(raw_date);

    match NaiveDate::parse_from_str(&prepared_raw_date, "%d.%m.%Y") {
        Ok(d) => {
            let parsed_date_string = d.format("%d.%m.%Y").to_string();
            if prepared_raw_date != parsed_date_string {
                println!("{}", parsed_date_string);
            }
            Ok(d)
        }
        Err(_) => read_date(theme),
    }
}

fn prepare_raw_date<S: Into<String>>(raw_date: S) -> String {
    let raw_date_string = raw_date.into();
    {
        let parts: Vec<_> = raw_date_string
            .split('.')
            .filter(|p| !p.trim().is_empty())
            .filter_map(|p| p.parse::<u32>().ok())
            .collect();
        let len = parts.len();
        let now = Local::now();
        if len == 2 {
            // The month is higher than the current month -> it must have been last year
            let year = if parts[1] > now.month() {
                now.year() - 1
            } else {
                now.year()
            };
            return format!("{}.{}.{:02}", parts[0], parts[1], year);
        } else if len == 1 {
            // The day is higher than the current day -> it must have been last year
            let year = if parts[0] > now.day() {
                now.year() - 1
            } else {
                now.year()
            };
            return format!("{}.{:02}.{:02}", parts[0], now.month(), year);
        } else {
        }
    }

    raw_date_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prepare_raw_date() {
        let now_m_y = Local::now().format("%m.%Y").to_string();
        assert_eq!(prepare_raw_date("23"), format!("23.{}", now_m_y));
        assert_eq!(prepare_raw_date("3"), format!("3.{}", now_m_y));
        assert_eq!(prepare_raw_date("03"), format!("03.{}", now_m_y));

        assert_eq!(prepare_raw_date("23."), format!("23.{}", now_m_y));
        assert_eq!(prepare_raw_date("3."), format!("3.{}", now_m_y));
        assert_eq!(prepare_raw_date("03."), format!("03.{}", now_m_y));

        let now_y = Local::now().format("%Y").to_string();
        assert_eq!(prepare_raw_date("23.11."), format!("23.11.{}", now_y));
        assert_eq!(prepare_raw_date("3.2."), format!("3.2.{}", now_y));
        assert_eq!(prepare_raw_date("03.04."), format!("03.04.{}", now_y));

        assert_eq!(prepare_raw_date("23.11"), format!("23.11.{}", now_y));
        assert_eq!(prepare_raw_date("3.2"), format!("3.2.{}", now_y));
        assert_eq!(prepare_raw_date("03.04"), format!("03.04.{}", now_y));
    }
}
