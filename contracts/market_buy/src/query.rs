use cosmwasm_std::{
    Api, BalanceResponse, BankQuery, Delegation, DistQuery, Extern, FullDelegation, HumanAddr,
    Querier, RewardsResponse, StdError, StdResult, Storage, Uint128,
};

use shade_protocol::{adapter, market_buy::QueryAnswer};

use crate::state::{config_r, self_address_r, unbonding_r};

pub fn config<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<QueryAnswer> {
    Ok(QueryAnswer::Config {
        config: config_r(&deps.storage).load()?,
    })
}

pub fn balance<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    asset: HumanAddr,
) -> StdResult<adapter::QueryAnswer> {
    let config = config_r(&deps.storage).load()?;

    let full_asset = match asset_r(&deps.storage).load(env.message.sender.as_str().as_bytes())? {
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

    let full_asset = match asset_r(&deps.storage).load(env.message.sender.as_str().as_bytes())? {
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

    let full_asset = match asset_r(&deps.storage).load(env.message.sender.as_str().as_bytes())? {
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

    let full_asset = match asset_r(&deps.storage).load(env.message.sender.as_str().as_bytes())? {
        Some(a) => a,
        None => {
            return Err(StdError::generic_err("Unrecognized Asset"));
        }
    };


    Ok(adapter::QueryAnswer::Unbondable {
        amount: Uint128::zero(),
    })
}
