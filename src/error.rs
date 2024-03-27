use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("invalid jwt kid")]
    InvalidJWTKid,

    #[error("invalid token")]
    InvalidToken,

    #[error("expired token")]
    ExpiredToken,

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

    #[error("Cannot migrate from different contract type: {previous_contract}")]
    CannotMigrate { previous_contract: String },

    #[error("Cannot migrate from unsupported version: {previous_version}")]
    CannotMigrateVersion { previous_version: String },
}
