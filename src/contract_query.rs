use crate::msg::QueryMsg;
use cosmwasm_std::{to_binary, Binary, Deps, Env, StdResult};

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        GetAdmins {} => to_binary(&query::get_admins(deps)?),
        GetMembers {} => to_binary(&query::get_members(deps)?),
    }
}

mod query {
    use super::*;
    use crate::{
        msg::{GetAdminsResp, GetMembersResp},
        state::{ADMINS, MEMBERS},
    };

    pub fn get_admins(deps: Deps) -> StdResult<GetAdminsResp> {
        let admins = ADMINS.load(deps.storage)?;
        let resp = GetAdminsResp { admins };
        Ok(resp)
    }

    pub fn get_members(deps: Deps) -> StdResult<GetMembersResp> {
        let members = match MEMBERS.may_load(deps.storage)? {
            Some(members) => members,
            _ => vec![],
        };
        let resp = GetMembersResp { members };
        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        constants::{ADDR_OWNER, ADDR_USER_1, CONTRACT_1},
        contract_execute::execute,
        contract_instantiate::instantiate,
        msg::{ExecuteMsg, GetAdminsResp, GetMembersResp, InstantiateMsg, MemberInfo},
        state::Member,
    };
    use cosmwasm_std::Addr;
    use cw_multi_test::{App, ContractWrapper, Executor};

    #[test]
    fn test_get_admins() {
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

        let resp: GetAdminsResp = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::GetAdmins {})
            .unwrap();

        assert_eq!(
            GetAdminsResp {
                admins: vec![Addr::unchecked(ADDR_OWNER.to_owned())],
            },
            resp
        )
    }

    #[test]
    fn test_get_members() {
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

        let resp: GetMembersResp = app
            .wrap()
            .query_wasm_smart(addr.clone(), &QueryMsg::GetMembers {})
            .unwrap();
        assert_eq!(GetMembersResp { members: vec![] }, resp);

        let new_member = MemberInfo {
            username: "username".to_owned(),
            intro: "Hello there!".to_owned(),
        };

        app.execute_contract(
            Addr::unchecked(ADDR_USER_1.to_owned()),
            addr.clone(),
            &ExecuteMsg::Register {
                member: MemberInfo {
                    username: new_member.username.clone(),
                    intro: new_member.intro.clone(),
                },
            },
            &[],
        )
        .unwrap();

        let resp: GetMembersResp = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::GetMembers {})
            .unwrap();
        assert_eq!(
            GetMembersResp {
                members: vec![Member {
                    address: Addr::unchecked(ADDR_USER_1.to_owned()),
                    username: new_member.username,
                    intro: new_member.intro,
                }]
            },
            resp
        );
    }
}
