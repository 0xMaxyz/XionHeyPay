#![cfg(test)]

use crate::{contract, jwt::verify, msg::TokenReceiveMsg};
use cosmwasm_std::{
    coins, to_binary, to_json_binary, Addr, Empty, Querier, QuerierWrapper, Uint128,
};
use cw20::{Cw20Coin, Cw20Contract, Cw20ExecuteMsg, Cw20QueryMsg};
use cw_multi_test::{App, Contract, ContractWrapper, Executor};

pub const SESSION_JWT_1:&str="eyJhbGciOiJSUzI1NiIsImtpZCI6Imp3ay1saXZlLTVjYjQwZjE4LTdiYjUtNGEwNi04ZjUzLTc4NjdiOGIzNjkzMCIsInR5cCI6IkpXVCJ9.eyJhdWQiOlsicHJvamVjdC1saXZlLTdlNGEzMjIxLTc5Y2QtNGYzNC1hYzFkLWZlZGFjNGJkZTEzZSJdLCJleHAiOjE3MDc0NjY2NzcsImh0dHBzOi8vc3R5dGNoLmNvbS9zZXNzaW9uIjp7ImlkIjoic2Vzc2lvbi1saXZlLWMwMzdkZTU0LTk5NzktNGY3MS04Y2NiLWMxODAwNzE2MzJkZiIsInN0YXJ0ZWRfYXQiOiIyMDI0LTAyLTA5VDA4OjEyOjU3WiIsImxhc3RfYWNjZXNzZWRfYXQiOiIyMDI0LTAyLTA5VDA4OjEyOjU3WiIsImV4cGlyZXNfYXQiOiIyMDI0LTAyLTA5VDA5OjEyOjU3WiIsImF0dHJpYnV0ZXMiOnsidXNlcl9hZ2VudCI6Ik1vemlsbGEvNS4wIChXaW5kb3dzIE5UIDEwLjA7IFdpbjY0OyB4NjQ7IHJ2OjEyMi4wKSBHZWNrby8yMDEwMDEwMSBGaXJlZm94LzEyMi4wIiwiaXBfYWRkcmVzcyI6IjEwNC4yOC4xNTguNDEifSwiYXV0aGVudGljYXRpb25fZmFjdG9ycyI6W3sidHlwZSI6Im90cCIsImRlbGl2ZXJ5X21ldGhvZCI6ImVtYWlsIiwibGFzdF9hdXRoZW50aWNhdGVkX2F0IjoiMjAyNC0wMi0wOVQwODoxMjo1N1oiLCJlbWFpbF9mYWN0b3IiOnsiZW1haWxfaWQiOiJlbWFpbC1saXZlLTQ0YmEwN2FkLTM4OTItNDJiMi05ZTdlLTA0YzZjMmI3MDkxZSIsImVtYWlsX2FkZHJlc3MiOiJtZXdheG9yNjUzQGxheW1yby5jb20ifX1dfSwiaWF0IjoxNzA3NDY2Mzc3LCJpc3MiOiJzdHl0Y2guY29tL3Byb2plY3QtbGl2ZS03ZTRhMzIyMS03OWNkLTRmMzQtYWMxZC1mZWRhYzRiZGUxM2UiLCJuYmYiOjE3MDc0NjYzNzcsInN1YiI6InVzZXItbGl2ZS1kOTNkNDcwNS1jMTBhLTRiMzEtYTgxOS0yOWJjNzY3YWI4NTcifQ.p-BvLuPacxaySZEzOD5m2i0qNkPlmspxV_xFTAJOtpn4hs724SrQuGoZqQZ-AifJ1NS_Q5DLyJxYHyStQrIQIH--_6R0xLRmJw_p_ZFm_nDjDNJ2R_e-ZFAUiNQ1_Ce_FXYQdhQzmrjofxRsY-vd9nGz-zQDwYM29J3yZkr2MDgt7wdu-ytDVg5fy_xbAw8JmYPy2qFhnp5Nj19cPHnSmr6kN7c3vD22vJTEAJgTY7Ru_MA9ScSagTdUKwC1psMTUjkn_uOykYEVWch8rwycIIf1RcNEVSsKGU6X9RvnayV7wVOegh4PiyC0H_M19YriWaSY03KqX0njGMvMvA0fBA";
pub const SESSION_JWT_2: &str = "eyJhbGciOiJSUzI1NiIsImtpZCI6Imp3ay1saXZlLTVjYjQwZjE4LTdiYjUtNGEwNi04ZjUzLTc4NjdiOGIzNjkzMCIsInR5cCI6IkpXVCJ9.eyJhdWQiOlsicHJvamVjdC1saXZlLTdlNGEzMjIxLTc5Y2QtNGYzNC1hYzFkLWZlZGFjNGJkZTEzZSJdLCJleHAiOjE3MDc0NjY4NDEsImh0dHBzOi8vc3R5dGNoLmNvbS9zZXNzaW9uIjp7ImlkIjoic2Vzc2lvbi1saXZlLTQ3ODdhYjlhLWUwMDgtNDA1YS05YTRlLWRjOWQxZjk5MjFjOCIsInN0YXJ0ZWRfYXQiOiIyMDI0LTAyLTA5VDA4OjE1OjQxWiIsImxhc3RfYWNjZXNzZWRfYXQiOiIyMDI0LTAyLTA5VDA4OjE1OjQxWiIsImV4cGlyZXNfYXQiOiIyMDI0LTAyLTA5VDA5OjE1OjQxWiIsImF0dHJpYnV0ZXMiOnsidXNlcl9hZ2VudCI6Ik1vemlsbGEvNS4wIChXaW5kb3dzIE5UIDEwLjA7IFdpbjY0OyB4NjQ7IHJ2OjEyMi4wKSBHZWNrby8yMDEwMDEwMSBGaXJlZm94LzEyMi4wIiwiaXBfYWRkcmVzcyI6IjEwNC4yOC4xNTguNDEifSwiYXV0aGVudGljYXRpb25fZmFjdG9ycyI6W3sidHlwZSI6Im90cCIsImRlbGl2ZXJ5X21ldGhvZCI6ImVtYWlsIiwibGFzdF9hdXRoZW50aWNhdGVkX2F0IjoiMjAyNC0wMi0wOVQwODoxNTo0MVoiLCJlbWFpbF9mYWN0b3IiOnsiZW1haWxfaWQiOiJlbWFpbC1saXZlLTg3NDNhYmZiLTBhNDctNDMxNS1hYjdlLWViZTdkNWUyYjk4NSIsImVtYWlsX2FkZHJlc3MiOiJ4YXNlYmU2MTE2QGZrY29kLmNvbSJ9fV19LCJpYXQiOjE3MDc0NjY1NDEsImlzcyI6InN0eXRjaC5jb20vcHJvamVjdC1saXZlLTdlNGEzMjIxLTc5Y2QtNGYzNC1hYzFkLWZlZGFjNGJkZTEzZSIsIm5iZiI6MTcwNzQ2NjU0MSwic3ViIjoidXNlci1saXZlLTRhYTg5MjdiLWI0OGItNDJkNS04NGU3LTk3NjU4OGY4MmE0ZiJ9.pF50HyXs6ArMX_AivnNFywxaZ-iWTCUvOucnIkZNbP8E2otbX46Kri9omcT0u7ce9aNblf8XbzuO7C3TDktbeWCbM78aVP-1yobek1LYJLRSXecAvNr4NqC9DXRuQ6fwDKvkHQDxT8O1cm_QFfp0-SoDtn7_0OSJzsycnjzEW8L95EvJUImOwvSZAv3CCUgLdkTXB2CrUB-k6OoR9RSytzzr9Ywbycvcr-npkfxHC8pYEuaRi1Io6-GW437GhjYsqlgTjRvkIa4kT4l6fGmrtyrduZy-GYilDZBWXlyS3kY0DIZJypMgCTwWDh7aBGkOpNSTf4STiV66XcpMy0wOgw";
pub const AUDIENCE: &str = "project-live-7e4a3221-79cd-4f34-ac1d-fedac4bde13e";
pub const EMAIL_1: &str = "mewaxor653@laymro.com";
pub const EMAIL_2: &str = "xasebe6116@fkcod.com";

