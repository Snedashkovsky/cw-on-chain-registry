use cosmwasm_std::{attr, Env};
use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::state::{CONFIG, ChainData, Asset, CHAINS};
use crate::validating::{validate_by_basic_rule, validate_map_addr};

pub fn execute_update_admins(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    new_admins: Vec<String>,
) -> Result<Response, ContractError> {
    let cfg = CONFIG.load(deps.storage)?;
    if !cfg.can_modify(info.sender.as_ref()) {
        return Err(ContractError::Unauthorized {});
    }

    let admins = validate_map_addr(deps.api, &new_admins)?;
    CONFIG.update(deps.storage, |mut cfg| -> StdResult<_> {
        cfg.admins = admins;
        Ok(cfg)
    })?;

    Ok(Response::new().add_attributes(vec![attr("action", "update_admins")]))
}

pub fn execute_update_executors(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    new_executors: Vec<String>,
) -> Result<Response, ContractError> {
    let cfg = CONFIG.load(deps.storage)?;
    if !cfg.can_modify(info.sender.as_ref()) {
        return Err(ContractError::Unauthorized {});
    }

    let executors = validate_map_addr(deps.api, &new_executors)?;
    CONFIG.update(deps.storage, |mut cfg| -> StdResult<_> {
        cfg.executors = executors;
        Ok(cfg)
    })?;

    Ok(Response::new().add_attributes(vec![attr("action", "update_executors")]))
}

pub fn execute_create_entry(
    deps: DepsMut,
    info: MessageInfo,
    chain_name: String,
    chain_id: String,
    assets: Vec<Asset>,
) -> Result<Response, ContractError> {
    let cfg = CONFIG.load(deps.storage)?;
    if !cfg.can_execute(info.sender.as_ref()) {
        return Err(ContractError::Unauthorized {});
    }

    let validate_chain_name = validate_by_basic_rule(chain_name.clone(), "chain_id".to_string());
    let validate_chain_id = validate_by_basic_rule(chain_id.clone(), "chain_id".to_string());

    if validate_chain_name.is_err() {
        return validate_chain_name;
    }
    if validate_chain_id.is_err() {
        return validate_chain_id;
    }

// TODO add assets validation

    let new_entry = ChainData {
        chain_id,
        assets
    };

    CHAINS.save(deps.storage, &chain_name, &new_entry)?;

    Ok(Response::new()
        .add_attribute("method", "execute_create_entry")
        .add_attribute("new_entry_id", chain_name.to_string()))
}

pub fn execute_update_entry(
    deps: DepsMut,
    info: MessageInfo,
    chain_name: String,
    chain_id: String,
    assets: Vec<Asset>,
) -> Result<Response, ContractError> {
    let cfg = CONFIG.load(deps.storage)?;
    if !cfg.can_execute(info.sender.as_ref()) {
        return Err(ContractError::Unauthorized {});
    }

    let validate_chain_name = validate_by_basic_rule(chain_name.clone(), "chain_id".to_string());
    if validate_chain_name.is_err() {
        return validate_chain_name;
    }

    let validate_chain_id = validate_by_basic_rule(chain_id.clone(), "chain_id".to_string());
    if validate_chain_id.is_err() {
        return validate_chain_id;
    }

    // TODO add assets validation

    let updated_entry = ChainData {
        chain_id,
        assets,
    };
    CHAINS.save(deps.storage, &chain_name, &updated_entry)?;

    Ok(Response::new()
        .add_attribute("method", "execute_update_entry")
        .add_attribute("updated_entry_id", chain_name.to_string()))
}

pub fn execute_delete_entry(
    deps: DepsMut,
    info: MessageInfo,
    chain_name: String,
) -> Result<Response, ContractError> {
    let cfg = CONFIG.load(deps.storage)?;
    if !cfg.can_execute(info.sender.as_ref()) {
        return Err(ContractError::Unauthorized {});
    }

    CHAINS.remove(deps.storage, &chain_name);

    Ok(Response::new()
        .add_attribute("method", "execute_delete_entry")
        .add_attribute("deleted_entry_id", chain_name.to_string()))
}
