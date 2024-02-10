#![cfg(test)]

use crate::{
    msg::{QueryClaimResponse, QueryMsg, TokenReceiveMsg},
    unit_tests::EMAIL_2,
};
use cosmwasm_std::{coins, to_json_binary, Addr, Empty, Uint128};
use cw20::{Cw20Coin, Cw20Contract, Cw20ExecuteMsg};

use cw_multi_test::{App, Contract, ContractWrapper, Executor};

#[test]
fn test_receive() {
    // setup owner with founds and App
    let owner = Addr::unchecked("owner");
    let init_balance = coins(1000, "cw20");

    let mut router = App::new(|router, _, storage| {
        router
            .bank
            .init_balance(storage, &owner, init_balance)
            .unwrap();
    });

    // setup sender cw20 account
    let cw20_id = router.store_code(contract_cw20());
    let msg = cw20_base::msg::InstantiateMsg {
        name: "token".to_owned(),
        symbol: "TOK".to_owned(),
        decimals: 18,
        initial_balances: vec![Cw20Coin {
            address: owner.to_string(),
            amount: Uint128::new(1000),
        }],
        mint: None,
        marketing: None,
    };

    let cw20_addr = router
        .instantiate_contract(cw20_id, owner.clone(), &msg, &[], "token", None)
        .unwrap();

    // setup haypay
    let haypay_id = router.store_code(haypay_contract());
    let haypay_addr = router
        .instantiate_contract(
            haypay_id,
            owner.clone(),
            &crate::msg::InstantiateMsg {},
            &[],
            "HayPay",
            None,
        )
        .unwrap();

    // two different contracts
    assert_ne!(haypay_addr, cw20_addr);

    let cash = Cw20Contract(cw20_addr.clone());

    let owner_balance = cash.balance(&router.wrap(), owner.clone()).unwrap();
    assert_eq!(owner_balance, Uint128::new(1000));

    // Create ReceiveMsg (for calling Send)
    let token_msg = TokenReceiveMsg {
        email: EMAIL_2.to_owned(),
    };

    // create send token message
    let send_token_msg = Cw20ExecuteMsg::Send {
        contract: haypay_addr.to_string(),
        amount: Uint128::new(1),
        msg: to_json_binary(&token_msg).unwrap(),
    };

    // send some cw20 tokens to haypay contract
    _ = router
        .execute_contract::<Cw20ExecuteMsg>(owner.clone(), cw20_addr.clone(), &send_token_msg, &[])
        .unwrap();

    // check owner balance

    let owner_balance = cash.balance(&router.wrap(), owner.clone()).unwrap();
    assert_eq!(owner_balance, Uint128::new(999));

    // create query request to get the claims for email_1
    let _qmsg = QueryMsg::Claims {
        email: EMAIL_2.to_owned(),
    };

    let query_resp: QueryClaimResponse =
        router.wrap().query_wasm_smart(haypay_addr, &_qmsg).unwrap();
    _ = &query_resp.total_claims;
}

//

fn haypay_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::contract::execute,
        crate::contract::instantiate,
        crate::contract::query,
    );
    Box::new(contract)
}

fn contract_cw20() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        cw20_base::contract::execute,
        cw20_base::contract::instantiate,
        cw20_base::contract::query,
    );
    Box::new(contract)
}
