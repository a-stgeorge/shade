pub mod handle;
pub mod query;
pub mod integration;

use std::mem::size_of_val;

use contract_harness::harness::{query_auth::QueryAuth, admin::Admin};
use shade_protocol::c_std::testing::mock_env;
use shade_protocol::c_std::{
    Binary,
    HumanAddr,
    StdResult,
};
use shade_protocol::contract_interfaces::query_auth::RngSeed;
use shade_protocol::contract_interfaces::query_auth::auth::Key;
use shade_protocol::fadroma::ensemble::{ContractEnsemble, MockEnv};
use shade_protocol::fadroma::core::ContractLink;
use shade_protocol::query_authentication::transaction::{PermitSignature, PubKey};
use shade_protocol::contract_interfaces::{
    query_auth::{self, PermitData, QueryPermit},
};
use shade_protocol::utils::asset::Contract;

pub fn init_contract() -> StdResult<(ContractEnsemble, ContractLink<HumanAddr>)> {
    let mut chain = ContractEnsemble::new(20);

    let admin = chain.register(Box::new(Admin));
    let admin = chain.instantiate(
        admin.id,
        &shade_admin::admin::InitMsg{},
        MockEnv::new("admin", ContractLink {
            address: "admin_contract".into(),
            code_hash: admin.code_hash,
        }),
    )?.instance;

    let auth = chain.register(Box::new(QueryAuth));
    let auth = chain
        .instantiate(
            auth.id,
            &query_auth::InitMsg {
                admin_auth: Contract {
                    address: admin.address.clone(),
                    code_hash: admin.code_hash.clone()
                },
                prng_seed: Binary::from("random".as_bytes()),
            },
            MockEnv::new("admin", ContractLink {
                address: "auth".into(),
                code_hash: auth.code_hash,
            }),
        )?
        .instance;

    chain.execute(&shade_admin::admin::HandleMsg::AddContract {
        contract_address: auth.address.to_string()
    }, MockEnv::new("admin", admin.clone()))?;

    chain.execute(&shade_admin::admin::HandleMsg::AddAuthorization {
        contract_address: auth.address.to_string(),
        admin_address: "admin".to_string()
    }, MockEnv::new("admin", admin.clone()))?;

    Ok((chain, auth))
}

pub fn get_permit() -> QueryPermit {
    QueryPermit {
        params: PermitData {
            key: "key".to_string(),
            data: Binary::from_base64("c29tZSBzdHJpbmc=").unwrap()
        },
        signature: PermitSignature {
            pub_key: PubKey::new(
                Binary::from_base64(
                    "A9NjbriiP7OXCpoTov9ox/35+h5k0y1K0qCY/B09YzAP"
                ).unwrap()
            ),
            signature: Binary::from_base64(
                "XRzykrPmMs0ZhksNXX+eU0TM21fYBZXZogr5wYZGGy11t2ntfySuQNQJEw6D4QKvPsiU9gYMsQ259dOzMZNAEg=="
            ).unwrap()
        },
        account_number: None,
        chain_id: Some(String::from("chain")),
        sequence: None,
        memo: None
    }
}

#[test]
fn generate_overflow() {
    println!("Key:");
    // let deps = mock_dependencies(20, &[]);
    let env = mock_env("creator00000000000000000000000000000000000000000000000000000000", &[]);
    let entropy: [u8; 2053672] = [0; 2053672];
    let seed = RngSeed::new(Binary::from("random".as_bytes()));
    let key = Key::generate(&env, &seed.0, &entropy);
    println!("{}", size_of_val(&entropy));
    println!("{}", seed.0.len());
    println!("{}", env.message.sender.len());
    // let entropy2: [u8; 10000000] = [0; 10000000];

    println!("{}", key.to_string());
    assert!(true)
}