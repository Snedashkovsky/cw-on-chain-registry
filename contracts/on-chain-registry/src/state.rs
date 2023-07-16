use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, MultiIndex, IndexedMap, IndexList, Index, UniqueIndex};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admins: Vec<Addr>,
    pub executors: Vec<Addr>,
}

impl Config {
    pub fn is_admin(&self, addr: impl AsRef<str>) -> bool {
        let addr = addr.as_ref();
        self.admins.iter().any(|a| a.as_ref() == addr)
    }

    pub fn is_executor(&self, addr: impl AsRef<str>) -> bool {
        let addr = addr.as_ref();
        self.executors.iter().any(|a| a.as_ref() == addr)
    }

    pub fn can_modify(&self, addr: &str) -> bool {
        self.is_admin(addr)
    }

    pub fn can_execute(&self, addr: &str) -> bool {
        self.is_executor(addr)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DenomUnit {
    pub denom: String,
    pub exponent: i32,
    pub aliases: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Counterparty {
    pub base_denom: String,
    pub chain_id: Option<String>,
    pub port: Option<String>,
    pub channel_id: Option<String>,
    pub chain_name: Option<String>,
    pub contract: Option<String>,
    pub base_supply: Option<u128>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Chain {
    pub port: Option<String>,
    pub channel_id: String,
    pub path: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Trace {
    pub trace_type: String,
    pub counterparty: Counterparty,
    pub chain: Option<Chain>,
    pub provider: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Ibc {
    pub source_channel: String,
    pub dst_channel: String,
    pub source_denom: String,
    pub base_supply: Option<u128>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LogoUri {
    pub png: Option<String>,
    pub svg: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Theme {
    pub primary_color_hex: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Image {
    pub png: Option<String>,
    pub svg: Option<String>,
    pub theme: Option<Theme>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Asset {
    pub chain_name: String,
    pub chain_id: String,
    pub base: String,
    pub type_asset: Option<String>,
    pub supply: Option<u128>,
    pub description: Option<String>,
    pub denom_units: Option<Vec<DenomUnit>>,
    pub address: Option<String>,
    pub admin: Option<String>,
    pub name: Option<String>,
    pub display: Option<String>,
    pub symbol: Option<String>,
    pub traces: Option<Vec<Trace>>,
    pub ibc: Option<Ibc>,
    pub logo_uris: Option<LogoUri>,
    pub images: Option<Vec<Image>>,
    pub coingecko_id: Option<String>,
    pub keywords: Option<Vec<String>>,
}

pub struct AssetIndexes<'a> {
    pub chain_name: MultiIndex<'a, String, Asset, String>,
    pub asset: UniqueIndex<'a, (String, String), Asset>,
}

impl<'a> IndexList<Asset> for AssetIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item=&'_ dyn Index<Asset>> + '_> {
        let v: Vec<&dyn Index<Asset>> = vec![&self.chain_name, &self.asset];
        Box::new(v.into_iter())
    }
}

pub fn assets_data<'a>() -> IndexedMap<'a, &'a str, Asset, AssetIndexes<'a>> {
    let indexes = AssetIndexes {
        chain_name: MultiIndex::new(
            asset_chain_name_idx,
            ASSETS_KEY,
            "assets__chain_name",
        ),
        asset: UniqueIndex::new(|t| (t.chain_name.clone(), t.base.clone()), "assets__asset"),
    };
    IndexedMap::new(ASSETS_KEY, indexes)
}

pub fn asset_chain_name_idx(_pk: &[u8], d: &Asset) -> String {
    d.chain_name.clone()
}

pub const CONFIG_KEY: &str = "config";
pub const CHAINS_KEY: &str = "chains";
pub const ASSETS_KEY: &str = "assets";
pub const CONFIG: Item<Config> = Item::new(CONFIG_KEY);
