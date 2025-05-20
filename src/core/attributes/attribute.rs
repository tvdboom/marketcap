pub trait Attribute {
    fn image(&self) -> &str;
    fn description(&self) -> String;
    fn current(&self) -> f32;
}
