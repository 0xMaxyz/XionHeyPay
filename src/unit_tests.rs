#![cfg(test)]

use crate::{
    contract::{self, instantiate},
    msg::{self, InstantiateMsg, KeysMsg, QueryClaimResponse, QueryMsg},
};
use cosmwasm_std::{
    from_json,
    testing::{mock_dependencies, mock_env, mock_info},
    to_json_binary, Deps, DepsMut, MessageInfo, Uint128,
};
use cw20::Cw20ReceiveMsg;

pub const AUDIENCE: &str =
    "965798652522-bn240k47q576vhqon2tuk0feg20bbt0u.apps.googleusercontent.com";
pub const EMAIL: &str = "meisamtaher71@gmail.com";
pub const SESSION_JWT: &str = "eyJhbGciOiJSUzI1NiIsImtpZCI6ImE0OTM5MWJmNTJiNThjMWQ1NjAyNTVjMmYyYTA0ZTU5ZTIyYTdiNjUiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiI5NjU3OTg2NTI1MjItYm4yNDBrNDdxNTc2dmhxb24ydHVrMGZlZzIwYmJ0MHUuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiI5NjU3OTg2NTI1MjItYm4yNDBrNDdxNTc2dmhxb24ydHVrMGZlZzIwYmJ0MHUuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTM1NTUwMjYxMjgxMDczODk5MTYiLCJlbWFpbCI6Im1laXNhbXRhaGVyNzFAZ21haWwuY29tIiwiZW1haWxfdmVyaWZpZWQiOnRydWUsIm5vbmNlIjoieGlvbjE1YTV3czJ5NjM2a2YydnM3d2V0czQ0YXk3OGRzcjV6cTMzOTllcWZ2NG5rbmt3N2phdmRzNnA3Y2NjIiwibmJmIjoxNzI0MzY2NzA3LCJuYW1lIjoiTWVpc2FtIFRhaGVyIiwicGljdHVyZSI6Imh0dHBzOi8vbGgzLmdvb2dsZXVzZXJjb250ZW50LmNvbS9hL0FDZzhvY0o5QzBLeTRYR2JWejEySk1FRFpBRk9oS2lZeEh5SnlIeUdrU2VIRHVwSkFhZHB1NEJWPXM5Ni1jIiwiZ2l2ZW5fbmFtZSI6Ik1laXNhbSIsImZhbWlseV9uYW1lIjoiVGFoZXIiLCJpYXQiOjE3MjQzNjcwMDcsImV4cCI6MTcyNDM3MDYwNywianRpIjoiMjRlNjU5OTc1M2Y0ZDAxZTA3YTZjNTZmODJiMGY2Y2U1YWEzNDY5MCJ9.P4kKcgI3zD_X-eA5HekOsbHH-MH_7F5LZ8oU_b14NEXB6liSDiVIABwiNIGYBXQdjsnoNycu3Su6FwxGOZaeJ7XPNx-C-4KCpJnBosXV2Reu49dA3wz5bo7N_xpqn154xoRHmC2Ymx3jKXKq-nYVqmQX0Ug4uVFqGcumhpGfYg6PfY3LT3z5F2hXGwkWJ_1DyMKkp_MwRzlwoLCt-sHov6qV6WqzyakHq3qGxUCUCu0JWFlRThmWQ7q2C4pk0P6yBJKN3URtvgq_I-5xOtxt097aZCbVc0S29jCvyerbV9ZFi2hj8dgyFj1hQm-m6TQiHzGEEwUMly80n_iyVidVzQ";
pub const NOUNCE: &str = "xion15a5ws2y636kf2vs7wets44ay78dsr5zq3399eqfv4nknkw7javds6p7ccc";

#[test]
fn test_instantiate() {
    let mut deps = mock_dependencies();

    let instantiate_msg = InstantiateMsg {
        keys_msg: KeysMsg {
            key1: "d2d444cf8c5e3ae8386d66a13a316976aa369991".to_string(),
            n1: "onV5tzUbqyPfkM6MwUqCtrqun9x20hEUbIUlmAYYuPuMhsaNHJqs1AVzRt2TzaNjmPVddEbU7VMDmeFWUt7vgDi7Xu0leevuIN4VSPbAMGBa0oj9Qopqkn9ePO_7DvIN13ktHgfQqatNBu6uXH6zkUl3VtXnubXrUhx7uyF22dARDc1-pJoj2NnsvgxDRElPMyDkU-siVv3c6cgIEwLEZZPWOcwplPTUB4qeTK6prrPBGQshuE1PWK2ZrYpIvXfzHyEbkGdPnrhcxgCzbKBUFvr8n_sfSurLRoDBLjkURKmgB8T8iRzLyXsCu9D3Hw61LKuex1aeSQLdwOFLuUEBdw".to_string(),
            e1: "AQAB".to_string(),
            key2: "a49391bf52b58c1d560255c2f2a04e59e22a7b65".to_string(),
            n2: "v7hTj49pNGYjxKbgMx_iDyjeErhfJFepMl306IV_TW5T_CEGE4lWFfBe9w0cwpi5KD6XlC1GO1AsrtzcYF29wJ283GNBZRkbl8iTe-LQYdjQsBtf_1fLIVt6LR7H2U1RPqa3pY16Kq6i6yC2osVg6tD7ApBCGw1WKe8uU3cm28biJzuV4gv6PzcbOdErd-hb4Cv6n2SoMPYlBfT4pWee75poQh8DYoQ1KJwowz3U1MaxOBMP260hmDK-QK0q4LYabCQiBNsz4FWWcaAAFxZFbiqGY5Gdu18uOkpMbdAN5FoZ_6nMDMSTmlf0CHv7gZe_cL38kZvTaynkWwDxqsW_Xw".to_string(),
            e2: "AQAB".to_string(),
            key3: "".to_string(),
            n3: "".to_string(),
            e3: "".to_string(),
        },
    };
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
        mock_info(NOUNCE, &[]),
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
        mock_info(NOUNCE, &[]),
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
        mock_info(NOUNCE, &[]),
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
        mock_info(NOUNCE, &[]),
        1,
    );

    // panics
    claim_by_email(
        AUDIENCE.to_string(),
        SESSION_JWT.to_string(),
        dep.as_mut(),
        mock_info(NOUNCE, &[]),
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
        mock_info(NOUNCE, &[]),
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
        mock_info(NOUNCE, &[]),
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
