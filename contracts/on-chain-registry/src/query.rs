use cosmwasm_std::{Deps, Order, StdResult};
use cw_storage_plus::Bound;

use crate::msg::{AssetResponse, ChainAssetsResponse, ListAssetsResponse, ChannelResponse, ChainChannelsResponse, ListChannelsResponse, ConfigResponse};
use crate::state::{Asset, CONFIG, assets_data, get_asset_key, Channel, channels_data, get_channel_key};

const MAX_LIMIT: u32 = 30000;
const DEFAULT_LIMIT: u32 = 100;



pub fn query_asset(deps: Deps, chain_name: String, base: String) -> StdResult<AssetResponse> {
    let entry = assets_data().load(
        deps.storage,
        &get_asset_key(chain_name.clone(), base)
    )?;

    Ok(AssetResponse {
        chain_name,
        asset: entry,
    })
}

pub fn query_assets_by_chain(deps: Deps, chain_name: String, limit: Option<u32>,
                             start_after_base: Option<String>) -> StdResult<ChainAssetsResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start_key = get_asset_key(
        chain_name.clone(),
        start_after_base.unwrap_or("".to_string()),
    );
    let start = Some(start_key).map(|s| Bound::ExclusiveRaw(s.into()));

    let entry = assets_data()
        .idx
        .chain_name
        .prefix(chain_name.clone())
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| item.map(|(_, _asset)| _asset))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(ChainAssetsResponse {
        chain_name,
        assets: entry,
    })
}

pub fn query_all_assets(deps: Deps, limit: Option<u32>, start_from_chain_name: Option<String>,
                        start_after_base: Option<String>) -> StdResult<ListAssetsResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start_key = get_asset_key(
        start_from_chain_name.unwrap_or("".to_string()),
        start_after_base.unwrap_or("".to_string()),
    );
    let start = Some(start_key).map(|s| Bound::ExclusiveRaw(s.into()));

    let entries: Vec<Asset> = assets_data()
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| item.map(|(_, _asset)| _asset))
        .collect::<StdResult<Vec<_>>>()?;

    let result = ListAssetsResponse {
        entries
    };

    Ok(result)
}

pub fn query_channel(deps: Deps, chain_name: String, source_channel: String) -> StdResult<ChannelResponse> {
    let entry = channels_data().load(
        deps.storage,
        &get_channel_key(chain_name.clone(), source_channel)
    )?;

    Ok(ChannelResponse {
        chain_name,
        channel: entry,
    })
}

pub fn query_channels_by_chain(deps: Deps, chain_name: String, limit: Option<u32>,
                             start_after_channel: Option<String>) -> StdResult<ChainChannelsResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start_key = get_channel_key(
        chain_name.clone(),
        start_after_channel.unwrap_or("".to_string()),
    );
    let start = Some(start_key).map(|s| Bound::ExclusiveRaw(s.into()));

    let entry = channels_data()
        .idx
        .chain_name
        .prefix(chain_name.clone())
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| item.map(|(_, _channel)| _channel))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(ChainChannelsResponse {
        chain_name,
        channels: entry,
    })
}

pub fn query_all_channels(deps: Deps, limit: Option<u32>, start_from_chain_name: Option<String>,
                        start_after_channel: Option<String>) -> StdResult<ListChannelsResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start_key = get_channel_key(
        start_from_chain_name.unwrap_or("".to_string()),
        start_after_channel.unwrap_or("".to_string()),
    );
    let start = Some(start_key).map(|s| Bound::ExclusiveRaw(s.into()));

    let entries: Vec<Channel> = channels_data()
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| item.map(|(_, _channel)| _channel))
        .collect::<StdResult<Vec<_>>>()?;

    let result = ListChannelsResponse {
        entries
    };

    Ok(result)
}

pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let cfg = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        admins: cfg.admins.into_iter().map(|a| a.into()).collect(),
        executors: cfg.executors.into_iter().map(|a| a.into()).collect(),
    })
}
