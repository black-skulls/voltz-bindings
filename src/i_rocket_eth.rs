pub use i_rocket_eth::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_rocket_eth {
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
    #[doc = "IRocketEth was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IROCKETETH_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_rethAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getEthValue\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct IRocketEth<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IRocketEth<M> {
        fn clone(&self) -> Self {
            IRocketEth(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IRocketEth<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IRocketEth<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IRocketEth))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IRocketEth<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IROCKETETH_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `getEthValue` (0x8b32fa23) function"]
        pub fn get_eth_value(
            &self,
            reth_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([139, 50, 250, 35], reth_amount)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IRocketEth<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `getEthValue` function with signature `getEthValue(uint256)` and selector `[139, 50, 250, 35]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getEthValue", abi = "getEthValue(uint256)")]
    pub struct GetEthValueCall {
        pub reth_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getEthValue` function with signature `getEthValue(uint256)` and selector `[139, 50, 250, 35]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetEthValueReturn(pub ethers::core::types::U256);
}
