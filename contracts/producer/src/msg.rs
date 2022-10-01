use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;

use crate::state::{Config};
use hydrogen::state::ColorSpectrum;
use hydrogen::msg::ContainersResponse;

#[cw_serde]
pub struct InstantiateMsg {
    pub hydrogen_contract: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Produces a container of hydrogen
    Produce {
        /// Color from color spectrum of hydrogen
        color_spectrum: ColorSpectrum,
        /// Price of whole container
        price: Coin,
        /// Price of whole container
        volume: u64,
    },
    /// Allows to update address of hydrogen contract
    UpdateConfig {
        /// Address of hydrogen contract
        new_hydrogen_address: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Returns current configuration
    #[returns(ConfigResponse)]
    Config { },
    /// Returns all containers of hydrogen belonging to that sender
    #[returns(ContainersResponse)]
    Containers { },
}
}

#[cw_serde]
pub struct ConfigResponse {
    /// Address of hydrogen contract
    pub config: Config,
}
