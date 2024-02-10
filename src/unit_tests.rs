#![cfg(test)]

use crate::{
    contract::{self, instantiate},
    jwt::verify,
    msg::{self, InstantiateMsg, QueryClaimResponse, QueryMsg},
};
use cosmwasm_std::{
    from_json,
    testing::{mock_dependencies, mock_env, mock_info},
    to_json_binary, Deps, DepsMut, MessageInfo, Uint128,
};
use cw20::Cw20ReceiveMsg;

pub const SESSION_JWT_1:&str="eyJhbGciOiJSUzI1NiIsImtpZCI6Imp3ay1saXZlLTVjYjQwZjE4LTdiYjUtNGEwNi04ZjUzLTc4NjdiOGIzNjkzMCIsInR5cCI6IkpXVCJ9.eyJhdWQiOlsicHJvamVjdC1saXZlLTdlNGEzMjIxLTc5Y2QtNGYzNC1hYzFkLWZlZGFjNGJkZTEzZSJdLCJleHAiOjE3MDc0NjY2NzcsImh0dHBzOi8vc3R5dGNoLmNvbS9zZXNzaW9uIjp7ImlkIjoic2Vzc2lvbi1saXZlLWMwMzdkZTU0LTk5NzktNGY3MS04Y2NiLWMxODAwNzE2MzJkZiIsInN0YXJ0ZWRfYXQiOiIyMDI0LTAyLTA5VDA4OjEyOjU3WiIsImxhc3RfYWNjZXNzZWRfYXQiOiIyMDI0LTAyLTA5VDA4OjEyOjU3WiIsImV4cGlyZXNfYXQiOiIyMDI0LTAyLTA5VDA5OjEyOjU3WiIsImF0dHJpYnV0ZXMiOnsidXNlcl9hZ2VudCI6Ik1vemlsbGEvNS4wIChXaW5kb3dzIE5UIDEwLjA7IFdpbjY0OyB4NjQ7IHJ2OjEyMi4wKSBHZWNrby8yMDEwMDEwMSBGaXJlZm94LzEyMi4wIiwiaXBfYWRkcmVzcyI6IjEwNC4yOC4xNTguNDEifSwiYXV0aGVudGljYXRpb25fZmFjdG9ycyI6W3sidHlwZSI6Im90cCIsImRlbGl2ZXJ5X21ldGhvZCI6ImVtYWlsIiwibGFzdF9hdXRoZW50aWNhdGVkX2F0IjoiMjAyNC0wMi0wOVQwODoxMjo1N1oiLCJlbWFpbF9mYWN0b3IiOnsiZW1haWxfaWQiOiJlbWFpbC1saXZlLTQ0YmEwN2FkLTM4OTItNDJiMi05ZTdlLTA0YzZjMmI3MDkxZSIsImVtYWlsX2FkZHJlc3MiOiJtZXdheG9yNjUzQGxheW1yby5jb20ifX1dfSwiaWF0IjoxNzA3NDY2Mzc3LCJpc3MiOiJzdHl0Y2guY29tL3Byb2plY3QtbGl2ZS03ZTRhMzIyMS03OWNkLTRmMzQtYWMxZC1mZWRhYzRiZGUxM2UiLCJuYmYiOjE3MDc0NjYzNzcsInN1YiI6InVzZXItbGl2ZS1kOTNkNDcwNS1jMTBhLTRiMzEtYTgxOS0yOWJjNzY3YWI4NTcifQ.p-BvLuPacxaySZEzOD5m2i0qNkPlmspxV_xFTAJOtpn4hs724SrQuGoZqQZ-AifJ1NS_Q5DLyJxYHyStQrIQIH--_6R0xLRmJw_p_ZFm_nDjDNJ2R_e-ZFAUiNQ1_Ce_FXYQdhQzmrjofxRsY-vd9nGz-zQDwYM29J3yZkr2MDgt7wdu-ytDVg5fy_xbAw8JmYPy2qFhnp5Nj19cPHnSmr6kN7c3vD22vJTEAJgTY7Ru_MA9ScSagTdUKwC1psMTUjkn_uOykYEVWch8rwycIIf1RcNEVSsKGU6X9RvnayV7wVOegh4PiyC0H_M19YriWaSY03KqX0njGMvMvA0fBA";
//pub const SESSION_JWT_2: &str = "eyJhbGciOiJSUzI1NiIsImtpZCI6Imp3ay1saXZlLTVjYjQwZjE4LTdiYjUtNGEwNi04ZjUzLTc4NjdiOGIzNjkzMCIsInR5cCI6IkpXVCJ9.eyJhdWQiOlsicHJvamVjdC1saXZlLTdlNGEzMjIxLTc5Y2QtNGYzNC1hYzFkLWZlZGFjNGJkZTEzZSJdLCJleHAiOjE3MDc0NjY4NDEsImh0dHBzOi8vc3R5dGNoLmNvbS9zZXNzaW9uIjp7ImlkIjoic2Vzc2lvbi1saXZlLTQ3ODdhYjlhLWUwMDgtNDA1YS05YTRlLWRjOWQxZjk5MjFjOCIsInN0YXJ0ZWRfYXQiOiIyMDI0LTAyLTA5VDA4OjE1OjQxWiIsImxhc3RfYWNjZXNzZWRfYXQiOiIyMDI0LTAyLTA5VDA4OjE1OjQxWiIsImV4cGlyZXNfYXQiOiIyMDI0LTAyLTA5VDA5OjE1OjQxWiIsImF0dHJpYnV0ZXMiOnsidXNlcl9hZ2VudCI6Ik1vemlsbGEvNS4wIChXaW5kb3dzIE5UIDEwLjA7IFdpbjY0OyB4NjQ7IHJ2OjEyMi4wKSBHZWNrby8yMDEwMDEwMSBGaXJlZm94LzEyMi4wIiwiaXBfYWRkcmVzcyI6IjEwNC4yOC4xNTguNDEifSwiYXV0aGVudGljYXRpb25fZmFjdG9ycyI6W3sidHlwZSI6Im90cCIsImRlbGl2ZXJ5X21ldGhvZCI6ImVtYWlsIiwibGFzdF9hdXRoZW50aWNhdGVkX2F0IjoiMjAyNC0wMi0wOVQwODoxNTo0MVoiLCJlbWFpbF9mYWN0b3IiOnsiZW1haWxfaWQiOiJlbWFpbC1saXZlLTg3NDNhYmZiLTBhNDctNDMxNS1hYjdlLWViZTdkNWUyYjk4NSIsImVtYWlsX2FkZHJlc3MiOiJ4YXNlYmU2MTE2QGZrY29kLmNvbSJ9fV19LCJpYXQiOjE3MDc0NjY1NDEsImlzcyI6InN0eXRjaC5jb20vcHJvamVjdC1saXZlLTdlNGEzMjIxLTc5Y2QtNGYzNC1hYzFkLWZlZGFjNGJkZTEzZSIsIm5iZiI6MTcwNzQ2NjU0MSwic3ViIjoidXNlci1saXZlLTRhYTg5MjdiLWI0OGItNDJkNS04NGU3LTk3NjU4OGY4MmE0ZiJ9.pF50HyXs6ArMX_AivnNFywxaZ-iWTCUvOucnIkZNbP8E2otbX46Kri9omcT0u7ce9aNblf8XbzuO7C3TDktbeWCbM78aVP-1yobek1LYJLRSXecAvNr4NqC9DXRuQ6fwDKvkHQDxT8O1cm_QFfp0-SoDtn7_0OSJzsycnjzEW8L95EvJUImOwvSZAv3CCUgLdkTXB2CrUB-k6OoR9RSytzzr9Ywbycvcr-npkfxHC8pYEuaRi1Io6-GW437GhjYsqlgTjRvkIa4kT4l6fGmrtyrduZy-GYilDZBWXlyS3kY0DIZJypMgCTwWDh7aBGkOpNSTf4STiV66XcpMy0wOgw";
pub const AUDIENCE: &str = "project-live-7e4a3221-79cd-4f34-ac1d-fedac4bde13e";
pub const EMAIL_1: &str = "mewaxor653@laymro.com";
pub const EMAIL_2: &str = "xasebe6116@fkcod.com";

