#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:pink-panther";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jwt::verify;

    #[test]
    fn test_jwt_verification() {
        let base64_jwt ="eyJ0eXAiOiJKV1QiLCJhbGciOiJFUzI1NiIsImtpZCI6ImEyYmQyNTRhMmRlZWEwYmVjYTc2NzQ2MGYxZmYyOGY3In0.eyJhdWQiOlsicHJvamVjdC1saXZlLTdlNGEzMjIxLTc5Y2QtNGYzNC1hYzFkLWZlZGFjNGJkZTEzZSJdLCJleHAiOjE3MDcwNjE1NjMsImh0dHBzOi8vc3R5dGNoLmNvbS9zZXNzaW9uIjp7ImlkIjoic2Vzc2lvbi1saXZlLTVkY2M4M2Q2LTg3NTEtNGQzNC04NzBjLWNmMmY3YzcxYjcyZCIsInN0YXJ0ZWRfYXQiOiIyMDI0LTAyLTA0VDE1OjM3OjQwWiIsImxhc3RfYWNjZXNzZWRfYXQiOiIyMDI0LTAyLTA0VDE1OjQxOjAzWiIsImV4cGlyZXNfYXQiOiIyMDI0LTAyLTA0VDE2OjM3OjQwWiIsImF0dHJpYnV0ZXMiOnsidXNlcl9hZ2VudCI6Ik1vemlsbGEvNS4wIChXaW5kb3dzIE5UIDEwLjA7IFdpbjY0OyB4NjQ7IHJ2OjEyMi4wKSBHZWNrby8yMDEwMDEwMSBGaXJlZm94LzEyMi4wIiwiaXBfYWRkcmVzcyI6IjUwLjcuODUuMjIxIn0sImF1dGhlbnRpY2F0aW9uX2ZhY3RvcnMiOlt7InR5cGUiOiJvdHAiLCJkZWxpdmVyeV9tZXRob2QiOiJlbWFpbCIsImxhc3RfYXV0aGVudGljYXRlZF9hdCI6IjIwMjQtMDItMDRUMTU6Mzc6NDBaIiwiZW1haWxfZmFjdG9yIjp7ImVtYWlsX2lkIjoiZW1haWwtbGl2ZS1iYWViMjk2Yy1kZDdjLTRhZWEtYmJmMy1jNjc0NGVlYjA0NWYiLCJlbWFpbF9hZGRyZXNzIjoiaGFtaWRyYWRwakBnbWFpbC5jb20ifX1dfSwiaWF0IjoxNzA3MDYxMjYzLCJpc3MiOiJzdHl0Y2guY29tL3Byb2plY3QtbGl2ZS03ZTRhMzIyMS03OWNkLTRmMzQtYWMxZC1mZWRhYzRiZGUxM2UiLCJuYmYiOjE3MDcwNjEyNjMsInN1YiI6InVzZXItbGl2ZS1lMjJkZGJmZC1jNjkxLTQ0YjktYjkyZS03MDRiNjRiYmIyZDkifQ.jWgIPQNaG5mwmZWwTRoaHusmj-f89s-9hMAhtHbxU09wUP06zetB2hwg_elJG6NkEwomPax6U_0hYv2tV9tQmg";
        let aud = "project-test-5ae234a7-6b74-46af-a7b7-969f3df38cc0";
        let email_in_token = "input email";

        let email_extracted_from_token = verify(base64_jwt.as_bytes(), &aud).unwrap();

        assert_eq!(email_in_token, email_extracted_from_token);
    }
}
