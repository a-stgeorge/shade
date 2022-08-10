use crate::{
    c_std::{Addr, Binary, Uint128},
    utils,
    utils::{
        asset::RawContract,
        callback::{ExecuteCallback, InstantiateCallback, Query},
        generic_response::ResponseStatus,
        storage::plus::ItemStorage,
    },
    Contract,
};
use cosmwasm_schema::cw_serde;
use secret_storage_plus::Item;

#[cw_serde]
pub struct Config {
    pub migrated: Option<Contract>,
    pub auth: Contract,
    pub admin: Contract,
}

impl ItemStorage for Config {
    const ITEM: Item<'static, Self> = Item::new("config-");
}

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: RawContract,
}

impl InstantiateCallback for InstantiateMsg {
    const BLOCK_SIZE: usize = 256;
}

#[cw_serde]
pub enum ExecuteMsg {
    // Admin

    // User
    Receive {
        sender: Addr,
        from: Addr,
        amount: Uint128,
        msg: Option<Binary>,
        memo: Option<String>,
        padding: Option<String>,
    },
    RequestUnbond {
        profile: String,
    },

    // User Migration
    MigrateData {},
    ReceiveMigration {},
}

#[cw_serde]
pub enum ExecuteAnswer {
    Receive { status: ResponseStatus },
}

impl ExecuteCallback for ExecuteMsg {
    const BLOCK_SIZE: usize = 256;
}

#[cw_serde]
pub enum QueryMsg {}

impl Query for QueryMsg {
    const BLOCK_SIZE: usize = 256;
}

#[cw_serde]
pub enum QueryAnswer {}
