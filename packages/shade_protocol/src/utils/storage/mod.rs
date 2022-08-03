#[cfg(feature = "storage_plus")]
pub mod plus;

#[cfg(feature = "storage")]
pub mod default;

#[cfg(feature = "newtype")]
pub use newtype::ForwardNewtype;

#[cfg(feature = "newtype")]
pub mod newtype {
    use cosmwasm_std::{StdError};
    use crate::serde::{de::DeserializeOwned, Serialize};

    pub trait ForwardNewtype<T: Serialize + DeserializeOwned> {
        fn item(&self) -> T;
        fn item_ref(&self) -> &T;
        fn item_mut(&mut self) -> &mut T;
        fn item_set(&mut self, item: T);
        // Avoid using StdResult because we dont know the data size at compile time
        fn item_update<A, E>(&mut self, action: A) -> Result<(), E>
        where
            A: FnOnce(T) -> Result<T, E>,
            E: From<StdError>,
            Self: Sized
        {
            self.item_set(action(self.item())?);
            Ok(())
        }
    }

    #[cfg(test)]
    mod newtype_tests {
        use cosmwasm_std::{StdResult, Storage};
        use crate::utils::storage::newtype::ForwardNewtype;

        struct Wrap(u64);

        impl ForwardNewtype<u64> for Wrap {
            fn item(&self) -> u64 {
                self.0
            }

            fn item_ref(&self) -> &u64 {
                &self.0
            }

            fn item_mut(&mut self) -> &mut u64 {
                &mut self.0
            }

            fn item_set(&mut self, item: u64) {
                self.0 = item;
            }
        }

        #[test]
        fn get() {
            let t = Wrap(10);

            assert_eq!(10, t.item());
        }

        #[test]
        fn set() {
            let mut t = Wrap(10);

            t.item_set(20);

            assert_eq!(20, t.item());
        }

        #[test]
        fn update() {
            let mut t = Wrap(10);

            t.item_update(|item| -> StdResult<u64> {
                Ok(item + 10)
            }).unwrap();

            assert_eq!(20, t.item());
        }
    }

    #[cfg(test)]
    mod newtype_derive_tests {
        use cosmwasm_std::{StdResult, Uint128};
        use newtype_derive::internal_forward_newtype;

        #[derive(internal_forward_newtype)]
        struct DerivedWrap(u64);

        #[test]
        fn get() {
            let t = DerivedWrap(10);

            assert_eq!(10, t.item());
        }

        #[test]
        fn set() {
            let mut t = DerivedWrap(10);

            t.item_set(20);

            assert_eq!(20, t.item());
        }

        #[test]
        fn update() {
            let mut t = DerivedWrap(10);

            t.item_update(|item| -> StdResult<u64> {
                Ok(item + 10)
            }).unwrap();

            assert_eq!(20, t.item());
        }
    }
}