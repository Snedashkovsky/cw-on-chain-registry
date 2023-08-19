// use std::iter;
use cosmwasm_std::{attr, Env};
use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::state::{CONFIG, Asset, assets_data, get_asset_key};
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

pub fn execute_create_assets(
    deps: DepsMut,
    info: MessageInfo,
    assets: Vec<Asset>,
) -> Result<Response, ContractError> {
    let cfg = CONFIG.load(deps.storage)?;
    if !cfg.can_execute(info.sender.as_ref()) {
        return Err(ContractError::Unauthorized {});
    }

    for asset in assets
    {
        let validate_chain_name = validate_by_basic_rule(asset.chain_name.clone(), "chain_name".to_string());
        if validate_chain_name.is_err() {
            return validate_chain_name;
        }

        let validate_chain_id = validate_by_basic_rule(asset.chain_id.clone(), "chain_id".to_string());
        if validate_chain_id.is_err() {
            return validate_chain_id;
        }

        // TODO add assets validation
        assets_data().update(
            deps.storage,
            &asset.key(),
            |old| match old {
                Some(_) => Err(ContractError::EntryExist {
                    val: asset.key()
                }),
                None => Ok(asset.clone()),
            },
        )?;
    }


    Ok(Response::new()
        .add_attribute("method", "execute_create_assets"))
}

pub fn execute_update_assets(
    deps: DepsMut,
    info: MessageInfo,
    assets: Vec<Asset>,
) -> Result<Response, ContractError> {
    let cfg = CONFIG.load(deps.storage)?;
    if !cfg.can_execute(info.sender.as_ref()) {
        return Err(ContractError::Unauthorized {});
    }

    for asset in assets
    {
        let validate_chain_name = validate_by_basic_rule(asset.chain_name.clone(), "chain_name".to_string());
        if validate_chain_name.is_err() {
            return validate_chain_name;
        }

        let validate_chain_id = validate_by_basic_rule(asset.chain_id.clone(), "chain_id".to_string());
        if validate_chain_id.is_err() {
            return validate_chain_id;
        }

        // TODO add assets validation

        assets_data().save(
            deps.storage,
            &asset.key(),
            &asset)?;
    }

    Ok(Response::new()
        .add_attribute("method", "execute_update_assets"))
}

pub fn execute_delete_assets(
    deps: DepsMut,
    info: MessageInfo,
    chain_name: String,
    bases: Vec<String>,
) -> Result<Response, ContractError> {
    let cfg = CONFIG.load(deps.storage)?;
    if !cfg.can_execute(info.sender.as_ref()) {
        return Err(ContractError::Unauthorized {});
    }

    let validate_chain_name = validate_by_basic_rule(chain_name.clone(), "chain_name".to_string());
    if validate_chain_name.is_err() {
        return validate_chain_name;
    }

    for base in bases
    {
        assets_data().remove(
            deps.storage,
            &get_asset_key(chain_name.clone(), base),
        )?;
    }

    Ok(Response::new()
        .add_attribute("method", "execute_delete_assets")
        .add_attribute("deleted_assets_id", chain_name.to_string()))
}
