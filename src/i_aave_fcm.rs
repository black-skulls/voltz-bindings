pub use i_aave_fcm::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_aave_fcm {
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
    #[doc = "IAaveFCM was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IAAVEFCM_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"aaveLendingPool\",\"outputs\":[{\"internalType\":\"contract IAaveV2LendingPool\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"underlyingYieldBearingToken\",\"outputs\":[{\"internalType\":\"contract IERC20Minimal\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct IAaveFCM<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IAaveFCM<M> {
        fn clone(&self) -> Self {
            IAaveFCM(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IAaveFCM<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IAaveFCM<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IAaveFCM))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IAaveFCM<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IAAVEFCM_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `aaveLendingPool` (0xe9d337b8) function"]
        pub fn aave_lending_pool(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([233, 211, 55, 184], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `underlyingYieldBearingToken` (0x357d8b5e) function"]
        pub fn underlying_yield_bearing_token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([53, 125, 139, 94], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IAaveFCM<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `aaveLendingPool` function with signature `aaveLendingPool()` and selector `[233, 211, 55, 184]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "aaveLendingPool", abi = "aaveLendingPool()")]
    pub struct AaveLendingPoolCall;
    #[doc = "Container type for all input parameters for the `underlyingYieldBearingToken` function with signature `underlyingYieldBearingToken()` and selector `[53, 125, 139, 94]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "underlyingYieldBearingToken",
        abi = "underlyingYieldBearingToken()"
    )]
    pub struct UnderlyingYieldBearingTokenCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IAaveFCMCalls {
        AaveLendingPool(AaveLendingPoolCall),
        UnderlyingYieldBearingToken(UnderlyingYieldBearingTokenCall),
    }
    impl ethers::core::abi::AbiDecode for IAaveFCMCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AaveLendingPoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveFCMCalls::AaveLendingPool(decoded));
            }
            if let Ok(decoded) =
                <UnderlyingYieldBearingTokenCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IAaveFCMCalls::UnderlyingYieldBearingToken(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IAaveFCMCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IAaveFCMCalls::AaveLendingPool(element) => element.encode(),
                IAaveFCMCalls::UnderlyingYieldBearingToken(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IAaveFCMCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IAaveFCMCalls::AaveLendingPool(element) => element.fmt(f),
                IAaveFCMCalls::UnderlyingYieldBearingToken(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AaveLendingPoolCall> for IAaveFCMCalls {
        fn from(var: AaveLendingPoolCall) -> Self {
            IAaveFCMCalls::AaveLendingPool(var)
        }
    }
    impl ::std::convert::From<UnderlyingYieldBearingTokenCall> for IAaveFCMCalls {
        fn from(var: UnderlyingYieldBearingTokenCall) -> Self {
            IAaveFCMCalls::UnderlyingYieldBearingToken(var)
        }
    }
    #[doc = "Container type for all return fields from the `aaveLendingPool` function with signature `aaveLendingPool()` and selector `[233, 211, 55, 184]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct AaveLendingPoolReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `underlyingYieldBearingToken` function with signature `underlyingYieldBearingToken()` and selector `[53, 125, 139, 94]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct UnderlyingYieldBearingTokenReturn(pub ethers::core::types::Address);
}
