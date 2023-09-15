use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response,
};

use cw2::set_contract_version;

use crate::{
    error::ContractError,
    execute as exec,
    msg::{ExecuteMsg, QueryMsg},
    query as qry, state,
};

const CONTRACT_NAME: &str = "contract.zk.me:zkMeSBT";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: Empty,
) -> Result<Response, ContractError> {
    // Set name and version
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // Init AccessControl Map
    for role in [
        state::DEFAULT_ADMIN_ROLE,
        state::OPERATOR_ROLE,
        state::COOPERATOR_ROLE,
        state::INSPECTOR_ROLE,
    ] {
        state::grant_role(deps.storage, role, info.sender.clone())?;
    }

    // Init Counter
    state::initialize_counter(deps.storage)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;
    match msg {
        GrantRole { role, user } => exec::grant_role(deps, env, info, &role, user),
        RevokeRole { role, user } => exec::revoke_role(deps, env, info, &role, user),

        SetQuestions {
            cooperator,
            questions,
        } => exec::set_questions(deps, env, info, cooperator, questions),

        Attest { to } => exec::attest(deps, env, info, to),
        Burn {
            from: _,
            token_id: _,
        } => unimplemented!(),
        SetKycData {
            token_id,
            key,
            validity,
            data,
            questions,
        } => exec::set_kyc_data(deps, env, info, token_id, key, validity, data, questions),
        SetTokenBaseUri { uri } => exec::set_token_base_uri(deps, env, info, uri),
        Approve {
            cooperator,
            token_id,
            cooperator_key,
        } => exec::approve(deps, env, info, cooperator, token_id, cooperator_key),
        Revoke {
            cooperator,
            token_id,
        } => exec::revoke(deps, env, info, cooperator, token_id),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    use QueryMsg::*;

    match msg {
        IsRole { role, user } => Ok(to_binary(&qry::is_role(deps, env, &role, user)?)?),
        GetQuestions { cooperator } => Ok(to_binary(&qry::get_questions(deps, env, cooperator)?)?),
        GetKycData { token_id } => Ok(to_binary(&qry::get_kyc_data(deps, env, token_id)?)?),
        BalanceOf { owner } => Ok(to_binary(&qry::balance_of(deps, env, owner)?)?),
        TokenIdOf { from } => Ok(to_binary(&qry::token_id_of(deps, env, from)?)?),
        OwnerOf { token_id } => Ok(to_binary(&qry::owner_of(deps, env, token_id)?)?),
        TotalSupply {} | NumTokens {} => Ok(to_binary(&qry::total_supply(deps, env)?)?),
        TokenUri { token_id } => Ok(to_binary(&qry::token_uri(deps, env, token_id)?)?),
        Verify { cooperator, user } => Ok(to_binary(&qry::verify(deps, env, cooperator, user)?)?),
        HasApproved { cooperator, user } => {
            Ok(to_binary(&qry::has_approved(deps, env, cooperator, user)?)?)
        }
        GetUserTokenId { cooperator, user } => Ok(to_binary(&qry::get_user_token_id(
            deps, env, cooperator, user,
        )?)?),
        GetUserData { cooperator, user } => Ok(to_binary(&qry::get_user_data(
            deps, env, cooperator, user,
        )?)?),
        GetApprovedTokenId {
            cooperator,
            start,
            page_size,
        } => Ok(to_binary(&qry::get_approved_token_id(
            deps, env, cooperator, start, page_size,
        )?)?),
        GetApprovedLength { cooperator } => Ok(to_binary(&qry::get_approved_length(
            deps, env, cooperator,
        )?)?),
        GetApprovedUserKycData { cooperator, user } => Ok(to_binary(
            &qry::get_approved_user_kyc_data(deps, env, cooperator, user)?,
        )?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: Empty) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg(test)]
mod tests {
    use crate::{
        contract, msg,
        state::{COOPERATOR_ROLE, DEFAULT_ADMIN_ROLE},
    };
    use cosmwasm_std::Addr;
    use cosmwasm_std::{from_binary, Empty};
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info},
        Timestamp,
    };
    use cw_multi_test::{App, ContractWrapper, Executor};

    #[test]
    fn test_instantiate() {
        let mut app = App::default();

        let code = ContractWrapper::new(contract::execute, contract::instantiate, contract::query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("deployer"),
                &Empty {},
                &[],
                "zkMeSBT",
                None,
            )
            .unwrap();

        let resp: msg::IsRoleResponse = app
            .wrap()
            .query_wasm_smart(
                addr,
                &msg::QueryMsg::IsRole {
                    role: DEFAULT_ADMIN_ROLE.to_string(),
                    user: Addr::unchecked("deployer"),
                },
            )
            .unwrap();

        assert_eq!(
            resp,
            msg::IsRoleResponse {
                role: DEFAULT_ADMIN_ROLE.to_string(),
                user: Addr::unchecked("deployer"),
                result: true
            }
        );
    }

    #[test]
    fn test_integrated() {
        let deployer = Addr::unchecked("deployer");
        let alice = Addr::unchecked("alice");
        let bob = Addr::unchecked("bob");
        let carol = Addr::unchecked("carol");

        // Deploy contract
        let mut app = App::default();

        let code = ContractWrapper::new(contract::execute, contract::instantiate, contract::query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(code_id, deployer.clone(), &Empty {}, &[], "zkMeSBT", None)
            .unwrap();

        // Mint sbt to alice
        app.execute_contract(
            alice.clone(),
            addr.clone(),
            &msg::ExecuteMsg::Attest { to: alice.clone() },
            &[],
        )
        .unwrap();

        // check token id is 1
        let resp: msg::TokenIdOfResponse = app
            .wrap()
            .query_wasm_smart(
                addr.clone(),
                &msg::QueryMsg::TokenIdOf {
                    from: alice.clone(),
                },
            )
            .unwrap();

        assert_eq!(
            resp,
            msg::TokenIdOfResponse {
                from: alice.clone(),
                token_id: 1,
            }
        );

        // set kyc data of token id 1
        let key = "aczxqeGazZPd8RAv5wWeoZuy66Qx7JgrSpnJlcrx7b7IWc0QrhaRoHwN9lCayOIeWAsoi2a0wxIpDEsoIdIrXKqsGcyItRoMJKt3kpsrPrQ=".to_string();
        let validity = Timestamp::from_seconds(1690527065).plus_days(30);
        let data = r#"{"country":"Australia","gender":"male"}"#.to_string();
        let questions = vec![
            "6168752826443568356578851982882135008485".to_string(),
            "7721528705884867793143365084876737116315".to_string(),
        ];
        app.execute_contract(
            deployer.clone(),
            addr.clone(),
            &msg::ExecuteMsg::SetKycData {
                token_id: 1,
                key: key.clone(),
                validity: validity.clone(),
                data: data.clone(),
                questions: questions.clone(),
            },
            &[],
        )
        .unwrap();

        let resp: msg::GetKycDataResponse = app
            .wrap()
            .query_wasm_smart(addr.clone(), &msg::QueryMsg::GetKycData { token_id: 1 })
            .unwrap();

        assert_eq!(
            resp,
            msg::GetKycDataResponse {
                owner: alice.clone(),
                token_id: 1,
                key: key.clone(),
                validity: validity.clone(),
                data: data.clone(),
                questions: questions.clone(),
            }
        );

        // grant bob as cooperator
        app.execute_contract(
            deployer.clone(),
            addr.clone(),
            &msg::ExecuteMsg::GrantRole {
                role: COOPERATOR_ROLE.to_string(),
                user: bob.clone(),
            },
            &[],
        )
        .unwrap();

        let resp: msg::IsRoleResponse = app
            .wrap()
            .query_wasm_smart(
                addr.clone(),
                &msg::QueryMsg::IsRole {
                    role: COOPERATOR_ROLE.to_string(),
                    user: bob.clone(),
                },
            )
            .unwrap();

        assert_eq!(
            resp,
            msg::IsRoleResponse {
                role: COOPERATOR_ROLE.to_string(),
                user: bob.clone(),
                result: true,
            }
        );

        // set bob conf questions
        app.execute_contract(
            deployer.clone(),
            addr.clone(),
            &msg::ExecuteMsg::SetQuestions {
                cooperator: bob.clone(),
                questions: questions.clone(),
            },
            &[],
        )
        .unwrap();

        let resp: msg::GetQuestionsResponse = app
            .wrap()
            .query_wasm_smart(
                addr.clone(),
                &msg::QueryMsg::GetQuestions {
                    cooperator: bob.clone(),
                },
            )
            .unwrap();

        assert_eq!(
            resp,
            msg::GetQuestionsResponse {
                cooperator: bob.clone(),
                questions: questions.clone(),
            }
        );

        // make alice approved to bob
        let cooperator_key = "bczxqeGazZPd8RAv5wWeoZuy66Qx7JgrSpnJlcrx7b7IWc0QrhaRoHwN9lCayOIeWAsoi2a0wxIpDEsoIdIrXKqsGcyItRoMJKt3kpsrPrQ=".to_string();
        app.execute_contract(
            alice.clone(),
            addr.clone(),
            &msg::ExecuteMsg::Approve {
                cooperator: bob.clone(),
                token_id: 1,
                cooperator_key: cooperator_key.clone(),
            },
            &[],
        )
        .unwrap();

        let resp: msg::HasApprovedResponse = app
            .wrap()
            .query_wasm_smart(
                addr.clone(),
                &msg::QueryMsg::HasApproved {
                    cooperator: bob.clone(),
                    user: alice.clone(),
                },
            )
            .unwrap();

        assert_eq!(
            resp,
            msg::HasApprovedResponse {
                cooperator: bob.clone(),
                user: alice.clone(),
                has_approved: true,
            }
        );

        // bob get approved kyc
        let resp: msg::GetApprovedLengthResponse = app
            .wrap()
            .query_wasm_smart(
                addr.clone(),
                &msg::QueryMsg::GetApprovedLength {
                    cooperator: bob.clone(),
                },
            )
            .unwrap();

        assert_eq!(resp.approved_length, 1);

        let resp: msg::GetApprovedTokenIdResponse = app
            .wrap()
            .query_wasm_smart(
                addr.clone(),
                &msg::QueryMsg::GetApprovedTokenId {
                    cooperator: bob.clone(),
                    start: 0,
                    page_size: 10,
                },
            )
            .unwrap();

        assert_eq!(
            resp,
            msg::GetApprovedTokenIdResponse {
                cooperator: bob.clone(),
                start: 0,
                page_size: 1,
                token_id_list: vec![1],
            },
        );

        let resp: msg::GetApprovedUserKycDataResponse = app
            .wrap()
            .query_wasm_smart(
                addr.clone(),
                &msg::QueryMsg::GetApprovedUserKycData {
                    cooperator: bob.clone(),
                    user: alice.clone(),
                },
            )
            .unwrap();

        assert_eq!(
            resp,
            msg::GetApprovedUserKycDataResponse {
                cooperator: bob.clone(),
                user: alice.clone(),
                token_id: 1,
                data: crate::state::KycData {
                    key: cooperator_key.clone(),
                    validity: validity.clone(),
                    data: data.clone(),
                    questions: questions.clone()
                },
            },
        );

        // revoke approve
        app.execute_contract(
            alice.clone(),
            addr.clone(),
            &&msg::ExecuteMsg::Revoke {
                cooperator: bob.clone(),
                token_id: 1,
            },
            &[],
        )
        .unwrap();
    }
}
