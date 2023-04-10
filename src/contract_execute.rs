use crate::{error::ContractError, msg::ExecuteMsg};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        Register { member } => execute::register(deps, info, member),
        Leave {} => execute::leave(deps, info),
    }
}

mod execute {
    use cosmwasm_std::StdResult;

    use super::*;
    use crate::{
        constants::{ATTR_ACTION, ATTR_ACTION_LEAVE, ATTR_ACTION_REGISTER},
        msg::MemberInfo,
        state::{Member, MEMBERS},
    };

    pub fn register(
        deps: DepsMut,
        info: MessageInfo,
        member: MemberInfo,
    ) -> Result<Response, ContractError> {
        let mut members: Vec<Member> = match MEMBERS.may_load(deps.storage)? {
            Some(members) => members,
            _ => vec![],
        };
        for m in members.iter() {
            if m.username.clone() == member.username.clone() {
                return Err(ContractError::MemberAlreadyExists {
                    username: m.username.clone(),
                });
            }
        }

        validate_username(member.username.clone())?;
        validate_intro(member.intro.clone())?;

        let new_member = Member {
            address: info.sender,
            username: member.username,
            intro: member.intro,
        };
        members.push(new_member);

        MEMBERS.save(deps.storage, &members)?;

        Ok(Response::new().add_attribute(ATTR_ACTION, ATTR_ACTION_REGISTER))
    }

    pub fn leave(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
        MEMBERS.update(deps.storage, move |members| -> StdResult<Vec<_>> {
            let members = members
                .into_iter()
                .filter(|member| member.address != info.sender)
                .collect();
            Ok(members)
        })?;

        Ok(Response::new().add_attribute(ATTR_ACTION, ATTR_ACTION_LEAVE))
    }

    // Validates the member's username
    fn validate_username(username: String) -> Result<Response, ContractError> {
        if username.len() < 4 {
            return Err(ContractError::InvalidMemberUsernameLen { username });
        }

        Ok(Response::new())
    }

    // Validates the member's intro
    fn validate_intro(intro: String) -> Result<Response, ContractError> {
        if intro.len() < 12 {
            return Err(ContractError::InvalidMemberIntroLen);
        }

        Ok(Response::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        constants::{
            ADDR_OWNER, ADDR_USER_1, ATTR_ACTION, ATTR_ACTION_LEAVE, ATTR_ACTION_REGISTER,
            CONTRACT_1,
        },
        contract_instantiate::instantiate,
        contract_query::query,
        msg::{GetMembersResp, InstantiateMsg, MemberInfo, QueryMsg},
    };
    use cosmwasm_std::Addr;
    use cw_multi_test::{App, ContractWrapper, Executor};

    #[test]
    fn test_register() {
        let mut app = App::default();
        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked(ADDR_OWNER.to_owned()),
                &InstantiateMsg {
                    admins: vec![ADDR_OWNER.to_owned()],
                },
                &[],
                CONTRACT_1,
                None,
            )
            .unwrap();

        let resp = app
            .execute_contract(
                Addr::unchecked(ADDR_USER_1.to_owned()),
                addr.clone(),
                &ExecuteMsg::Register {
                    member: MemberInfo {
                        username: "username".to_owned(),
                        intro: "Hello there!".to_owned(),
                    },
                },
                &[],
            )
            .unwrap();
        let wasm = resp.events.iter().find(|event| event.ty == "wasm").unwrap();
        assert_eq!(
            ATTR_ACTION_REGISTER,
            wasm.attributes
                .iter()
                .find(|attr| attr.key == ATTR_ACTION)
                .unwrap()
                .value
        );

        // Error case #1: member already exists
        let err = app
            .execute_contract(
                Addr::unchecked(ADDR_USER_1.to_owned()),
                addr.clone(),
                &ExecuteMsg::Register {
                    member: MemberInfo {
                        username: "username".to_owned(),
                        intro: "Hello, I am username!".to_owned(),
                    },
                },
                &[],
            )
            .unwrap_err();
        assert_eq!(
            ContractError::MemberAlreadyExists {
                username: "username".to_owned()
            },
            err.downcast().unwrap()
        );

        // Error case #2: member's username is less than 4 chars long
        let err = app
            .execute_contract(
                Addr::unchecked(ADDR_USER_1.to_owned()),
                addr.clone(),
                &ExecuteMsg::Register {
                    member: MemberInfo {
                        username: "usr".to_owned(),
                        intro: "Hi, my username is invalid.".to_owned(),
                    },
                },
                &[],
            )
            .unwrap_err();
        assert_eq!(
            ContractError::InvalidMemberUsernameLen {
                username: "usr".to_owned()
            },
            err.downcast().unwrap()
        );

        // Error case #3: member's intro is less than 12 chars long
        let err = app
            .execute_contract(
                Addr::unchecked(ADDR_USER_1.to_owned()),
                addr.clone(),
                &ExecuteMsg::Register {
                    member: MemberInfo {
                        username: "username2".to_owned(),
                        intro: "Hello!".to_owned(),
                    },
                },
                &[],
            )
            .unwrap_err();
        assert_eq!(
            ContractError::InvalidMemberIntroLen,
            err.downcast().unwrap()
        );
    }

    #[test]
    fn test_leave() {
        let mut app = App::default();
        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked(ADDR_OWNER.to_owned()),
                &InstantiateMsg {
                    admins: vec![ADDR_OWNER.to_owned()],
                },
                &[],
                CONTRACT_1,
                None,
            )
            .unwrap();

        app.execute_contract(
            Addr::unchecked(ADDR_USER_1.to_owned()),
            addr.clone(),
            &ExecuteMsg::Register {
                member: MemberInfo {
                    username: "username".to_owned(),
                    intro: "Hello there!".to_owned(),
                },
            },
            &[],
        )
        .unwrap();

        let resp = app
            .execute_contract(
                Addr::unchecked(ADDR_USER_1.to_owned()),
                addr.clone(),
                &ExecuteMsg::Leave {},
                &[],
            )
            .unwrap();
        let wasm = resp.events.iter().find(|event| event.ty == "wasm").unwrap();
        assert_eq!(
            ATTR_ACTION_LEAVE,
            wasm.attributes
                .iter()
                .find(|attr| attr.key == ATTR_ACTION)
                .unwrap()
                .value
        );

        let resp: GetMembersResp = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::GetMembers {})
            .unwrap();
        assert_eq!(0, resp.members.len());
    }
}
