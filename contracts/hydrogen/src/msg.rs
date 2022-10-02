use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin};

use crate::state::{ColorSpectrum, Coordinates, Status};

#[cw_serde]
pub struct InstantiateMsg {}

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
    /// Allows to update price of given container
    UpdatePrice {
        /// Id of container
        container_id: u64,
        /// New price for container
        new_price: Coin,
    },
    /// Buy container for price that is listed
    Buy {
        /// Id of the container to buy
        container_id: u64,
        /// Destination of the shipment
        destination: String,
        /// Coordinates of final destination
        coordinates: Coordinates,
    },
    /// If container have Shipped status, transfer ownership to the buyer
    CloseShipment { container_id: u64 },
    /// Allows producer to remove his container
    RemoveContainer { container_id: u64 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Returns details about containers based on given id
    #[returns(ContainerResponse)]
    Container { container_id: u64 },
    /// List all currently kept containers
    #[returns(ContainersResponse)]
    Containers {},
    /// List all currently kept containers produced by that owner
    #[returns(ContainersResponse)]
    ContainersByOwner { owner: String },
}

#[cw_serde]
pub struct ContainerResponse {
    /// Address of current owner
    pub owner: Addr,
    /// Container ID
    pub container_id: u64,
    /// Quantity in cubic meters
    pub volume: u64,
    /// Color from color spectrum of hydrogen
    pub color_spectrum: ColorSpectrum,
    /// Price of whole container
    pub price: Coin,
    /// Current status of container
    pub status: Status,
}

#[cw_serde]
pub struct ContainersResponse {
    /// List of all containers of this producer
    pub containers: Vec<ContainerResponse>,
}

#[cw_serde]
pub struct MigrateMsg {}
