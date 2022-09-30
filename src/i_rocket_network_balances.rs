pub use i_rocket_network_balances::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_rocket_network_balances {
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
    #[doc = "IRocketNetworkBalances was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IROCKETNETWORKBALANCES_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getBalancesBlock\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct IRocketNetworkBalances<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IRocketNetworkBalances<M> {
        fn clone(&self) -> Self {
            IRocketNetworkBalances(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IRocketNetworkBalances<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IRocketNetworkBalances<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IRocketNetworkBalances))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IRocketNetworkBalances<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                IROCKETNETWORKBALANCES_ABI.clone(),
                client,
            )
            .into()
        }
        #[doc = "Calls the contract's `getBalancesBlock` (0x9100c13d) function"]
        pub fn get_balances_block(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([145, 0, 193, 61], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for IRocketNetworkBalances<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `getBalancesBlock` function with signature `getBalancesBlock()` and selector `[145, 0, 193, 61]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getBalancesBlock", abi = "getBalancesBlock()")]
    pub struct GetBalancesBlockCall;
    #[doc = "Container type for all return fields from the `getBalancesBlock` function with signature `getBalancesBlock()` and selector `[145, 0, 193, 61]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetBalancesBlockReturn(pub ethers::core::types::U256);
}
