use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin, Decimal};
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
pub struct Coordinates {
    pub latitude: Decimal,
    pub longtitude: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ShipmentDetails {
    /// Address of buyer, to whom ownership will be transferred after delivery
    pub buyer: Addr,
    /// Destination of the shipment
    pub destination: String,
    /// Coordinates of final destination
    pub coordinates: Coordinates,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub enum Status {
    Created,
    Shipped(ShipmentDetails),
    Delivered,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct HydrogenContainer {
    /// Address of current owner
    pub owner: Addr,
    /// Quantity in cubic meters
    pub volume: u64,
    /// Color from color spectrum of hydrogen
    pub color_spectrum: ColorSpectrum,
    /// Price of whole container
    pub price: Coin,
    /// Current status of container
    pub status: Status,
}

pub const LAST_ID: Item<u64> = Item::new("last_id");
pub const CONTAINERS: Map<u64, HydrogenContainer> = Map::new("hydrogen");
