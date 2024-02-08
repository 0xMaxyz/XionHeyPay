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

    #[error("signature is invalid. expected: {expected}, received {received}")]
    InvalidSignatureDetail { expected: String, received: String },

    #[error("invalid time on signature. current: {current} received: {received}")]
    InvalidTime { current: u64, received: u64 },

    #[error("invalid token")]
    InvalidToken,
}
