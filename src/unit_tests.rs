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

pub const SESSION_JWT_1:&str="eyJhbGciOiJSUzI1NiIsImtpZCI6Imp3ay1saXZlLTVjYjQwZjE4LTdiYjUtNGEwNi04ZjUzLTc4NjdiOGIzNjkzMCIsInR5cCI6IkpXVCJ9.eyJhdWQiOlsicHJvamVjdC1saXZlLTdlNGEzMjIxLTc5Y2QtNGYzNC1hYzFkLWZlZGFjNGJkZTEzZSJdLCJleHAiOjE3MDc3NDYzMDgsImh0dHBzOi8vc3R5dGNoLmNvbS9zZXNzaW9uIjp7ImlkIjoic2Vzc2lvbi1saXZlLTg2NGJkNDMyLTcwMWItNGQ1OS05MmNjLWRlMGU5YjhiZjVjNSIsInN0YXJ0ZWRfYXQiOiIyMDI0LTAyLTEyVDEzOjUwOjU4WiIsImxhc3RfYWNjZXNzZWRfYXQiOiIyMDI0LTAyLTEyVDEzOjUzOjI4WiIsImV4cGlyZXNfYXQiOiIyMDI0LTAzLTEzVDEzOjUzOjI4WiIsImF0dHJpYnV0ZXMiOnsidXNlcl9hZ2VudCI6Ik1vemlsbGEvNS4wIChXaW5kb3dzIE5UIDEwLjA7IFdpbjY0OyB4NjQ7IHJ2OjEyMi4wKSBHZWNrby8yMDEwMDEwMSBGaXJlZm94LzEyMi4wIiwiaXBfYWRkcmVzcyI6IjMuOS4yMTIuMjU1In0sImF1dGhlbnRpY2F0aW9uX2ZhY3RvcnMiOlt7InR5cGUiOiJvdHAiLCJkZWxpdmVyeV9tZXRob2QiOiJlbWFpbCIsImxhc3RfYXV0aGVudGljYXRlZF9hdCI6IjIwMjQtMDItMTJUMTM6NTA6NThaIiwiZW1haWxfZmFjdG9yIjp7ImVtYWlsX2lkIjoiZW1haWwtbGl2ZS01ODlkZWRlNi01YzQwLTRjYTUtYjA1OC1iNWI1NjljZTc2YWQiLCJlbWFpbF9hZGRyZXNzIjoidGVzdDFAZ3JyLmxhIn19XX0sImlhdCI6MTcwNzc0NjAwOCwiaXNzIjoic3R5dGNoLmNvbS9wcm9qZWN0LWxpdmUtN2U0YTMyMjEtNzljZC00ZjM0LWFjMWQtZmVkYWM0YmRlMTNlIiwibmJmIjoxNzA3NzQ2MDA4LCJzdWIiOiJ1c2VyLWxpdmUtYTlmMzUwNDYtOTE5Zi00YjJjLWI2YTAtZjRmNGRkODRkZmQ2IiwidHJhbnNhY3Rpb25faGFzaCI6InR4aGFzaCIsInhpb25fYWRkcmVzcyI6ImFkZHJlc3MxIn0.yn7uwUKFz9VlVPKXIcVJirMGhzYdmDXI_fTlHolqGnFPiqoHubm9mpEwIUCJdFghD89Nh9AmZYsHPC4ySNOfTTZ9LWHnSVSPJoyh8JmGNuZM45Zu0S6RrfZaAPY7QHaPP1nUYGl3QsSEVQuDaGdM4KIni-b_HdSMqvAiJoB0-LnDK8VdcHl7KtBOkcngZeEP5hR_yre95rD2Na7lVVwTAh-WxvnPoc1AHl0IUQun0Kf112ej67LtRzDcC98A90QxYRHsA5lUlgywdL6LmWqiiDNsoWwZn0ELQ0FDG5naHVFHLdKReA51PZhN7gcGXqopFh6IzGWZWVNVhmPCZRwtLA";
//pub const SESSION_JWT_2: &str = "eyJhbGciOiJSUzI1NiIsImtpZCI6Imp3ay1saXZlLTVjYjQwZjE4LTdiYjUtNGEwNi04ZjUzLTc4NjdiOGIzNjkzMCIsInR5cCI6IkpXVCJ9.eyJhdWQiOlsicHJvamVjdC1saXZlLTdlNGEzMjIxLTc5Y2QtNGYzNC1hYzFkLWZlZGFjNGJkZTEzZSJdLCJleHAiOjE3MDc3NDY0NDksImh0dHBzOi8vc3R5dGNoLmNvbS9zZXNzaW9uIjp7ImlkIjoic2Vzc2lvbi1saXZlLWUzZGE1Y2I4LWRjYjctNDM0Yi04NmI1LWM3NWJmYzI4OGZkMSIsInN0YXJ0ZWRfYXQiOiIyMDI0LTAyLTEyVDEzOjU0OjQ3WiIsImxhc3RfYWNjZXNzZWRfYXQiOiIyMDI0LTAyLTEyVDEzOjU1OjQ5WiIsImV4cGlyZXNfYXQiOiIyMDI0LTAzLTEzVDEzOjU1OjQ5WiIsImF0dHJpYnV0ZXMiOnsidXNlcl9hZ2VudCI6Ik1vemlsbGEvNS4wIChXaW5kb3dzIE5UIDEwLjA7IFdpbjY0OyB4NjQ7IHJ2OjEyMi4wKSBHZWNrby8yMDEwMDEwMSBGaXJlZm94LzEyMi4wIiwiaXBfYWRkcmVzcyI6IjMuOS4yMTIuMjU1In0sImF1dGhlbnRpY2F0aW9uX2ZhY3RvcnMiOlt7InR5cGUiOiJvdHAiLCJkZWxpdmVyeV9tZXRob2QiOiJlbWFpbCIsImxhc3RfYXV0aGVudGljYXRlZF9hdCI6IjIwMjQtMDItMTJUMTM6NTQ6NDdaIiwiZW1haWxfZmFjdG9yIjp7ImVtYWlsX2lkIjoiZW1haWwtbGl2ZS0zZWVmZTFlOC01NDExLTQ0NDQtYmY5YS04ZTc0MjdhNzZhMGYiLCJlbWFpbF9hZGRyZXNzIjoidGVzdDJAZ3JyLmxhIn19XX0sImlhdCI6MTcwNzc0NjE0OSwiaXNzIjoic3R5dGNoLmNvbS9wcm9qZWN0LWxpdmUtN2U0YTMyMjEtNzljZC00ZjM0LWFjMWQtZmVkYWM0YmRlMTNlIiwibmJmIjoxNzA3NzQ2MTQ5LCJzdWIiOiJ1c2VyLWxpdmUtNGRhNWE1N2MtMGMzOC00YWUxLTlkZWEtZTMyYTAyZDJlZWY2IiwidHJhbnNhY3Rpb25faGFzaCI6InR4aGFzaCIsInhpb25fYWRkcmVzcyI6ImFkZHJlc3MyIn0.gvmsHDukM9Fn2cr_PflmZhG4aQq9f8EXAPW72Ci6Egw8IAmuLLiTd0RyvIRKYZAnj7w3U534ImKd19l6090urzO7E913SRCDvz_8BQ8GOU25Lvo671VWrCVzqcOV7Z43bGQJrVWsZqPuWnAtzYg8fj0KwCvMHJPjpJxlQHXt5q_Nn8ZG0oXgxIEdMN7ynNTxcZNcyDVxfnRyJNq9hGHdBJ3Eh-9EOFZBNVfILCTS7Liel91sAs5AVEaftTPtg0P3m1POK9Y-4-rsKa2PLJM2-54D3o9eUU7Jsq6OI-99cMb6ITueZQ8NPR293Vchq2kfSoeR-i2sDdAhEUwiXA2-9w";
pub const AUDIENCE: &str = "project-live-7e4a3221-79cd-4f34-ac1d-fedac4bde13e";
pub const EMAIL_1: &str = "test1@grr.la";
pub const EMAIL_2: &str = "test2@grr.la";

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
    assert_eq!(
        query_resp.claims.iter().map(|q| q.amount).sum::<Uint128>(),
        amount
    );
}

