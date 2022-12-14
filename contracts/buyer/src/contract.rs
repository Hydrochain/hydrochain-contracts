#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use cw_utils::ensure_from_older_version;

use crate::error::ContractError;
use crate::msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg, MigrateMsg};
use crate::state::{Config, ADMIN, CONFIG};
use hydrogen::msg::ContainersResponse;
use hydrogen::state::Coordinates;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:buyer";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let hydrogen_contract = deps.api.addr_validate(&msg.hydrogen_contract.clone())?;
    CONFIG.save(deps.storage, &Config { hydrogen_contract })?;

    ADMIN.save(deps.storage, &info.sender)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("admin", &info.sender)
        .add_attribute("hydrogen_contract", msg.hydrogen_contract))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Buy {
            container_id,
            destination,
            coordinates,
        } => execute::buy(deps, info.sender, container_id, destination, coordinates),
        ExecuteMsg::UpdateConfig {
            new_hydrogen_address,
        } => execute::update_config(deps, info.sender, new_hydrogen_address),
    }
}

pub mod execute {
    use super::*;

    pub fn buy(
        _deps: DepsMut,
        _sender: Addr,
        _container_id: u64,
        _destination: String,
        _coordinates: Coordinates,
    ) -> Result<Response, ContractError> {
        todo!();
    }

    pub fn update_config(
        _deps: DepsMut,
        _sender: Addr,
        _new_hydrogen_address: String,
    ) -> Result<Response, ContractError> {
        todo!();
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query::config(deps)?),
        QueryMsg::Containers {} => to_binary(&query::containers(deps)?),
    }
}

pub mod query {
    use super::*;

    pub fn config(deps: Deps) -> StdResult<ConfigResponse> {
        let config = CONFIG.load(deps.storage)?;
        Ok(ConfigResponse { config })
    }

    pub fn containers(deps: Deps) -> StdResult<ContainersResponse> {
        todo!();
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    ensure_from_older_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::default())
}