#[test]
fn test_get_email_from_valid_jwt() {
    let email_extracted_from_token = verify(&SESSION_JWT_1.as_bytes(), &AUDIENCE).unwrap();
    assert_eq!(EMAIL_1, email_extracted_from_token);
}

#[test]
fn test_email_extract_pabics_with_wrong_jwt() {
    _ = match verify(&SESSION_JWT_1[1..].as_bytes(), &AUDIENCE) {
        Ok(_) => {
            assert!(false)
        }
        Err(_) => {
            assert!(true)
        }
    };
}

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
        name: "money".to_owned(),
        symbol: "MONEY".to_owned(),
        decimals: 18,
        initial_balances: vec![Cw20Coin {
            address: owner.to_string(),
            amount: Uint128::new(1000),
        }],
        mint: None,
        marketing: None,
    };

    let cw20_addr = router
        .instantiate_contract(cw20_id, owner.clone(), &msg, &[], "MONEY", None)
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

    // Create ReceiveMsg (for calling Send)
    let token_msg = TokenReceiveMsg {
        audience: AUDIENCE.to_owned(),
        email: EMAIL_1.to_owned(),
        token: SESSION_JWT_1.to_owned(),
    };

    // create send token message
    let send_token_msg = Cw20ExecuteMsg::Send {
        contract: haypay_addr.to_string(),
        amount: Uint128::new(1),
        msg: to_json_binary(&token_msg).unwrap(),
    };

    // send some cw20 tokens to haypay contract
    let payment_result = router
        .execute_contract(owner.clone(), cw20_addr.clone(), &send_token_msg, &[])
        .unwrap();
    let a = 2;
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
