use std::fmt::Debug;

use bevy_egui::egui::TextStyle;
use chrono::{Datelike, NaiveDate};
use rand::distr::Alphanumeric;
use rand::{Rng, rng};
use regex::Regex;

use crate::core::constants::HEIGHT;

/// Get the text size ratio depending on the window size
pub fn get_ratio(width: f32, height: f32, style: TextStyle) -> f32 {
    let ratio = width.min(height).min(1.2 * HEIGHT);

    match style {
        TextStyle::Small => ratio * 0.016,
        TextStyle::Body => ratio * 0.018,
        TextStyle::Button => ratio * 0.021,
        TextStyle::Heading => ratio * 0.035,
        TextStyle::Monospace => ratio * 0.024,
        _ => unreachable!(), // We don't use custom text styles
    }
}

/// Create a random 5-character GUID
pub fn create_guid() -> String {
    rng()
        .sample_iter(&Alphanumeric)
        .take(5)
        .map(char::from)
        .map(|c| c.to_ascii_uppercase())
        .collect()
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

pub trait EnhFloat {
    fn round1(self) -> Self;
    fn clean(self) -> Self;
    fn format(self) -> String;
    fn signed(self) -> String;
}

impl EnhFloat for f32 {
    fn round1(self) -> Self {
        (self * 10.).round() / 10.
    }

    fn clean(self) -> Self {
        if self == 0. {
            return 0.;
        }

        match self {
            n if n < 1. => {
                // Round to first two non-zero decimals
                let mut scaled = self.abs();
                let mut factor = 1.0;
                while scaled < 1.0 {
                    scaled *= 10.0;
                    factor *= 10.0;
                }

                (self * factor * 10.0).round() / (factor * 10.0)
            },
            n if n < 10. => n.round1(),
            _ => self.floor(),
        }
    }

    fn format(self) -> String {
        match self {
            n if n > 1_000_000_000. => format!("{}B", (self / 1_000_000_000.).clean()),
            n if n > 1_000_000. => format!("{}M", (self / 1_000_000.).clean()),
            n if n >= 1_000. => format!("{}k", (self / 1_000.).clean()),
            _ => format!("{}", self.clean()),
        }
    }

    fn signed(self) -> String {
        match self.clean() {
            x if x > 0. => format!("+{}", x),
            x if x < 0. => x.to_string(),
            _ => "0".to_string(),
        }
    }
}
