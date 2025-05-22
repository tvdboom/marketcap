use chrono::{Datelike, NaiveDate};
use regex::Regex;
use std::fmt::Debug;

/// Gets the last day of the month for a given date
pub fn last_day_of_next_month(date: NaiveDate) -> NaiveDate {
    let (mut y, mut m) = (date.year(), date.month() + 1);

    if m > 12 {
        m = 1;
        y += 1;
    }

    NaiveDate::from_ymd_opt(y, m % 12 + 1, 1)
        .unwrap()
        .pred_opt()
        .unwrap()
}

/// Helper function to extract only the variant name (removes tuple/struct fields)
fn extract_variant_name(text: String) -> String {
    text.split_once('(')
        .or_else(|| text.split_once('{'))
        .map(|(variant, _)| variant)
        .unwrap_or(&text)
        .trim_matches(&['"', ' '][..])
        .to_string()
}

/// Trait to get the text of an enum variant
pub trait NameFromEnum {
    fn to_name(&self) -> String;
}

impl<T: Debug> NameFromEnum for T {
    fn to_name(&self) -> String {
        let re = Regex::new(r"([a-z])([A-Z])").unwrap();

        let text = extract_variant_name(format!("{:?}", self));
        re.replace_all(&text, "$1 $2").to_string()
    }
}
