use crate::state::TokenId;
use cosmwasm_std::{Addr, StdError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    StdError(#[from] StdError),

    #[error("{role} doesn't exist")]
    NonExistRole { role: String },

    #[error("unknown error occurred while updating the slot")]
    UpdatingErrorSlot,

    #[error("already have the token, could not mint another")]
    AlreadyMintedToken,

    #[error("{account} already grant role {role}")]
    AlreadyGrantRole { account: Addr, role: String },

    #[error("{token_id} kyc data doesn't exist")]
    NonExistKyc { token_id: TokenId },

    #[error("already approved to {cooperator}")]
    AlreadyApproved { cooperator: Addr },

    #[error("{account} is not a valid admin")]
    InvalidAdminAccount { account: Addr },

    #[error("{account} is not a valid operator")]
    InvalidOperatorAccount { account: Addr },

    #[error("{account} is not a valid cooperator")]
    InvalidCooperatorAccount { account: Addr },

    #[error("{token_id} has no owner")]
    InvalidTokenId { token_id: TokenId },

    #[error("{token_id} didn't belongs to {owner}")]
    InvalidOwner { token_id: TokenId, owner: Addr },

    #[error("{owner} didn't approved to {cooperator}")]
    InvalidRevokeFromCooperator { owner: Addr, cooperator: Addr },

    #[error("{user} didn't have any zkMe sbt")]
    NoSBTExist { user: Addr },

    #[error("{user} didn't approved to {cooperator}")]
    NoApprovementExist { cooperator: Addr, user: Addr },
}
