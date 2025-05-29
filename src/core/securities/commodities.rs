use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(EnumIter, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Commodity {
    Aluminium,
    Copper,
    Iron,
    NaturalGas,
    Gold,
    Oil,
    Wheat,
}

impl Commodity {
    
}