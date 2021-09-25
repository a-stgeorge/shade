use serde_json::Result;
use cosmwasm_std::{HumanAddr, Uint128};
use secretcli::{cli_types::NetContract,
                secretcli::{TestInit, TestHandle, list_contracts_by_code}};
use shade_protocol::{snip20::{InitialBalance}, snip20,
                     initializer, initializer::Snip20ContractInfo};
use crate::{utils::{print_header, generate_label, print_contract, print_warning,
                    gov_add_contract, STORE_GAS, GAS, VIEW_KEY, ACCOUNT_KEY},
            contract_helpers::minter::get_balance};

pub fn initialize_initializer(
    governance: &NetContract, sSCRT: &NetContract, account: String) -> Result<()> {
    print_header("Initializing Initializer");
    let mut shade = NetContract {
        label: generate_label(8),
        id: "".to_string(),
        address: "".to_string(),
        code_hash: sSCRT.code_hash.clone()
    };

    let mut silk = NetContract {
        label: generate_label(8),
        id: "".to_string(),
        address: "".to_string(),
        code_hash: sSCRT.code_hash.clone()
    };

    let initializer = initializer::InitMsg {
        snip20_id: sSCRT.id.parse::<u64>().unwrap(),
        snip20_code_hash: sSCRT.code_hash.clone(),
        shade: Snip20ContractInfo {
            label: shade.label.clone(),
            admin: Some(HumanAddr::from(governance.address.clone())),
            prng_seed: Default::default(),
            initial_balances: Some(vec![InitialBalance{
                address: HumanAddr::from(account.clone()), amount: Uint128(10000000) }])
        },
        silk: Snip20ContractInfo {
            label: silk.label.clone(),
            admin: Some(HumanAddr::from(governance.address.clone())),
            prng_seed: Default::default(),
            initial_balances: None
        }
    }.inst_init("../../compiled/initializer.wasm.gz", &*generate_label(8),
                ACCOUNT_KEY, Some(STORE_GAS), Some(GAS),
                Some("test"))?;
    print_contract(&initializer);

    print_header("Getting uploaded Snip20s");

    let contracts = list_contracts_by_code(sSCRT.id.clone())?;

    for contract in contracts {
        if &contract.label == &shade.label {
            print_warning("Found Shade");
            shade.id = contract.code_id.to_string();
            shade.address = contract.address;
            print_contract(&shade);
        }
        else if &contract.label == &silk.label {
            print_warning("Found Silk");
            silk.id = contract.code_id.to_string();
            silk.address = contract.address;
            print_contract(&silk);
        }
    }

    // Set View keys
    snip20::HandleMsg::SetViewingKey { key: String::from(VIEW_KEY), padding: None }.t_handle(
        &shade, ACCOUNT_KEY, Some(GAS), Some("test"), None)?;

    println!("\n\tTotal shade: {}", get_balance(&shade, account.clone()));

    snip20::HandleMsg::SetViewingKey { key: String::from(VIEW_KEY), padding: None }.t_handle(
        &silk, ACCOUNT_KEY, Some(GAS), Some("test"), None)?;

    println!("\tTotal silk: {}", get_balance(&silk, account.clone()));

    // Add contracts
    gov_add_contract("initializer".to_string(), &initializer, &governance)?;
    gov_add_contract("shade".to_string(), &shade, &governance)?;
    gov_add_contract("silk".to_string(), &silk, &governance)?;

    Ok(())
}