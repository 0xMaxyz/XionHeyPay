use std::str::FromStr;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;
use cw20::Cw20ReceiveMsg;

use crate::ContractError;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Receive(Cw20ReceiveMsg),
    Claim { msg: TokenClaimMsg },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(QueryClaimResponse)]
    Claims { email: String },
}

#[cw_serde]
pub struct QueryClaimResponse {
    pub claims: Vec<ClaimResponse>,
}

#[cw_serde]
pub struct TokenReceiveMsg {
    pub email: String,
    pub memo: String,
}

#[cw_serde]
pub struct TokenClaimMsg {
    pub jwt: String,
    pub aud: String,
}

#[cw_serde]
pub struct ClaimResponse {
    pub token: String,
    pub sender: String,
    pub memo: String,
    pub amount: Uint128,
}

impl FromStr for TokenReceiveMsg {
    type Err = ContractError;

    fn from_str(s: &str) -> Result<Self, ContractError> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Err(ContractError::InvalidMsg);
        }

        let email = parts[0].to_string();
        let memo = parts[1].to_string();
        if memo.len() > 256 {
            return Err(ContractError::MemoLength);
        }

        match email_address::EmailAddress::from_str(&email) {
            Ok(_) => Ok(TokenReceiveMsg { email, memo }),
            Err(_) => {
                return Err(ContractError::InvalidEmail);
            }
        }
    }
}
