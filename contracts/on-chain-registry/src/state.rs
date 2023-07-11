use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admins: Vec<Addr>,
    pub executors : Vec<Addr>
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
    pub aliases: Option<Vec<String>>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Counterparty {
    pub base_denom: String,
    pub chain_id: Option<String>,
    pub port: Option<String>,
    pub channel_id: Option<String>,
    pub chain_name: Option<String>,
    pub contract: Option<String>,
    pub base_supply: Option<u128>
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
    pub provider: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Ibc {
    pub source_channel: String,
    pub dst_channel: String,
    pub source_denom: String,
    pub base_supply: Option<u128>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LogoUri {
    pub png: Option<String>,
    pub svg: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Theme {
    pub primary_color_hex: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Image {
    pub png: Option<String>,
    pub svg: Option<String>,
    pub theme: Option<Theme>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Asset {
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
    pub keywords: Option<Vec<String>>

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ChainData {
    pub chain_id: String,
    pub assets: Vec<Asset>,
}

pub const CONFIG_KEY: &str = "config";
pub const CHAINS_KEY: &str = "chains";
pub const CONFIG: Item<Config> = Item::new(CONFIG_KEY);
pub const CHAINS: Map<&str, ChainData> = Map::new(CHAINS_KEY);