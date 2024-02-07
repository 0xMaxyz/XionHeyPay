use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
mod contract;
mod msg;
mod state;

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: msg::InstantiateMsg,
) -> StdResult<Response> {
    contract::instantiate(_deps, _env, _info, _msg)
}

#[entry_point]
pub fn query(_deps: Deps, _env: Env, _msg: msg::QueryMsg) -> StdResult<Binary> {
    contract::query(_deps, _env, _msg)
}
