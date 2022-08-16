use crate::tests::init_contract;
use shade_protocol::{
    c_std::{
        Binary,
        from_binary,
        HumanAddr,
        StdError,
        StdResult,
        Uint128,
    },
    fadroma::{
        ensemble::{ContractEnsemble, MockEnv},
        core::ContractLink,
    },
    contract_interfaces::query_auth::{
        self,
        ContractStatus,
        PermitData,
        QueryPermit,
    },
    query_authentication::transaction::{PermitSignature, PubKey},
};

pub fn testing_permit_1() -> QueryPermit {
    QueryPermit {
        params: PermitData {
            key: "key".to_string(),
            data: Binary::from_base64("e30=").unwrap(),
        },
        signature: PermitSignature {
            pub_key: PubKey::new(
                Binary::from_base64(
                    "Ao74ojXj3NLoTsyICacAb0FJq1iHRTxKX2J/CcRuRjmK"
                ).unwrap()
            ),
            signature: Binary::from_base64(
                "OHO/5U0TSo94eUDjKqXnJxfCJ4Z4zpnnd4B3DVA/KYYHQ5T45qcsUmP7VxnAhUkzWfvLCd35wjCMBfBt/IvkHw=="
            ).unwrap(),
        },
        account_number: Some(Uint128::zero()),
        chain_id: Some(String::from("secret-4")),
        sequence: Some(Uint128::zero()),
        memo: None,
    }
}

pub fn testing_permit_2() -> QueryPermit {
    QueryPermit {
        params: PermitData {
            key: "key2".to_string(),
            data: Binary::from_base64("eyJkaWZmZXJlbnQiOiAibXNnIn0=").unwrap(),
        },
        signature: PermitSignature {
            pub_key: PubKey::new(
                Binary::from_base64(
                    "Ao74ojXj3NLoTsyICacAb0FJq1iHRTxKX2J/CcRuRjmK"
                ).unwrap()
            ),
            signature: Binary::from_base64(
                "yoaX1soo51GKuW6paNMP2m+Mmisi/hGFyLvHZey++XBNjrKCcp3qzeWz15MyqWG/Nru1XF9VMQLlfG6yA9V32A=="
            ).unwrap(),
        },
        account_number: Some(Uint128::zero()),
        chain_id: Some(String::from("secret-4")),
        sequence: Some(Uint128::zero()),
        memo: None,
    }
}

struct TestingEnv {
    pub chain: ContractEnsemble,
    pub auth: ContractLink<HumanAddr>,
}

impl TestingEnv {
    fn new() -> Self {
        let (chain, auth) = init_contract().unwrap();
        TestingEnv { chain, auth }
    }

    fn set_runstate(&mut self, user: impl Into<String>, state: ContractStatus) -> StdResult<()>{
        let msg = query_auth::HandleMsg::SetRunState {
            state,
            padding: None,
        };
        let data = self.chain.execute(
            &msg, 
            MockEnv::new(user.into(), self.auth.clone())
        )?.response.data.unwrap();
        let msg: query_auth::HandleAnswer = from_binary(&data)?;
        
        match msg {
            query_auth::HandleAnswer::SetRunState { .. } => { Ok(()) },
            _ => {
                panic!("Bad response from SetRunState");
            },
        }
 
    }

    fn create_vk(&mut self, user: impl Into<String>) -> StdResult<String> {
        let msg = query_auth::HandleMsg::CreateViewingKey{
            entropy: "randomness".to_string(),
            padding: None,
        };
        let data = self.chain.execute(
            &msg, 
            MockEnv::new(user.into(), self.auth.clone())
        )?.response.data.unwrap();
        let msg: query_auth::HandleAnswer = from_binary(&data)?;
        
        match msg {
            query_auth::HandleAnswer::CreateViewingKey { key, .. } => Ok(key),
            _ => {
                panic!("Bad response from CreateViewingKey");
            },
        }
    }

    fn set_vk(&mut self, user: impl Into<String>, key: impl Into<String>) -> StdResult<()> {
        let msg = query_auth::HandleMsg::SetViewingKey {
            key: key.into(),
            padding: None,
        };
        let data = self.chain.execute(
            &msg, 
            MockEnv::new(user.into(), self.auth.clone())
        )?.response.data.unwrap();
        let msg: query_auth::HandleAnswer = from_binary(&data)?;

        match msg {
            query_auth::HandleAnswer::SetViewingKey { .. } => { Ok(()) },
            _ => { panic!("Bad response from SetViewingKey"); },
        }
    }

    fn verify_vk(&self, user: impl Into<String>, key: impl Into<String>) -> StdResult<bool> {
        let query: query_auth::QueryAnswer = self.chain.query(
            self.auth.address.clone(),
            &query_auth::QueryMsg::ValidateViewingKey {
                user: HumanAddr::from(user.into()),
                key: key.into(),
            },
        )?;

        match query {
            query_auth::QueryAnswer::ValidateViewingKey { is_valid } => Ok(is_valid),
            _ => { panic!("Bad response from ValidateViewingKey"); },
        }
    }

    fn block_permit(&mut self, user: impl Into<String>, key: impl Into<String>) -> StdResult<()> {
        let msg = query_auth::HandleMsg::BlockPermitKey {
            key: key.into(),
            padding: None,
        };
        let data = self.chain.execute(
            &msg, 
            MockEnv::new(user.into(), self.auth.clone())
        )?.response.data.unwrap();
        let msg: query_auth::HandleAnswer = from_binary(&data)?;
        
        match msg {
            query_auth::HandleAnswer::BlockPermitKey { .. } => { Ok(()) },
            _ => {
                panic!("Bad response from BlockPermitKey");
            },
        }
    }

