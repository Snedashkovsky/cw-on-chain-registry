use cosmwasm_std::{StdResult, Api, Addr, Response};

use crate::error::ContractError;

/*
 * Validation vec of addresses
 */
pub fn validate_map_addr(api: &dyn Api, addresses: &[String]) -> StdResult<Vec<Addr>> {
    addresses.iter().map(|addr| api.addr_validate(addr)).collect()
}

/*
 * Basic rule /[a-zA-Z0-9-]/
 */
pub fn validate_by_basic_rule(
    val: String,
    field_name: String,
) -> Result<Response, ContractError> {
    for byte in val.as_bytes().iter() {
        // - && _ && 0-9 && a-z
        if (*byte != 45) && (*byte != 46) && (*byte != 95) && (*byte < 48 || *byte > 57) && (*byte < 65 || *byte > 90) && (*byte < 97 || *byte > 122) {
            return Err(ContractError::IncorrectInputData { val: format!("Incorrect value for field field {} {}. Allowed expression /[a-zA-Z0-9-_]/", field_name, val).to_string() });
        }
    }

    Ok(Response::default())
}

