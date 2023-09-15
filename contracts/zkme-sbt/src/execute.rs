use crate::{
    error::ContractError,
    state::{self, TokenId},
};
use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response, Timestamp};

pub(crate) fn grant_role(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    role: &str,
    user: Addr,
) -> Result<Response, ContractError> {
    if state::has_role(deps.storage, state::OPERATOR_ROLE, &info.sender)? {
        state::grant_role(deps.storage, role, user.clone())?;
    } else {
        return Err(ContractError::InvalidAdminAccount {
            account: info.sender,
        });
    }
    let resp = Response::new()
        .add_attribute("action", "grantRole")
        .add_attribute("role", role)
        .add_attribute("account", user);

    Ok(resp)
}

pub(crate) fn revoke_role(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    role: &str,
    user: Addr,
) -> Result<Response, ContractError> {
    if state::has_role(deps.storage, state::DEFAULT_ADMIN_ROLE, &info.sender)? {
        state::revoke_role(deps.storage, role, &user)?;
    } else {
        return Err(ContractError::InvalidAdminAccount {
            account: info.sender,
        });
    }

    let resp = Response::new()
        .add_attribute("action", "revokeRole")
        .add_attribute("role", role)
        .add_attribute("account", user);

    Ok(resp)
}

pub(crate) fn set_questions(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    cooperator: Addr,
    questions: Vec<String>,
) -> Result<Response, ContractError> {
    if state::has_role(deps.storage, state::OPERATOR_ROLE, &info.sender)?
        && state::has_role(deps.storage, state::COOPERATOR_ROLE, &cooperator)?
    {
        state::set_questions(deps.storage, &cooperator, questions)?;
    } else {
        return Err(ContractError::InvalidOperatorAccount {
            account: info.sender,
        });
    }

    let resp = Response::new()
        .add_attribute("action", "setQuestions")
        .add_attribute("cooperator", cooperator);

    Ok(resp)
}

pub(crate) fn attest(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    to: Addr,
) -> Result<Response, ContractError> {
    if state::has_token(deps.storage, &to) {
        return Err(ContractError::AlreadyMintedToken);
    }

    let token_id = state::get_token_id(deps.storage)?;
    state::save_owner(deps.storage, &token_id, &to)?;
    state::save_token(deps.storage, &to, &token_id)?;
    state::increase_counter(deps.storage)?;

    let resp = Response::new()
        .add_attribute("action", "attest")
        .add_attribute("payer", info.sender)
        .add_attribute("to", to)
        .add_attribute("tokenId", token_id.to_string());

    Ok(resp)
}

pub(crate) fn set_kyc_data(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    token_id: TokenId,
    key: String,
    validity: Timestamp,
    data: String,
    questions: Vec<String>,
) -> Result<Response, ContractError> {
    if state::has_role(deps.storage, state::OPERATOR_ROLE, &info.sender)? {
        if state::has_owner(deps.storage, &token_id)? {
            state::save_kyc(
                deps.storage,
                &token_id,
                state::KycData {
                    key,
                    validity,
                    data,
                    questions,
                },
            )?;
        } else {
            return Err(ContractError::InvalidTokenId { token_id });
        }
    } else {
        return Err(ContractError::InvalidOperatorAccount {
            account: info.sender,
        });
    }

    let resp = Response::new()
        .add_attribute("action", "setKycData")
        .add_attribute("tokenId", token_id.to_string());

    Ok(resp)
}

pub(crate) fn set_token_base_uri(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    uri: String,
) -> Result<Response, ContractError> {
    if state::has_role(deps.storage, state::DEFAULT_ADMIN_ROLE, &info.sender)? {
        state::set_token_base_uri(deps.storage, uri.clone())?;
    } else {
        return Err(ContractError::InvalidAdminAccount {
            account: info.sender,
        });
    }

    let resp = Response::new()
        .add_attribute("action", "setTokenBaseUri")
        .add_attribute("newUri", uri);
    Ok(resp)
}

pub(crate) fn approve(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    cooperator: Addr,
    token_id: TokenId,
    cooperator_key: String,
) -> Result<Response, ContractError> {
    let owner = state::get_owner(deps.storage, &token_id)?
        .ok_or(ContractError::InvalidTokenId { token_id })?;
    if owner != info.sender {
        return Err(ContractError::InvalidOwner { token_id, owner });
    }

    if state::has_role(deps.storage, state::COOPERATOR_ROLE, &cooperator)? {
        state::save_pu(deps.storage, &cooperator, &owner, token_id)?;
        state::save_approved(deps.storage, &cooperator, token_id)?;
        state::save_approved_kyc_data(deps.storage, &cooperator, token_id, cooperator_key)?;
    } else {
        return Err(ContractError::InvalidCooperatorAccount {
            account: cooperator,
        });
    }

    let resp = Response::new()
        .add_attribute("action", "approve")
        .add_attribute("cooperator", cooperator)
        .add_attribute("tokenId", token_id.to_string())
        .add_attribute("user", owner);

    Ok(resp)
}

pub(crate) fn revoke(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    cooperator: Addr,
    token_id: TokenId,
) -> Result<Response, ContractError> {
    let owner = state::get_owner(deps.storage, &token_id)?
        .ok_or(ContractError::InvalidTokenId { token_id })?;
    if owner != info.sender {
        return Err(ContractError::InvalidOwner { token_id, owner });
    }

    if state::has_role(deps.storage, state::COOPERATOR_ROLE, &cooperator)? {
        if state::has_approve(deps.storage, &cooperator, &owner) {
            state::remove_pu(deps.storage, &cooperator, &owner);
            state::remove_approved(deps.storage, &cooperator, token_id)?;
        } else {
            return Err(ContractError::InvalidRevokeFromCooperator { owner, cooperator });
        }
    } else {
        return Err(ContractError::InvalidCooperatorAccount {
            account: cooperator,
        });
    }

    let resp = Response::new()
        .add_attribute("action", "revoke")
        .add_attribute("cooperator", cooperator)
        .add_attribute("tokenId", token_id.to_string())
        .add_attribute("user", owner);

    Ok(resp)
}
