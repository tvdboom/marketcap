use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Country {
    Brazil,
    Canada,
    EU,
    Japan,
    China,
    Russia,
    Ukraine,
    USA,
}
