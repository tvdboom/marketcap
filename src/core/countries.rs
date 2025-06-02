use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CountryName {
    Australia,
    Brazil,
    Canada,
    EU,
    Japan,
    China,
    Russia,
    Ukraine,
    USA,
    Venezuela,
}
