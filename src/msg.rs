use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

use crate::state::Member;

#[cw_serde]
pub struct InstantiateMsg {
    pub admins: Vec<String>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetAdminsResp)]
    GetAdmins {},
    #[returns(GetMembersResp)]
    GetMembers {},
}

#[cw_serde]
pub struct GetAdminsResp {
    pub admins: Vec<Addr>,
}

#[cw_serde]
pub struct GetMembersResp {
    pub members: Vec<Member>,
}

#[cw_serde]
pub enum ExecuteMsg {
    Register { member: MemberInfo },
    Leave {},
}

#[cw_serde]
pub struct MemberInfo {
    pub username: String,
    pub intro: String,
}
