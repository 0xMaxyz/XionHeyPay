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
    use cosmwasm_std::{
        from_json, to_json_binary, CosmosMsg, DepsMut, Env, Event, MessageInfo, Response,
    };
    use cw20::Cw20ReceiveMsg;

    use std::str::FromStr;

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

        if let Err(_) = email_address::EmailAddress::from_str(&token_msg.email) {
            return Err(ContractError::InvalidEmail);
        }
        if let Some(memo) = &token_msg.memo {
            if memo.len() > 255 {
                return Err(ContractError::MemoLength);
            }
        }

        // create a ClaimData
        let claim = ClaimData {
            amount: _wrapper.amount,
            token_address: _info.sender,
            sender_address: _deps.api.addr_validate(&_wrapper.sender).unwrap(),
            memo: token_msg.memo,
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
        match crate::jwt::Token::verify(&_msg.aud, &_msg.jwt) {
            Ok(payload) => {
                // check if sender is equal to address in token
                let addr_in_jwt = _deps.api.addr_validate(&payload.xion_address)?;
                if addr_in_jwt != _info.sender {
                    return Err(ContractError::Unauthorized {});
                }
                // Then the msg.sender is the one who owns the token
                let email = payload.email_address;
                // Check for Claims
                if CLAIMS.has(_deps.as_ref().storage, &email) {
                    let txs =
                        ClaimData::prepare_transfer(CLAIMS.load(_deps.as_ref().storage, &email)?)?;

                    // remove claims for this email
                    CLAIMS.remove(_deps.storage, &email);

                    // transfer the claims to sender and add atributes
                    if !txs.is_empty() {
                        Ok(Response::new()
                            .add_event(
                                Event::new("memos").add_attributes(
                                    txs.iter()
                                        .flat_map(|t| t.memos.iter().cloned())
                                        .filter(|(_, memo)| !memo.is_empty())
                                        .collect::<Vec<(String, String)>>(),
                                ),
                            ) //TODO add memo attribs
                            .add_event(
                                Event::new("claims").add_attributes(
                                    txs.iter()
                                        .flat_map(|t| t.attributes.iter().cloned())
                                        .collect::<Vec<(String, String)>>(),
                                ),
                            )
                            .add_attribute("email", &email)
                            .add_messages(
                                txs.iter()
                                    .map(|tx| {
                                        let ex_msg = cw20::Cw20ExecuteMsg::Transfer {
                                            recipient: _info.sender.to_string(),
                                            amount: tx.total_amount,
                                        };
                                        cosmwasm_std::WasmMsg::Execute {
                                            contract_addr: tx.token_address.to_string(),
                                            msg: to_json_binary(&ex_msg).unwrap(),
                                            funds: vec![],
                                        }
                                        .into()
                                    })
                                    .collect::<Vec<CosmosMsg>>(),
                            ))
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

    use crate::{
        msg::{ClaimResponse, QueryClaimResponse},
        state::CLAIMS,
    };

    use super::*;
    pub fn query_claims(_deps: Deps, _env: Env, email: String) -> StdResult<QueryClaimResponse> {
        match CLAIMS.may_load(_deps.storage, &email).unwrap() {
            Some(claims) => {
                //let mut _claims:Vec<(String, Uint128)> = vec![];
                let _claims = claims
                    .iter()
                    .map(
                        |claim| {
                            let memo = match &claim.memo {
                                Some(m) => m.to_string(),
                                None => "".to_string(), // or any other default value you prefer
                            };
                            ClaimResponse {
                                memo,
                                sender: claim.sender_address.to_string(),
                                token: claim.token_address.to_string(),
                                amount: claim.amount,
                            }
                        }, //(claim.token_address.to_string(), claim.amount)
                    )
                    .collect();
                Ok(QueryClaimResponse { claims: _claims })
            }
            None => Ok(QueryClaimResponse { claims: vec![] }),
        }
    }
}
