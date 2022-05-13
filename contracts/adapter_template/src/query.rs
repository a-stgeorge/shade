use cosmwasm_std::{
    Api, BalanceResponse, BankQuery, Delegation, DistQuery, Extern, FullDelegation, HumanAddr,
    Querier, RewardsResponse, StdError, StdResult, Storage, Uint128,
};

use shade_protocol::{adapter, adapter_template::QueryAnswer};
use secret_toolkit::snip20::balance_query;

use crate::state::{
    config_r, self_address_r, unbonding_r,
    asset_r, viewing_key_r,
};

pub fn config<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<QueryAnswer> {
    Ok(QueryAnswer::Config {
        config: config_r(&deps.storage).load()?,
    })
}

pub fn balance<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    asset: HumanAddr,
) -> StdResult<adapter::QueryAnswer> {

    match asset_r(&deps.storage).may_load(asset.as_str().as_bytes())? {
        Some(a) => {
            let mut balance = balance_query(
                &deps.querier,
                self_address_r(&deps.storage).load()?,
                viewing_key_r(&deps.storage).load()?,
                1,
                a.contract.code_hash.clone(),
                a.contract.address.clone(),
            )?.amount;

            Ok(adapter::QueryAnswer::Balance {
                amount: balance
            })
        },
        None => {
            Err(StdError::generic_err("Unrecognized Asset"))
        }
    }
}

pub fn claimable<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    asset: HumanAddr,
) -> StdResult<adapter::QueryAnswer> {

    match asset_r(&deps.storage).may_load(asset.as_str().as_bytes())? {
        Some(a) => {
            let mut claimable = balance_query(
                &deps.querier,
                self_address_r(&deps.storage).load()?,
                viewing_key_r(&deps.storage).load()?,
                1,
                a.contract.code_hash.clone(),
                a.contract.address.clone(),
            )?.amount;

            let unbonding = unbonding_r(&deps.storage).load(&a.contract.address.as_str().as_bytes())?;

            if unbonding <= claimable {
                claimable = unbonding;
            }

            Ok(adapter::QueryAnswer::Claimable {
                amount: claimable 
            })
        },
        None => {
            Err(StdError::generic_err("Unrecognized Asset"))
        }
    }
}

pub fn unbonding<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    asset: HumanAddr,
) -> StdResult<adapter::QueryAnswer> {

    match asset_r(&deps.storage).may_load(asset.as_str().as_bytes())? {
        Some(a) => {
            let unbonding = unbonding_r(&deps.storage).load(&a.contract.address.as_str().as_bytes())?;

            Ok(adapter::QueryAnswer::Unbonding {
                amount: unbonding,
            })
        },
        None => {
            Err(StdError::generic_err("Unrecognized Asset"))
        }
    }
}

pub fn unbondable<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    asset: HumanAddr,
) -> StdResult<adapter::QueryAnswer> {

    match asset_r(&deps.storage).may_load(asset.as_str().as_bytes())? {
        Some(a) => {
            let mut unbondable = balance_query(
                &deps.querier,
                self_address_r(&deps.storage).load()?,
                viewing_key_r(&deps.storage).load()?,
                1,
                a.contract.code_hash.clone(),
                a.contract.address.clone(),
            )?.amount;

            Ok(adapter::QueryAnswer::Unbondable {
                amount: unbondable 
            })
        },
        None => {
            Err(StdError::generic_err("Unrecognized Asset"))
        }
    }
}
