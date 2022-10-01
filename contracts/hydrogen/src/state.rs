use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub enum ColorSpectrum {
    GREEN,
    YELLOW,
    BLUE,
    WHITE,
    BLACK,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct HydrogenContainer {
    /// Address of the producer
    pub producer: Addr,
    /// Quantity in cubic meters
    pub volume: u64,
    /// Color from color spectrum of hydrogen
    pub color_spectrum: ColorSpectrum,
    /// Price of whole container
    pub price: Coin,
}

pub const LAST_ID: Item<u64> = Item::new("last_id");
pub const CONTAINERS: Map<u64, HydrogenContainer> = Map::new("hydrogen");