#[test]
fn test_get_email_from_valid_jwt() {
    let email_extracted_from_token = verify(&SESSION_JWT_1.as_bytes(), &AUDIENCE).unwrap();
    assert_eq!(EMAIL_1, email_extracted_from_token);
}

#[test]
fn test_email_extract_with_wrong_jwt() {
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
fn test_instantiate() {
    let mut deps = mock_dependencies();

    let instantiate_msg = InstantiateMsg {};
    let info = mock_info("sender", &[]);

    let result = instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();

    assert_eq!(0, result.messages.len());
}

#[test]
fn test_execute_receive_unit() {
    let mut dep = mock_dependencies();
    let info = mock_info("token", &[]);
    let amount = Uint128::new(100);

    send_token(
        dep.as_mut(),
        info.clone(),
        amount,
        "sender".to_string(),
        EMAIL_1.to_string(),
    );

    // query claim
    let query_resp = query_claim(dep.as_ref(), EMAIL_1.to_string());
    assert_eq!(query_resp.total_claims, amount);
}

#[test]
fn test_query_claims() {
    let query_resp = query_claim(mock_dependencies().as_ref(), EMAIL_1.to_string());
    assert!(query_resp.total_claims.is_zero());
}

#[test]
fn test_claim_by_email() {
    let mut dep = mock_dependencies();
    let info = mock_info("token", &[]);
    let amount = Uint128::new(100);

    send_token(
        dep.as_mut(),
        info.clone(),
        amount,
        "sender".to_string(),
        EMAIL_1.to_string(),
    );

    // query claim
    let query_resp = query_claim(dep.as_ref(), EMAIL_1.to_string());
    assert_eq!(query_resp.total_claims, amount);

    // claim the tokens by email
    claim_by_email(
        AUDIENCE.to_string(),
        SESSION_JWT_1.to_string(),
        dep.as_mut(),
        info.clone(),
        1,
    );
    // email1 shall have no more claims
    let query_resp = query_claim(dep.as_ref(), EMAIL_1.to_string());
    assert_eq!(query_resp.total_claims, Uint128::zero());
}

#[test]
fn test_aggregate_multiple_receives() {
    let mut dep = mock_dependencies();
    let info = mock_info("token", &[]);
    let amount = Uint128::new(100);

    for index in 0..=3 {
        send_token(
            dep.as_mut(),
            info.clone(),
            amount,
            "sender".to_string() + &index.to_string(),
            EMAIL_1.to_string(),
        );
    }
    // query claims
    let query_resp = query_claim(dep.as_ref(), EMAIL_1.to_string());
    assert_eq!(
        query_resp.total_claims,
        amount.saturating_mul(Uint128::new(4))
    );
    // Claim
    claim_by_email(
        AUDIENCE.to_string(),
        SESSION_JWT_1.to_string(),
        dep.as_mut(),
        info.clone(),
        4,
    );
}

#[test]
fn test_claim_multi_token() {
    let mut dep = mock_dependencies();
    let info = mock_info("token0", &[]);
    let amount = Uint128::new(100);

    for i in 0..4 {
        send_token(
            dep.as_mut(),
            info.clone(),
            amount,
            "sender".to_string() + &i.to_string(),
            EMAIL_1.to_string(),
        );
    }
    // tokenB
    let info = mock_info("token1", &[]);

    for i in 0..4 {
        send_token(
            dep.as_mut(),
            info.clone(),
            amount,
            "sender".to_string() + &i.to_string(),
            EMAIL_1.to_string(),
        );
    }

    // query claims
    let query_resp = query_claim(dep.as_ref(), EMAIL_1.to_string());
    assert_eq!(
        query_resp.total_claims,
        amount.saturating_mul(Uint128::new(8))
    );
    // Claim
    claim_by_email(
        AUDIENCE.to_string(),
        SESSION_JWT_1.to_string(),
        dep.as_mut(),
        info.clone(),
        8,
    );
}

#[test]
#[should_panic]
fn test_claim_only_once() {
    let mut dep = mock_dependencies();
    let info = mock_info("token", &[]);
    let amount = Uint128::new(100);

    send_token(
        dep.as_mut(),
        info.clone(),
        amount,
        "sender".to_string(),
        EMAIL_1.to_string(),
    );

    // query claim
    let query_resp = query_claim(dep.as_ref(), EMAIL_1.to_string());
    assert_eq!(query_resp.total_claims, amount);

    // claim the tokens by email
    claim_by_email(
        AUDIENCE.to_string(),
        SESSION_JWT_1.to_string(),
        dep.as_mut(),
        info.clone(),
        1,
    );

    // panics
    claim_by_email(
        AUDIENCE.to_string(),
        SESSION_JWT_1.to_string(),
        dep.as_mut(),
        info.clone(),
        1,
    );
}

fn query_claim(_deps: Deps, email: String) -> QueryClaimResponse {
    let _qmsg = QueryMsg::Claims { email };
    let resp = crate::contract::query(_deps, mock_env(), _qmsg).unwrap();
    from_json::<QueryClaimResponse>(resp).unwrap()
}

fn claim_by_email(_aud: String, _jwt: String, _dep: DepsMut, _info: MessageInfo, _attr_qty: usize) {
    let token_claim_msg = crate::msg::TokenClaimMsg {
        aud: _aud.to_owned(),
        jwt: _jwt.to_owned(),
    };
    let claim_msg = crate::msg::ExecuteMsg::Claim {
        msg: token_claim_msg,
    };
    match crate::contract::execute(_dep, mock_env(), _info.clone(), claim_msg) {
        Ok(resp) => {
            assert_eq!(resp.attributes.len(), _attr_qty);
        }
        Err(_) => {
            assert!(false)
        }
    }
}

fn send_token(
    _dep: DepsMut,
    _info: MessageInfo,
    _amount: Uint128,
    _sender: String,
    _email: String,
) {
    let tok_msg = msg::TokenReceiveMsg {
        email: _email.to_string(),
    };

    let rec_msg = Cw20ReceiveMsg {
        amount: _amount,
        sender: _sender.to_string(),
        msg: to_json_binary(&tok_msg).unwrap(),
    };

    let exec_msg = msg::ExecuteMsg::Receive(rec_msg);

    match contract::execute(_dep, mock_env(), _info.clone(), exec_msg) {
        Ok(resp) => {
            assert!(resp.attributes.len() == 2);
        }
        Err(_) => {
            assert!(false);
        }
    };
}
