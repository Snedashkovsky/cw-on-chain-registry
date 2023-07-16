use cosmwasm_std::{Deps, Order, StdResult};

use crate::msg::{EntryResponse, ListResponse, AssetResponse, ConfigResponse};
use crate::state::{Asset, CONFIG, assets_data};

const MAX_LIMIT: u32 = 30000;
const DEFAULT_LIMIT: u32 = 100;

pub fn query_chain(deps: Deps, chain_name: String) -> StdResult<EntryResponse> {
    let entry = assets_data()
        .idx
        .chain_name
        .prefix(chain_name.clone())
        .range(deps.storage, None, None, Order::Ascending)
        .map(|item| item.map(|(_, _asset)| _asset))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(EntryResponse {
        chain_name,
        assets: entry
    })
}

pub fn query_asset(deps: Deps, chain_name: String, base: String) -> StdResult<AssetResponse> {
    // let entry = CHAINS.load(deps.storage, &chain_name)?;

    let entry = assets_data().load(deps.storage, &(chain_name.clone() + &base))?;

    Ok(AssetResponse {
        chain_name,
        asset: entry
    })
}

pub fn query_all_assets(deps: Deps, limit: Option<u32>) -> StdResult<ListResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;

    let entries: Vec<Asset> = assets_data()
            .range(deps.storage, None, None, Order::Ascending)
            .take(limit)
            .map(|item| item.map(|(_, _asset)| _asset))
            .collect::<StdResult<Vec<_>>>()?;

    let result = ListResponse {
        entries
    };

    Ok(result)
}

pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let cfg = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        admins: cfg.admins.into_iter().map(|a| a.into()).collect(),
        executors: cfg.executors.into_iter().map(|a| a.into()).collect()
    })
}

