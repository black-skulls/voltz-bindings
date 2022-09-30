pub use i_rate_oracle::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_rate_oracle {
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
    #[doc = "IRateOracle was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IRATEORACLE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"type\":\"error\",\"name\":\"AavePoolGetReserveNormalizedIncomeReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"AavePoolGetReserveNormalizedVariableDebtReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CTokenExchangeRateReturnedZero\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"unlocked\",\"type\":\"bool\",\"components\":[]}],\"type\":\"error\",\"name\":\"CanOnlyTradeIfUnlocked\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CannotLiquidate\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CannotSettleBeforeMaturity\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"DebugError\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"amount0\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"amount1\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"ExpectedOppositeSigns\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint160\",\"name\":\"sqrtPriceX96\",\"type\":\"uint160\",\"components\":[]}],\"type\":\"error\",\"name\":\"ExpectedSqrtPriceZeroBeforeInit\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"IRSNotionalAmountSpecifiedMustBeNonZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidMarginDelta\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"LidoGetPooledEthBySharesReturnedZero\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"type\":\"error\",\"name\":\"LiquidityDeltaMustBePositiveInBurn\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"type\":\"error\",\"name\":\"LiquidityDeltaMustBePositiveInMint\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginLessThanMinimum\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tick\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"variableTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"cumulativeFeeIncurred\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenDeltaUnbalanced\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginRequirementNotMet\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginRequirementNotMetFCM\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"requested\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"available\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"NotEnoughFunds\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OOO\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyFCM\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyMarginEngine\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyOwnerCanUpdatePosition\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyVAMM\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PositionNetZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PositionNotSettled\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"RocketPoolGetEthValueReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"WithdrawalExceedsCurrentMargin\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"closeToOrBeyondMaturity\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_minSecondsSinceLastUpdate\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MinSecondsSinceLastUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"blockTimestampScaled\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"source\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint16\",\"name\":\"index\",\"type\":\"uint16\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint32\",\"name\":\"blockTimestamp\",\"type\":\"uint32\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"observedValue\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint16\",\"name\":\"cardinality\",\"type\":\"uint16\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint16\",\"name\":\"cardinalityNext\",\"type\":\"uint16\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"OracleBufferUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"observationCardinalityNextNew\",\"type\":\"uint16\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RateCardinalityNext\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"UNDERLYING_YIELD_BEARING_PROTOCOL_ID\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"yieldBearingProtocolID\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"from\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getApyFrom\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"apyFromTo\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"from\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"to\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getApyFromTo\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"apyFromTo\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getBlockSlope\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"blockChange\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"timeChange\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCurrentRateInRay\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"currentRate\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLastRateSlope\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"rateChange\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"timeChange\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_from\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRateFrom\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_from\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_to\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRateFromTo\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"rateCardinalityNext\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"increaseObservationCardinalityNext\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"minSecondsSinceLastUpdate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_minSecondsSinceLastUpdate\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setMinSecondsSinceLastUpdate\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"underlying\",\"outputs\":[{\"internalType\":\"contract IERC20Minimal\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"termStartTimestamp\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"termEndTimestamp\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"variableFactor\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"result\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"termStartTimestamp\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"termEndTimestamp\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"variableFactorNoCache\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"result\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"writeOracleEntry\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    pub struct IRateOracle<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IRateOracle<M> {
        fn clone(&self) -> Self {
            IRateOracle(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IRateOracle<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IRateOracle<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IRateOracle))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IRateOracle<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IRATEORACLE_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `UNDERLYING_YIELD_BEARING_PROTOCOL_ID` (0x22ff6568) function"]
        pub fn underlying_yield_bearing_protocol_id(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([34, 255, 101, 104], ())
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, IRateOracleEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IRateOracle<M> {
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
    pub enum IRateOracleErrors {
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
    impl ethers::core::abi::AbiDecode for IRateOracleErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok (decoded) = < AavePoolGetReserveNormalizedIncomeReturnedZero as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (IRateOracleErrors :: AavePoolGetReserveNormalizedIncomeReturnedZero (decoded)) }
            if let Ok (decoded) = < AavePoolGetReserveNormalizedVariableDebtReturnedZero as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (IRateOracleErrors :: AavePoolGetReserveNormalizedVariableDebtReturnedZero (decoded)) }
            if let Ok(decoded) =
                <CTokenExchangeRateReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IRateOracleErrors::CTokenExchangeRateReturnedZero(decoded));
            }
            if let Ok(decoded) =
                <CanOnlyTradeIfUnlocked as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRateOracleErrors::CanOnlyTradeIfUnlocked(decoded));
            }
            if let Ok(decoded) =
                <CannotLiquidate as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRateOracleErrors::CannotLiquidate(decoded));
            }
            if let Ok(decoded) =
                <CannotSettleBeforeMaturity as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRateOracleErrors::CannotSettleBeforeMaturity(decoded));
            }
            if let Ok(decoded) = <DebugError as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRateOracleErrors::DebugError(decoded));
            }
            if let Ok(decoded) =
                <ExpectedOppositeSigns as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRateOracleErrors::ExpectedOppositeSigns(decoded));
            }
            if let Ok(decoded) =
                <ExpectedSqrtPriceZeroBeforeInit as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IRateOracleErrors::ExpectedSqrtPriceZeroBeforeInit(decoded));
            }
            if let Ok(decoded) =
                <IRSNotionalAmountSpecifiedMustBeNonZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IRateOracleErrors::IRSNotionalAmountSpecifiedMustBeNonZero(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <InvalidMarginDelta as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRateOracleErrors::InvalidMarginDelta(decoded));
            }
            if let Ok(decoded) =
                <LidoGetPooledEthBySharesReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IRateOracleErrors::LidoGetPooledEthBySharesReturnedZero(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <LiquidityDeltaMustBePositiveInBurn as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IRateOracleErrors::LiquidityDeltaMustBePositiveInBurn(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <LiquidityDeltaMustBePositiveInMint as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IRateOracleErrors::LiquidityDeltaMustBePositiveInMint(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <MarginLessThanMinimum as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRateOracleErrors::MarginLessThanMinimum(decoded));
            }
            if let Ok(decoded) =
                <MarginRequirementNotMet as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRateOracleErrors::MarginRequirementNotMet(decoded));
            }
            if let Ok(decoded) =
                <MarginRequirementNotMetFCM as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRateOracleErrors::MarginRequirementNotMetFCM(decoded));
            }
            if let Ok(decoded) =
                <NotEnoughFunds as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRateOracleErrors::NotEnoughFunds(decoded));
            }
            if let Ok(decoded) = <OOO as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IRateOracleErrors::OOO(decoded));
            }
            if let Ok(decoded) = <OnlyFCM as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IRateOracleErrors::OnlyFCM(decoded));
            }
            if let Ok(decoded) =
                <OnlyMarginEngine as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRateOracleErrors::OnlyMarginEngine(decoded));
            }
            if let Ok(decoded) =
                <OnlyOwnerCanUpdatePosition as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRateOracleErrors::OnlyOwnerCanUpdatePosition(decoded));
            }
            if let Ok(decoded) = <OnlyVAMM as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IRateOracleErrors::OnlyVAMM(decoded));
            }
            if let Ok(decoded) =
                <PositionNetZero as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRateOracleErrors::PositionNetZero(decoded));
            }
            if let Ok(decoded) =
                <PositionNotSettled as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRateOracleErrors::PositionNotSettled(decoded));
            }
            if let Ok(decoded) =
                <RocketPoolGetEthValueReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IRateOracleErrors::RocketPoolGetEthValueReturnedZero(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <WithdrawalExceedsCurrentMargin as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IRateOracleErrors::WithdrawalExceedsCurrentMargin(decoded));
            }
            if let Ok(decoded) =
                <closeToOrBeyondMaturity as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRateOracleErrors::closeToOrBeyondMaturity(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IRateOracleErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                IRateOracleErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(element) => {
                    element.encode()
                }
                IRateOracleErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(
                    element,
                ) => element.encode(),
                IRateOracleErrors::CTokenExchangeRateReturnedZero(element) => element.encode(),
                IRateOracleErrors::CanOnlyTradeIfUnlocked(element) => element.encode(),
                IRateOracleErrors::CannotLiquidate(element) => element.encode(),
                IRateOracleErrors::CannotSettleBeforeMaturity(element) => element.encode(),
                IRateOracleErrors::DebugError(element) => element.encode(),
                IRateOracleErrors::ExpectedOppositeSigns(element) => element.encode(),
                IRateOracleErrors::ExpectedSqrtPriceZeroBeforeInit(element) => element.encode(),
                IRateOracleErrors::IRSNotionalAmountSpecifiedMustBeNonZero(element) => {
                    element.encode()
                }
                IRateOracleErrors::InvalidMarginDelta(element) => element.encode(),
                IRateOracleErrors::LidoGetPooledEthBySharesReturnedZero(element) => {
                    element.encode()
                }
                IRateOracleErrors::LiquidityDeltaMustBePositiveInBurn(element) => element.encode(),
                IRateOracleErrors::LiquidityDeltaMustBePositiveInMint(element) => element.encode(),
                IRateOracleErrors::MarginLessThanMinimum(element) => element.encode(),
                IRateOracleErrors::MarginRequirementNotMet(element) => element.encode(),
                IRateOracleErrors::MarginRequirementNotMetFCM(element) => element.encode(),
                IRateOracleErrors::NotEnoughFunds(element) => element.encode(),
                IRateOracleErrors::OOO(element) => element.encode(),
                IRateOracleErrors::OnlyFCM(element) => element.encode(),
                IRateOracleErrors::OnlyMarginEngine(element) => element.encode(),
                IRateOracleErrors::OnlyOwnerCanUpdatePosition(element) => element.encode(),
                IRateOracleErrors::OnlyVAMM(element) => element.encode(),
                IRateOracleErrors::PositionNetZero(element) => element.encode(),
                IRateOracleErrors::PositionNotSettled(element) => element.encode(),
                IRateOracleErrors::RocketPoolGetEthValueReturnedZero(element) => element.encode(),
                IRateOracleErrors::WithdrawalExceedsCurrentMargin(element) => element.encode(),
                IRateOracleErrors::closeToOrBeyondMaturity(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IRateOracleErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IRateOracleErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(element) => {
                    element.fmt(f)
                }
                IRateOracleErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(
                    element,
                ) => element.fmt(f),
                IRateOracleErrors::CTokenExchangeRateReturnedZero(element) => element.fmt(f),
                IRateOracleErrors::CanOnlyTradeIfUnlocked(element) => element.fmt(f),
                IRateOracleErrors::CannotLiquidate(element) => element.fmt(f),
                IRateOracleErrors::CannotSettleBeforeMaturity(element) => element.fmt(f),
                IRateOracleErrors::DebugError(element) => element.fmt(f),
                IRateOracleErrors::ExpectedOppositeSigns(element) => element.fmt(f),
                IRateOracleErrors::ExpectedSqrtPriceZeroBeforeInit(element) => element.fmt(f),
                IRateOracleErrors::IRSNotionalAmountSpecifiedMustBeNonZero(element) => {
                    element.fmt(f)
                }
                IRateOracleErrors::InvalidMarginDelta(element) => element.fmt(f),
                IRateOracleErrors::LidoGetPooledEthBySharesReturnedZero(element) => element.fmt(f),
                IRateOracleErrors::LiquidityDeltaMustBePositiveInBurn(element) => element.fmt(f),
                IRateOracleErrors::LiquidityDeltaMustBePositiveInMint(element) => element.fmt(f),
                IRateOracleErrors::MarginLessThanMinimum(element) => element.fmt(f),
                IRateOracleErrors::MarginRequirementNotMet(element) => element.fmt(f),
                IRateOracleErrors::MarginRequirementNotMetFCM(element) => element.fmt(f),
                IRateOracleErrors::NotEnoughFunds(element) => element.fmt(f),
                IRateOracleErrors::OOO(element) => element.fmt(f),
                IRateOracleErrors::OnlyFCM(element) => element.fmt(f),
                IRateOracleErrors::OnlyMarginEngine(element) => element.fmt(f),
                IRateOracleErrors::OnlyOwnerCanUpdatePosition(element) => element.fmt(f),
                IRateOracleErrors::OnlyVAMM(element) => element.fmt(f),
                IRateOracleErrors::PositionNetZero(element) => element.fmt(f),
                IRateOracleErrors::PositionNotSettled(element) => element.fmt(f),
                IRateOracleErrors::RocketPoolGetEthValueReturnedZero(element) => element.fmt(f),
                IRateOracleErrors::WithdrawalExceedsCurrentMargin(element) => element.fmt(f),
                IRateOracleErrors::closeToOrBeyondMaturity(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AavePoolGetReserveNormalizedIncomeReturnedZero> for IRateOracleErrors {
        fn from(var: AavePoolGetReserveNormalizedIncomeReturnedZero) -> Self {
            IRateOracleErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(var)
        }
    }
    impl ::std::convert::From<AavePoolGetReserveNormalizedVariableDebtReturnedZero>
        for IRateOracleErrors
    {
        fn from(var: AavePoolGetReserveNormalizedVariableDebtReturnedZero) -> Self {
            IRateOracleErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(var)
        }
    }
    impl ::std::convert::From<CTokenExchangeRateReturnedZero> for IRateOracleErrors {
        fn from(var: CTokenExchangeRateReturnedZero) -> Self {
            IRateOracleErrors::CTokenExchangeRateReturnedZero(var)
        }
    }
    impl ::std::convert::From<CanOnlyTradeIfUnlocked> for IRateOracleErrors {
        fn from(var: CanOnlyTradeIfUnlocked) -> Self {
            IRateOracleErrors::CanOnlyTradeIfUnlocked(var)
        }
    }
    impl ::std::convert::From<CannotLiquidate> for IRateOracleErrors {
        fn from(var: CannotLiquidate) -> Self {
            IRateOracleErrors::CannotLiquidate(var)
        }
    }
    impl ::std::convert::From<CannotSettleBeforeMaturity> for IRateOracleErrors {
        fn from(var: CannotSettleBeforeMaturity) -> Self {
            IRateOracleErrors::CannotSettleBeforeMaturity(var)
        }
    }
    impl ::std::convert::From<DebugError> for IRateOracleErrors {
        fn from(var: DebugError) -> Self {
            IRateOracleErrors::DebugError(var)
        }
    }
    impl ::std::convert::From<ExpectedOppositeSigns> for IRateOracleErrors {
        fn from(var: ExpectedOppositeSigns) -> Self {
            IRateOracleErrors::ExpectedOppositeSigns(var)
        }
    }
    impl ::std::convert::From<ExpectedSqrtPriceZeroBeforeInit> for IRateOracleErrors {
        fn from(var: ExpectedSqrtPriceZeroBeforeInit) -> Self {
            IRateOracleErrors::ExpectedSqrtPriceZeroBeforeInit(var)
        }
    }
    impl ::std::convert::From<IRSNotionalAmountSpecifiedMustBeNonZero> for IRateOracleErrors {
        fn from(var: IRSNotionalAmountSpecifiedMustBeNonZero) -> Self {
            IRateOracleErrors::IRSNotionalAmountSpecifiedMustBeNonZero(var)
        }
    }
    impl ::std::convert::From<InvalidMarginDelta> for IRateOracleErrors {
        fn from(var: InvalidMarginDelta) -> Self {
            IRateOracleErrors::InvalidMarginDelta(var)
        }
    }
    impl ::std::convert::From<LidoGetPooledEthBySharesReturnedZero> for IRateOracleErrors {
        fn from(var: LidoGetPooledEthBySharesReturnedZero) -> Self {
            IRateOracleErrors::LidoGetPooledEthBySharesReturnedZero(var)
        }
    }
    impl ::std::convert::From<LiquidityDeltaMustBePositiveInBurn> for IRateOracleErrors {
        fn from(var: LiquidityDeltaMustBePositiveInBurn) -> Self {
            IRateOracleErrors::LiquidityDeltaMustBePositiveInBurn(var)
        }
    }
    impl ::std::convert::From<LiquidityDeltaMustBePositiveInMint> for IRateOracleErrors {
        fn from(var: LiquidityDeltaMustBePositiveInMint) -> Self {
            IRateOracleErrors::LiquidityDeltaMustBePositiveInMint(var)
        }
    }
    impl ::std::convert::From<MarginLessThanMinimum> for IRateOracleErrors {
        fn from(var: MarginLessThanMinimum) -> Self {
            IRateOracleErrors::MarginLessThanMinimum(var)
        }
    }
    impl ::std::convert::From<MarginRequirementNotMet> for IRateOracleErrors {
        fn from(var: MarginRequirementNotMet) -> Self {
            IRateOracleErrors::MarginRequirementNotMet(var)
        }
    }
    impl ::std::convert::From<MarginRequirementNotMetFCM> for IRateOracleErrors {
        fn from(var: MarginRequirementNotMetFCM) -> Self {
            IRateOracleErrors::MarginRequirementNotMetFCM(var)
        }
    }
    impl ::std::convert::From<NotEnoughFunds> for IRateOracleErrors {
        fn from(var: NotEnoughFunds) -> Self {
            IRateOracleErrors::NotEnoughFunds(var)
        }
    }
    impl ::std::convert::From<OOO> for IRateOracleErrors {
        fn from(var: OOO) -> Self {
            IRateOracleErrors::OOO(var)
        }
    }
    impl ::std::convert::From<OnlyFCM> for IRateOracleErrors {
        fn from(var: OnlyFCM) -> Self {
            IRateOracleErrors::OnlyFCM(var)
        }
    }
    impl ::std::convert::From<OnlyMarginEngine> for IRateOracleErrors {
        fn from(var: OnlyMarginEngine) -> Self {
            IRateOracleErrors::OnlyMarginEngine(var)
        }
    }
    impl ::std::convert::From<OnlyOwnerCanUpdatePosition> for IRateOracleErrors {
        fn from(var: OnlyOwnerCanUpdatePosition) -> Self {
            IRateOracleErrors::OnlyOwnerCanUpdatePosition(var)
        }
    }
    impl ::std::convert::From<OnlyVAMM> for IRateOracleErrors {
        fn from(var: OnlyVAMM) -> Self {
            IRateOracleErrors::OnlyVAMM(var)
        }
    }
    impl ::std::convert::From<PositionNetZero> for IRateOracleErrors {
        fn from(var: PositionNetZero) -> Self {
            IRateOracleErrors::PositionNetZero(var)
        }
    }
    impl ::std::convert::From<PositionNotSettled> for IRateOracleErrors {
        fn from(var: PositionNotSettled) -> Self {
            IRateOracleErrors::PositionNotSettled(var)
        }
    }
    impl ::std::convert::From<RocketPoolGetEthValueReturnedZero> for IRateOracleErrors {
        fn from(var: RocketPoolGetEthValueReturnedZero) -> Self {
            IRateOracleErrors::RocketPoolGetEthValueReturnedZero(var)
        }
    }
    impl ::std::convert::From<WithdrawalExceedsCurrentMargin> for IRateOracleErrors {
        fn from(var: WithdrawalExceedsCurrentMargin) -> Self {
            IRateOracleErrors::WithdrawalExceedsCurrentMargin(var)
        }
    }
    impl ::std::convert::From<closeToOrBeyondMaturity> for IRateOracleErrors {
        fn from(var: closeToOrBeyondMaturity) -> Self {
            IRateOracleErrors::closeToOrBeyondMaturity(var)
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
    pub enum IRateOracleEvents {
        MinSecondsSinceLastUpdateFilter(MinSecondsSinceLastUpdateFilter),
        OracleBufferUpdateFilter(OracleBufferUpdateFilter),
        RateCardinalityNextFilter(RateCardinalityNextFilter),
    }
    impl ethers::contract::EthLogDecode for IRateOracleEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = MinSecondsSinceLastUpdateFilter::decode_log(log) {
                return Ok(IRateOracleEvents::MinSecondsSinceLastUpdateFilter(decoded));
            }
            if let Ok(decoded) = OracleBufferUpdateFilter::decode_log(log) {
                return Ok(IRateOracleEvents::OracleBufferUpdateFilter(decoded));
            }
            if let Ok(decoded) = RateCardinalityNextFilter::decode_log(log) {
                return Ok(IRateOracleEvents::RateCardinalityNextFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IRateOracleEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IRateOracleEvents::MinSecondsSinceLastUpdateFilter(element) => element.fmt(f),
                IRateOracleEvents::OracleBufferUpdateFilter(element) => element.fmt(f),
                IRateOracleEvents::RateCardinalityNextFilter(element) => element.fmt(f),
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
    pub enum IRateOracleCalls {
        UnderlyingYieldBearingProtocolId(UnderlyingYieldBearingProtocolIdCall),
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
    impl ethers::core::abi::AbiDecode for IRateOracleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <UnderlyingYieldBearingProtocolIdCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IRateOracleCalls::UnderlyingYieldBearingProtocolId(decoded));
            }
            if let Ok(decoded) =
                <GetApyFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRateOracleCalls::GetApyFrom(decoded));
            }
            if let Ok(decoded) =
                <GetApyFromToCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRateOracleCalls::GetApyFromTo(decoded));
            }
            if let Ok(decoded) =
                <GetBlockSlopeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRateOracleCalls::GetBlockSlope(decoded));
            }
            if let Ok(decoded) =
                <GetCurrentRateInRayCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRateOracleCalls::GetCurrentRateInRay(decoded));
            }
            if let Ok(decoded) =
                <GetLastRateSlopeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRateOracleCalls::GetLastRateSlope(decoded));
            }
            if let Ok(decoded) =
                <GetRateFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRateOracleCalls::GetRateFrom(decoded));
            }
            if let Ok(decoded) =
                <GetRateFromToCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRateOracleCalls::GetRateFromTo(decoded));
            }
            if let Ok(decoded) =
                <IncreaseObservationCardinalityNextCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IRateOracleCalls::IncreaseObservationCardinalityNext(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <MinSecondsSinceLastUpdateCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IRateOracleCalls::MinSecondsSinceLastUpdate(decoded));
            }
            if let Ok(decoded) =
                <SetMinSecondsSinceLastUpdateCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IRateOracleCalls::SetMinSecondsSinceLastUpdate(decoded));
            }
            if let Ok(decoded) =
                <UnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRateOracleCalls::Underlying(decoded));
            }
            if let Ok(decoded) =
                <VariableFactorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRateOracleCalls::VariableFactor(decoded));
            }
            if let Ok(decoded) =
                <VariableFactorNoCacheCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRateOracleCalls::VariableFactorNoCache(decoded));
            }
            if let Ok(decoded) =
                <WriteOracleEntryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRateOracleCalls::WriteOracleEntry(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IRateOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IRateOracleCalls::UnderlyingYieldBearingProtocolId(element) => element.encode(),
                IRateOracleCalls::GetApyFrom(element) => element.encode(),
                IRateOracleCalls::GetApyFromTo(element) => element.encode(),
                IRateOracleCalls::GetBlockSlope(element) => element.encode(),
                IRateOracleCalls::GetCurrentRateInRay(element) => element.encode(),
                IRateOracleCalls::GetLastRateSlope(element) => element.encode(),
                IRateOracleCalls::GetRateFrom(element) => element.encode(),
                IRateOracleCalls::GetRateFromTo(element) => element.encode(),
                IRateOracleCalls::IncreaseObservationCardinalityNext(element) => element.encode(),
                IRateOracleCalls::MinSecondsSinceLastUpdate(element) => element.encode(),
                IRateOracleCalls::SetMinSecondsSinceLastUpdate(element) => element.encode(),
                IRateOracleCalls::Underlying(element) => element.encode(),
                IRateOracleCalls::VariableFactor(element) => element.encode(),
                IRateOracleCalls::VariableFactorNoCache(element) => element.encode(),
                IRateOracleCalls::WriteOracleEntry(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IRateOracleCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IRateOracleCalls::UnderlyingYieldBearingProtocolId(element) => element.fmt(f),
                IRateOracleCalls::GetApyFrom(element) => element.fmt(f),
                IRateOracleCalls::GetApyFromTo(element) => element.fmt(f),
                IRateOracleCalls::GetBlockSlope(element) => element.fmt(f),
                IRateOracleCalls::GetCurrentRateInRay(element) => element.fmt(f),
                IRateOracleCalls::GetLastRateSlope(element) => element.fmt(f),
                IRateOracleCalls::GetRateFrom(element) => element.fmt(f),
                IRateOracleCalls::GetRateFromTo(element) => element.fmt(f),
                IRateOracleCalls::IncreaseObservationCardinalityNext(element) => element.fmt(f),
                IRateOracleCalls::MinSecondsSinceLastUpdate(element) => element.fmt(f),
                IRateOracleCalls::SetMinSecondsSinceLastUpdate(element) => element.fmt(f),
                IRateOracleCalls::Underlying(element) => element.fmt(f),
                IRateOracleCalls::VariableFactor(element) => element.fmt(f),
                IRateOracleCalls::VariableFactorNoCache(element) => element.fmt(f),
                IRateOracleCalls::WriteOracleEntry(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<UnderlyingYieldBearingProtocolIdCall> for IRateOracleCalls {
        fn from(var: UnderlyingYieldBearingProtocolIdCall) -> Self {
            IRateOracleCalls::UnderlyingYieldBearingProtocolId(var)
        }
    }
    impl ::std::convert::From<GetApyFromCall> for IRateOracleCalls {
        fn from(var: GetApyFromCall) -> Self {
            IRateOracleCalls::GetApyFrom(var)
        }
    }
    impl ::std::convert::From<GetApyFromToCall> for IRateOracleCalls {
        fn from(var: GetApyFromToCall) -> Self {
            IRateOracleCalls::GetApyFromTo(var)
        }
    }
    impl ::std::convert::From<GetBlockSlopeCall> for IRateOracleCalls {
        fn from(var: GetBlockSlopeCall) -> Self {
            IRateOracleCalls::GetBlockSlope(var)
        }
    }
    impl ::std::convert::From<GetCurrentRateInRayCall> for IRateOracleCalls {
        fn from(var: GetCurrentRateInRayCall) -> Self {
            IRateOracleCalls::GetCurrentRateInRay(var)
        }
    }
    impl ::std::convert::From<GetLastRateSlopeCall> for IRateOracleCalls {
        fn from(var: GetLastRateSlopeCall) -> Self {
            IRateOracleCalls::GetLastRateSlope(var)
        }
    }
    impl ::std::convert::From<GetRateFromCall> for IRateOracleCalls {
        fn from(var: GetRateFromCall) -> Self {
            IRateOracleCalls::GetRateFrom(var)
        }
    }
    impl ::std::convert::From<GetRateFromToCall> for IRateOracleCalls {
        fn from(var: GetRateFromToCall) -> Self {
            IRateOracleCalls::GetRateFromTo(var)
        }
    }
    impl ::std::convert::From<IncreaseObservationCardinalityNextCall> for IRateOracleCalls {
        fn from(var: IncreaseObservationCardinalityNextCall) -> Self {
            IRateOracleCalls::IncreaseObservationCardinalityNext(var)
        }
    }
    impl ::std::convert::From<MinSecondsSinceLastUpdateCall> for IRateOracleCalls {
        fn from(var: MinSecondsSinceLastUpdateCall) -> Self {
            IRateOracleCalls::MinSecondsSinceLastUpdate(var)
        }
    }
    impl ::std::convert::From<SetMinSecondsSinceLastUpdateCall> for IRateOracleCalls {
        fn from(var: SetMinSecondsSinceLastUpdateCall) -> Self {
            IRateOracleCalls::SetMinSecondsSinceLastUpdate(var)
        }
    }
    impl ::std::convert::From<UnderlyingCall> for IRateOracleCalls {
        fn from(var: UnderlyingCall) -> Self {
            IRateOracleCalls::Underlying(var)
        }
    }
    impl ::std::convert::From<VariableFactorCall> for IRateOracleCalls {
        fn from(var: VariableFactorCall) -> Self {
            IRateOracleCalls::VariableFactor(var)
        }
    }
    impl ::std::convert::From<VariableFactorNoCacheCall> for IRateOracleCalls {
        fn from(var: VariableFactorNoCacheCall) -> Self {
            IRateOracleCalls::VariableFactorNoCache(var)
        }
    }
    impl ::std::convert::From<WriteOracleEntryCall> for IRateOracleCalls {
        fn from(var: WriteOracleEntryCall) -> Self {
            IRateOracleCalls::WriteOracleEntry(var)
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
