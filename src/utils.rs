use std::collections::VecDeque;
use std::fmt::Debug;

use bevy_egui::egui::TextStyle;
use chrono::{Datelike, NaiveDate};
use rand::distr::Alphanumeric;
use rand::{Rng, rng};
use regex::Regex;
use serde::{Deserialize, Serialize};

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

/// Approximation of the cumulative distribution function for a standard normal distribution
pub fn norm_cdf(x: f32) -> f32 {
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();

    let t = 1.0 / (1.0 + 0.2316419 * x);
    let a1 = 0.319381530;
    let a2 = -0.356563782;
    let a3 = 1.781477937;
    let a4 = -1.821255978;
    let a5 = 1.330274429;

    let poly = ((((a5 * t + a4) * t + a3) * t + a2) * t + a1) * t;
    let pdf = (-x * x / 2.0).exp() / (2.0 * std::f32::consts::PI).sqrt();
    let cdf = 1.0 - pdf * poly;

    if sign == 1.0 { cdf } else { 1.0 - cdf }
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

/// Trait to enhance floating point numbers with additional methods
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

        let result = match self.abs() {
            n if n < 1. => {
                // Round to first two non-zero decimals
                let mut scaled = n;
                let mut factor = 1.;
                while scaled < 1. {
                    scaled *= 10.;
                    factor *= 10.;
                }

                (n * factor * 10.).round() / (factor * 10.)
            },
            n if n < 10. => n.round1(),
            n => n.floor(),
        };

        result * self.signum()
    }

    fn format(self) -> String {
        match self {
            n if n > 1_000_000_000. => format!("{}B", (self / 1_000_000_000.).clean()),
            n if n > 1_000_000. => format!("{}M", (self / 1_000_000.).clean()),
            n if n >= 1_000. => format!("{}k", (self / 1_000.).clean()),
            n if n >= 1. && n < 10. => format!("{:.1}", self.clean()),
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

/// Deque with a fixed capacity
#[derive(Clone, Serialize, Deserialize)]
pub struct DQueue<T> {
    queue: VecDeque<T>,
}

impl<T> DQueue<T> {
    pub const CAPACITY: usize = 365 * 2; // Default capacity of 2 years

    pub fn from<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            queue: VecDeque::from_iter(iter),
        }
    }

    pub fn push(&mut self, item: T) {
        if self.queue.len() == Self::CAPACITY {
            self.queue.pop_front();
        }
        self.queue.push_back(item);
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.queue.iter()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn front(&self) -> Option<&T> {
        self.queue.front()
    }

    pub fn back(&self) -> Option<&T> {
        self.queue.back()
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.queue.back_mut()
    }
}
