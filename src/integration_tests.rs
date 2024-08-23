#![cfg(test)]

use crate::{
    msg::{QueryClaimResponse, QueryMsg, TokenReceiveMsg},
    unit_tests::EMAIL,
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

    // setup heypay
    let heypay_id = router.store_code(heypay_contract());
    let heypay_addr = router
        .instantiate_contract(
            heypay_id,
            owner.clone(),
            &crate::msg::InstantiateMsg {
                keys_msg: crate::msg::KeysMsg {
                    // https://www.googleapis.com/oauth2/v3/certs
                    key1: "d2d444cf8c5e3ae8386d66a13a316976aa369991".to_string(),
                    n1: "onV5tzUbqyPfkM6MwUqCtrqun9x20hEUbIUlmAYYuPuMhsaNHJqs1AVzRt2TzaNjmPVddEbU7VMDmeFWUt7vgDi7Xu0leevuIN4VSPbAMGBa0oj9Qopqkn9ePO_7DvIN13ktHgfQqatNBu6uXH6zkUl3VtXnubXrUhx7uyF22dARDc1-pJoj2NnsvgxDRElPMyDkU-siVv3c6cgIEwLEZZPWOcwplPTUB4qeTK6prrPBGQshuE1PWK2ZrYpIvXfzHyEbkGdPnrhcxgCzbKBUFvr8n_sfSurLRoDBLjkURKmgB8T8iRzLyXsCu9D3Hw61LKuex1aeSQLdwOFLuUEBdw".to_string(),
                    e1: "AQAB".to_string(),
                    key2: "a49391bf52b58c1d560255c2f2a04e59e22a7b65".to_string(),
                    n2: "v7hTj49pNGYjxKbgMx_iDyjeErhfJFepMl306IV_TW5T_CEGE4lWFfBe9w0cwpi5KD6XlC1GO1AsrtzcYF29wJ283GNBZRkbl8iTe-LQYdjQsBtf_1fLIVt6LR7H2U1RPqa3pY16Kq6i6yC2osVg6tD7ApBCGw1WKe8uU3cm28biJzuV4gv6PzcbOdErd-hb4Cv6n2SoMPYlBfT4pWee75poQh8DYoQ1KJwowz3U1MaxOBMP260hmDK-QK0q4LYabCQiBNsz4FWWcaAAFxZFbiqGY5Gdu18uOkpMbdAN5FoZ_6nMDMSTmlf0CHv7gZe_cL38kZvTaynkWwDxqsW_Xw".to_string(),
                    e2: "AQAB".to_string(),
                    key3: "4529c409f77a106fb67ee1a85d168fd2cfb7c0b7".to_string(),
                    n3: "1crrYmsX8OVzrN9BTDD4RlVJDqSQIEbRby9ELqTmCpW1Qtt7y-pdmLPqlYG1ND5mprkTA83S7g_dcsxuV4wxK4_Vv5a8IBn86HfAX4VfCCOzqBYgACN6hlaffzPIWL1QA8yZ4w-D0fnN3xC5ULhtmtBG23qi__4yEo_FIY6irvbHrpRNI_-vjxFokm2X3ENP2ZOwgNhDIthwJo8l1KNbZa1riAJVcF86zWILQTy756hh8eH1Kt05wsGB3DeGPNV55zYv6sB2bzxARsVYAtCRJ8c28FYWwU8dCRJ70eJEmY4aKFOBO5g4fwYJlvMm9Le7qgAUH5-7wO52BayqXmqAOQ".to_string(),
                    e3: "AQAB".to_string(),
                },
            },
            &[],
            "HeyPay",
            None,
        )
        .unwrap();

    // two different contracts
    assert_ne!(heypay_addr, cw20_addr);

    let cash = Cw20Contract(cw20_addr.clone());

    let owner_balance = cash.balance(&router.wrap(), owner.clone()).unwrap();
    assert_eq!(owner_balance, Uint128::new(1000));

    // Create ReceiveMsg (for calling Send)
    let token_msg = TokenReceiveMsg {
        email: EMAIL.to_owned(),
        memo: Option::Some("This is a test memo".to_string()),
    };

    // create send token message
    let send_token_msg = Cw20ExecuteMsg::Send {
        contract: heypay_addr.to_string(),
        amount: Uint128::new(1),
        msg: to_json_binary(&token_msg).unwrap(),
    };

    // send some cw20 tokens to heypay contract
    _ = router
        .execute_contract::<Cw20ExecuteMsg>(owner.clone(), cw20_addr.clone(), &send_token_msg, &[])
        .unwrap();

    // check owner balance

    let owner_balance = cash.balance(&router.wrap(), owner.clone()).unwrap();
    assert_eq!(owner_balance, Uint128::new(999));

    // create query request to get the claims for email_1
    let _qmsg = QueryMsg::Claims {
        email: EMAIL.to_owned(),
    };

    let query_resp: QueryClaimResponse =
        router.wrap().query_wasm_smart(heypay_addr, &_qmsg).unwrap();
    _ = &query_resp.claims;
}

//

fn heypay_contract() -> Box<dyn Contract<Empty>> {
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
