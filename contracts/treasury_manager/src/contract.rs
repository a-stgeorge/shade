use shade_protocol::c_std::{
    debug_print,
    to_binary,
    Api,
    Binary,
    Env,
    DepsMut,
    Response,
    Querier,
    StdError,
    StdResult,
    Storage,
};

use shade_protocol::contract_interfaces::dao::treasury_manager::{
    Config,
    ExecuteMsg,
    InstantiateMsg,
    QueryMsg,
    Holder,
    Status,
};

use crate::{
    handle,
    query,
    state::{
        allocations_w, asset_list_w, config_w, self_address_w, viewing_key_w,
        holders_w, holder_w,
    },
};
use chrono::prelude::*;
use shade_protocol::contract_interfaces::dao::adapter;

pub fn init(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {

    config_w(deps.storage).save(&Config {
        admin: msg.admin.unwrap_or(info.sender.clone()),
        treasury: msg.treasury.clone(),
    })?;

    viewing_key_w(deps.storage).save(&msg.viewing_key)?;
    self_address_w(deps.storage).save(&env.contract.address)?;
    asset_list_w(deps.storage).save(&Vec::new())?;
    holders_w(deps.storage).save(&vec![msg.treasury.clone()])?;
    holder_w(deps.storage).save(
        msg.treasury.as_str().as_bytes(),
        &Holder {
            balances: vec![],
            unbondings: vec![],
            status: Status::Active,
        },
    )?;

    Ok(Response::new())
}

pub fn handle(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Receive {
            sender,
            from,
            amount,
            msg,
            ..
        } => handle::receive(deps, env, sender, from, amount, msg),
        ExecuteMsg::UpdateConfig { config } => handle::try_update_config(deps, env, config),
        ExecuteMsg::RegisterAsset { contract } => handle::try_register_asset(deps, &env, &contract),
        ExecuteMsg::Allocate { asset, allocation } => {
            handle::allocate(deps, &env, asset, allocation)
        },
        ExecuteMsg::AddHolder { holder } => handle::add_holder(deps, &env, holder),
        ExecuteMsg::RemoveHolder { holder } => handle::remove_holder(deps, &env, holder),
        ExecuteMsg::Adapter(a) => match a {
            adapter::SubHandleMsg::Unbond { asset, amount } => {
                handle::unbond(deps, &env, asset, amount)
            }
            adapter::SubHandleMsg::Claim { asset } => handle::claim(deps, &env, asset),
            adapter::SubHandleMsg::Update { asset } => handle::update(deps, &env, asset),
        },
    }
}

pub fn query(
    deps: Deps,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query::config(deps)?),
        QueryMsg::Assets {} => to_binary(&query::assets(deps)?),
        QueryMsg::Allocations { asset } => to_binary(&query::allocations(deps, asset)?),
        QueryMsg::PendingAllowance { asset } => to_binary(&query::pending_allowance(deps, asset)?),
        QueryMsg::Holders {} => to_binary(&query::holders(deps)),
        QueryMsg::Holder { holder } => to_binary(&query::holder(deps, holder)),

        // For holder specific queries
        QueryMsg::Balance { asset, holder } => to_binary(&query::balance(deps, asset, Some(holder))?),
        QueryMsg::Unbonding { asset, holder } => to_binary(&query::unbonding(deps, asset, Some(holder))?),
        QueryMsg::Unbondable { asset, holder } => to_binary(&query::unbondable(deps, asset, Some(holder))?),
        QueryMsg::Claimable { asset, holder } => to_binary(&query::claimable(deps, asset, Some(holder))?),

        QueryMsg::Adapter(a) => match a {
            adapter::SubQueryMsg::Balance { asset } => to_binary(&query::balance(deps, asset, None)?),
            adapter::SubQueryMsg::Unbonding { asset } => to_binary(&query::unbonding(deps, asset, None)?),
            adapter::SubQueryMsg::Unbondable { asset } => to_binary(&query::unbondable(deps, asset, None)?),
            adapter::SubQueryMsg::Claimable { asset } => to_binary(&query::claimable(deps, asset, None)?),
            adapter::SubQueryMsg::Reserves { asset } => to_binary(&query::reserves(deps, &asset)?),
        }
    }
}
