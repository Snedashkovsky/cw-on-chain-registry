use cosmwasm_std::{Deps, Order, StdResult};

use crate::msg::{EntryResponse, ListResponse, AssetResponse, ConfigResponse};
use crate::state::{Asset, CHAINS, CONFIG};

const MAX_LIMIT: u32 = 30;
const DEFAULT_LIMIT: u32 = 20;

pub fn query_entry(deps: Deps, chain_name: String) -> StdResult<EntryResponse> {
    let entry = CHAINS.load(deps.storage, &chain_name)?;

    Ok(EntryResponse {
        chain_name,
        chain_id: entry.chain_id,
        assets: entry.assets
    })
}

pub fn query_asset(deps: Deps, chain_name: String, base: String) -> StdResult<AssetResponse> {
    let entry = CHAINS.load(deps.storage, &chain_name)?;

    Ok(AssetResponse {
        chain_name,
        chain_id: entry.chain_id,
        asset: entry.assets
            .iter().
            filter(|asset| asset.base.as_str() == base)
            .cloned()
            .collect::<Vec<Asset>>()
            .first()
            .cloned()
            .unwrap()
    })
}

pub fn query_list(deps: Deps, limit: Option<u32>) -> StdResult<ListResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let entries: StdResult<Vec<_>> = CHAINS
        .range(deps.storage, None, None, Order::Ascending)
        .take(limit)
        .collect();

    let result = ListResponse {
        entries: entries?.into_iter().map(|l| l.1).collect(),
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

