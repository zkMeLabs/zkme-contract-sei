use cosmwasm_std::{Addr, Deps, Env};

use crate::{
    error::ContractError,
    msg,
    state::{self, TokenId},
};

pub(crate) fn is_role(
    deps: Deps,
    _env: Env,
    role: &str,
    user: Addr,
) -> Result<msg::IsRoleResponse, ContractError> {
    let result = state::has_role(deps.storage, role, &user)?;
    Ok(msg::IsRoleResponse {
        role: role.to_string(),
        user,
        result,
    })
}

pub(crate) fn get_questions(
    deps: Deps,
    _env: Env,
    cooperator: Addr,
) -> Result<msg::GetQuestionsResponse, ContractError> {
    let questions = state::get_questions(deps.storage, &cooperator)?.unwrap_or(vec![]);
    Ok(msg::GetQuestionsResponse {
        cooperator,
        questions,
    })
}

pub(crate) fn get_kyc_data(
    deps: Deps,
    _env: Env,
    token_id: TokenId,
) -> Result<msg::GetKycDataResponse, ContractError> {
    let owner = state::get_owner(deps.storage, &token_id)?
        .ok_or(ContractError::InvalidTokenId { token_id })?;
    let kyc_data =
        state::get_kyc(deps.storage, &token_id)?.ok_or(ContractError::NonExistKyc { token_id })?;

    Ok(msg::GetKycDataResponse {
        owner,
        token_id,
        key: kyc_data.key,
        validity: kyc_data.validity,
        data: kyc_data.data,
        questions: kyc_data.questions,
    })
}

pub(crate) fn balance_of(
    deps: Deps,
    _env: Env,
    owner: Addr,
) -> Result<msg::BalanceOfResponse, ContractError> {
    let balance = if state::has_token(deps.storage, &owner) {
        1
    } else {
        0
    };

    Ok(msg::BalanceOfResponse { owner, balance })
}

pub(crate) fn token_id_of(
    deps: Deps,
    _env: Env,
    from: Addr,
) -> Result<msg::TokenIdOfResponse, ContractError> {
    let token_id = state::get_token(deps.storage, &from)?
        .ok_or(ContractError::NoSBTExist { user: from.clone() })?;

    Ok(msg::TokenIdOfResponse { from, token_id })
}

pub(crate) fn owner_of(
    deps: Deps,
    _env: Env,
    token_id: TokenId,
) -> Result<msg::OwnerOfResponse, ContractError> {
    let owner = state::get_owner(deps.storage, &token_id)?
        .ok_or(ContractError::InvalidTokenId { token_id })?;

    Ok(msg::OwnerOfResponse { token_id, owner })
}

pub(crate) fn total_supply(
    deps: Deps,
    _env: Env,
) -> Result<msg::TotalSupplyResponse, ContractError> {
    let counter = state::get_token_id(deps.storage)?;

    Ok(msg::TotalSupplyResponse { total: counter - 1 })
}

pub(crate) fn token_uri(
    deps: Deps,
    _env: Env,
    token_id: TokenId,
) -> Result<msg::TokenUriResponse, ContractError> {
    let token_base_uri = state::get_token_base_uri(deps.storage)?;

    Ok(msg::TokenUriResponse {
        token_id,
        token_uri: format!("{token_base_uri}/{token_id}"),
    })
}

pub(crate) fn verify(
    deps: Deps,
    env: Env,
    cooperator: Addr,
    user: Addr,
) -> Result<msg::VerifyResponse, ContractError> {
    let token_id = state::get_token(deps.storage, &user)?
        .ok_or(ContractError::NoSBTExist { user: user.clone() })?;
    let user_data =
        state::get_kyc(deps.storage, &token_id)?.ok_or(ContractError::NonExistKyc { token_id })?;
    let cooperator_data = state::get_questions(deps.storage, &cooperator)?;

    let result: bool = user_data.validity >= env.block.time
        && cooperator_data
            .and_then(|questions| {
                Some(
                    questions
                        .iter()
                        .all(|question| user_data.questions.contains(question)),
                )
            })
            .unwrap_or(false);

    Ok(msg::VerifyResponse {
        cooperator,
        user,
        result,
    })
}

