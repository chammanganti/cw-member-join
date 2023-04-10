use crate::{
    constants::{ATTR_ACTION, ATTR_ACTION_INSTANTIATE, ERR_REQUIRED_ADMIN_ON_INIT},
    error::ContractError,
    msg::InstantiateMsg,
    state::ADMINS,
};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

const CONTRACT_NAME: &str = "crates.io:cw-member-announcement";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    if msg.admins.is_empty() {
        return Err(ContractError::GenericError {
            err: ERR_REQUIRED_ADMIN_ON_INIT.to_owned(),
        });
    }

    let admins: StdResult<Vec<_>> = msg
        .admins
        .into_iter()
        .map(|admin| deps.api.addr_validate(&admin))
        .collect();
    ADMINS.save(deps.storage, &admins?)?;

    Ok(Response::new().add_attribute(ATTR_ACTION, ATTR_ACTION_INSTANTIATE))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{constants::ADDR_OWNER, msg::InstantiateMsg};
    use cosmwasm_std::{
        attr,
        testing::{mock_dependencies, mock_env, mock_info},
    };

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR_OWNER, &[]);
        let msg = InstantiateMsg {
            admins: vec![ADDR_OWNER.to_owned()],
        };

        let resp = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(
            vec![attr(ATTR_ACTION, ATTR_ACTION_INSTANTIATE)],
            resp.attributes
        );

        // Error case #1: at least 1 admin during initialization
        let msg = InstantiateMsg { admins: vec![] };
        let resp = instantiate(deps.as_mut(), env, info, msg).unwrap_err();
        assert_eq!(
            ContractError::GenericError {
                err: ERR_REQUIRED_ADMIN_ON_INIT.to_owned()
            },
            resp
        )
    }
}
