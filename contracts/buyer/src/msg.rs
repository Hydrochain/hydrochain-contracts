use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::state::Config;
use hydrogen::msg::ContainersResponse;
use hydrogen::state::Coordinates;

#[cw_serde]
pub struct InstantiateMsg {
    pub hydrogen_contract: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Buys a container of hydrogen with specified id
    Buy {
        /// Id of the container to buy
        container_id: u64,
        /// Destination of the shipment
        destination: String,
        /// Coordinates of final destination
        coordinates: Coordinates,
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
    Config {},
    /// Returns all containers of hydrogen belonging to that sender
    #[returns(ContainersResponse)]
    Containers {},
}

#[cw_serde]
pub struct ConfigResponse {
    /// Address of hydrogen contract
    pub config: Config,
}

#[cw_serde]
pub struct MigrateMsg {}
