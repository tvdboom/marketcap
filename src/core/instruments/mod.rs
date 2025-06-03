pub mod commodities;

pub trait Instrument {
    fn name(&self) -> String;
    fn lowername(&self) -> String;
    fn all(&self) -> &Vec<f32>;
    fn current(&self) -> f32;

    fn diff(&self) -> f32 {
        let len = self.all().len();
        let slice = &self.all()[len - len.min(30)..];
        let avg = slice.iter().sum::<f32>() / slice.len() as f32;

        if avg == 0.0 {
            0.0
        } else {
            (self.current() - avg) / avg * 100.
        }
    }

    fn unit(&self) -> String {
        "".to_string()
    }

    fn storage_cost(&self) -> f32 {
        0.0
    }
}
