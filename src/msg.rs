use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;
use cw20::Cw20ReceiveMsg;

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
    pub total_claims: Uint128,
}

#[cw_serde]
pub struct TokenReceiveMsg {
    pub email: String,
}

#[cw_serde]
pub struct TokenClaimMsg {
    pub jwt: String,
    pub aud: String,
}
