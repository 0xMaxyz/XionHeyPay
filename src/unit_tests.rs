#![cfg(test)]

use crate::{
    contract::{self, instantiate},
    msg::{self, InstantiateMsg, QueryClaimResponse, QueryMsg},
};
use cosmwasm_std::{
    from_json,
    testing::{mock_dependencies, mock_env, mock_info},
    to_json_binary, Deps, DepsMut, MessageInfo, Uint128,
};
use cw20::Cw20ReceiveMsg;

pub const SESSION_JWT:&str="eyJhbGciOiJSUzI1NiIsImtpZCI6ImFkZjVlNzEwZWRmZWJlY2JlZmE5YTYxNDk1NjU0ZDAzYzBiOGVkZjgiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMjIwMzc4MzcxNTQtcG5oNXJkcjhkOWh2Zmo5aW9vcmU2YW1iMGdxczRiajkuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMjIwMzc4MzcxNTQtcG5oNXJkcjhkOWh2Zmo5aW9vcmU2YW1iMGdxczRiajkuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMDE5ODkxODYwMDE4MjUxMDI1NTAiLCJlbWFpbCI6IjB4bWF4eXpAZ21haWwuY29tIiwiZW1haWxfdmVyaWZpZWQiOnRydWUsIm5vbmNlIjoiYWRkcmVzczEiLCJuYmYiOjE3MTE1MzU2MzIsIm5hbWUiOiJNYXgiLCJwaWN0dXJlIjoiaHR0cHM6Ly9saDMuZ29vZ2xldXNlcmNvbnRlbnQuY29tL2EvQUNnOG9jSlA4RFVmbnhKbVo0NUlDd2JCR2h4VlByMTFmdWFGdnMyRVRNRWh6dmNQUnc9czk2LWMiLCJnaXZlbl9uYW1lIjoiTWF4IiwiaWF0IjoxNzExNTM1OTMyLCJleHAiOjE3MTE1Mzk1MzIsImp0aSI6IjllYzg1NGQwNzQ1NTdmZjEzZTgwMjRlZDEzYTlmY2ViNmI3YTI3OTMifQ.wUXRUSEgOytmdalPHzZtqcUNhfYkvEC1KpSQK5OMre5qVT6-sahR7VWLIkTKz7gs6SNTFmecM1Kceis3CSjKKLxESZU2RrDZPlE7lUvZFy7DSc5KfaE46jOj7hYnsheUmG2xxVqP2ismzfeQwKN00qf8BrmuF8-DEP4TwWUFZ6LfNBfHmjxnIdzx5rt-px_GHvSgs8P_SG1zpCrVov2_eDzcLd1yok3fHKv6LQQGsl91tcZwnJukb1_XIIQy7HYFpdj8ixrVRwAOMrYsT9n_U3BZDfe2wjoLjEwTSwBQuJdcYu2ie-KbReVO6lGtEmIA7eZzmIgNiBZgoWSrW89ULA";
pub const AUDIENCE: &str =
    "222037837154-pnh5rdr8d9hvfj9ioore6amb0gqs4bj9.apps.googleusercontent.com";
pub const EMAIL: &str = "0xmaxyz@gmail.com";

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
        EMAIL.to_string(),
    );

    // query claim
    let query_resp = query_claim(dep.as_ref(), EMAIL.to_string());
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
    let query_resp = query_claim(dep.as_ref(), EMAIL.to_string());
    assert_eq!(
        query_resp.claims.iter().map(|q| q.amount).sum::<Uint128>(),
        amount
    );
}

#[test]
fn test_query_claims() {
    let query_resp = query_claim(mock_dependencies().as_ref(), EMAIL.to_string());
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
        EMAIL.to_string(),
    );

    // query claim
    let query_resp = query_claim(dep.as_ref(), EMAIL.to_string());
    assert_eq!(
        query_resp.claims.iter().map(|q| q.amount).sum::<Uint128>(),
        amount
    );

    // claim the tokens by email
    claim_by_email(
        AUDIENCE.to_string(),
        SESSION_JWT.to_string(),
        dep.as_mut(),
        mock_info("address1", &[]), // for email1 -> address1 and for email2 -> address2
        1,
    );
    // email1 shall have no more claims
    let query_resp = query_claim(dep.as_ref(), EMAIL.to_string());
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
            EMAIL.to_string(),
        );
    }
    // query claims
    let query_resp = query_claim(dep.as_ref(), EMAIL.to_string());
    assert_eq!(
        query_resp.claims.iter().map(|q| q.amount).sum::<Uint128>(),
        amount.saturating_mul(Uint128::new(4))
    );
    // Claim
    claim_by_email(
        AUDIENCE.to_string(),
        SESSION_JWT.to_string(),
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
            EMAIL.to_string(),
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
            EMAIL.to_string(),
        );
    }

    // query claims
    let query_resp = query_claim(dep.as_ref(), EMAIL.to_string());
    assert_eq!(
        query_resp.claims.iter().map(|q| q.amount).sum::<Uint128>(),
        amount.saturating_mul(Uint128::new(8))
    );
    // Claim
    claim_by_email(
        AUDIENCE.to_string(),
        SESSION_JWT.to_string(),
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
        EMAIL.to_string(),
    );

    // query claim
    let query_resp = query_claim(dep.as_ref(), EMAIL.to_string());
    assert_eq!(
        query_resp.claims.iter().map(|q| q.amount).sum::<Uint128>(),
        amount
    );

    // claim the tokens by email
    claim_by_email(
        AUDIENCE.to_string(),
        SESSION_JWT.to_string(),
        dep.as_mut(),
        mock_info("address1", &[]), // for email1 -> address1 and for email2 -> address2
        1,
    );

    // panics
    claim_by_email(
        AUDIENCE.to_string(),
        SESSION_JWT.to_string(),
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
        EMAIL.to_string(),
    );

    // query claim
    let query_resp = query_claim(dep.as_ref(), EMAIL.to_string());
    assert_eq!(
        query_resp.claims.iter().map(|q| q.amount).sum::<Uint128>(),
        amount
    );

    // should panic
    claim_by_email(
        AUDIENCE.to_string(),
        SESSION_JWT.to_string(),
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
        EMAIL.to_string(),
        Option::None,
    );

    claim_by_email(
        AUDIENCE.to_string(),
        SESSION_JWT.to_string(),
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
        EMAIL.to_string(),
        Option::Some(SESSION_JWT.to_owned()),
    );

    claim_by_email(
        AUDIENCE.to_string(),
        SESSION_JWT.to_string(),
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
        jwt: _jwt.to_owned(),
        testing: true,
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
