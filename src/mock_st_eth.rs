pub use mock_st_eth::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod mock_st_eth {
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
    #[doc = "MockStEth was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static MOCKSTETH_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getFee\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"feeBasisPoints\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getInstantUpdates\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"sharesAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPooledEthByShares\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getlastUpdatedTimestamp\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"instantUpdates\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setInstantUpdates\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"lastUpdatedTimestamp\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLastUpdatedTimestamp\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"lastUpdatedTimestampManipulation\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLastUpdatedTimestampManipulation\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"sharesMultiplier\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setSharesMultiplierInRay\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MOCKSTETH_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60806040526000805534801561001457600080fd5b506002805461ffff191660011790556102b5806100326000396000f3fe608060405234801561001057600080fd5b50600436106100885760003560e01c806372228f9e1161005b57806372228f9e146101025780637a28fb8814610119578063ced72f871461012c578063faea0a1b1461013c57600080fd5b806303ac05e81461008d57806309471fa6146100a25780633c675687146100b9578063540810d0146100da575b600080fd5b6100a061009b3660046101ee565b610152565b005b6001545b6040519081526020015b60405180910390f35b6100a06100c7366004610207565b6002805460ff1916911515919091179055565b6100a06100e8366004610207565b600280549115156101000261ff0019909216919091179055565b6100a06101103660046101ee565b60005542600155565b6100a66101273660046101ee565b6101c1565b6040516103e881526020016100b0565b60025460ff1660405190151581526020016100b0565b600254610100900460ff166101bc5760405162461bcd60e51b815260206004820152602660248201527f456e61626c65206c617374207570646174656420626c6f636b206d616e6970756044820152653630ba34b7b760d11b606482015260840160405180910390fd5b600155565b60006b033b2e3c9fd0803ce8000000600054836101de9190610230565b6101e8919061025d565b92915050565b60006020828403121561020057600080fd5b5035919050565b60006020828403121561021957600080fd5b8135801515811461022957600080fd5b9392505050565b600081600019048311821515161561025857634e487b7160e01b600052601160045260246000fd5b500290565b60008261027a57634e487b7160e01b600052601260045260246000fd5b50049056fea26469706673582212200b0f4fd99b13fbab960b63ed47041849eeed2df8def8296b296392603a0bb1f964736f6c63430008090033" . parse () . expect ("invalid bytecode")
        });
    pub struct MockStEth<M>(ethers::contract::Contract<M>);
    impl<M> Clone for MockStEth<M> {
        fn clone(&self) -> Self {
            MockStEth(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for MockStEth<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for MockStEth<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MockStEth))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> MockStEth<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), MOCKSTETH_ABI.clone(), client).into()
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
                MOCKSTETH_ABI.clone(),
                MOCKSTETH_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `getFee` (0xced72f87) function"]
        pub fn get_fee(&self) -> ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([206, 215, 47, 135], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getInstantUpdates` (0xfaea0a1b) function"]
        pub fn get_instant_updates(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([250, 234, 10, 27], ())
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
        #[doc = "Calls the contract's `getlastUpdatedTimestamp` (0x09471fa6) function"]
        pub fn getlast_updated_timestamp(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([9, 71, 31, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setInstantUpdates` (0x3c675687) function"]
        pub fn set_instant_updates(
            &self,
            instant_updates: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 103, 86, 135], instant_updates)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLastUpdatedTimestamp` (0x03ac05e8) function"]
        pub fn set_last_updated_timestamp(
            &self,
            last_updated_timestamp: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([3, 172, 5, 232], last_updated_timestamp)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLastUpdatedTimestampManipulation` (0x540810d0) function"]
        pub fn set_last_updated_timestamp_manipulation(
            &self,
            last_updated_timestamp_manipulation: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([84, 8, 16, 208], last_updated_timestamp_manipulation)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setSharesMultiplierInRay` (0x72228f9e) function"]
        pub fn set_shares_multiplier_in_ray(
            &self,
            shares_multiplier: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([114, 34, 143, 158], shares_multiplier)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for MockStEth<M> {
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
    #[doc = "Container type for all input parameters for the `getInstantUpdates` function with signature `getInstantUpdates()` and selector `[250, 234, 10, 27]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getInstantUpdates", abi = "getInstantUpdates()")]
    pub struct GetInstantUpdatesCall;
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
    #[doc = "Container type for all input parameters for the `getlastUpdatedTimestamp` function with signature `getlastUpdatedTimestamp()` and selector `[9, 71, 31, 166]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getlastUpdatedTimestamp", abi = "getlastUpdatedTimestamp()")]
    pub struct GetlastUpdatedTimestampCall;
    #[doc = "Container type for all input parameters for the `setInstantUpdates` function with signature `setInstantUpdates(bool)` and selector `[60, 103, 86, 135]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setInstantUpdates", abi = "setInstantUpdates(bool)")]
    pub struct SetInstantUpdatesCall {
        pub instant_updates: bool,
    }
    #[doc = "Container type for all input parameters for the `setLastUpdatedTimestamp` function with signature `setLastUpdatedTimestamp(uint256)` and selector `[3, 172, 5, 232]`"]
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
        name = "setLastUpdatedTimestamp",
        abi = "setLastUpdatedTimestamp(uint256)"
    )]
    pub struct SetLastUpdatedTimestampCall {
        pub last_updated_timestamp: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setLastUpdatedTimestampManipulation` function with signature `setLastUpdatedTimestampManipulation(bool)` and selector `[84, 8, 16, 208]`"]
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
        name = "setLastUpdatedTimestampManipulation",
        abi = "setLastUpdatedTimestampManipulation(bool)"
    )]
    pub struct SetLastUpdatedTimestampManipulationCall {
        pub last_updated_timestamp_manipulation: bool,
    }
    #[doc = "Container type for all input parameters for the `setSharesMultiplierInRay` function with signature `setSharesMultiplierInRay(uint256)` and selector `[114, 34, 143, 158]`"]
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
        name = "setSharesMultiplierInRay",
        abi = "setSharesMultiplierInRay(uint256)"
    )]
    pub struct SetSharesMultiplierInRayCall {
        pub shares_multiplier: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MockStEthCalls {
        GetFee(GetFeeCall),
        GetInstantUpdates(GetInstantUpdatesCall),
        GetPooledEthByShares(GetPooledEthBySharesCall),
        GetlastUpdatedTimestamp(GetlastUpdatedTimestampCall),
        SetInstantUpdates(SetInstantUpdatesCall),
        SetLastUpdatedTimestamp(SetLastUpdatedTimestampCall),
        SetLastUpdatedTimestampManipulation(SetLastUpdatedTimestampManipulationCall),
        SetSharesMultiplierInRay(SetSharesMultiplierInRayCall),
    }
    impl ethers::core::abi::AbiDecode for MockStEthCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <GetFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockStEthCalls::GetFee(decoded));
            }
            if let Ok(decoded) =
                <GetInstantUpdatesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockStEthCalls::GetInstantUpdates(decoded));
            }
            if let Ok(decoded) =
                <GetPooledEthBySharesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockStEthCalls::GetPooledEthByShares(decoded));
            }
            if let Ok(decoded) =
                <GetlastUpdatedTimestampCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockStEthCalls::GetlastUpdatedTimestamp(decoded));
            }
            if let Ok(decoded) =
                <SetInstantUpdatesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockStEthCalls::SetInstantUpdates(decoded));
            }
            if let Ok(decoded) =
                <SetLastUpdatedTimestampCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockStEthCalls::SetLastUpdatedTimestamp(decoded));
            }
            if let Ok(decoded) =
                <SetLastUpdatedTimestampManipulationCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MockStEthCalls::SetLastUpdatedTimestampManipulation(decoded));
            }
            if let Ok(decoded) =
                <SetSharesMultiplierInRayCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MockStEthCalls::SetSharesMultiplierInRay(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MockStEthCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MockStEthCalls::GetFee(element) => element.encode(),
                MockStEthCalls::GetInstantUpdates(element) => element.encode(),
                MockStEthCalls::GetPooledEthByShares(element) => element.encode(),
                MockStEthCalls::GetlastUpdatedTimestamp(element) => element.encode(),
                MockStEthCalls::SetInstantUpdates(element) => element.encode(),
                MockStEthCalls::SetLastUpdatedTimestamp(element) => element.encode(),
                MockStEthCalls::SetLastUpdatedTimestampManipulation(element) => element.encode(),
                MockStEthCalls::SetSharesMultiplierInRay(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MockStEthCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MockStEthCalls::GetFee(element) => element.fmt(f),
                MockStEthCalls::GetInstantUpdates(element) => element.fmt(f),
                MockStEthCalls::GetPooledEthByShares(element) => element.fmt(f),
                MockStEthCalls::GetlastUpdatedTimestamp(element) => element.fmt(f),
                MockStEthCalls::SetInstantUpdates(element) => element.fmt(f),
                MockStEthCalls::SetLastUpdatedTimestamp(element) => element.fmt(f),
                MockStEthCalls::SetLastUpdatedTimestampManipulation(element) => element.fmt(f),
                MockStEthCalls::SetSharesMultiplierInRay(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetFeeCall> for MockStEthCalls {
        fn from(var: GetFeeCall) -> Self {
            MockStEthCalls::GetFee(var)
        }
    }
    impl ::std::convert::From<GetInstantUpdatesCall> for MockStEthCalls {
        fn from(var: GetInstantUpdatesCall) -> Self {
            MockStEthCalls::GetInstantUpdates(var)
        }
    }
    impl ::std::convert::From<GetPooledEthBySharesCall> for MockStEthCalls {
        fn from(var: GetPooledEthBySharesCall) -> Self {
            MockStEthCalls::GetPooledEthByShares(var)
        }
    }
    impl ::std::convert::From<GetlastUpdatedTimestampCall> for MockStEthCalls {
        fn from(var: GetlastUpdatedTimestampCall) -> Self {
            MockStEthCalls::GetlastUpdatedTimestamp(var)
        }
    }
    impl ::std::convert::From<SetInstantUpdatesCall> for MockStEthCalls {
        fn from(var: SetInstantUpdatesCall) -> Self {
            MockStEthCalls::SetInstantUpdates(var)
        }
    }
    impl ::std::convert::From<SetLastUpdatedTimestampCall> for MockStEthCalls {
        fn from(var: SetLastUpdatedTimestampCall) -> Self {
            MockStEthCalls::SetLastUpdatedTimestamp(var)
        }
    }
    impl ::std::convert::From<SetLastUpdatedTimestampManipulationCall> for MockStEthCalls {
        fn from(var: SetLastUpdatedTimestampManipulationCall) -> Self {
            MockStEthCalls::SetLastUpdatedTimestampManipulation(var)
        }
    }
    impl ::std::convert::From<SetSharesMultiplierInRayCall> for MockStEthCalls {
        fn from(var: SetSharesMultiplierInRayCall) -> Self {
            MockStEthCalls::SetSharesMultiplierInRay(var)
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
    #[doc = "Container type for all return fields from the `getInstantUpdates` function with signature `getInstantUpdates()` and selector `[250, 234, 10, 27]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetInstantUpdatesReturn(pub bool);
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
    #[doc = "Container type for all return fields from the `getlastUpdatedTimestamp` function with signature `getlastUpdatedTimestamp()` and selector `[9, 71, 31, 166]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetlastUpdatedTimestampReturn(pub ethers::core::types::U256);
}
