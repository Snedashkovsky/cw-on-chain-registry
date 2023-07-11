use cosmwasm_std::{
    Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cw2::{get_contract_version, set_contract_version};

use crate::error::ContractError;
use crate::execute::{execute_create_entry, execute_delete_entry, execute_update_entry, execute_update_admins, execute_update_executors};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::query::{query_entry, query_asset, query_list, query_config};
use crate::state::{Config, CONFIG};
use crate::validating::{validate_map_addr};

const CONTRACT_NAME: &str = "on-chain-registry";
const CONTRACT_VERSION: &str = "0.0.1";


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        admins: validate_map_addr(deps.api, &msg.admins)?,
        executors: validate_map_addr(deps.api, &msg.executors)?,
    };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateAdmins { new_admins } => execute_update_admins(deps, env, info, new_admins),
        ExecuteMsg::UpdateExecutors { new_executors } => execute_update_executors(deps, env, info, new_executors),
        // ExecuteMsg::UpdateOwner { new_owner } => execute_update_owner(deps, env, info, new_owner),
        ExecuteMsg::CreateEntry {
            chain_name,
            chain_id,
            assets,
        } => execute_create_entry(deps, info, chain_name, chain_id, assets),
        ExecuteMsg::UpdateEntry {
            chain_name,
            chain_id,
            assets,
        } => execute_update_entry(deps, info, chain_name, chain_id, assets),
        ExecuteMsg::DeleteEntry { chain_name } => execute_delete_entry(deps, info, chain_name),
    }
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetEntries { limit } => {
            to_binary(&query_list(deps, limit)?)
        }
        QueryMsg::GetEntry { chain_name } => {
            to_binary(&query_entry(deps, chain_name)?)
        }
        QueryMsg::GetAsset { chain_name , base} => {
            to_binary(&query_asset(deps, chain_name, base)?)
        }
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    let version = get_contract_version(deps.storage)?;
    if version.contract != CONTRACT_NAME {
        return Err(ContractError::CannotMigrate {
            previous_contract: version.contract,
        });
    }

    Ok(Response::default())
}
