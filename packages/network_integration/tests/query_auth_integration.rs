use cosmwasm_math_compat::Uint128;
use cosmwasm_std::{to_binary, Binary, HumanAddr, Uint128 as prevUint128};
use network_integration::{
    utils::{
        generate_label, print_header, ACCOUNT_KEY, GAS,
        STORE_GAS, SHD_ADMIN_FILE, QUERY_AUTH_FILE,
    },
};
use query_authentication::transaction::PubKey;
use query_authentication::{permit::Permit, transaction::PermitSignature};
use secretcli::{
    cli_types::NetContract,
    secretcli::{account_address, create_permit, handle, init, query, Report},
};
use serde::Serialize;
use serde_json::Result;
use shade_protocol::contract_interfaces::query_auth;
use shade_admin::admin;
use shade_protocol::utils::asset::Contract;
use std::{
    borrow::Borrow,
    io::{self, Repeat, Write},
};

pub const ADMIN_KEY: &str = "b";
pub const SUPER_ADMIN_KEY: &str = "c";
pub const ADMIN_KEY_2: &str = "d";

fn setup_contracts(
    reports: &mut Vec<Report>,
) -> Result<(
    NetContract,
    NetContract,
)> {
    println!("Starting setup of account_addresses");
    io::stdout().flush();
    let account_a = account_address(ACCOUNT_KEY)?;
    let account_admin = account_address(ADMIN_KEY)?;
    let account_super = account_address(SUPER_ADMIN_KEY)?;

    print_header("Set up account_addresses");

    let shade_admin_init_msg = admin::InitMsg { };

    let shade_admin = init(
        &shade_admin_init_msg,
        SHD_ADMIN_FILE,
        &*generate_label(8),
        SUPER_ADMIN_KEY,
        Some(STORE_GAS),
        Some(GAS),
        Some("test"),
        reports,
    )?;

    let query_auth_init_msg = query_auth::InitMsg {
        admin_auth: Contract {
            address: HumanAddr::from(shade_admin.address.clone()),
            code_hash: shade_admin.code_hash.clone(),
        },
        prng_seed: Binary::from("random".as_bytes()),
    };

    let query_auth = init(
        &query_auth_init_msg,
        QUERY_AUTH_FILE,
        &*generate_label(8),
        SUPER_ADMIN_KEY,
        Some(STORE_GAS),
        Some(GAS),
        Some("test"),
        reports,
    )?;

    Ok((query_auth, shade_admin))
}

fn try_set_admin(
    new_admin: &NetContract,
    padding: Option<String>,
    query_auth: &NetContract,
    sender: &str,
    reports: &mut Vec<Report>,
) -> Result<()> {
    let set_admin_msg = query_auth::HandleMsg::SetAdminAuth { 
        admin: Contract {
            address: HumanAddr::from(new_admin.address.clone()),
            code_hash: new_admin.code_hash.clone()
        }, 
        padding, 
    };

    let set_admin_info = handle(
        &set_admin_msg,
        query_auth,
        sender,
        Some(GAS),
        Some("test"),
        None,
        reports,
        None
    )?.1;

    println!("Gas used to attempt to set the admin: {}", set_admin_info.gas_used);

    Ok(())
}

fn try_set_run_state(
    new_state: query_auth::ContractStatus,
    padding: Option<String>,
    query_auth: &NetContract,
    sender: &str,
    reports: &mut Vec<Report>,
) -> Result<()> {
    let set_run_state_msg = query_auth::HandleMsg::SetRunState { 
        state: new_state, 
        padding 
    };

    let set_run_state_info = handle(
        &set_run_state_msg,
        query_auth,
        sender,
        Some(GAS),
        Some("test"),
        None,
        reports,
        None
    )?.1;

    println!("Gas used to attempt to set the run state: {}", set_run_state_info.gas_used);

    Ok(())
}

fn try_set_viewing_key(
    key: String,
    padding: Option<String>,
    query_auth: &NetContract,
    sender: &str,
    reports: &mut Vec<Report>,
) -> Result<()> {
    let set_viewing_key_msg = query_auth::HandleMsg::SetViewingKey { 
        key, 
        padding 
    };

    let set_viewing_key_info = handle(
        &set_viewing_key_msg,
        query_auth,
        sender,
        Some(GAS),
        Some("test"),
        None,
        reports,
        None
    )?.1;

    println!("Gas used to attempt to set the viewing key: {}", set_viewing_key_info.gas_used);

    Ok(())
}