    fn verify_permit(&mut self, addr: HumanAddr, permit: QueryPermit) -> StdResult<bool> {
        let query: query_auth::QueryAnswer = self.chain.query(
            self.auth.address.clone(),
            &query_auth::QueryMsg::ValidatePermit {
                permit,
            },
        )?;

        match query {
            query_auth::QueryAnswer::ValidatePermit { user, is_revoked } => {
                Ok((user == addr) && !is_revoked)
            },
            _ => { panic!("Bad response from ValidatePermit"); },
        }
    }
}

// Default State Tests
#[test]
fn default_vk() {
    let mut env = TestingEnv::new();
    env.set_runstate("admin", ContractStatus::Default).unwrap();

    let vk = env.create_vk("user").unwrap();
    assert!(env.verify_vk("user", vk.clone()).unwrap());

    env.set_vk("user", "new key").unwrap();
    assert!(!env.verify_vk("user", vk).unwrap()); // fail
    assert!(env.verify_vk("user", "new key").unwrap()); // pass
}

#[test]
fn default_permit() {
    let mut env = TestingEnv::new();
    env.set_runstate("admin", ContractStatus::Default).unwrap();

    let permit = testing_permit_1();
    let user = "secret186uq24ra2n7ugtfnrxkpa08j02zh2v5097rw3m";
    let addr = HumanAddr::from(user);
    assert!(env.verify_permit(addr.clone(), permit.clone()).unwrap());

    env.block_permit(user, "key").unwrap();
    assert!(!env.verify_permit(addr.clone(), permit).unwrap()); // fail
    
    let permit2 = testing_permit_2();
    assert!(env.verify_permit(addr.clone(), permit2).unwrap());
}

// DisablePermit State Tests
#[test]
fn disable_permit_vk() {
    let mut env = TestingEnv::new();
    env.set_runstate("admin", ContractStatus::DisablePermit).unwrap();

    let vk = env.create_vk("user").unwrap();
    assert!(env.verify_vk("user", vk.clone()).unwrap());

    env.set_vk("user", "new key").unwrap();
    assert!(!env.verify_vk("user", vk).unwrap()); // fail
    assert!(env.verify_vk("user", "new key").unwrap()); // pass
}

#[test]
fn disable_permit_permit() {
    let mut env = TestingEnv::new();
    env.set_runstate("admin", ContractStatus::DisablePermit).unwrap();

    let permit = testing_permit_1();
    let user = "secret186uq24ra2n7ugtfnrxkpa08j02zh2v5097rw3m";
    let addr = HumanAddr::from(user);
    assert_eq!(
        env.verify_permit(addr.clone(), permit.clone()),
        Err(StdError::unauthorized()),
    ); 

    assert_eq!(
        env.block_permit(user, "key"),
        Err(StdError::unauthorized()),
    );
}

// DisableVK State Tests
#[test]
fn disable_vk_vk() {
    let mut env = TestingEnv::new();
    env.set_runstate("admin", ContractStatus::DisableVK).unwrap();

    assert_eq!(
        env.create_vk("user"),
        Err(StdError::unauthorized()),
    );
    assert_eq!(
        env.set_vk("user", "new key"),
        Err(StdError::unauthorized()),
    );
    assert_eq!(
        env.verify_vk("user", "key_that_was_never_set_but_still_won't_work"),
        Err(StdError::unauthorized()),
    );
}

#[test]
fn disable_vk_permit() {
    let mut env = TestingEnv::new();
    env.set_runstate("admin", ContractStatus::DisableVK).unwrap();

    let permit = testing_permit_1();
    let user = "secret186uq24ra2n7ugtfnrxkpa08j02zh2v5097rw3m";
    let addr = HumanAddr::from(user);
    assert!(env.verify_permit(addr.clone(), permit.clone()).unwrap());

    env.block_permit(user, "key").unwrap();
    assert!(!env.verify_permit(addr.clone(), permit).unwrap()); // fail
    
    let permit2 = testing_permit_2();
    assert!(env.verify_permit(addr.clone(), permit2).unwrap());
}

// DisableAll State Tests
#[test]
fn disable_all_vk() {
    let mut env = TestingEnv::new();
    env.set_runstate("admin", ContractStatus::DisableVK).unwrap();

    assert_eq!(
        env.create_vk("user"),
        Err(StdError::unauthorized()),
    );
    assert_eq!(
        env.set_vk("user", "new key"),
        Err(StdError::unauthorized()),
    );
    assert_eq!(
        env.verify_vk("user", "key_that_was_never_set_but_still_won't_work"),
        Err(StdError::unauthorized()),
    );
}

#[test]
fn disable_all_permit() {
    let mut env = TestingEnv::new();
    env.set_runstate("admin", ContractStatus::DisablePermit).unwrap();

    let permit = testing_permit_1();
    let user = "secret186uq24ra2n7ugtfnrxkpa08j02zh2v5097rw3m";
    let addr = HumanAddr::from(user);
    assert_eq!(
        env.verify_permit(addr.clone(), permit.clone()),
        Err(StdError::unauthorized()),
    ); 

    assert_eq!(
        env.block_permit(user, "key"),
        Err(StdError::unauthorized()),
    );
}

