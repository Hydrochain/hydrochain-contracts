#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ContainerResponse, ContainersResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{ColorSpectrum, CONTAINERS, LAST_ID};

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
        ExecuteMsg::RemoveContainer { container_id } => {
            execute::remove_container(deps, info.sender, container_id)
        }
    }
}

pub mod execute {
    use super::*;

    pub fn produce(
        _deps: DepsMut,
        _sender: Addr,
        _color_spectrum: ColorSpectrum,
        _price: Coin,
        _volume: u64,
    ) -> Result<Response, ContractError> {
        todo!();
    }

    pub fn update_price(
        _deps: DepsMut,
        _sender: Addr,
        _container_id: u64,
        _new_price: Coin,
    ) -> Result<Response, ContractError> {
        todo!();
    }

    pub fn remove_container(
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
        QueryMsg::ContainersByProducer { producer } => {
            to_binary(&query::containers_by_producer(deps, producer)?)
        }
    }
}

pub mod query {
    use super::*;

    pub fn container(deps: Deps, container_id: u64) -> StdResult<ContainerResponse> {
        let hydrogen_container = CONTAINERS.load(deps.storage, container_id)?;
        Ok(ContainerResponse {
            producer: hydrogen_container.producer,
            container_id,
            color_spectrum: hydrogen_container.color_spectrum,
            price: hydrogen_container.price,
            volume: hydrogen_container.volume,
        })
    }

    pub fn containers(deps: Deps) -> StdResult<ContainersResponse> {
        let containers = CONTAINERS
            .range(deps.storage, None, None, Order::Ascending)
            .map(|container| {
                let (id, hc) = container?;
                Ok(ContainerResponse {
                    producer: hc.producer,
                    container_id: id,
                    color_spectrum: hc.color_spectrum,
                    price: hc.price,
                    volume: hc.volume,
                })
            })
            .collect::<StdResult<Vec<ContainerResponse>>>()?;
        Ok(ContainersResponse { containers })
    }

    pub fn containers_by_producer(deps: Deps, producer: String) -> StdResult<ContainersResponse> {
        let producer = deps.api.addr_validate(&producer)?;
        let containers = CONTAINERS
            .range(deps.storage, None, None, Order::Ascending)
            .filter_map(|container| {
                let (id, hc) = container.ok()?;
                if hc.producer != producer {
                    None
                } else {
                    Some(ContainerResponse {
                        producer: hc.producer,
                        container_id: id,
                        color_spectrum: hc.color_spectrum,
                        price: hc.price,
                        volume: hc.volume,
                    })
                }
            })
            .collect::<Vec<ContainerResponse>>();
        Ok(ContainersResponse { containers })
    }
}
