use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    StdError(#[from] StdError),
    #[error("Generic error: {err}")]
    GenericError { err: String },
    #[error("Member with username [{username}] already exists")]
    MemberAlreadyExists { username: String },
    #[error("Member username must be at least 4 characters long: {username}")]
    InvalidMemberUsernameLen { username: String },
    #[error("Member intro must be at least 12 characters long")]
    InvalidMemberIntroLen,
}
