use serde_json::{Result, Error};
use secretcli::{cli_types::NetContract, secretcli::{TestInit, TestHandle, TestQuery}};
use crate::utils::{print_header, generate_label, ACCOUNT_KEY, STORE_GAS, GAS,
                   print_contract, print_warning};
use shade_protocol::{governance, asset::Contract};
use cosmwasm_std::{HumanAddr, Uint128};
use shade_protocol::governance::GOVERNANCE_SELF;

pub fn init_contract<Init: TestInit>(
    governance: &NetContract, contract_name: String,
    contract_path: &str, contract_init: Init) -> Result<NetContract> {
    print_header(&format!("{}{}", "Initializing ", contract_name.clone()));

    let contract = contract_init.inst_init(contract_path,
                                           &*generate_label(8), ACCOUNT_KEY,
                                           Some(STORE_GAS), Some(GAS),
                                           Some("test"))?;

    print_contract(&contract);

    add_contract(contract_name, &contract, &governance);

    Ok(contract)
}

pub fn get_contract(governance: &NetContract, target: String) -> Result<Contract> {
    let query: governance::QueryAnswer = governance::QueryMsg::GetSupportedContract {
        name: target }.t_query(&governance)?;

    let mut ctrc = Contract {
        address: HumanAddr::from("not_found".to_string()),
        code_hash: "not_found".to_string()
    };

    if let governance::QueryAnswer::SupportedContract { contract } = query {
        ctrc = contract;
    }

    Ok(ctrc)
}

pub fn add_contract(name: String, target: &NetContract, governance: &NetContract) -> Result<()>{
    print_warning(&format!("{}{}", "Adding ", name.clone()));

    let msg = governance::HandleMsg::AddSupportedContract {
        name,
        contract: Contract{
            address: HumanAddr::from(target.address.clone()),
            code_hash: target.code_hash.clone()
        }
    };

    create_proposal(governance, GOVERNANCE_SELF.to_string(),
                    msg, Some("Add a contract"))?;

    Ok(())
}

pub fn create_proposal<Handle: serde::Serialize>(
    governance: &NetContract, target: String, handle: Handle, desc: Option<&str>) -> Result<()> {
    let msg = serde_json::to_string(&handle)?;

    governance::HandleMsg::CreateProposal {
        target_contract: target,
        proposal: msg,
        description: match desc {
            None => "Custom proposal".to_string(),
            Some(description) => description.to_string()
        }
    }.t_handle(&governance, ACCOUNT_KEY, Some(GAS),
               Some("test"), None)?;

    trigger_latest_proposal(governance)?;

    Ok(())
}

pub fn trigger_latest_proposal(governance: &NetContract) -> Result<Uint128> {
    let query: governance::QueryAnswer = governance::QueryMsg::GetTotalProposals {
    }.t_query(&governance)?;

    let mut proposals = Uint128(1);

    if let governance::QueryAnswer::TotalProposals { total } = query {
        governance::HandleMsg::TriggerProposal { proposal_id: total
        }.t_handle(&governance, ACCOUNT_KEY, Some(GAS),
                   Some("test"), None)?;

        proposals = total;
    }

    Ok(proposals)
}