#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

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
        ExecuteMsg::Receive(msg) => execute::execute_receive(_deps, _info, msg),
        ExecuteMsg::Claim { msg } => execute::claim(_deps, _info, _env, msg),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    match _msg {
        QueryMsg::Claims { email } => to_json_binary(&query::query_claims(_deps, _env, email)?),
    }
}

mod execute {
    use cosmwasm_std::{from_json, to_json_binary, DepsMut, Env, MessageInfo, Response, WasmMsg};
    use cw20::Cw20ReceiveMsg;

    use crate::{
        msg::{TokenClaimMsg, TokenReceiveMsg},
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

        // create a ClaimData
        let claim = ClaimData {
            amount: _wrapper.amount,
            token_address: _info.sender,
            sender_address: _deps.api.addr_validate(&_wrapper.sender).unwrap(),
        };
        // save the new claim
        CLAIMS.update(_deps.storage, &token_msg.email, |existing| match existing {
            Some(mut claims) => {
                claims.push(claim);
                Ok::<_, ContractError>(claims)
            }
            None => Ok(vec![claim]),
        })?;

        Ok(Response::new()
            .add_attribute("action", "receive tx")
            .add_attribute("email", token_msg.email))
    }
    pub fn claim(
        _deps: DepsMut,
        _info: MessageInfo,
        _env: Env,
        _msg: TokenClaimMsg,
    ) -> Result<Response, ContractError> {
        // verify token and get email
        match crate::jwt::verify(&_msg.jwt.as_bytes(), &_msg.aud) {
            Ok(email) => {
                if CLAIMS.has(_deps.as_ref().storage, &email) {
                    let txs = ClaimData::prepare_transfer(
                        CLAIMS.load(_deps.as_ref().storage, &email).unwrap(),
                    )
                    .unwrap();
                    // remove claims for this email
                    CLAIMS.remove(_deps.storage, &email);

                    // transfer the claims to sender
                    if !txs.is_empty() {
                        for tx in &txs {
                            let tx_msg = cw20::Cw20ExecuteMsg::Transfer {
                                recipient: _info.sender.to_string(),
                                amount: tx.total_amount,
                            };

                            _ = WasmMsg::Execute {
                                contract_addr: tx.token_address.to_string(),
                                msg: to_json_binary(&tx_msg).unwrap(),
                                funds: vec![],
                            };
                        }

                        let attribs: Vec<(String, String)> = txs
                            .iter()
                            .flat_map(|t| t.attributes.iter().cloned())
                            .collect();

                        Ok(Response::new().add_attributes(attribs))
                    } else {
                        return Err(ContractError::NotClaimable);
                    }
                } else {
                    return Err(ContractError::NotClaimable);
                }
            }
            Err(er) => Err(er),
        }
    }
}

mod query {
    use cosmwasm_std::Uint128;

    use crate::{msg::QueryClaimResponse, state::CLAIMS};

    use super::*;
    pub fn query_claims(_deps: Deps, _env: Env, email: String) -> StdResult<QueryClaimResponse> {
        match CLAIMS.may_load(_deps.storage, &email).unwrap() {
            Some(claims) => Ok(QueryClaimResponse {
                total_claims: claims.iter().map(|claim| claim.amount).sum(),
            }),
            None => Ok(QueryClaimResponse {
                total_claims: Uint128::zero(),
            }),
        }
    }
}
