use std::fmt::Debug;

use chrono::{Datelike, NaiveDate};
use rand::distr::Alphanumeric;
use rand::{Rng, rng};
use regex::Regex;

/// Create a random 5-character GUID
pub fn create_guid() -> String {
    rng()
        .sample_iter(&Alphanumeric)
        .take(5)
        .map(char::from)
        .map(|c| c.to_ascii_uppercase())
        .collect()
}

/// Format a number with k suffix
pub fn format_number(number: f32) -> String {
    if number >= 1_000_000. {
        format!("{:.1}M", number / 1_000_000.)
    } else if number >= 1_000. {
        format!("{:.1}k", number / 1_000.)
    } else {
        format!("{}", number.floor())
    }
}

/// Gets the first day of the next month after the next month
pub fn first_day_in_two_months(date: NaiveDate) -> NaiveDate {
    let (mut month, mut year) = (date.month() + 2, date.year());

    if month > 12 {
        month -= 12;
        year += 1;
    }

    NaiveDate::from_ymd_opt(year, month, 1).expect(format!("Invalid date: {}", date).as_str())
}

/// Extract only the variant name (removes tuple/struct fields)
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
    fn to_lowername(&self) -> String;
}

impl<T: Debug> NameFromEnum for T {
    fn to_name(&self) -> String {
        let re = Regex::new(r"([a-z])([A-Z])").unwrap();

        let text = extract_variant_name(format!("{:?}", self));
        re.replace_all(&text, "$1 $2").to_string()
    }

    fn to_lowername(&self) -> String {
        self.to_name().to_lowercase()
    }
}

/// Trait to round a number to one decimal place
pub trait Round1 {
    fn round1(self) -> Self;
}

impl Round1 for f32 {
    fn round1(self) -> Self {
        (self * 10.).round() / 10.
    }
}
