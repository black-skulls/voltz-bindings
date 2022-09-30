pub use mock_rocket_network_balances::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod mock_rocket_network_balances {
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
    #[doc = "MockRocketNetworkBalances was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static MOCKROCKETNETWORKBALANCES_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"contract MockRocketEth\",\"name\":\"mockRocketEth\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_mockRocketEth\",\"outputs\":[{\"internalType\":\"contract MockRocketEth\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getBalancesBlock\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MOCKROCKETNETWORKBALANCES_BYTECODE: ethers::contract::Lazy<
        ethers::core::types::Bytes,
    > = ethers::contract::Lazy::new(|| {
        "0x608060405234801561001057600080fd5b506040516101ef3803806101ef83398101604081905261002f91610054565b600080546001600160a01b0319166001600160a01b0392909216919091179055610084565b60006020828403121561006657600080fd5b81516001600160a01b038116811461007d57600080fd5b9392505050565b61015c806100936000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c80634cff6e041461003b5780639100c13d1461006b575b600080fd5b60005461004e906001600160a01b031681565b6040516001600160a01b0390911681526020015b60405180910390f35b610073610081565b604051908152602001610062565b60008060009054906101000a90046001600160a01b03166001600160a01b03166387c891bd6040518163ffffffff1660e01b815260040160206040518083038186803b1580156100d057600080fd5b505afa1580156100e4573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610108919061010d565b905090565b60006020828403121561011f57600080fd5b505191905056fea2646970667358221220ad05a28dbab0f20c080c563f1fbfff8c2916cb91bbd568780f695fd3e999889f64736f6c63430008090033" . parse () . expect ("invalid bytecode")
    });
    pub struct MockRocketNetworkBalances<M>(ethers::contract::Contract<M>);
    impl<M> Clone for MockRocketNetworkBalances<M> {
        fn clone(&self) -> Self {
            MockRocketNetworkBalances(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for MockRocketNetworkBalances<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for MockRocketNetworkBalances<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MockRocketNetworkBalances))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> MockRocketNetworkBalances<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                MOCKROCKETNETWORKBALANCES_ABI.clone(),
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
                MOCKROCKETNETWORKBALANCES_ABI.clone(),
                MOCKROCKETNETWORKBALANCES_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `_mockRocketEth` (0x4cff6e04) function"]
        pub fn mock_rocket_eth(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([76, 255, 110, 4], ())
                .expect("method not found (this should never happen)")
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
        for MockRocketNetworkBalances<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `_mockRocketEth` function with signature `_mockRocketEth()` and selector `[76, 255, 110, 4]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "_mockRocketEth", abi = "_mockRocketEth()")]
    pub struct MockRocketEthCall;
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MockRocketNetworkBalancesCalls {
        MockRocketEth(MockRocketEthCall),
        GetBalancesBlock(GetBalancesBlockCall),
    }
    impl ethers::core::abi::AbiDecode for MockRocketNetworkBalancesCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <MockRocketEthCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockRocketNetworkBalancesCalls::MockRocketEth(decoded));
            }
            if let Ok(decoded) =
                <GetBalancesBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockRocketNetworkBalancesCalls::GetBalancesBlock(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MockRocketNetworkBalancesCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MockRocketNetworkBalancesCalls::MockRocketEth(element) => element.encode(),
                MockRocketNetworkBalancesCalls::GetBalancesBlock(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MockRocketNetworkBalancesCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MockRocketNetworkBalancesCalls::MockRocketEth(element) => element.fmt(f),
                MockRocketNetworkBalancesCalls::GetBalancesBlock(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<MockRocketEthCall> for MockRocketNetworkBalancesCalls {
        fn from(var: MockRocketEthCall) -> Self {
            MockRocketNetworkBalancesCalls::MockRocketEth(var)
        }
    }
    impl ::std::convert::From<GetBalancesBlockCall> for MockRocketNetworkBalancesCalls {
        fn from(var: GetBalancesBlockCall) -> Self {
            MockRocketNetworkBalancesCalls::GetBalancesBlock(var)
        }
    }
    #[doc = "Container type for all return fields from the `_mockRocketEth` function with signature `_mockRocketEth()` and selector `[76, 255, 110, 4]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MockRocketEthReturn(pub ethers::core::types::Address);
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
