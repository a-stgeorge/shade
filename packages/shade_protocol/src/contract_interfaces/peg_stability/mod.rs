use crate::{
    contract_interfaces::sky::cycles::{ArbPair, Offer},
    utils::{
        asset::Contract,
        storage::plus::{GenericItemStorage, ItemStorage},
        ExecuteCallback,
        InstantiateCallback,
        Query,
    },
};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};
use secret_storage_plus::Item;

#[cw_serde]
pub struct Config {
    pub shd_admin: Contract,
    pub snip20: Contract,
    pub pairs: Vec<ArbPair>,
    pub oracle: Contract,
    pub treasury: Contract,
    pub symbols: Vec<String>,
    pub payback: Decimal,
    pub self_addr: Addr,
    pub dump_contract: Contract,
}

impl ItemStorage for Config {
    const ITEM: Item<'static, Config> = Item::new("item_config");
}

#[cw_serde]
pub struct ViewingKey;

impl GenericItemStorage<String> for ViewingKey {
    const ITEM: Item<'static, String> = Item::new("item_view_key");
}

#[cw_serde]
pub struct InstantiateMsg {
    pub shd_admin: Contract,
    pub snip20: Contract,
    pub oracle: Contract,
    pub treasury: Contract,
    pub payback: Decimal,
    pub viewing_key: String,
    pub dump_contract: Contract,
}

impl InstantiateCallback for InstantiateMsg {
    const BLOCK_SIZE: usize = 256;
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        shd_admin: Option<Contract>,
        snip20: Option<Contract>,
        oracle: Option<Contract>,
        treasury: Option<Contract>,
        symbols: Option<Vec<String>>,
        payback: Option<Decimal>,
        dump_contract: Option<Contract>,
        padding: Option<String>,
    },
    SetPairs {
        pairs: Vec<ArbPair>,
        symbol: Option<String>,
        padding: Option<String>,
    },
    AppendPairs {
        pairs: Vec<ArbPair>,
        symbol: Option<String>,
        padding: Option<String>,
    },
    RemovePair {
        index: Uint128,
        padding: Option<String>,
    },
    Swap {
        padding: Option<String>,
    },
}

impl ExecuteCallback for ExecuteMsg {
    const BLOCK_SIZE: usize = 256;
}

#[cw_serde]
pub enum ExecuteAnswer {
    Init {
        status: bool,
    },
    UpdateConfig {
        config: Config,
        status: bool,
    },
    SetPairs {
        pairs: Vec<ArbPair>,
        status: bool,
    },
    AppendPairs {
        pairs: Vec<ArbPair>,
        status: bool,
    },
    RemovePair {
        pairs: Vec<ArbPair>,
        status: bool,
    },
    Swap {
        profit: Uint128,
        payback: Uint128,
        status: bool,
    },
}

#[cw_serde]
pub enum QueryMsg {
    GetConfig {},
    Balance {},
    GetPairs {},
    Profitable {},
}

impl Query for QueryMsg {
    const BLOCK_SIZE: usize = 256;
}

#[cw_serde]
pub enum QueryAnswer {
    Config { config: Config },
    Balance { snip20_bal: Uint128 },
    GetPairs { pairs: Vec<ArbPair> },
    Profitable { profit: Uint128, payback: Uint128 },
}

#[cw_serde]
pub struct CalculateRes {
    pub profit: Uint128,
    pub payback: Uint128,
    pub index: usize,
    pub config: Config,
    pub offer: Offer,
    pub min_expected: Uint128,
}
