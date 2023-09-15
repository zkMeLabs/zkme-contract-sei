use std::ops::Not;

use crate::ContractError;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Storage, Timestamp};
use cw_storage_plus::{Item, Map};

pub type TokenId = u64;

pub const DEFAULT_ADMIN_ROLE: &'static str = "default_admin_role";
pub const OPERATOR_ROLE: &'static str = "zkme_operator";
pub const COOPERATOR_ROLE: &'static str = "zkme_cooperator";
pub const INSPECTOR_ROLE: &'static str = "zkme_inspector";

#[cw_serde]
pub struct RoleData {
    pub members: Vec<Addr>,
}

pub const ROLES: Map<&str, RoleData> = Map::new("roles");

pub fn existing_role(storage: &dyn Storage, role: &str) -> bool {
    ROLES.has(storage, role)
}

pub fn has_role(storage: &dyn Storage, role: &str, account: &Addr) -> Result<bool, ContractError> {
    if existing_role(storage, role) {
        let roles = ROLES.load(storage, role)?;
        return Ok(roles.members.contains(account));
    }
    Ok(false)
}

pub fn grant_role(
    storage: &mut dyn Storage,
    role: &str,
    account: Addr,
) -> Result<(), ContractError> {
    if existing_role(storage, role) {
        let _ = ROLES.update(
            storage,
            role,
            move |role_data| -> Result<_, ContractError> {
                if let Some(mut data) = role_data {
                    if data.members.contains(&account) {
                        Err(ContractError::AlreadyGrantRole {
                            account,
                            role: role.to_string(),
                        })
                    } else {
                        data.members.push(account);
                        Ok(data)
                    }
                } else {
                    Err(ContractError::UpdatingErrorSlot)
                }
            },
        )?;
        Ok(())
    } else {
        let new_role_data = RoleData {
            members: vec![account],
        };
        Ok(ROLES.save(storage, role, &new_role_data)?)
    }
}

pub fn revoke_role(
    storage: &mut dyn Storage,
    role: &str,
    account: &Addr,
) -> Result<(), ContractError> {
    if has_role(storage, role, &account)? {
        ROLES.update(
            storage,
            role,
            move |role_data| -> Result<_, ContractError> {
                if let Some(mut data) = role_data {
                    data.members = data
                        .members
                        .into_iter()
                        .filter(|addr| addr != &account)
                        .collect();
                    Ok(data)
                } else {
                    Err(ContractError::UpdatingErrorSlot)
                }
            },
        )?;
    }
    Ok(())
}

pub const CONF_QUESTIONS: Map<&Addr, Vec<String>> = Map::new("conf_questions");

pub fn has_questions(storage: &dyn Storage, cooperator: &Addr) -> bool {
    CONF_QUESTIONS.has(storage, cooperator)
}

pub fn set_questions(
    storage: &mut dyn Storage,
    cooperator: &Addr,
    questions: Vec<String>,
) -> Result<(), ContractError> {
    if has_questions(storage, cooperator) {
        let _ = CONF_QUESTIONS.update(
            storage,
            cooperator,
            move |_old_questions| -> Result<_, ContractError> { Ok(questions) },
        );
        Ok(())
    } else {
        Ok(CONF_QUESTIONS.save(storage, cooperator, &questions)?)
    }
}

pub fn get_questions(
    storage: &dyn Storage,
    cooperator: &Addr,
) -> Result<Option<Vec<String>>, ContractError> {
    if has_questions(storage, cooperator) {
        Ok(Some(CONF_QUESTIONS.load(storage, cooperator)?))
    } else {
        Ok(None)
    }
}

pub const COUNTER: Item<TokenId> = Item::new("token_id");

pub fn initialize_counter(storage: &mut dyn Storage) -> Result<(), ContractError> {
    Ok(COUNTER.save(storage, &1)?)
}

pub fn increase_counter(storage: &mut dyn Storage) -> Result<(), ContractError> {
    let _ = COUNTER.update(storage, |counter| -> Result<_, ContractError> {
        Ok(counter + 1)
    })?;
    Ok(())
}

pub fn get_token_id(storage: &dyn Storage) -> Result<TokenId, ContractError> {
    Ok(COUNTER.load(storage)?)
}

pub const OWNER_MAP: Map<&TokenId, Addr> = Map::new("owner_map");

pub fn has_owner(storage: &dyn Storage, k: &TokenId) -> Result<bool, ContractError> {
    Ok(OWNER_MAP.has(storage, k))
}

pub fn save_owner(storage: &mut dyn Storage, k: &TokenId, v: &Addr) -> Result<(), ContractError> {
    if has_owner(storage, k)?.not() {
        OWNER_MAP.save(storage, k, v)?;
    }
    Ok(())
}

pub fn get_owner(storage: &dyn Storage, k: &TokenId) -> Result<Option<Addr>, ContractError> {
    Ok(OWNER_MAP.may_load(storage, k)?)
}

pub const TOKEN_MAP: Map<&Addr, TokenId> = Map::new("token_map");

pub fn has_token(storage: &dyn Storage, k: &Addr) -> bool {
    TOKEN_MAP.has(storage, k)
}

pub fn save_token(storage: &mut dyn Storage, k: &Addr, v: &TokenId) -> Result<(), ContractError> {
    Ok(TOKEN_MAP.save(storage, k, v)?)
}

pub fn get_token(storage: &dyn Storage, k: &Addr) -> Result<Option<TokenId>, ContractError> {
    Ok(TOKEN_MAP.may_load(storage, k)?)
}

pub const TOKEN_NAME: &'static str = "zkMe Identity Soulbound Token";
pub const TOKEN_SYMBOL: &'static str = "ZIS";

