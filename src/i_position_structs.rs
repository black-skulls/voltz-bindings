pub use i_position_structs::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_position_structs {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "IPositionStructs was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IPOSITIONSTRUCTS_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str("[]").expect("invalid abi")
        });
    pub struct IPositionStructs<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IPositionStructs<M> {
        fn clone(&self) -> Self {
            IPositionStructs(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IPositionStructs<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IPositionStructs<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IPositionStructs))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IPositionStructs<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IPOSITIONSTRUCTS_ABI.clone(), client)
                .into()
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IPositionStructs<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
}
