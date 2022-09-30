pub use i_lido_oracle::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_lido_oracle {
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
    #[doc = "ILidoOracle was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ILIDOORACLE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getBeaconSpec\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"epochsPerFrame\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"slotsPerEpoch\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"secondsPerSlot\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"genesisTime\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCurrentFrame\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"frameEpochId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"frameStartTime\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"frameEndTime\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLastCompletedEpochId\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLastCompletedReportDelta\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"postTotalPooledEther\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"preTotalPooledEther\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"timeElapsed\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct ILidoOracle<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ILidoOracle<M> {
        fn clone(&self) -> Self {
            ILidoOracle(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ILidoOracle<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ILidoOracle<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ILidoOracle))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ILidoOracle<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ILIDOORACLE_ABI.clone(), client).into()
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
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ILidoOracle<M> {
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ILidoOracleCalls {
        GetBeaconSpec(GetBeaconSpecCall),
        GetCurrentFrame(GetCurrentFrameCall),
        GetLastCompletedEpochId(GetLastCompletedEpochIdCall),
        GetLastCompletedReportDelta(GetLastCompletedReportDeltaCall),
    }
    impl ethers::core::abi::AbiDecode for ILidoOracleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetBeaconSpecCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ILidoOracleCalls::GetBeaconSpec(decoded));
            }
            if let Ok(decoded) =
                <GetCurrentFrameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ILidoOracleCalls::GetCurrentFrame(decoded));
            }
            if let Ok(decoded) =
                <GetLastCompletedEpochIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ILidoOracleCalls::GetLastCompletedEpochId(decoded));
            }
            if let Ok(decoded) =
                <GetLastCompletedReportDeltaCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ILidoOracleCalls::GetLastCompletedReportDelta(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ILidoOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ILidoOracleCalls::GetBeaconSpec(element) => element.encode(),
                ILidoOracleCalls::GetCurrentFrame(element) => element.encode(),
                ILidoOracleCalls::GetLastCompletedEpochId(element) => element.encode(),
                ILidoOracleCalls::GetLastCompletedReportDelta(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ILidoOracleCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ILidoOracleCalls::GetBeaconSpec(element) => element.fmt(f),
                ILidoOracleCalls::GetCurrentFrame(element) => element.fmt(f),
                ILidoOracleCalls::GetLastCompletedEpochId(element) => element.fmt(f),
                ILidoOracleCalls::GetLastCompletedReportDelta(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetBeaconSpecCall> for ILidoOracleCalls {
        fn from(var: GetBeaconSpecCall) -> Self {
            ILidoOracleCalls::GetBeaconSpec(var)
        }
    }
    impl ::std::convert::From<GetCurrentFrameCall> for ILidoOracleCalls {
        fn from(var: GetCurrentFrameCall) -> Self {
            ILidoOracleCalls::GetCurrentFrame(var)
        }
    }
    impl ::std::convert::From<GetLastCompletedEpochIdCall> for ILidoOracleCalls {
        fn from(var: GetLastCompletedEpochIdCall) -> Self {
            ILidoOracleCalls::GetLastCompletedEpochId(var)
        }
    }
    impl ::std::convert::From<GetLastCompletedReportDeltaCall> for ILidoOracleCalls {
        fn from(var: GetLastCompletedReportDeltaCall) -> Self {
            ILidoOracleCalls::GetLastCompletedReportDelta(var)
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
}
