pub use i_compound_fcm::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_compound_fcm {
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
    #[doc = "ICompoundFCM was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ICOMPOUNDFCM_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"cToken\",\"outputs\":[{\"internalType\":\"contract ICToken\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct ICompoundFCM<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ICompoundFCM<M> {
        fn clone(&self) -> Self {
            ICompoundFCM(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ICompoundFCM<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ICompoundFCM<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ICompoundFCM))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ICompoundFCM<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ICOMPOUNDFCM_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `cToken` (0x69e527da) function"]
        pub fn c_token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([105, 229, 39, 218], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ICompoundFCM<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `cToken` function with signature `cToken()` and selector `[105, 229, 39, 218]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "cToken", abi = "cToken()")]
    pub struct CtokenCall;
    #[doc = "Container type for all return fields from the `cToken` function with signature `cToken()` and selector `[105, 229, 39, 218]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CtokenReturn(pub ethers::core::types::Address);
}
