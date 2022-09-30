pub use ia_token::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod ia_token {
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
    #[doc = "IAToken was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IATOKEN_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"receiverOfUnderlying\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burn\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    pub struct IAToken<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IAToken<M> {
        fn clone(&self) -> Self {
            IAToken(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IAToken<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IAToken<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IAToken))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IAToken<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IATOKEN_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `burn` (0xd7020d0a) function"]
        pub fn burn(
            &self,
            user: ethers::core::types::Address,
            receiver_of_underlying: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [215, 2, 13, 10],
                    (user, receiver_of_underlying, amount, index),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IAToken<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `burn` function with signature `burn(address,address,uint256,uint256)` and selector `[215, 2, 13, 10]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "burn", abi = "burn(address,address,uint256,uint256)")]
    pub struct BurnCall {
        pub user: ethers::core::types::Address,
        pub receiver_of_underlying: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub index: ethers::core::types::U256,
    }
}
