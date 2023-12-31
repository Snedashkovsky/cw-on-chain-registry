use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::{Asset};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admins: Vec<String>,
    pub executors: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    UpdateAdmins {
        new_admins: Vec<String>,
    },
    UpdateExecutors {
        new_executors: Vec<String>,
    },
    CreateAssets {
        assets: Vec<Asset>,
    },
    UpdateAssets {
        assets: Vec<Asset>,
    },
    DeleteAssets {
        chain_name: String,
        bases: Vec<String>
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetAsset {
        chain_name: String,
        base: String,
    },
    GetAssetsByChain {
        chain_name: String,
        limit: Option<u32>,
        start_after_base: Option<String>,
    },
    GetAllAssets {
        limit: Option<u32>,
        start_from_chain_name: Option<String>,
        start_after_base: Option<String>,
    },
    Config {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EntryResponse {
    pub chain_name: String,
    pub assets: Vec<Asset>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AssetResponse {
    pub chain_name: String,
    pub asset: Asset,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ListResponse {
    pub entries: Vec<Asset>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ConfigResponse {
    pub admins: Vec<String>,
    pub executors: Vec<String>,
}