pub(crate) fn has_approved(
    deps: Deps,
    _env: Env,
    cooperator: Addr,
    user: Addr,
) -> Result<msg::HasApprovedResponse, ContractError> {
    let is_approved = state::has_approve(deps.storage, &cooperator, &user);
    Ok(msg::HasApprovedResponse {
        cooperator,
        user,
        has_approved: is_approved,
    })
}

pub(crate) fn get_user_token_id(
    deps: Deps,
    _env: Env,
    cooperator: Addr,
    user: Addr,
) -> Result<msg::GetUserTokenIdResponse, ContractError> {
    if !state::has_approve(deps.storage, &cooperator, &user) {
        return Err(ContractError::NoApprovementExist {
            cooperator,
            user: user.clone(),
        });
    }
    let token_id = state::get_token(deps.storage, &user)?
        .ok_or(ContractError::NoSBTExist { user: user.clone() })?;

    Ok(msg::GetUserTokenIdResponse { user, token_id })
}

pub(crate) fn get_user_data(
    deps: Deps,
    _env: Env,
    cooperator: Addr,
    user: Addr,
) -> Result<msg::GetUserDataResponse, ContractError> {
    if !state::has_approve(deps.storage, &cooperator, &user) {
        return Err(ContractError::NoApprovementExist {
            cooperator,
            user: user.clone(),
        });
    }
    let token_id = state::get_token(deps.storage, &user)?
        .ok_or(ContractError::NoSBTExist { user: user.clone() })?;
    let kyc_data = state::get_approved_kyc_data(deps.storage, &cooperator, token_id)?
        .ok_or(ContractError::NonExistKyc { token_id })?;

    Ok(msg::GetUserDataResponse { user, kyc_data })
}

pub(crate) fn get_approved_token_id(
    deps: Deps,
    _env: Env,
    cooperator: Addr,
    start: u64,
    page_size: u64,
) -> Result<msg::GetApprovedTokenIdResponse, ContractError> {
    let approved_id_list = state::get_approved(deps.storage, &cooperator)?;
    let length = approved_id_list.len().try_into().unwrap();

    let (offset, real_page_size) = if start > length {
        (0, 0)
    } else {
        (start, u64::min(length - start, page_size))
    };

    let token_id_list = approved_id_list
        .into_iter()
        .skip(offset as usize)
        .take(real_page_size as usize)
        .collect();

    Ok(msg::GetApprovedTokenIdResponse {
        cooperator,
        start: offset,
        page_size: real_page_size,
        token_id_list,
    })
}

pub(crate) fn get_approved_length(
    deps: Deps,
    _env: Env,
    cooperator: Addr,
) -> Result<msg::GetApprovedLengthResponse, ContractError> {
    let approved_length = state::get_approved(deps.storage, &cooperator)?
        .len()
        .try_into()
        .unwrap();
    Ok(msg::GetApprovedLengthResponse {
        cooperator,
        approved_length,
    })
}

pub(crate) fn get_approved_user_kyc_data(
    deps: Deps,
    _env: Env,
    cooperator: Addr,
    user: Addr,
) -> Result<msg::GetApprovedUserKycDataResponse, ContractError> {
    let token_id = state::get_token(deps.storage, &user)?
        .ok_or(ContractError::NoSBTExist { user: user.clone() })?;

    let data = state::get_approved_kyc_data(deps.storage, &cooperator, token_id)?.ok_or(
        ContractError::NoApprovementExist {
            cooperator: cooperator.clone(),
            user: user.clone(),
        },
    )?;

    Ok(msg::GetApprovedUserKycDataResponse {
        cooperator,
        user,
        token_id,
        data,
    })
}
