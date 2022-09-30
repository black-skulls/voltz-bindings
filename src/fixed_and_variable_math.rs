pub use fixed_and_variable_math::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod fixed_and_variable_math {
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
    #[doc = "FixedAndVariableMath was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static FIXEDANDVARIABLEMATH_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ONE_HUNDRED_IN_WAD\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"SECONDS_IN_YEAR_IN_WAD\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static FIXEDANDVARIABLEMATH_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60aa610038600b82828239805160001a607314602b57634e487b7160e01b600052600060045260246000fd5b30600052607381538281f3fe7300000000000000000000000000000000000000003014608060405260043610603d5760003560e01c8063652ec9bf146042578063cee7121f146063575b600080fd5b605168056bc75e2d6310000081565b60405190815260200160405180910390f35b60516a1a1601fc4ea7109e0000008156fea26469706673582212206caa5504cf529a7d11bda97bc9a4fd618881830ae2f63cf1fab6cef423bff5d764736f6c63430008090033" . parse () . expect ("invalid bytecode")
        });
    pub struct FixedAndVariableMath<M>(ethers::contract::Contract<M>);
    impl<M> Clone for FixedAndVariableMath<M> {
        fn clone(&self) -> Self {
            FixedAndVariableMath(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for FixedAndVariableMath<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for FixedAndVariableMath<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(FixedAndVariableMath))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> FixedAndVariableMath<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                FIXEDANDVARIABLEMATH_ABI.clone(),
                client,
            )
            .into()
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
                FIXEDANDVARIABLEMATH_ABI.clone(),
                FIXEDANDVARIABLEMATH_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `ONE_HUNDRED_IN_WAD` (0x652ec9bf) function"]
        pub fn one_hundred_in_wad(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([101, 46, 201, 191], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `SECONDS_IN_YEAR_IN_WAD` (0xcee7121f) function"]
        pub fn seconds_in_year_in_wad(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([206, 231, 18, 31], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for FixedAndVariableMath<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `ONE_HUNDRED_IN_WAD` function with signature `ONE_HUNDRED_IN_WAD()` and selector `[101, 46, 201, 191]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "ONE_HUNDRED_IN_WAD", abi = "ONE_HUNDRED_IN_WAD()")]
    pub struct OneHundredInWadCall;
    #[doc = "Container type for all input parameters for the `SECONDS_IN_YEAR_IN_WAD` function with signature `SECONDS_IN_YEAR_IN_WAD()` and selector `[206, 231, 18, 31]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "SECONDS_IN_YEAR_IN_WAD", abi = "SECONDS_IN_YEAR_IN_WAD()")]
    pub struct SecondsInYearInWadCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum FixedAndVariableMathCalls {
        OneHundredInWad(OneHundredInWadCall),
        SecondsInYearInWad(SecondsInYearInWadCall),
    }
    impl ethers::core::abi::AbiDecode for FixedAndVariableMathCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <OneHundredInWadCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FixedAndVariableMathCalls::OneHundredInWad(decoded));
            }
            if let Ok(decoded) =
                <SecondsInYearInWadCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FixedAndVariableMathCalls::SecondsInYearInWad(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for FixedAndVariableMathCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                FixedAndVariableMathCalls::OneHundredInWad(element) => element.encode(),
                FixedAndVariableMathCalls::SecondsInYearInWad(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for FixedAndVariableMathCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                FixedAndVariableMathCalls::OneHundredInWad(element) => element.fmt(f),
                FixedAndVariableMathCalls::SecondsInYearInWad(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<OneHundredInWadCall> for FixedAndVariableMathCalls {
        fn from(var: OneHundredInWadCall) -> Self {
            FixedAndVariableMathCalls::OneHundredInWad(var)
        }
    }
    impl ::std::convert::From<SecondsInYearInWadCall> for FixedAndVariableMathCalls {
        fn from(var: SecondsInYearInWadCall) -> Self {
            FixedAndVariableMathCalls::SecondsInYearInWad(var)
        }
    }
    #[doc = "Container type for all return fields from the `ONE_HUNDRED_IN_WAD` function with signature `ONE_HUNDRED_IN_WAD()` and selector `[101, 46, 201, 191]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct OneHundredInWadReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `SECONDS_IN_YEAR_IN_WAD` function with signature `SECONDS_IN_YEAR_IN_WAD()` and selector `[206, 231, 18, 31]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SecondsInYearInWadReturn(pub ethers::core::types::U256);
}
