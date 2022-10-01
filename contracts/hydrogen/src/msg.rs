use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::state::ColorSpectrum;

#[cw_serde]
pub struct InstantiateMsg {
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Produces a container of hydrogen
    Produces {
        /// Color from color spectrum of hydrogen
        color_spectrum: ColorSpectrum,
        /// Price of whole container
        price: Coin,
        /// Price of whole container
        volume: u64,
    }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
}

