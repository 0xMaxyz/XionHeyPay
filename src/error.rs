use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("invalid jwt aud")]
    InvalidJWTAud,

    #[error("invalid token")]
    InvalidToken,

    #[error("nothing to claim")]
    NotClaimable,
}
