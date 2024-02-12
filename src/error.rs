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

    #[error("arithmatic error happened")]
    Arithmatic,

    #[error("couldn't transfer tokens")]
    Transfer,

    #[error("invalid key")]
    InvalidKey,

    #[error("memo length is limited to 256 characters")]
    MemoLength,

    #[error("Invalid instantiate msg")]
    InvalidMsg,

    #[error("Invalid email")]
    InvalidEmail,
}
