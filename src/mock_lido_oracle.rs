pub use mock_lido_oracle::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod mock_lido_oracle {
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
    #[doc = "MockLidoOracle was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static MOCKLIDOORACLE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"contract MockStEth\",\"name\":\"_mockStEth\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getBeaconSpec\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"epochsPerFrame\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"slotsPerEpoch\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"secondsPerSlot\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"genesisTime\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCurrentFrame\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"frameEpochId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"frameStartTime\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"frameEndTime\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLastCompletedEpochId\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLastCompletedReportDelta\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"postTotalPooledEther\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"preTotalPooledEther\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"timeElapsed\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"mockStEth\",\"outputs\":[{\"internalType\":\"contract MockStEth\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MOCKLIDOORACLE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60806040526b033b2e3c9fd0803ce800000060005534801561002057600080fd5b5060405161041338038061041383398101604081905261003f91610064565b600180546001600160a01b0319166001600160a01b0392909216919091179055610094565b60006020828403121561007657600080fd5b81516001600160a01b038116811461008d57600080fd5b9392505050565b610370806100a36000396000f3fe608060405234801561001057600080fd5b50600436106100575760003560e01c8063534649c41461005c57806372f79b131461009557806376ea14d41461009d57806389896aef146100c8578063e547c77c146100de575b600080fd5b680579a814e10a74000068056bc75e2d63100000620151805b604080519384526020840192909252908201526060015b60405180910390f35b610075610104565b6001546100b0906001600160a01b031681565b6040516001600160a01b03909116815260200161008c565b6100d06101ae565b60405190815260200161008c565b60408051600180825260208201819052918101919091526000606082015260800161008c565b600080600080600160009054906101000a90046001600160a01b03166001600160a01b03166309471fa66040518163ffffffff1660e01b815260040160206040518083038186803b15801561015857600080fd5b505afa15801561016c573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061019091906102d2565b90506000816101a281620151806102eb565b93509350935050909192565b600080600160009054906101000a90046001600160a01b03166001600160a01b031663faea0a1b6040518163ffffffff1660e01b815260040160206040518083038186803b1580156101ff57600080fd5b505afa158015610213573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906102379190610311565b90508015610246574291505090565b600160009054906101000a90046001600160a01b03166001600160a01b03166309471fa66040518163ffffffff1660e01b815260040160206040518083038186803b15801561029457600080fd5b505afa1580156102a8573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906102cc91906102d2565b91505090565b6000602082840312156102e457600080fd5b5051919050565b6000821982111561030c57634e487b7160e01b600052601160045260246000fd5b500190565b60006020828403121561032357600080fd5b8151801515811461033357600080fd5b939250505056fea2646970667358221220efc59b78c7187aab8611af66fbd66c86627e5c2df06b1064a2bb2408f081dce264736f6c63430008090033" . parse () . expect ("invalid bytecode")
        });
    pub struct MockLidoOracle<M>(ethers::contract::Contract<M>);
    impl<M> Clone for MockLidoOracle<M> {
        fn clone(&self) -> Self {
            MockLidoOracle(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for MockLidoOracle<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for MockLidoOracle<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MockLidoOracle))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> MockLidoOracle<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), MOCKLIDOORACLE_ABI.clone(), client)
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
                MOCKLIDOORACLE_ABI.clone(),
                MOCKLIDOORACLE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `getBeaconSpec` (0xe547c77c) function"]
        pub fn get_beacon_spec(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, (u64, u64, u64, u64)> {
            self.0
                .method_hash([229, 71, 199, 124], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCurrentFrame` (0x72f79b13) function"]
        pub fn get_current_frame(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([114, 247, 155, 19], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLastCompletedEpochId` (0x89896aef) function"]
        pub fn get_last_completed_epoch_id(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([137, 137, 106, 239], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLastCompletedReportDelta` (0x534649c4) function"]
        pub fn get_last_completed_report_delta(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([83, 70, 73, 196], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mockStEth` (0x76ea14d4) function"]
        pub fn mock_st_eth(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([118, 234, 20, 212], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for MockLidoOracle<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `getBeaconSpec` function with signature `getBeaconSpec()` and selector `[229, 71, 199, 124]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getBeaconSpec", abi = "getBeaconSpec()")]
    pub struct GetBeaconSpecCall;
    #[doc = "Container type for all input parameters for the `getCurrentFrame` function with signature `getCurrentFrame()` and selector `[114, 247, 155, 19]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getCurrentFrame", abi = "getCurrentFrame()")]
    pub struct GetCurrentFrameCall;
    #[doc = "Container type for all input parameters for the `getLastCompletedEpochId` function with signature `getLastCompletedEpochId()` and selector `[137, 137, 106, 239]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getLastCompletedEpochId", abi = "getLastCompletedEpochId()")]
    pub struct GetLastCompletedEpochIdCall;
    #[doc = "Container type for all input parameters for the `getLastCompletedReportDelta` function with signature `getLastCompletedReportDelta()` and selector `[83, 70, 73, 196]`"]
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
        name = "getLastCompletedReportDelta",
        abi = "getLastCompletedReportDelta()"
    )]
    pub struct GetLastCompletedReportDeltaCall;
    #[doc = "Container type for all input parameters for the `mockStEth` function with signature `mockStEth()` and selector `[118, 234, 20, 212]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "mockStEth", abi = "mockStEth()")]
    pub struct MockStEthCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MockLidoOracleCalls {
        GetBeaconSpec(GetBeaconSpecCall),
        GetCurrentFrame(GetCurrentFrameCall),
        GetLastCompletedEpochId(GetLastCompletedEpochIdCall),
        GetLastCompletedReportDelta(GetLastCompletedReportDeltaCall),
        MockStEth(MockStEthCall),
    }
    impl ethers::core::abi::AbiDecode for MockLidoOracleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetBeaconSpecCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockLidoOracleCalls::GetBeaconSpec(decoded));
            }
            if let Ok(decoded) =
                <GetCurrentFrameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockLidoOracleCalls::GetCurrentFrame(decoded));
            }
            if let Ok(decoded) =
                <GetLastCompletedEpochIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockLidoOracleCalls::GetLastCompletedEpochId(decoded));
            }
            if let Ok(decoded) =
                <GetLastCompletedReportDeltaCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MockLidoOracleCalls::GetLastCompletedReportDelta(decoded));
            }
            if let Ok(decoded) =
                <MockStEthCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockLidoOracleCalls::MockStEth(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MockLidoOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MockLidoOracleCalls::GetBeaconSpec(element) => element.encode(),
                MockLidoOracleCalls::GetCurrentFrame(element) => element.encode(),
                MockLidoOracleCalls::GetLastCompletedEpochId(element) => element.encode(),
                MockLidoOracleCalls::GetLastCompletedReportDelta(element) => element.encode(),
                MockLidoOracleCalls::MockStEth(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MockLidoOracleCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MockLidoOracleCalls::GetBeaconSpec(element) => element.fmt(f),
                MockLidoOracleCalls::GetCurrentFrame(element) => element.fmt(f),
                MockLidoOracleCalls::GetLastCompletedEpochId(element) => element.fmt(f),
                MockLidoOracleCalls::GetLastCompletedReportDelta(element) => element.fmt(f),
                MockLidoOracleCalls::MockStEth(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetBeaconSpecCall> for MockLidoOracleCalls {
        fn from(var: GetBeaconSpecCall) -> Self {
            MockLidoOracleCalls::GetBeaconSpec(var)
        }
    }
    impl ::std::convert::From<GetCurrentFrameCall> for MockLidoOracleCalls {
        fn from(var: GetCurrentFrameCall) -> Self {
            MockLidoOracleCalls::GetCurrentFrame(var)
        }
    }
    impl ::std::convert::From<GetLastCompletedEpochIdCall> for MockLidoOracleCalls {
        fn from(var: GetLastCompletedEpochIdCall) -> Self {
            MockLidoOracleCalls::GetLastCompletedEpochId(var)
        }
    }
    impl ::std::convert::From<GetLastCompletedReportDeltaCall> for MockLidoOracleCalls {
        fn from(var: GetLastCompletedReportDeltaCall) -> Self {
            MockLidoOracleCalls::GetLastCompletedReportDelta(var)
        }
    }
    impl ::std::convert::From<MockStEthCall> for MockLidoOracleCalls {
        fn from(var: MockStEthCall) -> Self {
            MockLidoOracleCalls::MockStEth(var)
        }
    }
    #[doc = "Container type for all return fields from the `getBeaconSpec` function with signature `getBeaconSpec()` and selector `[229, 71, 199, 124]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetBeaconSpecReturn {
        pub epochs_per_frame: u64,
        pub slots_per_epoch: u64,
        pub seconds_per_slot: u64,
        pub genesis_time: u64,
    }
    #[doc = "Container type for all return fields from the `getCurrentFrame` function with signature `getCurrentFrame()` and selector `[114, 247, 155, 19]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetCurrentFrameReturn {
        pub frame_epoch_id: ethers::core::types::U256,
        pub frame_start_time: ethers::core::types::U256,
        pub frame_end_time: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getLastCompletedEpochId` function with signature `getLastCompletedEpochId()` and selector `[137, 137, 106, 239]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetLastCompletedEpochIdReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getLastCompletedReportDelta` function with signature `getLastCompletedReportDelta()` and selector `[83, 70, 73, 196]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetLastCompletedReportDeltaReturn {
        pub post_total_pooled_ether: ethers::core::types::U256,
        pub pre_total_pooled_ether: ethers::core::types::U256,
        pub time_elapsed: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `mockStEth` function with signature `mockStEth()` and selector `[118, 234, 20, 212]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MockStEthReturn(pub ethers::core::types::Address);
}
