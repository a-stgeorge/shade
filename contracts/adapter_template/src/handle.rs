use cosmwasm_std::{
    debug_print, to_binary, Api, BalanceResponse, BankQuery, Binary, Coin, CosmosMsg, Env, Extern,
    HandleResponse, HumanAddr, Querier, StakingMsg, StdError, StdResult, Storage, Uint128,
};

use secret_toolkit::snip20::{
    deposit_msg, redeem_msg, send_msg, balance_query,
    register_receive_msg, set_viewing_key_msg,
};

use shade_protocol::{
    adapter_template::{HandleAnswer, Config},
    treasury::Flag,
    snip20,
    adapter,
    utils::{
        generic_response::ResponseStatus,
        asset::{
            Contract,
            scrt_balance,
        },
        wrap::{wrap_and_send, unwrap},
    },
};

use crate::{
    query,
    state::{
        config_r, config_w,
        self_address_r,
        unbonding_w, unbonding_r,
        viewing_key_r,
        asset_r, asset_w,
        asset_list_r, asset_list_w,
    },
};

pub fn receive<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    _sender: HumanAddr,
    _from: HumanAddr,
    amount: Uint128,
    _msg: Option<Binary>,
) -> StdResult<HandleResponse> {
    debug_print!("Received {}", amount);

    let config = config_r(&deps.storage).load()?;

    Ok(HandleResponse {
        messages: vec![ ],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::Receive {
            status: ResponseStatus::Success,
        })?),
    })
}

pub fn try_update_config<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    config: Config,
) -> StdResult<HandleResponse> {
    let cur_config = config_r(&deps.storage).load()?;

    if !config.admins.contains(&env.message.sender) {
        return Err(StdError::unauthorized());
    }

    // Save new info
    config_w(&mut deps.storage).save(&config)?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::UpdateConfig {
            status: ResponseStatus::Success,
        })?),
    })
}

pub fn register_asset<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    contract: Contract,
) -> StdResult<HandleResponse> {

    let config = config_r(&deps.storage).load()?;

    if !config.admins.contains(&env.message.sender) {
        return Err(StdError::unauthorized());
    }

    asset_list_w(&mut deps.storage).update(|mut list| {
        if list.contains(&contract.address.clone()) {
            return Err(StdError::generic_err("Asset already registered"));
        }
        list.push(contract.address.clone());
        Ok(list)
    })?;

    asset_w(&mut deps.storage).save(
        contract.address.to_string().as_bytes(),
        &snip20::fetch_snip20(&contract, &deps.querier)?,
    )?;

    unbonding_w(&mut deps.storage).save(&contract.address.as_str().as_bytes(), &Uint128::zero())?;

    Ok(HandleResponse {
        messages: vec![
            // Register contract in asset
            register_receive_msg(
                env.contract_code_hash.clone(),
                None,
                256,
                contract.code_hash.clone(),
                contract.address.clone(),
            )?,
            // Set viewing key
            set_viewing_key_msg(
                viewing_key_r(&deps.storage).load()?,
                None,
                256,
                contract.code_hash.clone(),
                contract.address.clone(),
            )?,
        ],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::RegisterAsset {
            status: ResponseStatus::Success,
        })?),
    })
}

/* Claim rewards and restake, hold enough for pending unbondings
 * Send available unbonded funds to treasury
 */
pub fn update<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    asset: HumanAddr,
) -> StdResult<HandleResponse> {

    let mut messages = vec![];

    let config = config_r(&deps.storage).load()?;

    let full_asset = match asset_r(&deps.storage).may_load(env.message.sender.as_str().as_bytes())? {
        Some(a) => a,
        None => {
            return Err(StdError::generic_err("Unrecognized Asset"));
        }
    };

    // Do maintenance things here

    Ok(HandleResponse {
        messages,
        log: vec![],
        data: Some(to_binary(&adapter::HandleAnswer::Update {
            status: ResponseStatus::Success,
        })?),
    })
}

pub fn unbond<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    asset: HumanAddr,
    amount: Uint128,
) -> StdResult<HandleResponse> {
    /* Unbonding to the scrt staking contract
     * Once scrt is on balance sheet, treasury can claim
     * and this contract will take all scrt->sscrt and send
     */

    let config = config_r(&deps.storage).load()?;

    if !config.admins.contains(&env.message.sender) && env.message.sender != config.treasury {
        return Err(StdError::unauthorized());
    }

    match asset_r(&deps.storage).may_load(env.message.sender.as_str().as_bytes())? {
        Some(a) => {
            let balance = balance_query(
                &deps.querier,
                self_address_r(&deps.storage).load()?,
                viewing_key_r(&deps.storage).load()?,
                1,
                a.contract.code_hash.clone(),
                a.contract.address.clone(),
            )?.amount;

            let unbonding = unbonding_r(&deps.storage).load(&a.contract.address.as_str().as_bytes())?;

            if unbonding + amount > balance {
                return Err(StdError::generic_err("Not enough funds to unbond"));
            }

            unbonding_w(&mut deps.storage).save(&a.contract.address.as_str().as_bytes(), &Uint128::zero())?;

            Ok(HandleResponse {
                messages: vec![
                    send_msg(
                        config.treasury,
                        unbonding + amount,
                        None,
                        None,
                        None,
                        1,
                        a.contract.code_hash.clone(),
                        a.contract.address.clone(),
                    )?
                ],
                log: vec![],
                data: Some(to_binary(&adapter::HandleAnswer::Unbond {
                    status: ResponseStatus::Success,
                    amount,
                })?),
            })
        },
        None => Err(StdError::generic_err("Unrecognized Asset")),
    }
}

/* Claims completed unbondings, wraps them, 
 * and returns them to treasury
 */
pub fn claim<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    asset: HumanAddr,
) -> StdResult<HandleResponse> {
    let config = config_r(&deps.storage).load()?;

    if !config.admins.contains(&env.message.sender) && env.message.sender != config.treasury {
        return Err(StdError::unauthorized());
    }

    match asset_r(&deps.storage).may_load(env.message.sender.as_str().as_bytes())? {
        Some(a) => {
            let balance = balance_query(
                &deps.querier,
                self_address_r(&deps.storage).load()?,
                viewing_key_r(&deps.storage).load()?,
                1,
                a.contract.code_hash.clone(),
                a.contract.address.clone(),
            )?.amount;

            let mut claimable = unbonding_r(&deps.storage).load(&a.contract.address.as_str().as_bytes())?;

            if claimable > balance {
                claimable = balance;
            }

            unbonding_w(&mut deps.storage).update(&a.contract.address.as_str().as_bytes(), |u| u.unwrap() - claimable)?;

            Ok(HandleResponse {
                messages: vec![
                    send_msg(
                        config.treasury,
                        claimable,
                        None,
                        None,
                        None,
                        1,
                        a.contract.code_hash.clone(),
                        a.contract.address.clone(),
                    )?
                ],
                log: vec![],
                data: Some(to_binary(&adapter::HandleAnswer::Claim {
                    status: ResponseStatus::Success,
                    amount: claimable,
                })?),
            })
        },
        None => Err(StdError::generic_err("Unrecognized Asset")),
    }
}
