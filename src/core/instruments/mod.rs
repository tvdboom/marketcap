use crate::core::instruments::bonds::BondQuality;
use crate::core::player::InstrumentKind;

pub mod bonds;
pub mod commodities;
pub mod crypto;

pub trait Instrument {
    fn name(&self) -> String;
    fn lowername(&self) -> String;
    fn description(&self) -> &str;
    fn kind(&self) -> InstrumentKind;
    fn all(&self) -> &Vec<f32>;
    fn current(&self) -> f32;

    /// Calculates the percentage difference from the average of the last 30 values
    fn diff(&self) -> f32 {
        // Add 30 initial values to ensure we always have at least 30 values
        let mut slice = vec![self.all()[0]; 29];
        slice.extend(self.all());

        let len = slice.len();
        let slice = &slice[len - 30..];

        let avg = slice.iter().sum::<f32>() / slice.len() as f32;

        if avg == 0.0 {
            0.0
        } else {
            (self.current() - avg) / avg * 100.
        }
    }

    fn interest(&self) -> f32 {
        0.0
    }
    fn market_cap(&self) -> f32 {
        0.
    }
    fn quality(&self) -> BondQuality {
        BondQuality::InvestmentGrade
    }
    fn storage_cost(&self) -> f32 {
        0.0
    }
    fn volatility(&self) -> f32 {
        0.0
    }
    fn unit(&self) -> String {
        "".to_string()
    }
    fn per_unit(&self) -> String {
        if self.unit().is_empty() {
            "".to_string()
        } else {
            format!("/{}", self.unit())
        }
    }
}
