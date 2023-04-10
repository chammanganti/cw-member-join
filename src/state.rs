use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[cw_serde]
pub struct Member {
    pub address: Addr,
    pub username: String,
    pub intro: String,
}

pub const ADMINS: Item<Vec<Addr>> = Item::new("admins");
pub const MEMBERS: Item<Vec<Member>> = Item::new("members");
