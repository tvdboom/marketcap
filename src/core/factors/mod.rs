pub mod cash;
pub mod credit_score;
pub mod economy;
pub mod inflation;
pub mod interest;

pub trait Factor {
    fn image(&self) -> &str;
    fn description(&self) -> String;
    fn current(&self) -> f32;
}
