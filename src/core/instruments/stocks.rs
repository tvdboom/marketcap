use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(EnumIter, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum CompanyName {
    Apple,
    Boeing,
    GoldManSachs,
    Inditex,
    LockheedMartin,
    LVMH,
    Maersk,
    Moderna,
    Nestle,
    Nvidia,
    Pfizer,
    RioTinto,
    Shell,
    Toyota,
    Unilever,
}
