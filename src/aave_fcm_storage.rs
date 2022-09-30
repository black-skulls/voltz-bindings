pub use aave_fcm_storage::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod aave_fcm_storage {
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
    #[doc = "AaveFCMStorage was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static AAVEFCMSTORAGE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"paused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"traders\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"marginInScaledYieldBearingTokens\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenBalance\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"variableTokenBalance\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isSettled\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"underlyingToken\",\"outputs\":[{\"internalType\":\"contract IERC20Minimal\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static AAVEFCMSTORAGE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b5061015c806100206000396000f3fe608060405234801561001057600080fd5b50600436106100415760003560e01c80632495a599146100465780635c975abb1461007657806392a88fa21461009a575b600080fd5b600454610059906001600160a01b031681565b6040516001600160a01b0390911681526020015b60405180910390f35b60045461008a90600160a01b900460ff1681565b604051901515815260200161006d565b6100d46100a83660046100f6565b600360208190526000918252604090912080546001820154600283015492909301549092919060ff1684565b604080519485526020850193909352918301521515606082015260800161006d565b60006020828403121561010857600080fd5b81356001600160a01b038116811461011f57600080fd5b939250505056fea26469706673582212207b4ac280680953fcd44a984ac2d8208f387c4c12e3f4102183e305b6b358eeb064736f6c63430008090033" . parse () . expect ("invalid bytecode")
        });
    pub struct AaveFCMStorage<M>(ethers::contract::Contract<M>);
    impl<M> Clone for AaveFCMStorage<M> {
        fn clone(&self) -> Self {
            AaveFCMStorage(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for AaveFCMStorage<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for AaveFCMStorage<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(AaveFCMStorage))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> AaveFCMStorage<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), AAVEFCMSTORAGE_ABI.clone(), client)
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
                AAVEFCMSTORAGE_ABI.clone(),
                AAVEFCMSTORAGE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `paused` (0x5c975abb) function"]
        pub fn paused(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([92, 151, 90, 187], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `traders` (0x92a88fa2) function"]
        pub fn traders(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, I256, I256, bool),
        > {
            self.0
                .method_hash([146, 168, 143, 162], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `underlyingToken` (0x2495a599) function"]
        pub fn underlying_token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([36, 149, 165, 153], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for AaveFCMStorage<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `paused` function with signature `paused()` and selector `[92, 151, 90, 187]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "paused", abi = "paused()")]
    pub struct PausedCall;
    #[doc = "Container type for all input parameters for the `traders` function with signature `traders(address)` and selector `[146, 168, 143, 162]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "traders", abi = "traders(address)")]
    pub struct TradersCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `underlyingToken` function with signature `underlyingToken()` and selector `[36, 149, 165, 153]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "underlyingToken", abi = "underlyingToken()")]
    pub struct UnderlyingTokenCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum AaveFCMStorageCalls {
        Paused(PausedCall),
        Traders(TradersCall),
        UnderlyingToken(UnderlyingTokenCall),
    }
    impl ethers::core::abi::AbiDecode for AaveFCMStorageCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <PausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveFCMStorageCalls::Paused(decoded));
            }
            if let Ok(decoded) =
                <TradersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveFCMStorageCalls::Traders(decoded));
            }
            if let Ok(decoded) =
                <UnderlyingTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveFCMStorageCalls::UnderlyingToken(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for AaveFCMStorageCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                AaveFCMStorageCalls::Paused(element) => element.encode(),
                AaveFCMStorageCalls::Traders(element) => element.encode(),
                AaveFCMStorageCalls::UnderlyingToken(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for AaveFCMStorageCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                AaveFCMStorageCalls::Paused(element) => element.fmt(f),
                AaveFCMStorageCalls::Traders(element) => element.fmt(f),
                AaveFCMStorageCalls::UnderlyingToken(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<PausedCall> for AaveFCMStorageCalls {
        fn from(var: PausedCall) -> Self {
            AaveFCMStorageCalls::Paused(var)
        }
    }
    impl ::std::convert::From<TradersCall> for AaveFCMStorageCalls {
        fn from(var: TradersCall) -> Self {
            AaveFCMStorageCalls::Traders(var)
        }
    }
    impl ::std::convert::From<UnderlyingTokenCall> for AaveFCMStorageCalls {
        fn from(var: UnderlyingTokenCall) -> Self {
            AaveFCMStorageCalls::UnderlyingToken(var)
        }
    }
    #[doc = "Container type for all return fields from the `paused` function with signature `paused()` and selector `[92, 151, 90, 187]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PausedReturn(pub bool);
    #[doc = "Container type for all return fields from the `traders` function with signature `traders(address)` and selector `[146, 168, 143, 162]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TradersReturn {
        pub margin_in_scaled_yield_bearing_tokens: ethers::core::types::U256,
        pub fixed_token_balance: I256,
        pub variable_token_balance: I256,
        pub is_settled: bool,
    }
    #[doc = "Container type for all return fields from the `underlyingToken` function with signature `underlyingToken()` and selector `[36, 149, 165, 153]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct UnderlyingTokenReturn(pub ethers::core::types::Address);
}
