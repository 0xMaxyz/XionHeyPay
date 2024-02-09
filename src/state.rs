use cosmwasm_schema::cw_serde;

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Map;

#[cw_serde]
pub struct ClaimData {
    pub contract_address: Addr,
    pub amount: Uint128,
    pub sender_email: String,
    pub eoa_address: Addr,
}

pub const CLAIMS: Map<&str, Vec<ClaimData>> = Map::new("haypay");
