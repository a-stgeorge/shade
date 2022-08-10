use crate::utils::storage::plus::MapStorage;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{StdResult, Uint128, Uint256};
use secret_storage_plus::{Json, Map, PrimaryKey};
use std::{convert::TryInto, marker::PhantomData};

#[cw_serde]
pub struct SharePool<'a, K: PrimaryKey<'a>> {
    pub shares: Uint256,
    pub tokens: Uint128,
    pub multiplier: u32,
    _key: PhantomData<K>,
    _lifetime: PhantomData<&'a ()>,
}

impl<'a, K: PrimaryKey<'a>> Default for SharePool<'a, K> {
    fn default() -> Self {
        Self {
            shares: Default::default(),
            tokens: Default::default(),
            multiplier: 0,
            _key: Default::default(),
            _lifetime: Default::default(),
        }
    }
}

impl<'a, K: PrimaryKey<'a>> MapStorage<'a, K> for SharePool<'a, K> {
    const MAP: Map<'static, K, Self, Json> = Map::new("share-pool-");
}

impl<'a, K: PrimaryKey<'a>> SharePool<'a, K> {
    pub fn is_zero(&self) -> bool {
        self.shares.is_zero() && self.tokens.is_zero()
    }

    pub fn multiplier(&self) -> StdResult<Uint256> {
        Ok(Uint256::from(10u128).checked_pow(self.multiplier)?)
    }

    pub fn user_shares_per_token<'b, K2: PrimaryKey<'b>>(
        &self,
        user: SharePool<'b, K2>,
    ) -> StdResult<Uint256> {
        let t_tokens = Uint256::from(self.tokens);
        let t_shares = self.shares;
        let tokens = Uint256::from(user.shares);

        if self.is_zero() {
            let token_multiplier = self.multiplier()?;

            return Ok(tokens.checked_mul(token_multiplier)?);
        }

        // (user tokens * total shares) / total tokens = user shares
        let m = tokens.checked_mul(t_shares)?;

        Ok(m / t_tokens)
    }

    pub fn user_tokens_per_share<'b, K2: PrimaryKey<'b>>(
        &self,
        user: SharePool<'b, K2>,
    ) -> StdResult<Uint128> {
        let t_tokens = Uint256::from(self.tokens);
        let t_shares = self.shares;
        let shares = Uint256::from(user.tokens);

        if self.is_zero() {
            let token_multiplier = self.multiplier()?;

            return Ok(shares.checked_div(token_multiplier)?.try_into()?);
        }

        // (user shares * total tokens) / total shares

        let m = shares.checked_mul(t_tokens)?;

        Ok((m / t_shares).try_into()?)
    }
}

#[cw_serde]
pub struct TokenPool {
    token_1: Uint128,
    token_2: Uint128,
}
