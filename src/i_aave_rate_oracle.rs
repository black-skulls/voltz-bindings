pub use i_aave_rate_oracle::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_aave_rate_oracle {
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
    #[doc = "IAaveRateOracle was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IAAVERATEORACLE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"type\":\"error\",\"name\":\"AavePoolGetReserveNormalizedIncomeReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"AavePoolGetReserveNormalizedVariableDebtReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CTokenExchangeRateReturnedZero\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"unlocked\",\"type\":\"bool\",\"components\":[]}],\"type\":\"error\",\"name\":\"CanOnlyTradeIfUnlocked\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CannotLiquidate\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CannotSettleBeforeMaturity\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"DebugError\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"amount0\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"amount1\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"ExpectedOppositeSigns\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint160\",\"name\":\"sqrtPriceX96\",\"type\":\"uint160\",\"components\":[]}],\"type\":\"error\",\"name\":\"ExpectedSqrtPriceZeroBeforeInit\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"IRSNotionalAmountSpecifiedMustBeNonZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidMarginDelta\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"LidoGetPooledEthBySharesReturnedZero\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"type\":\"error\",\"name\":\"LiquidityDeltaMustBePositiveInBurn\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"type\":\"error\",\"name\":\"LiquidityDeltaMustBePositiveInMint\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginLessThanMinimum\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tick\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"variableTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"cumulativeFeeIncurred\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenDeltaUnbalanced\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginRequirementNotMet\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginRequirementNotMetFCM\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"requested\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"available\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"NotEnoughFunds\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OOO\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyFCM\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyMarginEngine\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyOwnerCanUpdatePosition\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyVAMM\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PositionNetZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PositionNotSettled\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"RocketPoolGetEthValueReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"WithdrawalExceedsCurrentMargin\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"closeToOrBeyondMaturity\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_minSecondsSinceLastUpdate\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MinSecondsSinceLastUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"blockTimestampScaled\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"source\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint16\",\"name\":\"index\",\"type\":\"uint16\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint32\",\"name\":\"blockTimestamp\",\"type\":\"uint32\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"observedValue\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint16\",\"name\":\"cardinality\",\"type\":\"uint16\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint16\",\"name\":\"cardinalityNext\",\"type\":\"uint16\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"OracleBufferUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"observationCardinalityNextNew\",\"type\":\"uint16\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RateCardinalityNext\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"UNDERLYING_YIELD_BEARING_PROTOCOL_ID\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"yieldBearingProtocolID\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"aaveLendingPool\",\"outputs\":[{\"internalType\":\"contract IAaveV2LendingPool\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"from\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getApyFrom\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"apyFromTo\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"from\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"to\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getApyFromTo\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"apyFromTo\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getBlockSlope\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"blockChange\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"timeChange\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCurrentRateInRay\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"currentRate\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLastRateSlope\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"rateChange\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"timeChange\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_from\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRateFrom\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_from\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_to\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRateFromTo\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"rateCardinalityNext\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"increaseObservationCardinalityNext\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"minSecondsSinceLastUpdate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_minSecondsSinceLastUpdate\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setMinSecondsSinceLastUpdate\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"underlying\",\"outputs\":[{\"internalType\":\"contract IERC20Minimal\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"termStartTimestamp\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"termEndTimestamp\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"variableFactor\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"result\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"termStartTimestamp\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"termEndTimestamp\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"variableFactorNoCache\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"result\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"writeOracleEntry\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    pub struct IAaveRateOracle<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IAaveRateOracle<M> {
        fn clone(&self) -> Self {
            IAaveRateOracle(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IAaveRateOracle<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IAaveRateOracle<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IAaveRateOracle))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IAaveRateOracle<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IAAVERATEORACLE_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `UNDERLYING_YIELD_BEARING_PROTOCOL_ID` (0x22ff6568) function"]
        pub fn underlying_yield_bearing_protocol_id(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([34, 255, 101, 104], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `aaveLendingPool` (0xe9d337b8) function"]
        pub fn aave_lending_pool(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([233, 211, 55, 184], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getApyFrom` (0x17221ef1) function"]
        pub fn get_apy_from(
            &self,
            from: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([23, 34, 30, 241], from)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getApyFromTo` (0x93556dbd) function"]
        pub fn get_apy_from_to(
            &self,
            from: ethers::core::types::U256,
            to: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([147, 85, 109, 189], (from, to))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getBlockSlope` (0x91aa375d) function"]
        pub fn get_block_slope(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, (ethers::core::types::U256, u32)> {
            self.0
                .method_hash([145, 170, 55, 93], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCurrentRateInRay` (0xefdf5d8b) function"]
        pub fn get_current_rate_in_ray(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([239, 223, 93, 139], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLastRateSlope` (0xfe115fbe) function"]
        pub fn get_last_rate_slope(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, (ethers::core::types::U256, u32)> {
            self.0
                .method_hash([254, 17, 95, 190], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRateFrom` (0x163e9c4f) function"]
        pub fn get_rate_from(
            &self,
            from: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([22, 62, 156, 79], from)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRateFromTo` (0xf739670c) function"]
        pub fn get_rate_from_to(
            &self,
            from: ethers::core::types::U256,
            to: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([247, 57, 103, 12], (from, to))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `increaseObservationCardinalityNext` (0x32148f67) function"]
        pub fn increase_observation_cardinality_next(
            &self,
            rate_cardinality_next: u16,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([50, 20, 143, 103], rate_cardinality_next)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `minSecondsSinceLastUpdate` (0x7cf2cc9f) function"]
        pub fn min_seconds_since_last_update(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([124, 242, 204, 159], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setMinSecondsSinceLastUpdate` (0xbdb05092) function"]
        pub fn set_min_seconds_since_last_update(
            &self,
            min_seconds_since_last_update: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([189, 176, 80, 146], min_seconds_since_last_update)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `underlying` (0x6f307dc3) function"]
        pub fn underlying(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([111, 48, 125, 195], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `variableFactor` (0x25f258dd) function"]
        pub fn variable_factor(
            &self,
            term_start_timestamp: ethers::core::types::U256,
            term_end_timestamp: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [37, 242, 88, 221],
                    (term_start_timestamp, term_end_timestamp),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `variableFactorNoCache` (0x41453528) function"]
        pub fn variable_factor_no_cache(
            &self,
            term_start_timestamp: ethers::core::types::U256,
            term_end_timestamp: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([65, 69, 53, 40], (term_start_timestamp, term_end_timestamp))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `writeOracleEntry` (0x7aa4db13) function"]
        pub fn write_oracle_entry(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([122, 164, 219, 19], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `MinSecondsSinceLastUpdate` event"]
        pub fn min_seconds_since_last_update_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, MinSecondsSinceLastUpdateFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OracleBufferUpdate` event"]
        pub fn oracle_buffer_update_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OracleBufferUpdateFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RateCardinalityNext` event"]
        pub fn rate_cardinality_next_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RateCardinalityNextFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, IAaveRateOracleEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IAaveRateOracle<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Custom Error type `AavePoolGetReserveNormalizedIncomeReturnedZero` with signature `AavePoolGetReserveNormalizedIncomeReturnedZero()` and selector `[189, 137, 197, 175]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "AavePoolGetReserveNormalizedIncomeReturnedZero",
        abi = "AavePoolGetReserveNormalizedIncomeReturnedZero()"
    )]
    pub struct AavePoolGetReserveNormalizedIncomeReturnedZero;
    #[doc = "Custom Error type `AavePoolGetReserveNormalizedVariableDebtReturnedZero` with signature `AavePoolGetReserveNormalizedVariableDebtReturnedZero()` and selector `[195, 75, 105, 61]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "AavePoolGetReserveNormalizedVariableDebtReturnedZero",
        abi = "AavePoolGetReserveNormalizedVariableDebtReturnedZero()"
    )]
    pub struct AavePoolGetReserveNormalizedVariableDebtReturnedZero;
    #[doc = "Custom Error type `CTokenExchangeRateReturnedZero` with signature `CTokenExchangeRateReturnedZero()` and selector `[36, 152, 138, 123]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "CTokenExchangeRateReturnedZero",
        abi = "CTokenExchangeRateReturnedZero()"
    )]
    pub struct CTokenExchangeRateReturnedZero;
    #[doc = "Custom Error type `CanOnlyTradeIfUnlocked` with signature `CanOnlyTradeIfUnlocked(bool)` and selector `[121, 143, 4, 94]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "CanOnlyTradeIfUnlocked", abi = "CanOnlyTradeIfUnlocked(bool)")]
    pub struct CanOnlyTradeIfUnlocked {
        pub unlocked: bool,
    }
    #[doc = "Custom Error type `CannotLiquidate` with signature `CannotLiquidate()` and selector `[191, 135, 199, 213]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "CannotLiquidate", abi = "CannotLiquidate()")]
    pub struct CannotLiquidate;
    #[doc = "Custom Error type `CannotSettleBeforeMaturity` with signature `CannotSettleBeforeMaturity()` and selector `[2, 230, 23, 2]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "CannotSettleBeforeMaturity",
        abi = "CannotSettleBeforeMaturity()"
    )]
    pub struct CannotSettleBeforeMaturity;
    #[doc = "Custom Error type `DebugError` with signature `DebugError(uint256,uint256)` and selector `[186, 113, 84, 234]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "DebugError", abi = "DebugError(uint256,uint256)")]
    pub struct DebugError {
        pub x: ethers::core::types::U256,
        pub y: ethers::core::types::U256,
    }
    #[doc = "Custom Error type `ExpectedOppositeSigns` with signature `ExpectedOppositeSigns(int256,int256)` and selector `[35, 133, 254, 176]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "ExpectedOppositeSigns",
        abi = "ExpectedOppositeSigns(int256,int256)"
    )]
    pub struct ExpectedOppositeSigns {
        pub amount_0: I256,
        pub amount_1: I256,
    }
    #[doc = "Custom Error type `ExpectedSqrtPriceZeroBeforeInit` with signature `ExpectedSqrtPriceZeroBeforeInit(uint160)` and selector `[162, 248, 112, 60]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "ExpectedSqrtPriceZeroBeforeInit",
        abi = "ExpectedSqrtPriceZeroBeforeInit(uint160)"
    )]
    pub struct ExpectedSqrtPriceZeroBeforeInit {
        pub sqrt_price_x96: ethers::core::types::U256,
    }
    #[doc = "Custom Error type `IRSNotionalAmountSpecifiedMustBeNonZero` with signature `IRSNotionalAmountSpecifiedMustBeNonZero()` and selector `[63, 82, 15, 170]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "IRSNotionalAmountSpecifiedMustBeNonZero",
        abi = "IRSNotionalAmountSpecifiedMustBeNonZero()"
    )]
    pub struct IRSNotionalAmountSpecifiedMustBeNonZero;
    #[doc = "Custom Error type `InvalidMarginDelta` with signature `InvalidMarginDelta()` and selector `[138, 204, 109, 127]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "InvalidMarginDelta", abi = "InvalidMarginDelta()")]
    pub struct InvalidMarginDelta;
    #[doc = "Custom Error type `LidoGetPooledEthBySharesReturnedZero` with signature `LidoGetPooledEthBySharesReturnedZero()` and selector `[255, 248, 220, 91]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "LidoGetPooledEthBySharesReturnedZero",
        abi = "LidoGetPooledEthBySharesReturnedZero()"
    )]
    pub struct LidoGetPooledEthBySharesReturnedZero;
    #[doc = "Custom Error type `LiquidityDeltaMustBePositiveInBurn` with signature `LiquidityDeltaMustBePositiveInBurn(uint128)` and selector `[192, 157, 38, 9]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "LiquidityDeltaMustBePositiveInBurn",
        abi = "LiquidityDeltaMustBePositiveInBurn(uint128)"
    )]
    pub struct LiquidityDeltaMustBePositiveInBurn {
        pub amount: u128,
    }
    #[doc = "Custom Error type `LiquidityDeltaMustBePositiveInMint` with signature `LiquidityDeltaMustBePositiveInMint(uint128)` and selector `[216, 69, 154, 52]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "LiquidityDeltaMustBePositiveInMint",
        abi = "LiquidityDeltaMustBePositiveInMint(uint128)"
    )]
    pub struct LiquidityDeltaMustBePositiveInMint {
        pub amount: u128,
    }
    #[doc = "Custom Error type `MarginLessThanMinimum` with signature `MarginLessThanMinimum(int256)` and selector `[107, 79, 255, 36]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "MarginLessThanMinimum", abi = "MarginLessThanMinimum(int256)")]
    pub struct MarginLessThanMinimum {
        pub margin_requirement: I256,
    }
    #[doc = "Custom Error type `MarginRequirementNotMet` with signature `MarginRequirementNotMet(int256,int24,int256,int256,uint256,int256)` and selector `[67, 242, 131, 33]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "MarginRequirementNotMet",
        abi = "MarginRequirementNotMet(int256,int24,int256,int256,uint256,int256)"
    )]
    pub struct MarginRequirementNotMet {
        pub margin_requirement: I256,
        pub tick: i32,
        pub fixed_token_delta: I256,
        pub variable_token_delta: I256,
        pub cumulative_fee_incurred: ethers::core::types::U256,
        pub fixed_token_delta_unbalanced: I256,
    }
    #[doc = "Custom Error type `MarginRequirementNotMetFCM` with signature `MarginRequirementNotMetFCM(int256)` and selector `[65, 213, 168, 59]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "MarginRequirementNotMetFCM",
        abi = "MarginRequirementNotMetFCM(int256)"
    )]
    pub struct MarginRequirementNotMetFCM {
        pub margin_requirement: I256,
    }
    #[doc = "Custom Error type `NotEnoughFunds` with signature `NotEnoughFunds(uint256,uint256)` and selector `[140, 144, 83, 104]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "NotEnoughFunds", abi = "NotEnoughFunds(uint256,uint256)")]
    pub struct NotEnoughFunds {
        pub requested: ethers::core::types::U256,
        pub available: ethers::core::types::U256,
    }
    #[doc = "Custom Error type `OOO` with signature `OOO()` and selector `[191, 198, 99, 148]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "OOO", abi = "OOO()")]
    pub struct OOO;
    #[doc = "Custom Error type `OnlyFCM` with signature `OnlyFCM()` and selector `[93, 138, 54, 117]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "OnlyFCM", abi = "OnlyFCM()")]
    pub struct OnlyFCM;
    #[doc = "Custom Error type `OnlyMarginEngine` with signature `OnlyMarginEngine()` and selector `[40, 52, 210, 124]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "OnlyMarginEngine", abi = "OnlyMarginEngine()")]
    pub struct OnlyMarginEngine;
    #[doc = "Custom Error type `OnlyOwnerCanUpdatePosition` with signature `OnlyOwnerCanUpdatePosition()` and selector `[125, 164, 92, 231]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "OnlyOwnerCanUpdatePosition",
        abi = "OnlyOwnerCanUpdatePosition()"
    )]
    pub struct OnlyOwnerCanUpdatePosition;
    #[doc = "Custom Error type `OnlyVAMM` with signature `OnlyVAMM()` and selector `[123, 216, 216, 210]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "OnlyVAMM", abi = "OnlyVAMM()")]
    pub struct OnlyVAMM;
    #[doc = "Custom Error type `PositionNetZero` with signature `PositionNetZero()` and selector `[89, 32, 46, 189]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "PositionNetZero", abi = "PositionNetZero()")]
    pub struct PositionNetZero;
    #[doc = "Custom Error type `PositionNotSettled` with signature `PositionNotSettled()` and selector `[90, 108, 31, 220]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "PositionNotSettled", abi = "PositionNotSettled()")]
    pub struct PositionNotSettled;
    #[doc = "Custom Error type `RocketPoolGetEthValueReturnedZero` with signature `RocketPoolGetEthValueReturnedZero()` and selector `[18, 70, 158, 245]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "RocketPoolGetEthValueReturnedZero",
        abi = "RocketPoolGetEthValueReturnedZero()"
    )]
    pub struct RocketPoolGetEthValueReturnedZero;
    #[doc = "Custom Error type `WithdrawalExceedsCurrentMargin` with signature `WithdrawalExceedsCurrentMargin()` and selector `[41, 126, 28, 124]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "WithdrawalExceedsCurrentMargin",
        abi = "WithdrawalExceedsCurrentMargin()"
    )]
    pub struct WithdrawalExceedsCurrentMargin;
    #[doc = "Custom Error type `closeToOrBeyondMaturity` with signature `closeToOrBeyondMaturity()` and selector `[83, 173, 234, 156]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "closeToOrBeyondMaturity", abi = "closeToOrBeyondMaturity()")]
    pub struct closeToOrBeyondMaturity;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IAaveRateOracleErrors {
        AavePoolGetReserveNormalizedIncomeReturnedZero(
            AavePoolGetReserveNormalizedIncomeReturnedZero,
        ),
        AavePoolGetReserveNormalizedVariableDebtReturnedZero(
            AavePoolGetReserveNormalizedVariableDebtReturnedZero,
        ),
        CTokenExchangeRateReturnedZero(CTokenExchangeRateReturnedZero),
        CanOnlyTradeIfUnlocked(CanOnlyTradeIfUnlocked),
        CannotLiquidate(CannotLiquidate),
        CannotSettleBeforeMaturity(CannotSettleBeforeMaturity),
        DebugError(DebugError),
        ExpectedOppositeSigns(ExpectedOppositeSigns),
        ExpectedSqrtPriceZeroBeforeInit(ExpectedSqrtPriceZeroBeforeInit),
        IRSNotionalAmountSpecifiedMustBeNonZero(IRSNotionalAmountSpecifiedMustBeNonZero),
        InvalidMarginDelta(InvalidMarginDelta),
        LidoGetPooledEthBySharesReturnedZero(LidoGetPooledEthBySharesReturnedZero),
        LiquidityDeltaMustBePositiveInBurn(LiquidityDeltaMustBePositiveInBurn),
        LiquidityDeltaMustBePositiveInMint(LiquidityDeltaMustBePositiveInMint),
        MarginLessThanMinimum(MarginLessThanMinimum),
        MarginRequirementNotMet(MarginRequirementNotMet),
        MarginRequirementNotMetFCM(MarginRequirementNotMetFCM),
        NotEnoughFunds(NotEnoughFunds),
        OOO(OOO),
        OnlyFCM(OnlyFCM),
        OnlyMarginEngine(OnlyMarginEngine),
        OnlyOwnerCanUpdatePosition(OnlyOwnerCanUpdatePosition),
        OnlyVAMM(OnlyVAMM),
        PositionNetZero(PositionNetZero),
        PositionNotSettled(PositionNotSettled),
        RocketPoolGetEthValueReturnedZero(RocketPoolGetEthValueReturnedZero),
        WithdrawalExceedsCurrentMargin(WithdrawalExceedsCurrentMargin),
        closeToOrBeyondMaturity(closeToOrBeyondMaturity),
    }
    impl ethers::core::abi::AbiDecode for IAaveRateOracleErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok (decoded) = < AavePoolGetReserveNormalizedIncomeReturnedZero as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (IAaveRateOracleErrors :: AavePoolGetReserveNormalizedIncomeReturnedZero (decoded)) }
            if let Ok (decoded) = < AavePoolGetReserveNormalizedVariableDebtReturnedZero as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (IAaveRateOracleErrors :: AavePoolGetReserveNormalizedVariableDebtReturnedZero (decoded)) }
            if let Ok(decoded) =
                <CTokenExchangeRateReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IAaveRateOracleErrors::CTokenExchangeRateReturnedZero(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <CanOnlyTradeIfUnlocked as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveRateOracleErrors::CanOnlyTradeIfUnlocked(decoded));
            }
            if let Ok(decoded) =
                <CannotLiquidate as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveRateOracleErrors::CannotLiquidate(decoded));
            }
            if let Ok(decoded) =
                <CannotSettleBeforeMaturity as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveRateOracleErrors::CannotSettleBeforeMaturity(decoded));
            }
            if let Ok(decoded) = <DebugError as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveRateOracleErrors::DebugError(decoded));
            }
            if let Ok(decoded) =
                <ExpectedOppositeSigns as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveRateOracleErrors::ExpectedOppositeSigns(decoded));
            }
            if let Ok(decoded) =
                <ExpectedSqrtPriceZeroBeforeInit as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IAaveRateOracleErrors::ExpectedSqrtPriceZeroBeforeInit(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <IRSNotionalAmountSpecifiedMustBeNonZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IAaveRateOracleErrors::IRSNotionalAmountSpecifiedMustBeNonZero(decoded));
            }
            if let Ok(decoded) =
                <InvalidMarginDelta as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveRateOracleErrors::InvalidMarginDelta(decoded));
            }
            if let Ok(decoded) =
                <LidoGetPooledEthBySharesReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IAaveRateOracleErrors::LidoGetPooledEthBySharesReturnedZero(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <LiquidityDeltaMustBePositiveInBurn as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IAaveRateOracleErrors::LiquidityDeltaMustBePositiveInBurn(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <LiquidityDeltaMustBePositiveInMint as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IAaveRateOracleErrors::LiquidityDeltaMustBePositiveInMint(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <MarginLessThanMinimum as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveRateOracleErrors::MarginLessThanMinimum(decoded));
            }
            if let Ok(decoded) =
                <MarginRequirementNotMet as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveRateOracleErrors::MarginRequirementNotMet(decoded));
            }
            if let Ok(decoded) =
                <MarginRequirementNotMetFCM as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveRateOracleErrors::MarginRequirementNotMetFCM(decoded));
            }
            if let Ok(decoded) =
                <NotEnoughFunds as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveRateOracleErrors::NotEnoughFunds(decoded));
            }
            if let Ok(decoded) = <OOO as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IAaveRateOracleErrors::OOO(decoded));
            }
            if let Ok(decoded) = <OnlyFCM as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IAaveRateOracleErrors::OnlyFCM(decoded));
            }
            if let Ok(decoded) =
                <OnlyMarginEngine as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveRateOracleErrors::OnlyMarginEngine(decoded));
            }
            if let Ok(decoded) =
                <OnlyOwnerCanUpdatePosition as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveRateOracleErrors::OnlyOwnerCanUpdatePosition(decoded));
            }
            if let Ok(decoded) = <OnlyVAMM as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IAaveRateOracleErrors::OnlyVAMM(decoded));
            }
            if let Ok(decoded) =
                <PositionNetZero as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveRateOracleErrors::PositionNetZero(decoded));
            }
            if let Ok(decoded) =
                <PositionNotSettled as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveRateOracleErrors::PositionNotSettled(decoded));
            }
            if let Ok(decoded) =
                <RocketPoolGetEthValueReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IAaveRateOracleErrors::RocketPoolGetEthValueReturnedZero(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <WithdrawalExceedsCurrentMargin as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IAaveRateOracleErrors::WithdrawalExceedsCurrentMargin(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <closeToOrBeyondMaturity as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveRateOracleErrors::closeToOrBeyondMaturity(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IAaveRateOracleErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                IAaveRateOracleErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(element) => {
                    element.encode()
                }
                IAaveRateOracleErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(
                    element,
                ) => element.encode(),
                IAaveRateOracleErrors::CTokenExchangeRateReturnedZero(element) => element.encode(),
                IAaveRateOracleErrors::CanOnlyTradeIfUnlocked(element) => element.encode(),
                IAaveRateOracleErrors::CannotLiquidate(element) => element.encode(),
                IAaveRateOracleErrors::CannotSettleBeforeMaturity(element) => element.encode(),
                IAaveRateOracleErrors::DebugError(element) => element.encode(),
                IAaveRateOracleErrors::ExpectedOppositeSigns(element) => element.encode(),
                IAaveRateOracleErrors::ExpectedSqrtPriceZeroBeforeInit(element) => element.encode(),
                IAaveRateOracleErrors::IRSNotionalAmountSpecifiedMustBeNonZero(element) => {
                    element.encode()
                }
                IAaveRateOracleErrors::InvalidMarginDelta(element) => element.encode(),
                IAaveRateOracleErrors::LidoGetPooledEthBySharesReturnedZero(element) => {
                    element.encode()
                }
                IAaveRateOracleErrors::LiquidityDeltaMustBePositiveInBurn(element) => {
                    element.encode()
                }
                IAaveRateOracleErrors::LiquidityDeltaMustBePositiveInMint(element) => {
                    element.encode()
                }
                IAaveRateOracleErrors::MarginLessThanMinimum(element) => element.encode(),
                IAaveRateOracleErrors::MarginRequirementNotMet(element) => element.encode(),
                IAaveRateOracleErrors::MarginRequirementNotMetFCM(element) => element.encode(),
                IAaveRateOracleErrors::NotEnoughFunds(element) => element.encode(),
                IAaveRateOracleErrors::OOO(element) => element.encode(),
                IAaveRateOracleErrors::OnlyFCM(element) => element.encode(),
                IAaveRateOracleErrors::OnlyMarginEngine(element) => element.encode(),
                IAaveRateOracleErrors::OnlyOwnerCanUpdatePosition(element) => element.encode(),
                IAaveRateOracleErrors::OnlyVAMM(element) => element.encode(),
                IAaveRateOracleErrors::PositionNetZero(element) => element.encode(),
                IAaveRateOracleErrors::PositionNotSettled(element) => element.encode(),
                IAaveRateOracleErrors::RocketPoolGetEthValueReturnedZero(element) => {
                    element.encode()
                }
                IAaveRateOracleErrors::WithdrawalExceedsCurrentMargin(element) => element.encode(),
                IAaveRateOracleErrors::closeToOrBeyondMaturity(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IAaveRateOracleErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IAaveRateOracleErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(element) => {
                    element.fmt(f)
                }
                IAaveRateOracleErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(
                    element,
                ) => element.fmt(f),
                IAaveRateOracleErrors::CTokenExchangeRateReturnedZero(element) => element.fmt(f),
                IAaveRateOracleErrors::CanOnlyTradeIfUnlocked(element) => element.fmt(f),
                IAaveRateOracleErrors::CannotLiquidate(element) => element.fmt(f),
                IAaveRateOracleErrors::CannotSettleBeforeMaturity(element) => element.fmt(f),
                IAaveRateOracleErrors::DebugError(element) => element.fmt(f),
                IAaveRateOracleErrors::ExpectedOppositeSigns(element) => element.fmt(f),
                IAaveRateOracleErrors::ExpectedSqrtPriceZeroBeforeInit(element) => element.fmt(f),
                IAaveRateOracleErrors::IRSNotionalAmountSpecifiedMustBeNonZero(element) => {
                    element.fmt(f)
                }
                IAaveRateOracleErrors::InvalidMarginDelta(element) => element.fmt(f),
                IAaveRateOracleErrors::LidoGetPooledEthBySharesReturnedZero(element) => {
                    element.fmt(f)
                }
                IAaveRateOracleErrors::LiquidityDeltaMustBePositiveInBurn(element) => {
                    element.fmt(f)
                }
                IAaveRateOracleErrors::LiquidityDeltaMustBePositiveInMint(element) => {
                    element.fmt(f)
                }
                IAaveRateOracleErrors::MarginLessThanMinimum(element) => element.fmt(f),
                IAaveRateOracleErrors::MarginRequirementNotMet(element) => element.fmt(f),
                IAaveRateOracleErrors::MarginRequirementNotMetFCM(element) => element.fmt(f),
                IAaveRateOracleErrors::NotEnoughFunds(element) => element.fmt(f),
                IAaveRateOracleErrors::OOO(element) => element.fmt(f),
                IAaveRateOracleErrors::OnlyFCM(element) => element.fmt(f),
                IAaveRateOracleErrors::OnlyMarginEngine(element) => element.fmt(f),
                IAaveRateOracleErrors::OnlyOwnerCanUpdatePosition(element) => element.fmt(f),
                IAaveRateOracleErrors::OnlyVAMM(element) => element.fmt(f),
                IAaveRateOracleErrors::PositionNetZero(element) => element.fmt(f),
                IAaveRateOracleErrors::PositionNotSettled(element) => element.fmt(f),
                IAaveRateOracleErrors::RocketPoolGetEthValueReturnedZero(element) => element.fmt(f),
                IAaveRateOracleErrors::WithdrawalExceedsCurrentMargin(element) => element.fmt(f),
                IAaveRateOracleErrors::closeToOrBeyondMaturity(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AavePoolGetReserveNormalizedIncomeReturnedZero>
        for IAaveRateOracleErrors
    {
        fn from(var: AavePoolGetReserveNormalizedIncomeReturnedZero) -> Self {
            IAaveRateOracleErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(var)
        }
    }
    impl ::std::convert::From<AavePoolGetReserveNormalizedVariableDebtReturnedZero>
        for IAaveRateOracleErrors
    {
        fn from(var: AavePoolGetReserveNormalizedVariableDebtReturnedZero) -> Self {
            IAaveRateOracleErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(var)
        }
    }
    impl ::std::convert::From<CTokenExchangeRateReturnedZero> for IAaveRateOracleErrors {
        fn from(var: CTokenExchangeRateReturnedZero) -> Self {
            IAaveRateOracleErrors::CTokenExchangeRateReturnedZero(var)
        }
    }
    impl ::std::convert::From<CanOnlyTradeIfUnlocked> for IAaveRateOracleErrors {
        fn from(var: CanOnlyTradeIfUnlocked) -> Self {
            IAaveRateOracleErrors::CanOnlyTradeIfUnlocked(var)
        }
    }
    impl ::std::convert::From<CannotLiquidate> for IAaveRateOracleErrors {
        fn from(var: CannotLiquidate) -> Self {
            IAaveRateOracleErrors::CannotLiquidate(var)
        }
    }
    impl ::std::convert::From<CannotSettleBeforeMaturity> for IAaveRateOracleErrors {
        fn from(var: CannotSettleBeforeMaturity) -> Self {
            IAaveRateOracleErrors::CannotSettleBeforeMaturity(var)
        }
    }
    impl ::std::convert::From<DebugError> for IAaveRateOracleErrors {
        fn from(var: DebugError) -> Self {
            IAaveRateOracleErrors::DebugError(var)
        }
    }
    impl ::std::convert::From<ExpectedOppositeSigns> for IAaveRateOracleErrors {
        fn from(var: ExpectedOppositeSigns) -> Self {
            IAaveRateOracleErrors::ExpectedOppositeSigns(var)
        }
    }
    impl ::std::convert::From<ExpectedSqrtPriceZeroBeforeInit> for IAaveRateOracleErrors {
        fn from(var: ExpectedSqrtPriceZeroBeforeInit) -> Self {
            IAaveRateOracleErrors::ExpectedSqrtPriceZeroBeforeInit(var)
        }
    }
    impl ::std::convert::From<IRSNotionalAmountSpecifiedMustBeNonZero> for IAaveRateOracleErrors {
        fn from(var: IRSNotionalAmountSpecifiedMustBeNonZero) -> Self {
            IAaveRateOracleErrors::IRSNotionalAmountSpecifiedMustBeNonZero(var)
        }
    }
    impl ::std::convert::From<InvalidMarginDelta> for IAaveRateOracleErrors {
        fn from(var: InvalidMarginDelta) -> Self {
            IAaveRateOracleErrors::InvalidMarginDelta(var)
        }
    }
    impl ::std::convert::From<LidoGetPooledEthBySharesReturnedZero> for IAaveRateOracleErrors {
        fn from(var: LidoGetPooledEthBySharesReturnedZero) -> Self {
            IAaveRateOracleErrors::LidoGetPooledEthBySharesReturnedZero(var)
        }
    }
    impl ::std::convert::From<LiquidityDeltaMustBePositiveInBurn> for IAaveRateOracleErrors {
        fn from(var: LiquidityDeltaMustBePositiveInBurn) -> Self {
            IAaveRateOracleErrors::LiquidityDeltaMustBePositiveInBurn(var)
        }
    }
    impl ::std::convert::From<LiquidityDeltaMustBePositiveInMint> for IAaveRateOracleErrors {
        fn from(var: LiquidityDeltaMustBePositiveInMint) -> Self {
            IAaveRateOracleErrors::LiquidityDeltaMustBePositiveInMint(var)
        }
    }
    impl ::std::convert::From<MarginLessThanMinimum> for IAaveRateOracleErrors {
        fn from(var: MarginLessThanMinimum) -> Self {
            IAaveRateOracleErrors::MarginLessThanMinimum(var)
        }
    }
    impl ::std::convert::From<MarginRequirementNotMet> for IAaveRateOracleErrors {
        fn from(var: MarginRequirementNotMet) -> Self {
            IAaveRateOracleErrors::MarginRequirementNotMet(var)
        }
    }
    impl ::std::convert::From<MarginRequirementNotMetFCM> for IAaveRateOracleErrors {
        fn from(var: MarginRequirementNotMetFCM) -> Self {
            IAaveRateOracleErrors::MarginRequirementNotMetFCM(var)
        }
    }
    impl ::std::convert::From<NotEnoughFunds> for IAaveRateOracleErrors {
        fn from(var: NotEnoughFunds) -> Self {
            IAaveRateOracleErrors::NotEnoughFunds(var)
        }
    }
    impl ::std::convert::From<OOO> for IAaveRateOracleErrors {
        fn from(var: OOO) -> Self {
            IAaveRateOracleErrors::OOO(var)
        }
    }
    impl ::std::convert::From<OnlyFCM> for IAaveRateOracleErrors {
        fn from(var: OnlyFCM) -> Self {
            IAaveRateOracleErrors::OnlyFCM(var)
        }
    }
    impl ::std::convert::From<OnlyMarginEngine> for IAaveRateOracleErrors {
        fn from(var: OnlyMarginEngine) -> Self {
            IAaveRateOracleErrors::OnlyMarginEngine(var)
        }
    }
    impl ::std::convert::From<OnlyOwnerCanUpdatePosition> for IAaveRateOracleErrors {
        fn from(var: OnlyOwnerCanUpdatePosition) -> Self {
            IAaveRateOracleErrors::OnlyOwnerCanUpdatePosition(var)
        }
    }
    impl ::std::convert::From<OnlyVAMM> for IAaveRateOracleErrors {
        fn from(var: OnlyVAMM) -> Self {
            IAaveRateOracleErrors::OnlyVAMM(var)
        }
    }
    impl ::std::convert::From<PositionNetZero> for IAaveRateOracleErrors {
        fn from(var: PositionNetZero) -> Self {
            IAaveRateOracleErrors::PositionNetZero(var)
        }
    }
    impl ::std::convert::From<PositionNotSettled> for IAaveRateOracleErrors {
        fn from(var: PositionNotSettled) -> Self {
            IAaveRateOracleErrors::PositionNotSettled(var)
        }
    }
    impl ::std::convert::From<RocketPoolGetEthValueReturnedZero> for IAaveRateOracleErrors {
        fn from(var: RocketPoolGetEthValueReturnedZero) -> Self {
            IAaveRateOracleErrors::RocketPoolGetEthValueReturnedZero(var)
        }
    }
    impl ::std::convert::From<WithdrawalExceedsCurrentMargin> for IAaveRateOracleErrors {
        fn from(var: WithdrawalExceedsCurrentMargin) -> Self {
            IAaveRateOracleErrors::WithdrawalExceedsCurrentMargin(var)
        }
    }
    impl ::std::convert::From<closeToOrBeyondMaturity> for IAaveRateOracleErrors {
        fn from(var: closeToOrBeyondMaturity) -> Self {
            IAaveRateOracleErrors::closeToOrBeyondMaturity(var)
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
    #[ethevent(
        name = "MinSecondsSinceLastUpdate",
        abi = "MinSecondsSinceLastUpdate(uint256)"
    )]
    pub struct MinSecondsSinceLastUpdateFilter {
        pub min_seconds_since_last_update: ethers::core::types::U256,
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
        name = "OracleBufferUpdate",
        abi = "OracleBufferUpdate(uint256,address,uint16,uint32,uint256,uint16,uint16)"
    )]
    pub struct OracleBufferUpdateFilter {
        pub block_timestamp_scaled: ethers::core::types::U256,
        pub source: ethers::core::types::Address,
        pub index: u16,
        pub block_timestamp: u32,
        pub observed_value: ethers::core::types::U256,
        pub cardinality: u16,
        pub cardinality_next: u16,
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
    #[ethevent(name = "RateCardinalityNext", abi = "RateCardinalityNext(uint16)")]
    pub struct RateCardinalityNextFilter {
        pub observation_cardinality_next_new: u16,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IAaveRateOracleEvents {
        MinSecondsSinceLastUpdateFilter(MinSecondsSinceLastUpdateFilter),
        OracleBufferUpdateFilter(OracleBufferUpdateFilter),
        RateCardinalityNextFilter(RateCardinalityNextFilter),
    }
    impl ethers::contract::EthLogDecode for IAaveRateOracleEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = MinSecondsSinceLastUpdateFilter::decode_log(log) {
                return Ok(IAaveRateOracleEvents::MinSecondsSinceLastUpdateFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = OracleBufferUpdateFilter::decode_log(log) {
                return Ok(IAaveRateOracleEvents::OracleBufferUpdateFilter(decoded));
            }
            if let Ok(decoded) = RateCardinalityNextFilter::decode_log(log) {
                return Ok(IAaveRateOracleEvents::RateCardinalityNextFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IAaveRateOracleEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IAaveRateOracleEvents::MinSecondsSinceLastUpdateFilter(element) => element.fmt(f),
                IAaveRateOracleEvents::OracleBufferUpdateFilter(element) => element.fmt(f),
                IAaveRateOracleEvents::RateCardinalityNextFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `UNDERLYING_YIELD_BEARING_PROTOCOL_ID` function with signature `UNDERLYING_YIELD_BEARING_PROTOCOL_ID()` and selector `[34, 255, 101, 104]`"]
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
        name = "UNDERLYING_YIELD_BEARING_PROTOCOL_ID",
        abi = "UNDERLYING_YIELD_BEARING_PROTOCOL_ID()"
    )]
    pub struct UnderlyingYieldBearingProtocolIdCall;
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
    #[doc = "Container type for all input parameters for the `getApyFrom` function with signature `getApyFrom(uint256)` and selector `[23, 34, 30, 241]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getApyFrom", abi = "getApyFrom(uint256)")]
    pub struct GetApyFromCall {
        pub from: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getApyFromTo` function with signature `getApyFromTo(uint256,uint256)` and selector `[147, 85, 109, 189]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getApyFromTo", abi = "getApyFromTo(uint256,uint256)")]
    pub struct GetApyFromToCall {
        pub from: ethers::core::types::U256,
        pub to: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getBlockSlope` function with signature `getBlockSlope()` and selector `[145, 170, 55, 93]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getBlockSlope", abi = "getBlockSlope()")]
    pub struct GetBlockSlopeCall;
    #[doc = "Container type for all input parameters for the `getCurrentRateInRay` function with signature `getCurrentRateInRay()` and selector `[239, 223, 93, 139]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getCurrentRateInRay", abi = "getCurrentRateInRay()")]
    pub struct GetCurrentRateInRayCall;
    #[doc = "Container type for all input parameters for the `getLastRateSlope` function with signature `getLastRateSlope()` and selector `[254, 17, 95, 190]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getLastRateSlope", abi = "getLastRateSlope()")]
    pub struct GetLastRateSlopeCall;
    #[doc = "Container type for all input parameters for the `getRateFrom` function with signature `getRateFrom(uint256)` and selector `[22, 62, 156, 79]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getRateFrom", abi = "getRateFrom(uint256)")]
    pub struct GetRateFromCall {
        pub from: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getRateFromTo` function with signature `getRateFromTo(uint256,uint256)` and selector `[247, 57, 103, 12]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getRateFromTo", abi = "getRateFromTo(uint256,uint256)")]
    pub struct GetRateFromToCall {
        pub from: ethers::core::types::U256,
        pub to: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `increaseObservationCardinalityNext` function with signature `increaseObservationCardinalityNext(uint16)` and selector `[50, 20, 143, 103]`"]
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
        name = "increaseObservationCardinalityNext",
        abi = "increaseObservationCardinalityNext(uint16)"
    )]
    pub struct IncreaseObservationCardinalityNextCall {
        pub rate_cardinality_next: u16,
    }
    #[doc = "Container type for all input parameters for the `minSecondsSinceLastUpdate` function with signature `minSecondsSinceLastUpdate()` and selector `[124, 242, 204, 159]`"]
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
        name = "minSecondsSinceLastUpdate",
        abi = "minSecondsSinceLastUpdate()"
    )]
    pub struct MinSecondsSinceLastUpdateCall;
    #[doc = "Container type for all input parameters for the `setMinSecondsSinceLastUpdate` function with signature `setMinSecondsSinceLastUpdate(uint256)` and selector `[189, 176, 80, 146]`"]
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
        name = "setMinSecondsSinceLastUpdate",
        abi = "setMinSecondsSinceLastUpdate(uint256)"
    )]
    pub struct SetMinSecondsSinceLastUpdateCall {
        pub min_seconds_since_last_update: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `underlying` function with signature `underlying()` and selector `[111, 48, 125, 195]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "underlying", abi = "underlying()")]
    pub struct UnderlyingCall;
    #[doc = "Container type for all input parameters for the `variableFactor` function with signature `variableFactor(uint256,uint256)` and selector `[37, 242, 88, 221]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "variableFactor", abi = "variableFactor(uint256,uint256)")]
    pub struct VariableFactorCall {
        pub term_start_timestamp: ethers::core::types::U256,
        pub term_end_timestamp: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `variableFactorNoCache` function with signature `variableFactorNoCache(uint256,uint256)` and selector `[65, 69, 53, 40]`"]
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
        name = "variableFactorNoCache",
        abi = "variableFactorNoCache(uint256,uint256)"
    )]
    pub struct VariableFactorNoCacheCall {
        pub term_start_timestamp: ethers::core::types::U256,
        pub term_end_timestamp: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `writeOracleEntry` function with signature `writeOracleEntry()` and selector `[122, 164, 219, 19]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "writeOracleEntry", abi = "writeOracleEntry()")]
    pub struct WriteOracleEntryCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IAaveRateOracleCalls {
        UnderlyingYieldBearingProtocolId(UnderlyingYieldBearingProtocolIdCall),
        AaveLendingPool(AaveLendingPoolCall),
        GetApyFrom(GetApyFromCall),
        GetApyFromTo(GetApyFromToCall),
        GetBlockSlope(GetBlockSlopeCall),
        GetCurrentRateInRay(GetCurrentRateInRayCall),
        GetLastRateSlope(GetLastRateSlopeCall),
        GetRateFrom(GetRateFromCall),
        GetRateFromTo(GetRateFromToCall),
        IncreaseObservationCardinalityNext(IncreaseObservationCardinalityNextCall),
        MinSecondsSinceLastUpdate(MinSecondsSinceLastUpdateCall),
        SetMinSecondsSinceLastUpdate(SetMinSecondsSinceLastUpdateCall),
        Underlying(UnderlyingCall),
        VariableFactor(VariableFactorCall),
        VariableFactorNoCache(VariableFactorNoCacheCall),
        WriteOracleEntry(WriteOracleEntryCall),
    }
    impl ethers::core::abi::AbiDecode for IAaveRateOracleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <UnderlyingYieldBearingProtocolIdCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IAaveRateOracleCalls::UnderlyingYieldBearingProtocolId(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <AaveLendingPoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveRateOracleCalls::AaveLendingPool(decoded));
            }
            if let Ok(decoded) =
                <GetApyFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveRateOracleCalls::GetApyFrom(decoded));
            }
            if let Ok(decoded) =
                <GetApyFromToCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveRateOracleCalls::GetApyFromTo(decoded));
            }
            if let Ok(decoded) =
                <GetBlockSlopeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveRateOracleCalls::GetBlockSlope(decoded));
            }
            if let Ok(decoded) =
                <GetCurrentRateInRayCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveRateOracleCalls::GetCurrentRateInRay(decoded));
            }
            if let Ok(decoded) =
                <GetLastRateSlopeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveRateOracleCalls::GetLastRateSlope(decoded));
            }
            if let Ok(decoded) =
                <GetRateFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveRateOracleCalls::GetRateFrom(decoded));
            }
            if let Ok(decoded) =
                <GetRateFromToCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveRateOracleCalls::GetRateFromTo(decoded));
            }
            if let Ok(decoded) =
                <IncreaseObservationCardinalityNextCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IAaveRateOracleCalls::IncreaseObservationCardinalityNext(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <MinSecondsSinceLastUpdateCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IAaveRateOracleCalls::MinSecondsSinceLastUpdate(decoded));
            }
            if let Ok(decoded) =
                <SetMinSecondsSinceLastUpdateCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IAaveRateOracleCalls::SetMinSecondsSinceLastUpdate(decoded));
            }
            if let Ok(decoded) =
                <UnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveRateOracleCalls::Underlying(decoded));
            }
            if let Ok(decoded) =
                <VariableFactorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveRateOracleCalls::VariableFactor(decoded));
            }
            if let Ok(decoded) =
                <VariableFactorNoCacheCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveRateOracleCalls::VariableFactorNoCache(decoded));
            }
            if let Ok(decoded) =
                <WriteOracleEntryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveRateOracleCalls::WriteOracleEntry(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IAaveRateOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IAaveRateOracleCalls::UnderlyingYieldBearingProtocolId(element) => element.encode(),
                IAaveRateOracleCalls::AaveLendingPool(element) => element.encode(),
                IAaveRateOracleCalls::GetApyFrom(element) => element.encode(),
                IAaveRateOracleCalls::GetApyFromTo(element) => element.encode(),
                IAaveRateOracleCalls::GetBlockSlope(element) => element.encode(),
                IAaveRateOracleCalls::GetCurrentRateInRay(element) => element.encode(),
                IAaveRateOracleCalls::GetLastRateSlope(element) => element.encode(),
                IAaveRateOracleCalls::GetRateFrom(element) => element.encode(),
                IAaveRateOracleCalls::GetRateFromTo(element) => element.encode(),
                IAaveRateOracleCalls::IncreaseObservationCardinalityNext(element) => {
                    element.encode()
                }
                IAaveRateOracleCalls::MinSecondsSinceLastUpdate(element) => element.encode(),
                IAaveRateOracleCalls::SetMinSecondsSinceLastUpdate(element) => element.encode(),
                IAaveRateOracleCalls::Underlying(element) => element.encode(),
                IAaveRateOracleCalls::VariableFactor(element) => element.encode(),
                IAaveRateOracleCalls::VariableFactorNoCache(element) => element.encode(),
                IAaveRateOracleCalls::WriteOracleEntry(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IAaveRateOracleCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IAaveRateOracleCalls::UnderlyingYieldBearingProtocolId(element) => element.fmt(f),
                IAaveRateOracleCalls::AaveLendingPool(element) => element.fmt(f),
                IAaveRateOracleCalls::GetApyFrom(element) => element.fmt(f),
                IAaveRateOracleCalls::GetApyFromTo(element) => element.fmt(f),
                IAaveRateOracleCalls::GetBlockSlope(element) => element.fmt(f),
                IAaveRateOracleCalls::GetCurrentRateInRay(element) => element.fmt(f),
                IAaveRateOracleCalls::GetLastRateSlope(element) => element.fmt(f),
                IAaveRateOracleCalls::GetRateFrom(element) => element.fmt(f),
                IAaveRateOracleCalls::GetRateFromTo(element) => element.fmt(f),
                IAaveRateOracleCalls::IncreaseObservationCardinalityNext(element) => element.fmt(f),
                IAaveRateOracleCalls::MinSecondsSinceLastUpdate(element) => element.fmt(f),
                IAaveRateOracleCalls::SetMinSecondsSinceLastUpdate(element) => element.fmt(f),
                IAaveRateOracleCalls::Underlying(element) => element.fmt(f),
                IAaveRateOracleCalls::VariableFactor(element) => element.fmt(f),
                IAaveRateOracleCalls::VariableFactorNoCache(element) => element.fmt(f),
                IAaveRateOracleCalls::WriteOracleEntry(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<UnderlyingYieldBearingProtocolIdCall> for IAaveRateOracleCalls {
        fn from(var: UnderlyingYieldBearingProtocolIdCall) -> Self {
            IAaveRateOracleCalls::UnderlyingYieldBearingProtocolId(var)
        }
    }
    impl ::std::convert::From<AaveLendingPoolCall> for IAaveRateOracleCalls {
        fn from(var: AaveLendingPoolCall) -> Self {
            IAaveRateOracleCalls::AaveLendingPool(var)
        }
    }
    impl ::std::convert::From<GetApyFromCall> for IAaveRateOracleCalls {
        fn from(var: GetApyFromCall) -> Self {
            IAaveRateOracleCalls::GetApyFrom(var)
        }
    }
    impl ::std::convert::From<GetApyFromToCall> for IAaveRateOracleCalls {
        fn from(var: GetApyFromToCall) -> Self {
            IAaveRateOracleCalls::GetApyFromTo(var)
        }
    }
    impl ::std::convert::From<GetBlockSlopeCall> for IAaveRateOracleCalls {
        fn from(var: GetBlockSlopeCall) -> Self {
            IAaveRateOracleCalls::GetBlockSlope(var)
        }
    }
    impl ::std::convert::From<GetCurrentRateInRayCall> for IAaveRateOracleCalls {
        fn from(var: GetCurrentRateInRayCall) -> Self {
            IAaveRateOracleCalls::GetCurrentRateInRay(var)
        }
    }
    impl ::std::convert::From<GetLastRateSlopeCall> for IAaveRateOracleCalls {
        fn from(var: GetLastRateSlopeCall) -> Self {
            IAaveRateOracleCalls::GetLastRateSlope(var)
        }
    }
    impl ::std::convert::From<GetRateFromCall> for IAaveRateOracleCalls {
        fn from(var: GetRateFromCall) -> Self {
            IAaveRateOracleCalls::GetRateFrom(var)
        }
    }
    impl ::std::convert::From<GetRateFromToCall> for IAaveRateOracleCalls {
        fn from(var: GetRateFromToCall) -> Self {
            IAaveRateOracleCalls::GetRateFromTo(var)
        }
    }
    impl ::std::convert::From<IncreaseObservationCardinalityNextCall> for IAaveRateOracleCalls {
        fn from(var: IncreaseObservationCardinalityNextCall) -> Self {
            IAaveRateOracleCalls::IncreaseObservationCardinalityNext(var)
        }
    }
    impl ::std::convert::From<MinSecondsSinceLastUpdateCall> for IAaveRateOracleCalls {
        fn from(var: MinSecondsSinceLastUpdateCall) -> Self {
            IAaveRateOracleCalls::MinSecondsSinceLastUpdate(var)
        }
    }
    impl ::std::convert::From<SetMinSecondsSinceLastUpdateCall> for IAaveRateOracleCalls {
        fn from(var: SetMinSecondsSinceLastUpdateCall) -> Self {
            IAaveRateOracleCalls::SetMinSecondsSinceLastUpdate(var)
        }
    }
    impl ::std::convert::From<UnderlyingCall> for IAaveRateOracleCalls {
        fn from(var: UnderlyingCall) -> Self {
            IAaveRateOracleCalls::Underlying(var)
        }
    }
    impl ::std::convert::From<VariableFactorCall> for IAaveRateOracleCalls {
        fn from(var: VariableFactorCall) -> Self {
            IAaveRateOracleCalls::VariableFactor(var)
        }
    }
    impl ::std::convert::From<VariableFactorNoCacheCall> for IAaveRateOracleCalls {
        fn from(var: VariableFactorNoCacheCall) -> Self {
            IAaveRateOracleCalls::VariableFactorNoCache(var)
        }
    }
    impl ::std::convert::From<WriteOracleEntryCall> for IAaveRateOracleCalls {
        fn from(var: WriteOracleEntryCall) -> Self {
            IAaveRateOracleCalls::WriteOracleEntry(var)
        }
    }
    #[doc = "Container type for all return fields from the `UNDERLYING_YIELD_BEARING_PROTOCOL_ID` function with signature `UNDERLYING_YIELD_BEARING_PROTOCOL_ID()` and selector `[34, 255, 101, 104]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct UnderlyingYieldBearingProtocolIdReturn {
        pub yield_bearing_protocol_id: u8,
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
    #[doc = "Container type for all return fields from the `getApyFrom` function with signature `getApyFrom(uint256)` and selector `[23, 34, 30, 241]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetApyFromReturn {
        pub apy_from_to: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getApyFromTo` function with signature `getApyFromTo(uint256,uint256)` and selector `[147, 85, 109, 189]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetApyFromToReturn {
        pub apy_from_to: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getBlockSlope` function with signature `getBlockSlope()` and selector `[145, 170, 55, 93]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetBlockSlopeReturn {
        pub block_change: ethers::core::types::U256,
        pub time_change: u32,
    }
    #[doc = "Container type for all return fields from the `getCurrentRateInRay` function with signature `getCurrentRateInRay()` and selector `[239, 223, 93, 139]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetCurrentRateInRayReturn {
        pub current_rate: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getLastRateSlope` function with signature `getLastRateSlope()` and selector `[254, 17, 95, 190]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetLastRateSlopeReturn {
        pub rate_change: ethers::core::types::U256,
        pub time_change: u32,
    }
    #[doc = "Container type for all return fields from the `getRateFrom` function with signature `getRateFrom(uint256)` and selector `[22, 62, 156, 79]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetRateFromReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getRateFromTo` function with signature `getRateFromTo(uint256,uint256)` and selector `[247, 57, 103, 12]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetRateFromToReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `minSecondsSinceLastUpdate` function with signature `minSecondsSinceLastUpdate()` and selector `[124, 242, 204, 159]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MinSecondsSinceLastUpdateReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `underlying` function with signature `underlying()` and selector `[111, 48, 125, 195]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct UnderlyingReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `variableFactor` function with signature `variableFactor(uint256,uint256)` and selector `[37, 242, 88, 221]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct VariableFactorReturn {
        pub result: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `variableFactorNoCache` function with signature `variableFactorNoCache(uint256,uint256)` and selector `[65, 69, 53, 40]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct VariableFactorNoCacheReturn {
        pub result: ethers::core::types::U256,
    }
}
