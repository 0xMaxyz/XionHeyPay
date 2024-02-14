use std::collections::HashMap;

use crate::error::ContractError;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct ClaimData {
    pub token_address: Addr,
    pub amount: Uint128,
    pub sender_address: Addr,
    pub memo: Option<String>,
}

pub struct ClaimTransfer {
    pub token_address: Addr,
    pub total_amount: Uint128,
    pub attributes: Vec<(String, String)>,
    pub memos: Vec<(String, String)>,
}

impl ClaimData {
    pub fn prepare_transfer(claims: Vec<ClaimData>) -> Result<Vec<ClaimTransfer>, ContractError> {
        if claims.is_empty() {
            return Err(ContractError::NotClaimable);
        }

        let mut grouped_claims: HashMap<
            Addr,
            (Uint128, Vec<(String, String)>, Vec<(String, String)>),
        > = HashMap::new();

        for claim in claims {
            grouped_claims
                .entry(claim.token_address.clone())
                .and_modify(|(total_amount, attributes, memos)| {
                    *total_amount = Self::sum_amounts(*total_amount, claim.amount);
                    attributes.push((claim.sender_address.to_string(), claim.amount.to_string()));
                    if let Some(memo) = &claim.memo {
                        memos.push((claim.sender_address.to_string(), memo.to_string()))
                    }
                })
                .or_insert_with(|| {
                    let mut attributes = Vec::new();
                    attributes.push((claim.sender_address.to_string(), claim.amount.to_string()));
                    let mut memos = Vec::new();
                    if let Some(memo) = &claim.memo {
                        memos.push((claim.sender_address.to_string(), memo.to_string()));
                    }
                    (claim.amount, attributes, memos)
                });
        }
        let mut result: Vec<ClaimTransfer> = vec![];

        for (c_addr, (amount, attr, mem)) in grouped_claims {
            result.push(ClaimTransfer {
                token_address: c_addr,
                total_amount: amount,
                attributes: attr,
                memos: mem,
            });
        }
        Ok(result)
    }
    fn sum_amounts(total_amount: Uint128, new_amount: Uint128) -> Uint128 {
        // Logic to sum Uint128 values, replace with actual implementation
        total_amount.saturating_add(new_amount)
    }
}

pub const CLAIMS: Map<&str, Vec<ClaimData>> = Map::new("haypay");
pub const ADMIN: Item<String> = Item::new("Admin");
