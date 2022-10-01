use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Coin, Addr};
use cw_storage_plus::Item;


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub enum ColorSpectrum {
    GREEN,
    YELLOW,
    BLUE,
    WHITE,
    BLACK
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Hydrogen {
    /// Quantity in cubic meters
    pub quantity: u64,
    /// Color from color spectrum of hydrogen
    pub color_spectrum: ColorSpectrum,
    /// Price of whole container
    pub price: Coin,
}

pub const PRODUCER: Item<Addr> = Item::new("producer");
pub const CONTAINERS: Map<u64, &Hydrogen> = Map::new("hydrogen");
