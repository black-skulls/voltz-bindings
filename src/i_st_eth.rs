pub use i_st_eth::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_st_eth {
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
    #[doc = "IStETH was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ISTETH_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getFee\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"feeBasisPoints\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_sharesAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPooledEthByShares\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct IStETH<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IStETH<M> {
        fn clone(&self) -> Self {
            IStETH(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IStETH<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IStETH<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IStETH))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IStETH<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ISTETH_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `getFee` (0xced72f87) function"]
        pub fn get_fee(&self) -> ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([206, 215, 47, 135], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPooledEthByShares` (0x7a28fb88) function"]
        pub fn get_pooled_eth_by_shares(
            &self,
            shares_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([122, 40, 251, 136], shares_amount)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IStETH<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `getFee` function with signature `getFee()` and selector `[206, 215, 47, 135]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getFee", abi = "getFee()")]
    pub struct GetFeeCall;
    #[doc = "Container type for all input parameters for the `getPooledEthByShares` function with signature `getPooledEthByShares(uint256)` and selector `[122, 40, 251, 136]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getPooledEthByShares", abi = "getPooledEthByShares(uint256)")]
    pub struct GetPooledEthBySharesCall {
        pub shares_amount: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IStETHCalls {
        GetFee(GetFeeCall),
        GetPooledEthByShares(GetPooledEthBySharesCall),
    }
    impl ethers::core::abi::AbiDecode for IStETHCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <GetFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IStETHCalls::GetFee(decoded));
            }
            if let Ok(decoded) =
                <GetPooledEthBySharesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IStETHCalls::GetPooledEthByShares(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IStETHCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IStETHCalls::GetFee(element) => element.encode(),
                IStETHCalls::GetPooledEthByShares(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IStETHCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IStETHCalls::GetFee(element) => element.fmt(f),
                IStETHCalls::GetPooledEthByShares(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetFeeCall> for IStETHCalls {
        fn from(var: GetFeeCall) -> Self {
            IStETHCalls::GetFee(var)
        }
    }
    impl ::std::convert::From<GetPooledEthBySharesCall> for IStETHCalls {
        fn from(var: GetPooledEthBySharesCall) -> Self {
            IStETHCalls::GetPooledEthByShares(var)
        }
    }
    #[doc = "Container type for all return fields from the `getFee` function with signature `getFee()` and selector `[206, 215, 47, 135]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetFeeReturn {
        pub fee_basis_points: u16,
    }
    #[doc = "Container type for all return fields from the `getPooledEthByShares` function with signature `getPooledEthByShares(uint256)` and selector `[122, 40, 251, 136]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetPooledEthBySharesReturn(pub ethers::core::types::U256);
}
