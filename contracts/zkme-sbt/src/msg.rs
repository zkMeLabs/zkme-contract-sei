use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Timestamp};

use crate::state::{KycData, TokenId};

#[cw_serde]
pub struct InitMsg {
    pub roles: Vec<String>,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Access Control
    GrantRole {
        role: String,
        user: Addr,
    },

    RevokeRole {
        role: String,
        user: Addr,
    },

    /// Conf
    SetQuestions {
        cooperator: Addr,
        questions: Vec<String>,
    },

    /// zkMeSBT
    Attest {
        to: Addr,
    },

    Burn {
        from: Addr,
        token_id: TokenId,
    },

    SetKycData {
        token_id: TokenId,
        key: String,
        validity: Timestamp,
        data: String,
        questions: Vec<String>,
    },

    SetTokenBaseUri {
        uri: String,
    },

    // TODO cw721 methods
    /// zkMeVerifyLite
    Approve {
        cooperator: Addr,
        token_id: TokenId,
        cooperator_key: String,
    },

    Revoke {
        cooperator: Addr,
        token_id: TokenId,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Access Control
    #[returns(IsRoleResponse)]
    IsRole { role: String, user: Addr },

    /// Conf
    #[returns(GetQuestionsResponse)]
    GetQuestions { cooperator: Addr },

    /// zkMeSBT
    #[returns(GetKycDataResponse)]
    GetKycData { token_id: TokenId },

    #[returns(BalanceOfResponse)]
    BalanceOf { owner: Addr },

    #[returns(TokenIdOfResponse)]
    TokenIdOf { from: Addr },

    #[returns(OwnerOfResponse)]
    OwnerOf { token_id: TokenId },

    #[returns(TotalSupplyResponse)]
    TotalSupply {},
    #[returns(TotalSupplyResponse)]
    NumTokens {},

    #[returns(TokenUriResponse)]
    TokenUri { token_id: TokenId },

    /// zkMeVerify
    #[returns(VerifyResponse)]
    Verify { cooperator: Addr, user: Addr },

    #[returns(HasApprovedResponse)]
    HasApproved { cooperator: Addr, user: Addr },

    #[returns(GetUserTokenIdResponse)]
    GetUserTokenId { cooperator: Addr, user: Addr },

    #[returns(GetUserDataResponse)]
    GetUserData { cooperator: Addr, user: Addr },

    #[returns(GetApprovedTokenIdResponse)]
    GetApprovedTokenId {
        cooperator: Addr,
        start: u64,
        page_size: u64,
    },

    #[returns(GetApprovedLengthResponse)]
    GetApprovedLength { cooperator: Addr },

    #[returns(GetApprovedUserKycDataResponse)]
    GetApprovedUserKycData { cooperator: Addr, user: Addr },
}

#[cw_serde]
pub struct IsRoleResponse {
    pub role: String,
    pub user: Addr,
    pub result: bool,
}

#[cw_serde]
pub struct GetQuestionsResponse {
    pub cooperator: Addr,
    pub questions: Vec<String>,
}

#[cw_serde]
pub struct GetKycDataResponse {
    pub owner: Addr,
    pub token_id: TokenId,
    pub key: String,
    pub validity: Timestamp,
    pub data: String,
    pub questions: Vec<String>,
}

#[cw_serde]
pub struct BalanceOfResponse {
    pub owner: Addr,
    pub balance: u8,
}

#[cw_serde]
pub struct TokenIdOfResponse {
    pub from: Addr,
    pub token_id: TokenId,
}

#[cw_serde]
pub struct OwnerOfResponse {
    pub token_id: TokenId,
    pub owner: Addr,
}

#[cw_serde]
pub struct TotalSupplyResponse {
    pub total: u64,
}

#[cw_serde]
pub struct TokenUriResponse {
    pub token_id: TokenId,
    pub token_uri: String,
}

#[cw_serde]
pub struct VerifyResponse {
    pub cooperator: Addr,
    pub user: Addr,
    pub result: bool,
}

#[cw_serde]
pub struct HasApprovedResponse {
    pub cooperator: Addr,
    pub user: Addr,
    pub has_approved: bool,
}

#[cw_serde]
pub struct GetUserTokenIdResponse {
    pub user: Addr,
    pub token_id: TokenId,
}

#[cw_serde]
pub struct GetUserDataResponse {
    pub user: Addr,
    pub kyc_data: KycData,
}

#[cw_serde]
pub struct GetApprovedTokenIdResponse {
    pub cooperator: Addr,
    pub start: u64,
    pub page_size: u64,
    pub token_id_list: Vec<TokenId>,
}

#[cw_serde]
pub struct GetApprovedLengthResponse {
    pub cooperator: Addr,
    pub approved_length: u64,
}

#[cw_serde]
pub struct GetApprovedUserKycDataResponse {
    pub cooperator: Addr,
    pub user: Addr,
    pub token_id: TokenId,
    pub data: KycData,
}
