pub use wad_ray_math::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod wad_ray_math {
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
    #[doc = "WadRayMath was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static WADRAYMATH_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"HALF_RAY\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"HALF_WAD\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"RAY\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"WAD\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"WAD_RAY_RATIO\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static WADRAYMATH_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x61015161003a600b82828239805160001a60731461002d57634e487b7160e01b600052600060045260246000fd5b30600052607381538281f3fe73000000000000000000000000000000000000000030146080604052600436106100615760003560e01c8063552033c4146100665780635fcd68091461008b578063629fa5a7146100935780636a1460241461009b57806385aa454e146100aa575b600080fd5b6100796b033b2e3c9fd0803ce800000081565b60405190815260200160405180910390f35b6100796100b2565b6100796100cc565b610079670de0b6b3a764000081565b6100796100ea565b6100c960026b033b2e3c9fd0803ce80000006100f9565b81565b6100c9670de0b6b3a76400006b033b2e3c9fd0803ce80000006100f9565b6100c96002670de0b6b3a76400005b60008261011657634e487b7160e01b600052601260045260246000fd5b50049056fea26469706673582212205bce87045170692e5861f16385689ffe8f8cae3354c320064f5c92b47b61daf264736f6c63430008090033" . parse () . expect ("invalid bytecode")
        });
    pub struct WadRayMath<M>(ethers::contract::Contract<M>);
    impl<M> Clone for WadRayMath<M> {
        fn clone(&self) -> Self {
            WadRayMath(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for WadRayMath<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for WadRayMath<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(WadRayMath))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> WadRayMath<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), WADRAYMATH_ABI.clone(), client).into()
        }
        #[doc = r" Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it."]
        #[doc = r" Returns a new instance of a deployer that returns an instance of this contract after sending the transaction"]
        #[doc = r""]
        #[doc = r" Notes:"]
        #[doc = r" 1. If there are no constructor arguments, you should pass `()` as the argument."]
        #[doc = r" 1. The default poll duration is 7 seconds."]
        #[doc = r" 1. The default number of confirmations is 1 block."]
        #[doc = r""]
        #[doc = r""]
        #[doc = r" # Example"]
        #[doc = r""]
        #[doc = r" Generate contract bindings with `abigen!` and deploy a new contract instance."]
        #[doc = r""]
        #[doc = r" *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact."]
        #[doc = r""]
        #[doc = r" ```ignore"]
        #[doc = r" # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {"]
        #[doc = r#"     abigen!(Greeter,"../greeter.json");"#]
        #[doc = r""]
        #[doc = r#"    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();"#]
        #[doc = r"    let msg = greeter_contract.greet().call().await.unwrap();"]
        #[doc = r" # }"]
        #[doc = r" ```"]
        pub fn deploy<T: ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::std::result::Result<
            ethers::contract::builders::ContractDeployer<M, Self>,
            ethers::contract::ContractError<M>,
        > {
            let factory = ethers::contract::ContractFactory::new(
                WADRAYMATH_ABI.clone(),
                WADRAYMATH_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `HALF_RAY` (0x5fcd6809) function"]
        pub fn half_ray(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([95, 205, 104, 9], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `HALF_WAD` (0x85aa454e) function"]
        pub fn half_wad(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([133, 170, 69, 78], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `RAY` (0x552033c4) function"]
        pub fn ray(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([85, 32, 51, 196], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `WAD` (0x6a146024) function"]
        pub fn wad(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([106, 20, 96, 36], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `WAD_RAY_RATIO` (0x629fa5a7) function"]
        pub fn wad_ray_ratio(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([98, 159, 165, 167], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for WadRayMath<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `HALF_RAY` function with signature `HALF_RAY()` and selector `[95, 205, 104, 9]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "HALF_RAY", abi = "HALF_RAY()")]
    pub struct HalfRayCall;
    #[doc = "Container type for all input parameters for the `HALF_WAD` function with signature `HALF_WAD()` and selector `[133, 170, 69, 78]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "HALF_WAD", abi = "HALF_WAD()")]
    pub struct HalfWadCall;
    #[doc = "Container type for all input parameters for the `RAY` function with signature `RAY()` and selector `[85, 32, 51, 196]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "RAY", abi = "RAY()")]
    pub struct RayCall;
    #[doc = "Container type for all input parameters for the `WAD` function with signature `WAD()` and selector `[106, 20, 96, 36]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "WAD", abi = "WAD()")]
    pub struct WadCall;
    #[doc = "Container type for all input parameters for the `WAD_RAY_RATIO` function with signature `WAD_RAY_RATIO()` and selector `[98, 159, 165, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "WAD_RAY_RATIO", abi = "WAD_RAY_RATIO()")]
    pub struct WadRayRatioCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum WadRayMathCalls {
        HalfRay(HalfRayCall),
        HalfWad(HalfWadCall),
        Ray(RayCall),
        Wad(WadCall),
        WadRayRatio(WadRayRatioCall),
    }
    impl ethers::core::abi::AbiDecode for WadRayMathCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <HalfRayCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WadRayMathCalls::HalfRay(decoded));
            }
            if let Ok(decoded) =
                <HalfWadCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WadRayMathCalls::HalfWad(decoded));
            }
            if let Ok(decoded) = <RayCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(WadRayMathCalls::Ray(decoded));
            }
            if let Ok(decoded) = <WadCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(WadRayMathCalls::Wad(decoded));
            }
            if let Ok(decoded) =
                <WadRayRatioCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WadRayMathCalls::WadRayRatio(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for WadRayMathCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                WadRayMathCalls::HalfRay(element) => element.encode(),
                WadRayMathCalls::HalfWad(element) => element.encode(),
                WadRayMathCalls::Ray(element) => element.encode(),
                WadRayMathCalls::Wad(element) => element.encode(),
                WadRayMathCalls::WadRayRatio(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for WadRayMathCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                WadRayMathCalls::HalfRay(element) => element.fmt(f),
                WadRayMathCalls::HalfWad(element) => element.fmt(f),
                WadRayMathCalls::Ray(element) => element.fmt(f),
                WadRayMathCalls::Wad(element) => element.fmt(f),
                WadRayMathCalls::WadRayRatio(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<HalfRayCall> for WadRayMathCalls {
        fn from(var: HalfRayCall) -> Self {
            WadRayMathCalls::HalfRay(var)
        }
    }
    impl ::std::convert::From<HalfWadCall> for WadRayMathCalls {
        fn from(var: HalfWadCall) -> Self {
            WadRayMathCalls::HalfWad(var)
        }
    }
    impl ::std::convert::From<RayCall> for WadRayMathCalls {
        fn from(var: RayCall) -> Self {
            WadRayMathCalls::Ray(var)
        }
    }
    impl ::std::convert::From<WadCall> for WadRayMathCalls {
        fn from(var: WadCall) -> Self {
            WadRayMathCalls::Wad(var)
        }
    }
    impl ::std::convert::From<WadRayRatioCall> for WadRayMathCalls {
        fn from(var: WadRayRatioCall) -> Self {
            WadRayMathCalls::WadRayRatio(var)
        }
    }
    #[doc = "Container type for all return fields from the `HALF_RAY` function with signature `HALF_RAY()` and selector `[95, 205, 104, 9]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct HalfRayReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `HALF_WAD` function with signature `HALF_WAD()` and selector `[133, 170, 69, 78]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct HalfWadReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `RAY` function with signature `RAY()` and selector `[85, 32, 51, 196]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct RayReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `WAD` function with signature `WAD()` and selector `[106, 20, 96, 36]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct WadReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `WAD_RAY_RATIO` function with signature `WAD_RAY_RATIO()` and selector `[98, 159, 165, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct WadRayRatioReturn(pub ethers::core::types::U256);
}
