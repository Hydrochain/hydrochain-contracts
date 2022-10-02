#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ContainerResponse, ContainersResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{
    ColorSpectrum, Coordinates, HydrogenContainer, ShipmentDetails, Status, CONTAINERS, LAST_ID,
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:hydrogen";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    LAST_ID.save(deps.storage, &0)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("producer", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Produce {
            color_spectrum,
            price,
            volume,
        } => execute::produce(deps, info.sender, color_spectrum, price, volume),
        ExecuteMsg::UpdatePrice {
            container_id,
            new_price,
        } => execute::update_price(deps, info.sender, container_id, new_price),
        ExecuteMsg::Buy {
            container_id,
            destination,
            coordinates,
        } => execute::buy(deps, info.sender, container_id, destination, coordinates),
        ExecuteMsg::CloseShipment { container_id } => {
            execute::close_shipment(deps, info.sender, container_id)
        }
        ExecuteMsg::RemoveContainer { container_id } => {
            execute::remove_container(deps, info.sender, container_id)
        }
    }
}

pub mod execute {
    use super::*;

    pub fn produce(
        deps: DepsMut,
        sender: Addr,
        color_spectrum: ColorSpectrum,
        price: Coin,
        volume: u64,
    ) -> Result<Response, ContractError> {
        let container = HydrogenContainer {
            owner: sender.clone(),
            volume,
            color_spectrum,
            price,
            status: Status::Created,
        };

        let index = LAST_ID.load(deps.storage)?;
        CONTAINERS.save(deps.storage, index, &container)?;

        LAST_ID.save(deps.storage, &(index + 1))?;

        Ok(Response::new()
            .add_attribute("produce", "container")
            .add_attribute("owner", &sender))
    }

    pub fn update_price(
        deps: DepsMut,
        sender: Addr,
        container_id: u64,
        new_price: Coin,
    ) -> Result<Response, ContractError> {
        let mut container = CONTAINERS.load(deps.storage, container_id)?;
        if sender != container.owner {
            return Err(ContractError::Unauthorized {});
        }

        let old_price = container.price.clone();
        container.price = new_price.clone();

        CONTAINERS.save(deps.storage, container_id, &container)?;

        Ok(Response::new()
            .add_attribute("price", "update")
            .add_attribute("container_id", container_id.to_string())
            .add_attribute("old_price", old_price.amount.clone())
            .add_attribute("old_denom", old_price.denom.clone())
            .add_attribute("new_price", new_price.amount.clone())
            .add_attribute("new_denom", new_price.denom.clone()))
    }

    pub fn buy(
        deps: DepsMut,
        sender: Addr,
        container_id: u64,
        destination: String,
        coordinates: Coordinates,
    ) -> Result<Response, ContractError> {
        let mut container = CONTAINERS.load(deps.storage, container_id)?;
        if sender == container.owner {
            return Err(ContractError::ProducerCannotBuy {});
        }

        let shipment_details = ShipmentDetails {
            buyer: sender.clone(),
            destination: destination.clone(),
            coordinates,
        };

        container.status = Status::Shipped(shipment_details);

        CONTAINERS.save(deps.storage, container_id, &container)?;

        Ok(Response::new()
            .add_attribute("buy", "container")
            .add_attribute("container_id", container_id.to_string())
            .add_attribute("buyer", &sender)
            .add_attribute("destination", &destination))
    }

    pub fn remove_container(
        _deps: DepsMut,
        _sender: Addr,
        _container_id: u64,
    ) -> Result<Response, ContractError> {
        todo!();
    }

    pub fn close_shipment(
        _deps: DepsMut,
        _sender: Addr,
        _container_id: u64,
    ) -> Result<Response, ContractError> {
        todo!();
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Container { container_id } => to_binary(&query::container(deps, container_id)?),
        QueryMsg::Containers {} => to_binary(&query::containers(deps)?),
        QueryMsg::ContainersByOwner { owner } => {
            to_binary(&query::containers_by_owner(deps, owner)?)
        }
    }
}

pub mod query {
    use super::*;

    pub fn container(deps: Deps, container_id: u64) -> StdResult<ContainerResponse> {
        let hydrogen_container = CONTAINERS.load(deps.storage, container_id)?;
        Ok(ContainerResponse {
            owner: hydrogen_container.owner,
            container_id,
            color_spectrum: hydrogen_container.color_spectrum,
            price: hydrogen_container.price,
            volume: hydrogen_container.volume,
            status: hydrogen_container.status,
        })
    }

    pub fn containers(deps: Deps) -> StdResult<ContainersResponse> {
        let containers = CONTAINERS
            .range(deps.storage, None, None, Order::Ascending)
            .map(|container| {
                let (id, hc) = container?;
                Ok(ContainerResponse {
                    owner: hc.owner,
                    container_id: id,
                    color_spectrum: hc.color_spectrum,
                    price: hc.price,
                    volume: hc.volume,
                    status: hc.status,
                })
            })
            .collect::<StdResult<Vec<ContainerResponse>>>()?;
        Ok(ContainersResponse { containers })
    }

    pub fn containers_by_owner(deps: Deps, owner: String) -> StdResult<ContainersResponse> {
        let owner = deps.api.addr_validate(&owner)?;
        let containers = CONTAINERS
            .range(deps.storage, None, None, Order::Ascending)
            .filter_map(|container| {
                let (id, hc) = container.ok()?;
                if hc.owner != owner {
                    None
                } else {
                    Some(ContainerResponse {
                        owner: hc.owner,
                        container_id: id,
                        color_spectrum: hc.color_spectrum,
                        price: hc.price,
                        volume: hc.volume,
                        status: hc.status,
                    })
                }
            })
            .collect::<Vec<ContainerResponse>>();
        Ok(ContainersResponse { containers })
    }
}
