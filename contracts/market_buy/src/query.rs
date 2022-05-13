use cosmwasm_std::{
    Api, BalanceResponse, BankQuery, Delegation, DistQuery, Extern, FullDelegation, HumanAddr,
    Querier, RewardsResponse, StdError, StdResult, Storage, Uint128,
};

use shade_protocol::{
    adapter, 
    market_buy::QueryAnswer,
};

use crate::state::{
    config_r, self_address_r, unbonding_r,
    asset_r, viewing_key_r,
};

use secret_toolkit::snip20::balance_query;

pub fn config<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<QueryAnswer> {
    Ok(QueryAnswer::Config {
        config: config_r(&deps.storage).load()?,
    })
}

pub fn expected<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    offer: HumanAddr,
    amount: Uint128,
    desired: HumanAddr,
) -> StdResult<QueryAnswer> {

    Ok(QueryAnswer::Expected {
        amount: Uint128::zero(),
        price_impact: Uint128::zero(),
    })
}

pub fn balance<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    asset: HumanAddr,
) -> StdResult<adapter::QueryAnswer> {
    let config = config_r(&deps.storage).load()?;

    let full_asset = match asset_r(&deps.storage).may_load(asset.as_str().as_bytes())? {
        Some(a) => a,
        None => {
            return Err(StdError::generic_err("Unrecognized Asset"));
        }
    };

    Ok(adapter::QueryAnswer::Balance {
        amount: Uint128::zero(),
    })
}

pub fn claimable<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    asset: HumanAddr,
) -> StdResult<adapter::QueryAnswer> {

    let config = config_r(&deps.storage).load()?;

    let full_asset = match asset_r(&deps.storage).may_load(asset.as_str().as_bytes())? {
        Some(a) => a,
        None => {
            return Err(StdError::generic_err("Unrecognized Asset"));
        }
    };

    Ok(adapter::QueryAnswer::Claimable {
        amount: Uint128::zero(),
    })
}

pub fn unbonding<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    asset: HumanAddr,
) -> StdResult<adapter::QueryAnswer> {

    let config = config_r(&deps.storage).load()?;

    let full_asset = match asset_r(&deps.storage).may_load(asset.as_str().as_bytes())? {
        Some(a) => a,
        None => {
            return Err(StdError::generic_err("Unrecognized Asset"));
        }
    };

    Ok(adapter::QueryAnswer::Unbonding {
        amount: Uint128::zero(),
    })
}

pub fn unbondable<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    asset: HumanAddr,
) -> StdResult<adapter::QueryAnswer> {

    let config = config_r(&deps.storage).load()?;

    let full_asset = match asset_r(&deps.storage).may_load(asset.as_str().as_bytes())? {
        Some(a) => a,
        None => {
            return Err(StdError::generic_err("Unrecognized Asset"));
        }
    };


    Ok(adapter::QueryAnswer::Unbondable {
        amount: Uint128::zero(),
    })
}

pub fn reserves<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    asset: HumanAddr,
) -> StdResult<adapter::QueryAnswer> {

    match asset_r(&deps.storage).may_load(asset.as_str().as_bytes())? {
        Some(a) => {
            let reserves = balance_query(
                &deps.querier,
                self_address_r(&deps.storage).load()?,
                viewing_key_r(&deps.storage).load()?,
                1,
                a.contract.code_hash.clone(),
                a.contract.address.clone(),
            )?.amount;

            Ok(adapter::QueryAnswer::Reserves {
                amount: reserves,
            })
        },
        None => {
            Err(StdError::generic_err("Unrecognized Asset"))
        }
    }
}
