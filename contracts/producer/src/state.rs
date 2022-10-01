use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Config {
    /// Address of hydrogen contract
    pub hydrogen_contract: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");
/// Admin - creator of contract - is allowed to change configuration
pub const ADMIN: Item<Addr> = Item::new("admin");