#[test]
#[should_panic]
fn test_execute_receive_should_fail_with_wrong_email() {
    let mut dep = mock_dependencies();
    let info = mock_info("token", &[]);
    let amount = Uint128::new(100);

    send_token(
        dep.as_mut(),
        info.clone(),
        amount,
        "sender".to_string(),
        "EMAIL_1".to_string(),
    );

    // query claim
    let query_resp = query_claim(dep.as_ref(), EMAIL_1.to_string());
    assert_eq!(
        query_resp.claims.iter().map(|q| q.amount).sum::<Uint128>(),
        amount
    );
}

#[test]
fn test_query_claims() {
    let query_resp = query_claim(mock_dependencies().as_ref(), EMAIL_1.to_string());
    assert!(query_resp
        .claims
        .iter()
        .map(|q| q.amount)
        .sum::<Uint128>()
        .is_zero());
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
    assert_eq!(
        query_resp.claims.iter().map(|q| q.amount).sum::<Uint128>(),
        amount
    );

    // claim the tokens by email
    claim_by_email(
        AUDIENCE.to_string(),
        SESSION_JWT_1.to_string(),
        dep.as_mut(),
        mock_info("address1", &[]), // for email1 -> address1 and for email2 -> address2
        1,
    );
    // email1 shall have no more claims
    let query_resp = query_claim(dep.as_ref(), EMAIL_1.to_string());
    assert_eq!(
        query_resp.claims.iter().map(|q| q.amount).sum::<Uint128>(),
        Uint128::zero()
    );
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
        query_resp.claims.iter().map(|q| q.amount).sum::<Uint128>(),
        amount.saturating_mul(Uint128::new(4))
    );
    // Claim
    claim_by_email(
        AUDIENCE.to_string(),
        SESSION_JWT_1.to_string(),
        dep.as_mut(),
        mock_info("address1", &[]), // for email1 -> address1 and for email2 -> address2
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
        query_resp.claims.iter().map(|q| q.amount).sum::<Uint128>(),
        amount.saturating_mul(Uint128::new(8))
    );
    // Claim
    claim_by_email(
        AUDIENCE.to_string(),
        SESSION_JWT_1.to_string(),
        dep.as_mut(),
        mock_info("address1", &[]), // for email1 -> address1 and for email2 -> address2
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
    assert_eq!(
        query_resp.claims.iter().map(|q| q.amount).sum::<Uint128>(),
        amount
    );

    // claim the tokens by email
    claim_by_email(
        AUDIENCE.to_string(),
        SESSION_JWT_1.to_string(),
        dep.as_mut(),
        mock_info("address1", &[]), // for email1 -> address1 and for email2 -> address2
        1,
    );

    // panics
    claim_by_email(
        AUDIENCE.to_string(),
        SESSION_JWT_1.to_string(),
        dep.as_mut(),
        mock_info("address1", &[]), // for email1 -> address1 and for email2 -> address2
        1,
    );
}

