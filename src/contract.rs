#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:pink-panther";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match _msg {
        ExecuteMsg::Receive(_msg) => execute::execute_receive(_deps, _info, _msg),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

mod execute {
    use cosmwasm_std::{from_json, DepsMut, MessageInfo, Response};
    use cw20::Cw20ReceiveMsg;

    use crate::{
        jwt::verify,
        msg::TokenReceiveMsg,
        state::{ClaimData, CLAIMS},
        ContractError,
    };

    pub fn execute_receive(
        _deps: DepsMut,
        _info: MessageInfo,
        _wrapper: Cw20ReceiveMsg,
    ) -> Result<Response, ContractError> {
        // get the token
        let token_msg: TokenReceiveMsg = from_json(&_wrapper.msg)?;
        // verify token and get email
        let email = verify(&token_msg.token.as_bytes(), &token_msg.audience)?;
        // create a ClaimData
        let claim = ClaimData {
            amount: _wrapper.amount,
            contract_address: _info.sender,
            sender_email: email,
            eoa_address: _deps.api.addr_validate(&_wrapper.sender).unwrap(),
        };
        // save the new claim
        CLAIMS.update(_deps.storage, &token_msg.email, |existing| match existing {
            Some(mut claims) => {
                claims.push(claim);
                Ok::<_, ContractError>(claims)
            }
            None => Ok(vec![claim]),
        })?;

        Ok(Response::new())
    }
}
