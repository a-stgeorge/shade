pub mod assembly;
pub mod assembly_msg;
pub mod contract;
pub mod profile;
pub mod proposal;

use crate::tests::{admin_only_governance, get_config};
use contract_harness::harness::snip20::Snip20;
use shade_protocol::c_std::Addr;
use shade_protocol::utils::{ExecuteCallback, InstantiateCallback, Query};
use shade_protocol::{contract_interfaces::{governance, snip20}, utils::asset::Contract};

#[test]
fn init_contract() {
    admin_only_governance().unwrap();
}

#[test]
fn set_config_msg() {
    let (mut chain, gov) = admin_only_governance().unwrap();

    let old_config = get_config(&mut chain, &gov).unwrap();

    let snip20 = chain.register(Box::new(Snip20));
    let snip20 = chain
        .instantiate(
            snip20.id,
            &snip20::InstantiateMsg {
                name: "funding_token".to_string(),
                admin: None,
                symbol: "FND".to_string(),
                decimals: 6,
                initial_balances: None,
                prng_seed: Default::default(),
                config: None,
            }.test_exec("admin", ContractLink {
                address: "funding_token".into(),
                code_hash: snip20.code_hash,
            }),
        )
        .unwrap()
        .instance;

    governance::ExecuteMsg::SetConfig {
                treasury: Some(Addr::unchecked("random")),
                funding_token: Some(Contract {
                    address: snip20.address.clone(),
                    code_hash: snip20.code_hash.clone(),
                }),
                vote_token: Some(Contract {
                    address: snip20.address,
                    code_hash: snip20.code_hash,
                }),
                padding: None,
            }.test_exec(// Sender is self
                &gov, &mut chain, gov.address.clone(), &[]
        )
        .unwrap();

    let new_config = get_config(&mut chain, &gov).unwrap();

    assert_ne!(old_config.treasury, new_config.treasury);
    assert_ne!(old_config.funding_token, new_config.funding_token);
    assert_ne!(old_config.vote_token, new_config.vote_token);
}

#[test]
fn unauthorised_set_config_msg() {
    let (mut chain, gov) = admin_only_governance().unwrap();

    assert!(governance::ExecuteMsg::SetConfig {
                treasury: None,
                funding_token: None,
                vote_token: None,
                padding: None,
            }.test_exec(// Sender is self
                &gov, &mut chain, Addr::unchecked("random"), &[]
        )
        .is_err());
}

#[test]
fn reject_disable_config_tokens() {
    let (mut chain, gov) = admin_only_governance().unwrap();

    let snip20 = chain.register(Box::new(Snip20));
    let snip20 = chain
        .instantiate(
            snip20.id,
            &snip20::InstantiateMsg {
                name: "funding_token".to_string(),
                admin: None,
                symbol: "FND".to_string(),
                decimals: 6,
                initial_balances: None,
                prng_seed: Default::default(),
                config: None,
            }.test_exec("admin", ContractLink {
                address: "funding_token".into(),
                code_hash: snip20.code_hash,
            }),
        )
        .unwrap()
        .instance;

    governance::ExecuteMsg::SetConfig {
                treasury: Some(Addr::unchecked("random")),
                funding_token: Some(Contract {
                    address: snip20.address.clone(),
                    code_hash: snip20.code_hash.clone(),
                }),
                vote_token: Some(Contract {
                    address: snip20.address,
                    code_hash: snip20.code_hash,
                }),
                padding: None,
            }.test_exec(// Sender is self
                &gov, &mut chain, gov.address.clone(), &[]
        )
        .unwrap();

    let old_config = get_config(&mut chain, &gov).unwrap();

    governance::ExecuteMsg::SetConfig {
                treasury: None,
                funding_token: None,
                vote_token: None,
                padding: None,
            }.test_exec(// Sender is self
                &gov, &mut chain, gov.address.clone(), &[]
        )
        .unwrap();

    let new_config = get_config(&mut chain, &gov).unwrap();

    assert_eq!(old_config.treasury, new_config.treasury);
    assert_eq!(old_config.funding_token, new_config.funding_token);
    assert_eq!(old_config.vote_token, new_config.vote_token);
}