fn try_create_viewing_key(
    entropy: String,
    padding: Option<String>,
    query_auth: &NetContract,
    sender: &str,
    reports: &mut Vec<Report>
) -> Result<()> {
    let create_viewing_key_msg = query_auth::HandleMsg::CreateViewingKey { 
        entropy, 
        padding 
    };

    let create_viewing_key_info = handle(
        &create_viewing_key_msg,
        query_auth,
        sender,
        Some(GAS),
        Some("test"),
        None,
        reports,
        None
    )?.1;

    println!("Gas used to attempt to create the viewing key: {}", create_viewing_key_info.gas_used);

    Ok(())
}

fn try_block_permit_key(
    key: String,
    padding: Option<String>,
    query_auth: &NetContract,
    sender: &str,
    reports: &mut Vec<Report>
) -> Result<()> {
    let block_permit_key_msg = query_auth::HandleMsg::BlockPermitKey { 
        key, 
        padding 
    };

    let block_permit_key_info = handle(
        &block_permit_key_msg,
        query_auth,
        sender,
        Some(GAS),
        Some("test"),
        None,
        reports,
        None
    )?.1;

    println!("Gas used to attempt to block the permit key: {}", block_permit_key_info.gas_used);

    Ok(())
}

// fn print_pending_bonds(bonds: &NetContract, reports: &mut Vec<Report>) -> Result<()> {
//     // Create permit
//     let account_permit = create_signed_permit(
//         AccountPermitMsg {
//             contracts: vec![HumanAddr(bonds.address.clone())],
//             key: "key".to_string(),
//         },
//         None,
//         None,
//         ACCOUNT_KEY,
//     );

//     let account_quer_msg = bonds::QueryMsg::Account {
//         permit: account_permit,
//     };
//     let account_query: bonds::QueryAnswer = query(&bonds, account_quer_msg, None)?;

//     if let bonds::QueryAnswer::Account { pending_bonds } = account_query {
//         let pend_iter = pending_bonds.iter();
//         for pending in pend_iter {
//             println!("\nBond opp: {}\n Ends: {}\n Deposit Amount: {}\n Deposit Price: {}\n Claim Amount: {}\n Claim Price: {}\n Discount: {}\n Discount Price: {}", 
//             pending.deposit_denom.token_info.symbol,
//             pending.end_time,
//             pending.deposit_amount,
//             pending.deposit_price,
//             pending.claim_amount,
//             pending.claim_price,
//             pending.discount,
//             pending.discount_price,
//         )
//         }
//     }

//     Ok(())
// }

fn create_signed_permit<T: Clone + Serialize>(
    params: T,
    memo: Option<String>,
    msg_type: Option<String>,
    signer: &str,
) -> Permit<T> {
    let mut permit = Permit {
        params,
        signature: PermitSignature {
            pub_key: PubKey {
                r#type: "".to_string(),
                value: Default::default(),
            },
            signature: Default::default(),
        },
        account_number: None,
        chain_id: Some("testnet".to_string()),
        sequence: None,
        memo,
    };

    let unsigned_msg = permit.create_signed_tx(msg_type);

    let signed_info = create_permit(unsigned_msg, signer).unwrap();

    permit.signature = PermitSignature {
        pub_key: query_authentication::transaction::PubKey {
            r#type: signed_info.pub_key.msg_type,
            value: Binary::from_base64(&signed_info.pub_key.value).unwrap(),
        },
        signature: Binary::from_base64(&signed_info.signature).unwrap(),
    };

    permit
}

fn print_config(
    query_auth: &NetContract,
) -> Result<()> {
    let msg = query_auth::QueryMsg::Config {  };

    let query_info = query(
        &query_auth,
        msg,
        None,
    )?;

    if let query_auth::QueryAnswer::Config { admin, state } = query_info {
        println!("Admin address: {}", admin.address);
        let mut run_state = "";
        match state {
            query_auth::ContractStatus::Default => run_state = "Default",
            query_auth::ContractStatus::DisableAll => run_state = "Disable All",
            query_auth::ContractStatus::DisablePermit => run_state = "Disable Permit",
            query_auth::ContractStatus::DisableVK => run_state = "Disable VK"
        }
        println!("Contract run state: {}", run_state)
    }
    
    Ok(())
}

#[test]
fn run_query_auth_integration() -> Result<()> {
    let account_a = account_address(ACCOUNT_KEY)?;
    let account_super = account_address(SUPER_ADMIN_KEY)?;
    // let mut reports = vec![];

    // let (query_auth, shade_admin) = setup_contracts(&reports)?;


    Ok(())   
}