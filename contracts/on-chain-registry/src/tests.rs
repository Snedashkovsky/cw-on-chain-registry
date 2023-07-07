#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{attr, from_binary, Addr};
    use std::vec::Vec;

    use crate::state::{Asset, ChainData, CONFIG, Config};
    use crate::msg::{ListResponse, QueryMsg, InstantiateMsg, ExecuteMsg};
    use crate::contract::{query, execute, instantiate};


    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        //an one owner address specified in the instantiation message
        let msg = InstantiateMsg {
            executors: Vec::from(["creator".to_string()]),
            admins: Vec::from(["creator".to_string()]),
        };
        let env = mock_env();
        let info = mock_info("creator", &[]);

        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let state = CONFIG.load(&deps.storage).unwrap();
        assert_eq!(
            state,
            Config {
                admins: Vec::from([Addr::unchecked("creator".to_string())]),
                executors: Vec::from([Addr::unchecked("creator".to_string())]),
            }
        );
        //specifying two addresses in the instantiation message
        let msg = InstantiateMsg {
            admins: Vec::from(["creator".to_string(), "specified_owner".to_string()]),
            executors: Vec::from(["creator".to_string(), "specified_owner".to_string()]),
        };

        let res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the config
        let state = CONFIG.load(&deps.storage).unwrap();
        assert_eq!(
            state,
            Config {
                admins: Vec::from([Addr::unchecked("creator".to_string()), Addr::unchecked("specified_owner".to_string())]),
                executors: Vec::from([Addr::unchecked("creator".to_string()), Addr::unchecked("specified_owner".to_string())]),
            }
        );

        // Query the config
        let res_2 = query(
            deps.as_ref(),
            env.clone(),
            QueryMsg::Config {},
        )
            .unwrap();
        let list: Config = from_binary(&res_2).unwrap();
        assert_eq!(
            state,
            list
        );
    }

    #[test]
    fn create_update_delete_entry() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);
        let msg = InstantiateMsg {
            executors: Vec::from(["creator".to_string()]),
            admins: Vec::from(["creator".to_string()]),
        };

        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(0, res.messages.len());

        let msg = ExecuteMsg::CreateEntry {
            chain_name: "osmosis".to_string(),
            chain_id: "osmosis-1".to_string(),
            assets: Vec::from([
                Asset {
                    base: "uosmo".to_string(),
                    type_asset: Some("sdk.coin".to_string()),
                    supply: Some(1000),
                    description: None,
                    denom_units: None,
                    address: None,
                    admin: None,
                    name: None,
                    display: None,
                    symbol: None,
                    traces: None,
                    ibc: None,
                    logo_uris: None,
                    images: None,
                    coingecko_id: None,
                    keywords: None,
                }]),
        };

        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(
            res.attributes,
            vec![
                attr("method", "execute_create_entry"),
                attr("new_entry_id", "osmosis"),
            ]
        );

        let msg = ExecuteMsg::CreateEntry {
            chain_name: "cosmoshub".to_string(),
            chain_id: "cosmoshub-4".to_string(),
            assets: Vec::from([
                Asset {
                    base: "uatom".to_string(),
                    type_asset: Some("sdk.coin".to_string()),
                    supply: Some(10000),
                    description: None,
                    denom_units: None,
                    address: None,
                    admin: None,
                    name: None,
                    display: None,
                    symbol: None,
                    traces: None,
                    ibc: None,
                    logo_uris: None,
                    images: None,
                    coingecko_id: None,
                    keywords: None,
                }]),
        };

        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(
            res.attributes,
            vec![
                attr("method", "execute_create_entry"),
                attr("new_entry_id", "cosmoshub"),
            ]
        );

        // Query the list of entries
        let res = query(
            deps.as_ref(),
            env.clone(),
            QueryMsg::GetEntries {
                limit: None,
            },
        )
            .unwrap();

        let list: ListResponse = from_binary(&res).unwrap();
        assert_eq!(
            Vec::from([
                ChainData {
                    chain_id: "cosmoshub-4".to_string(),
                    assets: Vec::from([
                        Asset {
                            base: "uatom".to_string(),
                            type_asset: Some("sdk.coin".to_string()),
                            supply: Some(10000),
                            description: None,
                            denom_units: None,
                            address: None,
                            admin: None,
                            name: None,
                            display: None,
                            symbol: None,
                            traces: None,
                            ibc: None,
                            logo_uris: None,
                            images: None,
                            coingecko_id: None,
                            keywords: None,
                        }]),
                },
                ChainData {
                    chain_id: "osmosis-1".to_string(),
                    assets: Vec::from([
                        Asset {
                            base: "uosmo".to_string(),
                            type_asset: Some("sdk.coin".to_string()),
                            supply: Some(1000),
                            description: None,
                            denom_units: None,
                            address: None,
                            admin: None,
                            name: None,
                            display: None,
                            symbol: None,
                            traces: None,
                            ibc: None,
                            logo_uris: None,
                            images: None,
                            coingecko_id: None,
                            keywords: None,
                        }]),
                },
            ]),
            list.entries
        );
        //
        // Update entry
        let message = ExecuteMsg::UpdateEntry {
            chain_name: "osmosis".to_string(),
            chain_id: "osmosis-4".to_string(),
            assets: Vec::from([
                Asset {
                    base: "uosmo".to_string(),
                    type_asset: Some("sdk.coin".to_string()),
                    supply: Some(10000),
                    description: None,
                    denom_units: None,
                    address: None,
                    admin: None,
                    name: None,
                    display: None,
                    symbol: None,
                    traces: None,
                    ibc: None,
                    logo_uris: None,
                    images: None,
                    coingecko_id: None,
                    keywords: None,
                }]),
        };

        let res = execute(deps.as_mut(), env.clone(), info.clone(), message).unwrap();
        assert_eq!(
            res.attributes,
            vec![
                attr("method", "execute_update_entry"),
                attr("updated_entry_id", "osmosis"),
            ]
        );

        // Query the list of entries
        let res = query(
            deps.as_ref(),
            env.clone(),
            QueryMsg::GetEntries {
                limit: None,
            },
        )
            .unwrap();

        let list: ListResponse = from_binary(&res).unwrap();
        assert_eq!(
            Vec::from([
                ChainData {
                    chain_id: "cosmoshub-4".to_string(),
                    assets: Vec::from([
                        Asset {
                            base: "uatom".to_string(),
                            type_asset: Some("sdk.coin".to_string()),
                            supply: Some(10000),
                            description: None,
                            denom_units: None,
                            address: None,
                            admin: None,
                            name: None,
                            display: None,
                            symbol: None,
                            traces: None,
                            ibc: None,
                            logo_uris: None,
                            images: None,
                            coingecko_id: None,
                            keywords: None,
                        }]),
                },
                ChainData {
                    chain_id: "osmosis-4".to_string(),
                    assets: Vec::from([
                        Asset {
                            base: "uosmo".to_string(),
                            type_asset: Some("sdk.coin".to_string()),
                            supply: Some(10000),
                            description: None,
                            denom_units: None,
                            address: None,
                            admin: None,
                            name: None,
                            display: None,
                            symbol: None,
                            traces: None,
                            ibc: None,
                            logo_uris: None,
                            images: None,
                            coingecko_id: None,
                            keywords: None,
                        }]),
                },
            ]),
            list.entries
        );

        //Delete Entry
        let message = ExecuteMsg::DeleteEntry { chain_name: "osmosis".to_string() };

        let res = execute(deps.as_mut(), env.clone(), info, message).unwrap();
        assert_eq!(
            res.attributes,
            vec![
                attr("method", "execute_delete_entry"),
                attr("deleted_entry_id", "osmosis"),
            ]
        );
        // Query the list of entries
        let res = query(
            deps.as_ref(),
            env,
            QueryMsg::GetEntries {
                limit: None,
            },
        )
            .unwrap();
        let list: ListResponse = from_binary(&res).unwrap();
        assert_eq!(
            Vec::from([
                ChainData {
                    chain_id: "cosmoshub-4".to_string(),
                    assets: Vec::from([
                        Asset {
                            base: "uatom".to_string(),
                            type_asset: Some("sdk.coin".to_string()),
                            supply: Some(10000),
                            description: None,
                            denom_units: None,
                            address: None,
                            admin: None,
                            name: None,
                            display: None,
                            symbol: None,
                            traces: None,
                            ibc: None,
                            logo_uris: None,
                            images: None,
                            coingecko_id: None,
                            keywords: None,
                        }]),
                }]),
            list.entries
        );
    }
}

