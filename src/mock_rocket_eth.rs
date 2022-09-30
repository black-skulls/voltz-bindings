pub use mock_rocket_eth::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod mock_rocket_eth {
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
    #[doc = "MockRocketEth was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static MOCKROCKETETH_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"rethAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getEthValue\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLastUpdatedBlock\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"instantUpdates\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setInstantUpdates\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"lastUpdatedBlock\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLastUpdatedBlock\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"lastUpdatedBlockManipulation\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLastUpdatedBlockManipulation\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"rethMultiplier\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setRethMultiplierInRay\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MOCKROCKETETH_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60806040526000805534801561001457600080fd5b506002805461ffff1916600117905561029e806100326000396000f3fe608060405234801561001057600080fd5b50600436106100625760003560e01c806327e7b263146100675780633c6756871461007c57806387c891bd1461009d5780638b32fa23146100b75780639ac60cd2146100ca578063ea764620146100f2575b600080fd5b61007a6100753660046101d7565b610105565b005b61007a61008a3660046101f0565b6002805460ff1916911515919091179055565b6100a5610174565b60405190815260200160405180910390f35b6100a56100c53660046101d7565b61018e565b61007a6100d83660046101f0565b600280549115156101000261ff0019909216919091179055565b61007a6101003660046101d7565b6101bb565b600254610100900460ff1661016f5760405162461bcd60e51b815260206004820152602660248201527f456e61626c65206c617374207570646174656420626c6f636b206d616e6970756044820152653630ba34b7b760d11b606482015260840160405180910390fd5b600155565b60025460009060ff161561018757504390565b5060015490565b60006b033b2e3c9fd0803ce8000000600054836101ab9190610219565b6101b59190610246565b92915050565b6000819055600254610100900460ff166101d457436001555b50565b6000602082840312156101e957600080fd5b5035919050565b60006020828403121561020257600080fd5b8135801515811461021257600080fd5b9392505050565b600081600019048311821515161561024157634e487b7160e01b600052601160045260246000fd5b500290565b60008261026357634e487b7160e01b600052601260045260246000fd5b50049056fea2646970667358221220bde2e89c5507babf75d763e9a8e32b23f9e99eb716a82eb1c687165f254c81fb64736f6c63430008090033" . parse () . expect ("invalid bytecode")
        });
    pub struct MockRocketEth<M>(ethers::contract::Contract<M>);
    impl<M> Clone for MockRocketEth<M> {
        fn clone(&self) -> Self {
            MockRocketEth(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for MockRocketEth<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for MockRocketEth<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MockRocketEth))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> MockRocketEth<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), MOCKROCKETETH_ABI.clone(), client)
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
                MOCKROCKETETH_ABI.clone(),
                MOCKROCKETETH_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
        #[doc = "Calls the contract's `getLastUpdatedBlock` (0x87c891bd) function"]
        pub fn get_last_updated_block(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([135, 200, 145, 189], ())
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
        #[doc = "Calls the contract's `setLastUpdatedBlock` (0x27e7b263) function"]
        pub fn set_last_updated_block(
            &self,
            last_updated_block: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([39, 231, 178, 99], last_updated_block)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLastUpdatedBlockManipulation` (0x9ac60cd2) function"]
        pub fn set_last_updated_block_manipulation(
            &self,
            last_updated_block_manipulation: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([154, 198, 12, 210], last_updated_block_manipulation)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setRethMultiplierInRay` (0xea764620) function"]
        pub fn set_reth_multiplier_in_ray(
            &self,
            reth_multiplier: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([234, 118, 70, 32], reth_multiplier)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for MockRocketEth<M> {
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
    #[doc = "Container type for all input parameters for the `getLastUpdatedBlock` function with signature `getLastUpdatedBlock()` and selector `[135, 200, 145, 189]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getLastUpdatedBlock", abi = "getLastUpdatedBlock()")]
    pub struct GetLastUpdatedBlockCall;
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
    #[doc = "Container type for all input parameters for the `setLastUpdatedBlock` function with signature `setLastUpdatedBlock(uint256)` and selector `[39, 231, 178, 99]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setLastUpdatedBlock", abi = "setLastUpdatedBlock(uint256)")]
    pub struct SetLastUpdatedBlockCall {
        pub last_updated_block: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setLastUpdatedBlockManipulation` function with signature `setLastUpdatedBlockManipulation(bool)` and selector `[154, 198, 12, 210]`"]
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
        name = "setLastUpdatedBlockManipulation",
        abi = "setLastUpdatedBlockManipulation(bool)"
    )]
    pub struct SetLastUpdatedBlockManipulationCall {
        pub last_updated_block_manipulation: bool,
    }
    #[doc = "Container type for all input parameters for the `setRethMultiplierInRay` function with signature `setRethMultiplierInRay(uint256)` and selector `[234, 118, 70, 32]`"]
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
        name = "setRethMultiplierInRay",
        abi = "setRethMultiplierInRay(uint256)"
    )]
    pub struct SetRethMultiplierInRayCall {
        pub reth_multiplier: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MockRocketEthCalls {
        GetEthValue(GetEthValueCall),
        GetLastUpdatedBlock(GetLastUpdatedBlockCall),
        SetInstantUpdates(SetInstantUpdatesCall),
        SetLastUpdatedBlock(SetLastUpdatedBlockCall),
        SetLastUpdatedBlockManipulation(SetLastUpdatedBlockManipulationCall),
        SetRethMultiplierInRay(SetRethMultiplierInRayCall),
    }
    impl ethers::core::abi::AbiDecode for MockRocketEthCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetEthValueCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockRocketEthCalls::GetEthValue(decoded));
            }
            if let Ok(decoded) =
                <GetLastUpdatedBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockRocketEthCalls::GetLastUpdatedBlock(decoded));
            }
            if let Ok(decoded) =
                <SetInstantUpdatesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockRocketEthCalls::SetInstantUpdates(decoded));
            }
            if let Ok(decoded) =
                <SetLastUpdatedBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockRocketEthCalls::SetLastUpdatedBlock(decoded));
            }
            if let Ok(decoded) =
                <SetLastUpdatedBlockManipulationCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MockRocketEthCalls::SetLastUpdatedBlockManipulation(decoded));
            }
            if let Ok(decoded) =
                <SetRethMultiplierInRayCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockRocketEthCalls::SetRethMultiplierInRay(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MockRocketEthCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MockRocketEthCalls::GetEthValue(element) => element.encode(),
                MockRocketEthCalls::GetLastUpdatedBlock(element) => element.encode(),
                MockRocketEthCalls::SetInstantUpdates(element) => element.encode(),
                MockRocketEthCalls::SetLastUpdatedBlock(element) => element.encode(),
                MockRocketEthCalls::SetLastUpdatedBlockManipulation(element) => element.encode(),
                MockRocketEthCalls::SetRethMultiplierInRay(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MockRocketEthCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MockRocketEthCalls::GetEthValue(element) => element.fmt(f),
                MockRocketEthCalls::GetLastUpdatedBlock(element) => element.fmt(f),
                MockRocketEthCalls::SetInstantUpdates(element) => element.fmt(f),
                MockRocketEthCalls::SetLastUpdatedBlock(element) => element.fmt(f),
                MockRocketEthCalls::SetLastUpdatedBlockManipulation(element) => element.fmt(f),
                MockRocketEthCalls::SetRethMultiplierInRay(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetEthValueCall> for MockRocketEthCalls {
        fn from(var: GetEthValueCall) -> Self {
            MockRocketEthCalls::GetEthValue(var)
        }
    }
    impl ::std::convert::From<GetLastUpdatedBlockCall> for MockRocketEthCalls {
        fn from(var: GetLastUpdatedBlockCall) -> Self {
            MockRocketEthCalls::GetLastUpdatedBlock(var)
        }
    }
    impl ::std::convert::From<SetInstantUpdatesCall> for MockRocketEthCalls {
        fn from(var: SetInstantUpdatesCall) -> Self {
            MockRocketEthCalls::SetInstantUpdates(var)
        }
    }
    impl ::std::convert::From<SetLastUpdatedBlockCall> for MockRocketEthCalls {
        fn from(var: SetLastUpdatedBlockCall) -> Self {
            MockRocketEthCalls::SetLastUpdatedBlock(var)
        }
    }
    impl ::std::convert::From<SetLastUpdatedBlockManipulationCall> for MockRocketEthCalls {
        fn from(var: SetLastUpdatedBlockManipulationCall) -> Self {
            MockRocketEthCalls::SetLastUpdatedBlockManipulation(var)
        }
    }
    impl ::std::convert::From<SetRethMultiplierInRayCall> for MockRocketEthCalls {
        fn from(var: SetRethMultiplierInRayCall) -> Self {
            MockRocketEthCalls::SetRethMultiplierInRay(var)
        }
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
    #[doc = "Container type for all return fields from the `getLastUpdatedBlock` function with signature `getLastUpdatedBlock()` and selector `[135, 200, 145, 189]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetLastUpdatedBlockReturn(pub ethers::core::types::U256);
}