#[test]
#[should_panic]
fn test_claim_panicks_with_wrong_sender() {
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
    assert_eq!(
        query_resp.claims.iter().map(|q| q.amount).sum::<Uint128>(),
        amount
    );

    // should panic
    claim_by_email(
        AUDIENCE.to_string(),
        SESSION_JWT_1.to_string(),
        dep.as_mut(),
        // another sender tries to claim tokens with valid jwt, but jwt is signed for address1
        mock_info("address2", &[]),
        1,
    );
}

#[test]
fn test_empty_memo() {
    let mut dep = mock_dependencies();
    let info = mock_info("token", &[]);
    let amount = Uint128::new(100);

    send_token_custom_memo(
        dep.as_mut(),
        info.clone(),
        amount,
        "sender".to_string(),
        EMAIL_1.to_string(),
        Option::None,
    );

    claim_by_email(
        AUDIENCE.to_string(),
        SESSION_JWT_1.to_string(),
        dep.as_mut(),
        mock_info("address1", &[]), // for email1 -> address1 and for email2 -> address2
        1,
    );
}

#[test]
#[should_panic]
fn test_panic_long_memo() {
    let mut dep = mock_dependencies();
    let info = mock_info("token", &[]);
    let amount = Uint128::new(100);

    send_token_custom_memo(
        dep.as_mut(),
        info.clone(),
        amount,
        "sender".to_string(),
        EMAIL_1.to_string(),
        Option::Some(SESSION_JWT_1.to_owned()),
    );

    claim_by_email(
        AUDIENCE.to_string(),
        SESSION_JWT_1.to_string(),
        dep.as_mut(),
        mock_info("address1", &[]), // for email1 -> address1 and for email2 -> address2
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
            assert_eq!(
                resp.events[resp.events.len() - 1].attributes.len(),
                _attr_qty
            );
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
        memo: Option::Some("This is a test memo".to_string()),
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

fn send_token_custom_memo(
    _dep: DepsMut,
    _info: MessageInfo,
    _amount: Uint128,
    _sender: String,
    _email: String,
    _memo: Option<String>,
) {
    let tok_msg = msg::TokenReceiveMsg {
        email: _email.to_string(),
        memo: _memo,
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
