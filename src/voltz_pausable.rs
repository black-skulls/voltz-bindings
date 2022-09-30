pub use voltz_pausable::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod voltz_pausable {
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
    #[doc = "VoltzPausable was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static VOLTZPAUSABLE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"version\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Initialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"VOLTZ_PAUSER\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"permission\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"changePauser\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"paused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"state\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPausability\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static VOLTZPAUSABLE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b506103d6806100206000396000f3fe608060405234801561001057600080fd5b506004361061007d5760003560e01c80635c975abb1161005b5780635c975abb146100e4578063715018a6146101015780638da5cb5b14610109578063f2fde38b1461012457600080fd5b80630d211954146100825780631b398e0614610097578063478de324146100d1575b600080fd5b610095610090366004610319565b610137565b005b6100be7fc02e2d3f2633adb184196f6ae17c8476d7912f8727b7c1cc7da0b7ddac86bc6581565b6040519081526020015b60405180910390f35b6100956100df366004610352565b610198565b6066546100f19060ff1681565b60405190151581526020016100c8565b6100956101cb565b6033546040516001600160a01b0390911681526020016100c8565b610095610132366004610385565b6101df565b3360009081526065602052604090205460ff166101855760405162461bcd60e51b81526020600482015260076024820152666e6f20726f6c6560c81b60448201526064015b60405180910390fd5b6066805460ff1916911515919091179055565b6101a0610258565b6001600160a01b03919091166000908152606560205260409020805460ff1916911515919091179055565b6101d3610258565b6101dd60006102b2565b565b6101e7610258565b6001600160a01b03811661024c5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b606482015260840161017c565b610255816102b2565b50565b6033546001600160a01b031633146101dd5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e6572604482015260640161017c565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b8035801515811461031457600080fd5b919050565b60006020828403121561032b57600080fd5b61033482610304565b9392505050565b80356001600160a01b038116811461031457600080fd5b6000806040838503121561036557600080fd5b61036e8361033b565b915061037c60208401610304565b90509250929050565b60006020828403121561039757600080fd5b6103348261033b56fea264697066735822122024c7e633d6d91e0ec11b2b9987a09d9befb4519d09760bad4be1ddbcc70131b564736f6c63430008090033" . parse () . expect ("invalid bytecode")
        });
    pub struct VoltzPausable<M>(ethers::contract::Contract<M>);
    impl<M> Clone for VoltzPausable<M> {
        fn clone(&self) -> Self {
            VoltzPausable(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for VoltzPausable<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for VoltzPausable<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(VoltzPausable))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> VoltzPausable<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), VOLTZPAUSABLE_ABI.clone(), client)
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
                VOLTZPAUSABLE_ABI.clone(),
                VOLTZPAUSABLE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `VOLTZ_PAUSER` (0x1b398e06) function"]
        pub fn voltz_pauser(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([27, 57, 142, 6], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `changePauser` (0x478de324) function"]
        pub fn change_pauser(
            &self,
            account: ethers::core::types::Address,
            permission: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([71, 141, 227, 36], (account, permission))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `paused` (0x5c975abb) function"]
        pub fn paused(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([92, 151, 90, 187], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPausability` (0x0d211954) function"]
        pub fn set_pausability(
            &self,
            state: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 33, 25, 84], state)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(
            &self,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Initialized` event"]
        pub fn initialized_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, InitializedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, VoltzPausableEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for VoltzPausable<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "Initialized", abi = "Initialized(uint8)")]
    pub struct InitializedFilter {
        pub version: u8,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum VoltzPausableEvents {
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ethers::contract::EthLogDecode for VoltzPausableEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(VoltzPausableEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(VoltzPausableEvents::OwnershipTransferredFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for VoltzPausableEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                VoltzPausableEvents::InitializedFilter(element) => element.fmt(f),
                VoltzPausableEvents::OwnershipTransferredFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `VOLTZ_PAUSER` function with signature `VOLTZ_PAUSER()` and selector `[27, 57, 142, 6]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "VOLTZ_PAUSER", abi = "VOLTZ_PAUSER()")]
    pub struct VoltzPauserCall;
    #[doc = "Container type for all input parameters for the `changePauser` function with signature `changePauser(address,bool)` and selector `[71, 141, 227, 36]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "changePauser", abi = "changePauser(address,bool)")]
    pub struct ChangePauserCall {
        pub account: ethers::core::types::Address,
        pub permission: bool,
    }
    #[doc = "Container type for all input parameters for the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
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
    #[doc = "Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `[113, 80, 24, 166]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    #[doc = "Container type for all input parameters for the `setPausability` function with signature `setPausability(bool)` and selector `[13, 33, 25, 84]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setPausability", abi = "setPausability(bool)")]
    pub struct SetPausabilityCall {
        pub state: bool,
    }
    #[doc = "Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `[242, 253, 227, 139]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum VoltzPausableCalls {
        VoltzPauser(VoltzPauserCall),
        ChangePauser(ChangePauserCall),
        Owner(OwnerCall),
        Paused(PausedCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetPausability(SetPausabilityCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ethers::core::abi::AbiDecode for VoltzPausableCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <VoltzPauserCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VoltzPausableCalls::VoltzPauser(decoded));
            }
            if let Ok(decoded) =
                <ChangePauserCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VoltzPausableCalls::ChangePauser(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VoltzPausableCalls::Owner(decoded));
            }
            if let Ok(decoded) = <PausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VoltzPausableCalls::Paused(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VoltzPausableCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <SetPausabilityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VoltzPausableCalls::SetPausability(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VoltzPausableCalls::TransferOwnership(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for VoltzPausableCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                VoltzPausableCalls::VoltzPauser(element) => element.encode(),
                VoltzPausableCalls::ChangePauser(element) => element.encode(),
                VoltzPausableCalls::Owner(element) => element.encode(),
                VoltzPausableCalls::Paused(element) => element.encode(),
                VoltzPausableCalls::RenounceOwnership(element) => element.encode(),
                VoltzPausableCalls::SetPausability(element) => element.encode(),
                VoltzPausableCalls::TransferOwnership(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for VoltzPausableCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                VoltzPausableCalls::VoltzPauser(element) => element.fmt(f),
                VoltzPausableCalls::ChangePauser(element) => element.fmt(f),
                VoltzPausableCalls::Owner(element) => element.fmt(f),
                VoltzPausableCalls::Paused(element) => element.fmt(f),
                VoltzPausableCalls::RenounceOwnership(element) => element.fmt(f),
                VoltzPausableCalls::SetPausability(element) => element.fmt(f),
                VoltzPausableCalls::TransferOwnership(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<VoltzPauserCall> for VoltzPausableCalls {
        fn from(var: VoltzPauserCall) -> Self {
            VoltzPausableCalls::VoltzPauser(var)
        }
    }
    impl ::std::convert::From<ChangePauserCall> for VoltzPausableCalls {
        fn from(var: ChangePauserCall) -> Self {
            VoltzPausableCalls::ChangePauser(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for VoltzPausableCalls {
        fn from(var: OwnerCall) -> Self {
            VoltzPausableCalls::Owner(var)
        }
    }
    impl ::std::convert::From<PausedCall> for VoltzPausableCalls {
        fn from(var: PausedCall) -> Self {
            VoltzPausableCalls::Paused(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for VoltzPausableCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            VoltzPausableCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<SetPausabilityCall> for VoltzPausableCalls {
        fn from(var: SetPausabilityCall) -> Self {
            VoltzPausableCalls::SetPausability(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for VoltzPausableCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            VoltzPausableCalls::TransferOwnership(var)
        }
    }
    #[doc = "Container type for all return fields from the `VOLTZ_PAUSER` function with signature `VOLTZ_PAUSER()` and selector `[27, 57, 142, 6]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct VoltzPauserReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct OwnerReturn(pub ethers::core::types::Address);
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
}