pub const TOKEN_BASE_URI: Item<String> = Item::new("token_base_uri");

pub fn get_token_base_uri(storage: &dyn Storage) -> Result<String, ContractError> {
    if TOKEN_BASE_URI.exists(storage) {
        Ok(TOKEN_BASE_URI.load(storage)?)
    } else {
        Ok("".to_string())
    }
}

pub fn set_token_base_uri(storage: &mut dyn Storage, new_uri: String) -> Result<(), ContractError> {
    Ok(TOKEN_BASE_URI.save(storage, &new_uri)?)
}

#[cw_serde]
pub struct KycData {
    pub key: String,
    pub validity: Timestamp,
    pub data: String,
    pub questions: Vec<String>,
}

pub const KYC_MAP: Map<&TokenId, KycData> = Map::new("kyc_map");

pub fn has_kyc(storage: &dyn Storage, k: &TokenId) -> bool {
    KYC_MAP.has(storage, k)
}

pub fn save_kyc(storage: &mut dyn Storage, k: &TokenId, v: KycData) -> Result<(), ContractError> {
    if has_kyc(storage, k) {
        let _ = KYC_MAP.update(storage, k, move |_kyc| -> Result<_, ContractError> {
            Ok(v)
        })?;
        Ok(())
    } else {
        Ok(KYC_MAP.save(storage, &k, &v)?)
    }
}

pub fn get_kyc(storage: &dyn Storage, k: &TokenId) -> Result<Option<KycData>, ContractError> {
    Ok(KYC_MAP.may_load(storage, k)?)
}

pub const PU_MAP: Map<(&Addr, &Addr), TokenId> = Map::new("pu_map");

pub fn has_approve(storage: &dyn Storage, cooperator: &Addr, user: &Addr) -> bool {
    PU_MAP.has(storage, (cooperator, user))
}

pub fn save_pu(
    storage: &mut dyn Storage,
    cooperator: &Addr,
    user: &Addr,
    v: TokenId,
) -> Result<(), ContractError> {
    if has_approve(storage, cooperator, user) {
        Ok(())
    } else {
        Ok(PU_MAP.save(storage, (cooperator, user), &v)?)
    }
}

pub fn remove_pu(storage: &mut dyn Storage, cooperator: &Addr, user: &Addr) {
    PU_MAP.remove(storage, (cooperator, user))
}

pub const APPROVE_MAP: Map<&Addr, Vec<TokenId>> = Map::new("approve_map");

pub fn get_approved(
    storage: &dyn Storage,
    cooperator: &Addr,
) -> Result<Vec<TokenId>, ContractError> {
    if APPROVE_MAP.has(storage, cooperator) {
        Ok(APPROVE_MAP.load(storage, cooperator)?)
    } else {
        Ok(vec![])
    }
}

pub fn save_approved(
    storage: &mut dyn Storage,
    cooperator: &Addr,
    v: TokenId,
) -> Result<(), ContractError> {
    if APPROVE_MAP.has(storage, cooperator) {
        let _ = APPROVE_MAP.update(
            storage,
            cooperator,
            move |token_list| -> Result<_, ContractError> {
                let mut token_list = token_list.unwrap();
                token_list.push(v);
                Ok(token_list)
            },
        )?;
        Ok(())
    } else {
        Ok(APPROVE_MAP.save(storage, cooperator, &vec![v])?)
    }
}

pub fn remove_approved(
    storage: &mut dyn Storage,
    cooperator: &Addr,
    v: TokenId,
) -> Result<(), ContractError> {
    if APPROVE_MAP.has(storage, cooperator) {
        APPROVE_MAP.update(
            storage,
            cooperator,
            move |token_list| -> Result<_, ContractError> {
                let token_list = token_list.unwrap();
                let new_token_list = token_list.into_iter().filter(|id| id != &v).collect();
                Ok(new_token_list)
            },
        )?;
    }
    Ok(())
}

pub const APPROVED_KYC_MAP: Map<(&Addr, TokenId), KycData> = Map::new("approved_kyc_map");

pub fn has_approved_kyc_data(storage: &dyn Storage, cooperator: &Addr, token_id: TokenId) -> bool {
    APPROVED_KYC_MAP.has(storage, (cooperator, token_id))
}

pub fn save_approved_kyc_data(
    storage: &mut dyn Storage,
    cooperator: &Addr,
    token_id: TokenId,
    cooperator_key: String,
) -> Result<(), ContractError> {
    let KycData {
        key: _,
        validity,
        data,
        questions,
    } = get_kyc(storage, &token_id)?.ok_or(ContractError::InvalidTokenId { token_id })?;

    if has_approved_kyc_data(storage, cooperator, token_id) {
        let _ = APPROVED_KYC_MAP.update(
            storage,
            (cooperator, token_id),
            move |kyc| -> Result<_, ContractError> {
                let mut kyc = kyc.unwrap();
                kyc.key = cooperator_key;
                kyc.validity = validity;
                kyc.data = data;
                kyc.questions = questions;

                Ok(kyc)
            },
        )?;
        Ok(())
    } else {
        let new_kyc = KycData {
            key: cooperator_key,
            validity,
            data,
            questions,
        };
        Ok(APPROVED_KYC_MAP.save(storage, (cooperator, token_id), &new_kyc)?)
    }
}

pub fn get_approved_kyc_data(
    storage: &dyn Storage,
    cooperator: &Addr,
    token_id: TokenId,
) -> Result<Option<KycData>, ContractError> {
    Ok(APPROVED_KYC_MAP.may_load(storage, (cooperator, token_id))?)
}
