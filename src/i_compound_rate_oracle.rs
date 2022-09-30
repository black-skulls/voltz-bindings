pub use i_compound_rate_oracle::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_compound_rate_oracle {
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
    #[doc = "ICompoundRateOracle was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ICOMPOUNDRATEORACLE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"type\":\"error\",\"name\":\"AavePoolGetReserveNormalizedIncomeReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"AavePoolGetReserveNormalizedVariableDebtReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CTokenExchangeRateReturnedZero\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"unlocked\",\"type\":\"bool\",\"components\":[]}],\"type\":\"error\",\"name\":\"CanOnlyTradeIfUnlocked\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CannotLiquidate\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CannotSettleBeforeMaturity\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"DebugError\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"amount0\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"amount1\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"ExpectedOppositeSigns\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint160\",\"name\":\"sqrtPriceX96\",\"type\":\"uint160\",\"components\":[]}],\"type\":\"error\",\"name\":\"ExpectedSqrtPriceZeroBeforeInit\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"IRSNotionalAmountSpecifiedMustBeNonZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidMarginDelta\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"LidoGetPooledEthBySharesReturnedZero\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"type\":\"error\",\"name\":\"LiquidityDeltaMustBePositiveInBurn\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"type\":\"error\",\"name\":\"LiquidityDeltaMustBePositiveInMint\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginLessThanMinimum\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tick\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"variableTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"cumulativeFeeIncurred\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenDeltaUnbalanced\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginRequirementNotMet\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginRequirementNotMetFCM\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"requested\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"available\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"NotEnoughFunds\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OOO\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyFCM\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyMarginEngine\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyOwnerCanUpdatePosition\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyVAMM\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PositionNetZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PositionNotSettled\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"RocketPoolGetEthValueReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"WithdrawalExceedsCurrentMargin\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"closeToOrBeyondMaturity\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_minSecondsSinceLastUpdate\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MinSecondsSinceLastUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"blockTimestampScaled\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"source\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint16\",\"name\":\"index\",\"type\":\"uint16\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint32\",\"name\":\"blockTimestamp\",\"type\":\"uint32\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"observedValue\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint16\",\"name\":\"cardinality\",\"type\":\"uint16\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint16\",\"name\":\"cardinalityNext\",\"type\":\"uint16\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"OracleBufferUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"observationCardinalityNextNew\",\"type\":\"uint16\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RateCardinalityNext\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"UNDERLYING_YIELD_BEARING_PROTOCOL_ID\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"yieldBearingProtocolID\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ctoken\",\"outputs\":[{\"internalType\":\"contract ICToken\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"from\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getApyFrom\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"apyFromTo\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"from\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"to\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getApyFromTo\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"apyFromTo\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getBlockSlope\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"blockChange\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"timeChange\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCurrentRateInRay\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"currentRate\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLastRateSlope\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"rateChange\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"timeChange\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_from\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRateFrom\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_from\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_to\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRateFromTo\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"rateCardinalityNext\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"increaseObservationCardinalityNext\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"minSecondsSinceLastUpdate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_minSecondsSinceLastUpdate\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setMinSecondsSinceLastUpdate\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"underlying\",\"outputs\":[{\"internalType\":\"contract IERC20Minimal\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"termStartTimestamp\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"termEndTimestamp\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"variableFactor\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"result\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"termStartTimestamp\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"termEndTimestamp\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"variableFactorNoCache\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"result\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"writeOracleEntry\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    pub struct ICompoundRateOracle<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ICompoundRateOracle<M> {
        fn clone(&self) -> Self {
            ICompoundRateOracle(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ICompoundRateOracle<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ICompoundRateOracle<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ICompoundRateOracle))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ICompoundRateOracle<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ICOMPOUNDRATEORACLE_ABI.clone(), client)
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
        #[doc = "Calls the contract's `ctoken` (0xccf3eb9f) function"]
        pub fn ctoken(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([204, 243, 235, 159], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, ICompoundRateOracleEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for ICompoundRateOracle<M>
    {
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
    pub enum ICompoundRateOracleErrors {
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
    impl ethers::core::abi::AbiDecode for ICompoundRateOracleErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok (decoded) = < AavePoolGetReserveNormalizedIncomeReturnedZero as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (ICompoundRateOracleErrors :: AavePoolGetReserveNormalizedIncomeReturnedZero (decoded)) }
            if let Ok (decoded) = < AavePoolGetReserveNormalizedVariableDebtReturnedZero as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (ICompoundRateOracleErrors :: AavePoolGetReserveNormalizedVariableDebtReturnedZero (decoded)) }
            if let Ok(decoded) =
                <CTokenExchangeRateReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ICompoundRateOracleErrors::CTokenExchangeRateReturnedZero(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <CanOnlyTradeIfUnlocked as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICompoundRateOracleErrors::CanOnlyTradeIfUnlocked(decoded));
            }
            if let Ok(decoded) =
                <CannotLiquidate as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICompoundRateOracleErrors::CannotLiquidate(decoded));
            }
            if let Ok(decoded) =
                <CannotSettleBeforeMaturity as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICompoundRateOracleErrors::CannotSettleBeforeMaturity(
                    decoded,
                ));
            }
            if let Ok(decoded) = <DebugError as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICompoundRateOracleErrors::DebugError(decoded));
            }
            if let Ok(decoded) =
                <ExpectedOppositeSigns as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICompoundRateOracleErrors::ExpectedOppositeSigns(decoded));
            }
            if let Ok(decoded) =
                <ExpectedSqrtPriceZeroBeforeInit as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ICompoundRateOracleErrors::ExpectedSqrtPriceZeroBeforeInit(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <IRSNotionalAmountSpecifiedMustBeNonZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    ICompoundRateOracleErrors::IRSNotionalAmountSpecifiedMustBeNonZero(decoded),
                );
            }
            if let Ok(decoded) =
                <InvalidMarginDelta as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICompoundRateOracleErrors::InvalidMarginDelta(decoded));
            }
            if let Ok(decoded) =
                <LidoGetPooledEthBySharesReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ICompoundRateOracleErrors::LidoGetPooledEthBySharesReturnedZero(decoded));
            }
            if let Ok(decoded) =
                <LiquidityDeltaMustBePositiveInBurn as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ICompoundRateOracleErrors::LiquidityDeltaMustBePositiveInBurn(decoded));
            }
            if let Ok(decoded) =
                <LiquidityDeltaMustBePositiveInMint as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ICompoundRateOracleErrors::LiquidityDeltaMustBePositiveInMint(decoded));
            }
            if let Ok(decoded) =
                <MarginLessThanMinimum as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICompoundRateOracleErrors::MarginLessThanMinimum(decoded));
            }
            if let Ok(decoded) =
                <MarginRequirementNotMet as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICompoundRateOracleErrors::MarginRequirementNotMet(decoded));
            }
            if let Ok(decoded) =
                <MarginRequirementNotMetFCM as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICompoundRateOracleErrors::MarginRequirementNotMetFCM(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <NotEnoughFunds as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICompoundRateOracleErrors::NotEnoughFunds(decoded));
            }
            if let Ok(decoded) = <OOO as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ICompoundRateOracleErrors::OOO(decoded));
            }
            if let Ok(decoded) = <OnlyFCM as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ICompoundRateOracleErrors::OnlyFCM(decoded));
            }
            if let Ok(decoded) =
                <OnlyMarginEngine as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICompoundRateOracleErrors::OnlyMarginEngine(decoded));
            }
            if let Ok(decoded) =
                <OnlyOwnerCanUpdatePosition as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICompoundRateOracleErrors::OnlyOwnerCanUpdatePosition(
                    decoded,
                ));
            }
            if let Ok(decoded) = <OnlyVAMM as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ICompoundRateOracleErrors::OnlyVAMM(decoded));
            }
            if let Ok(decoded) =
                <PositionNetZero as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICompoundRateOracleErrors::PositionNetZero(decoded));
            }
            if let Ok(decoded) =
                <PositionNotSettled as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICompoundRateOracleErrors::PositionNotSettled(decoded));
            }
            if let Ok(decoded) =
                <RocketPoolGetEthValueReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ICompoundRateOracleErrors::RocketPoolGetEthValueReturnedZero(decoded));
            }
            if let Ok(decoded) =
                <WithdrawalExceedsCurrentMargin as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ICompoundRateOracleErrors::WithdrawalExceedsCurrentMargin(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <closeToOrBeyondMaturity as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICompoundRateOracleErrors::closeToOrBeyondMaturity(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ICompoundRateOracleErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                ICompoundRateOracleErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(
                    element,
                ) => element.encode(),
                ICompoundRateOracleErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(
                    element,
                ) => element.encode(),
                ICompoundRateOracleErrors::CTokenExchangeRateReturnedZero(element) => {
                    element.encode()
                }
                ICompoundRateOracleErrors::CanOnlyTradeIfUnlocked(element) => element.encode(),
                ICompoundRateOracleErrors::CannotLiquidate(element) => element.encode(),
                ICompoundRateOracleErrors::CannotSettleBeforeMaturity(element) => element.encode(),
                ICompoundRateOracleErrors::DebugError(element) => element.encode(),
                ICompoundRateOracleErrors::ExpectedOppositeSigns(element) => element.encode(),
                ICompoundRateOracleErrors::ExpectedSqrtPriceZeroBeforeInit(element) => {
                    element.encode()
                }
                ICompoundRateOracleErrors::IRSNotionalAmountSpecifiedMustBeNonZero(element) => {
                    element.encode()
                }
                ICompoundRateOracleErrors::InvalidMarginDelta(element) => element.encode(),
                ICompoundRateOracleErrors::LidoGetPooledEthBySharesReturnedZero(element) => {
                    element.encode()
                }
                ICompoundRateOracleErrors::LiquidityDeltaMustBePositiveInBurn(element) => {
                    element.encode()
                }
                ICompoundRateOracleErrors::LiquidityDeltaMustBePositiveInMint(element) => {
                    element.encode()
                }
                ICompoundRateOracleErrors::MarginLessThanMinimum(element) => element.encode(),
                ICompoundRateOracleErrors::MarginRequirementNotMet(element) => element.encode(),
                ICompoundRateOracleErrors::MarginRequirementNotMetFCM(element) => element.encode(),
                ICompoundRateOracleErrors::NotEnoughFunds(element) => element.encode(),
                ICompoundRateOracleErrors::OOO(element) => element.encode(),
                ICompoundRateOracleErrors::OnlyFCM(element) => element.encode(),
                ICompoundRateOracleErrors::OnlyMarginEngine(element) => element.encode(),
                ICompoundRateOracleErrors::OnlyOwnerCanUpdatePosition(element) => element.encode(),
                ICompoundRateOracleErrors::OnlyVAMM(element) => element.encode(),
                ICompoundRateOracleErrors::PositionNetZero(element) => element.encode(),
                ICompoundRateOracleErrors::PositionNotSettled(element) => element.encode(),
                ICompoundRateOracleErrors::RocketPoolGetEthValueReturnedZero(element) => {
                    element.encode()
                }
                ICompoundRateOracleErrors::WithdrawalExceedsCurrentMargin(element) => {
                    element.encode()
                }
                ICompoundRateOracleErrors::closeToOrBeyondMaturity(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ICompoundRateOracleErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ICompoundRateOracleErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(
                    element,
                ) => element.fmt(f),
                ICompoundRateOracleErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(
                    element,
                ) => element.fmt(f),
                ICompoundRateOracleErrors::CTokenExchangeRateReturnedZero(element) => {
                    element.fmt(f)
                }
                ICompoundRateOracleErrors::CanOnlyTradeIfUnlocked(element) => element.fmt(f),
                ICompoundRateOracleErrors::CannotLiquidate(element) => element.fmt(f),
                ICompoundRateOracleErrors::CannotSettleBeforeMaturity(element) => element.fmt(f),
                ICompoundRateOracleErrors::DebugError(element) => element.fmt(f),
                ICompoundRateOracleErrors::ExpectedOppositeSigns(element) => element.fmt(f),
                ICompoundRateOracleErrors::ExpectedSqrtPriceZeroBeforeInit(element) => {
                    element.fmt(f)
                }
                ICompoundRateOracleErrors::IRSNotionalAmountSpecifiedMustBeNonZero(element) => {
                    element.fmt(f)
                }
                ICompoundRateOracleErrors::InvalidMarginDelta(element) => element.fmt(f),
                ICompoundRateOracleErrors::LidoGetPooledEthBySharesReturnedZero(element) => {
                    element.fmt(f)
                }
                ICompoundRateOracleErrors::LiquidityDeltaMustBePositiveInBurn(element) => {
                    element.fmt(f)
                }
                ICompoundRateOracleErrors::LiquidityDeltaMustBePositiveInMint(element) => {
                    element.fmt(f)
                }
                ICompoundRateOracleErrors::MarginLessThanMinimum(element) => element.fmt(f),
                ICompoundRateOracleErrors::MarginRequirementNotMet(element) => element.fmt(f),
                ICompoundRateOracleErrors::MarginRequirementNotMetFCM(element) => element.fmt(f),
                ICompoundRateOracleErrors::NotEnoughFunds(element) => element.fmt(f),
                ICompoundRateOracleErrors::OOO(element) => element.fmt(f),
                ICompoundRateOracleErrors::OnlyFCM(element) => element.fmt(f),
                ICompoundRateOracleErrors::OnlyMarginEngine(element) => element.fmt(f),
                ICompoundRateOracleErrors::OnlyOwnerCanUpdatePosition(element) => element.fmt(f),
                ICompoundRateOracleErrors::OnlyVAMM(element) => element.fmt(f),
                ICompoundRateOracleErrors::PositionNetZero(element) => element.fmt(f),
                ICompoundRateOracleErrors::PositionNotSettled(element) => element.fmt(f),
                ICompoundRateOracleErrors::RocketPoolGetEthValueReturnedZero(element) => {
                    element.fmt(f)
                }
                ICompoundRateOracleErrors::WithdrawalExceedsCurrentMargin(element) => {
                    element.fmt(f)
                }
                ICompoundRateOracleErrors::closeToOrBeyondMaturity(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AavePoolGetReserveNormalizedIncomeReturnedZero>
        for ICompoundRateOracleErrors
    {
        fn from(var: AavePoolGetReserveNormalizedIncomeReturnedZero) -> Self {
            ICompoundRateOracleErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(var)
        }
    }
    impl ::std::convert::From<AavePoolGetReserveNormalizedVariableDebtReturnedZero>
        for ICompoundRateOracleErrors
    {
        fn from(var: AavePoolGetReserveNormalizedVariableDebtReturnedZero) -> Self {
            ICompoundRateOracleErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(var)
        }
    }
    impl ::std::convert::From<CTokenExchangeRateReturnedZero> for ICompoundRateOracleErrors {
        fn from(var: CTokenExchangeRateReturnedZero) -> Self {
            ICompoundRateOracleErrors::CTokenExchangeRateReturnedZero(var)
        }
    }
    impl ::std::convert::From<CanOnlyTradeIfUnlocked> for ICompoundRateOracleErrors {
        fn from(var: CanOnlyTradeIfUnlocked) -> Self {
            ICompoundRateOracleErrors::CanOnlyTradeIfUnlocked(var)
        }
    }
    impl ::std::convert::From<CannotLiquidate> for ICompoundRateOracleErrors {
        fn from(var: CannotLiquidate) -> Self {
            ICompoundRateOracleErrors::CannotLiquidate(var)
        }
    }
    impl ::std::convert::From<CannotSettleBeforeMaturity> for ICompoundRateOracleErrors {
        fn from(var: CannotSettleBeforeMaturity) -> Self {
            ICompoundRateOracleErrors::CannotSettleBeforeMaturity(var)
        }
    }
    impl ::std::convert::From<DebugError> for ICompoundRateOracleErrors {
        fn from(var: DebugError) -> Self {
            ICompoundRateOracleErrors::DebugError(var)
        }
    }
    impl ::std::convert::From<ExpectedOppositeSigns> for ICompoundRateOracleErrors {
        fn from(var: ExpectedOppositeSigns) -> Self {
            ICompoundRateOracleErrors::ExpectedOppositeSigns(var)
        }
    }
    impl ::std::convert::From<ExpectedSqrtPriceZeroBeforeInit> for ICompoundRateOracleErrors {
        fn from(var: ExpectedSqrtPriceZeroBeforeInit) -> Self {
            ICompoundRateOracleErrors::ExpectedSqrtPriceZeroBeforeInit(var)
        }
    }
    impl ::std::convert::From<IRSNotionalAmountSpecifiedMustBeNonZero> for ICompoundRateOracleErrors {
        fn from(var: IRSNotionalAmountSpecifiedMustBeNonZero) -> Self {
            ICompoundRateOracleErrors::IRSNotionalAmountSpecifiedMustBeNonZero(var)
        }
    }
    impl ::std::convert::From<InvalidMarginDelta> for ICompoundRateOracleErrors {
        fn from(var: InvalidMarginDelta) -> Self {
            ICompoundRateOracleErrors::InvalidMarginDelta(var)
        }
    }
    impl ::std::convert::From<LidoGetPooledEthBySharesReturnedZero> for ICompoundRateOracleErrors {
        fn from(var: LidoGetPooledEthBySharesReturnedZero) -> Self {
            ICompoundRateOracleErrors::LidoGetPooledEthBySharesReturnedZero(var)
        }
    }
    impl ::std::convert::From<LiquidityDeltaMustBePositiveInBurn> for ICompoundRateOracleErrors {
        fn from(var: LiquidityDeltaMustBePositiveInBurn) -> Self {
            ICompoundRateOracleErrors::LiquidityDeltaMustBePositiveInBurn(var)
        }
    }
    impl ::std::convert::From<LiquidityDeltaMustBePositiveInMint> for ICompoundRateOracleErrors {
        fn from(var: LiquidityDeltaMustBePositiveInMint) -> Self {
            ICompoundRateOracleErrors::LiquidityDeltaMustBePositiveInMint(var)
        }
    }
    impl ::std::convert::From<MarginLessThanMinimum> for ICompoundRateOracleErrors {
        fn from(var: MarginLessThanMinimum) -> Self {
            ICompoundRateOracleErrors::MarginLessThanMinimum(var)
        }
    }
    impl ::std::convert::From<MarginRequirementNotMet> for ICompoundRateOracleErrors {
        fn from(var: MarginRequirementNotMet) -> Self {
            ICompoundRateOracleErrors::MarginRequirementNotMet(var)
        }
    }
    impl ::std::convert::From<MarginRequirementNotMetFCM> for ICompoundRateOracleErrors {
        fn from(var: MarginRequirementNotMetFCM) -> Self {
            ICompoundRateOracleErrors::MarginRequirementNotMetFCM(var)
        }
    }
    impl ::std::convert::From<NotEnoughFunds> for ICompoundRateOracleErrors {
        fn from(var: NotEnoughFunds) -> Self {
            ICompoundRateOracleErrors::NotEnoughFunds(var)
        }
    }
    impl ::std::convert::From<OOO> for ICompoundRateOracleErrors {
        fn from(var: OOO) -> Self {
            ICompoundRateOracleErrors::OOO(var)
        }
    }
    impl ::std::convert::From<OnlyFCM> for ICompoundRateOracleErrors {
        fn from(var: OnlyFCM) -> Self {
            ICompoundRateOracleErrors::OnlyFCM(var)
        }
    }
    impl ::std::convert::From<OnlyMarginEngine> for ICompoundRateOracleErrors {
        fn from(var: OnlyMarginEngine) -> Self {
            ICompoundRateOracleErrors::OnlyMarginEngine(var)
        }
    }
    impl ::std::convert::From<OnlyOwnerCanUpdatePosition> for ICompoundRateOracleErrors {
        fn from(var: OnlyOwnerCanUpdatePosition) -> Self {
            ICompoundRateOracleErrors::OnlyOwnerCanUpdatePosition(var)
        }
    }
    impl ::std::convert::From<OnlyVAMM> for ICompoundRateOracleErrors {
        fn from(var: OnlyVAMM) -> Self {
            ICompoundRateOracleErrors::OnlyVAMM(var)
        }
    }
    impl ::std::convert::From<PositionNetZero> for ICompoundRateOracleErrors {
        fn from(var: PositionNetZero) -> Self {
            ICompoundRateOracleErrors::PositionNetZero(var)
        }
    }
    impl ::std::convert::From<PositionNotSettled> for ICompoundRateOracleErrors {
        fn from(var: PositionNotSettled) -> Self {
            ICompoundRateOracleErrors::PositionNotSettled(var)
        }
    }
    impl ::std::convert::From<RocketPoolGetEthValueReturnedZero> for ICompoundRateOracleErrors {
        fn from(var: RocketPoolGetEthValueReturnedZero) -> Self {
            ICompoundRateOracleErrors::RocketPoolGetEthValueReturnedZero(var)
        }
    }
    impl ::std::convert::From<WithdrawalExceedsCurrentMargin> for ICompoundRateOracleErrors {
        fn from(var: WithdrawalExceedsCurrentMargin) -> Self {
            ICompoundRateOracleErrors::WithdrawalExceedsCurrentMargin(var)
        }
    }
    impl ::std::convert::From<closeToOrBeyondMaturity> for ICompoundRateOracleErrors {
        fn from(var: closeToOrBeyondMaturity) -> Self {
            ICompoundRateOracleErrors::closeToOrBeyondMaturity(var)
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
    pub enum ICompoundRateOracleEvents {
        MinSecondsSinceLastUpdateFilter(MinSecondsSinceLastUpdateFilter),
        OracleBufferUpdateFilter(OracleBufferUpdateFilter),
        RateCardinalityNextFilter(RateCardinalityNextFilter),
    }
    impl ethers::contract::EthLogDecode for ICompoundRateOracleEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = MinSecondsSinceLastUpdateFilter::decode_log(log) {
                return Ok(ICompoundRateOracleEvents::MinSecondsSinceLastUpdateFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = OracleBufferUpdateFilter::decode_log(log) {
                return Ok(ICompoundRateOracleEvents::OracleBufferUpdateFilter(decoded));
            }
            if let Ok(decoded) = RateCardinalityNextFilter::decode_log(log) {
                return Ok(ICompoundRateOracleEvents::RateCardinalityNextFilter(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ICompoundRateOracleEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ICompoundRateOracleEvents::MinSecondsSinceLastUpdateFilter(element) => {
                    element.fmt(f)
                }
                ICompoundRateOracleEvents::OracleBufferUpdateFilter(element) => element.fmt(f),
                ICompoundRateOracleEvents::RateCardinalityNextFilter(element) => element.fmt(f),
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
    #[doc = "Container type for all input parameters for the `ctoken` function with signature `ctoken()` and selector `[204, 243, 235, 159]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "ctoken", abi = "ctoken()")]
    pub struct CtokenCall;
    #[doc = "Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `[49, 60, 229, 103]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
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
    pub enum ICompoundRateOracleCalls {
        UnderlyingYieldBearingProtocolId(UnderlyingYieldBearingProtocolIdCall),
        Ctoken(CtokenCall),
        Decimals(DecimalsCall),
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
    impl ethers::core::abi::AbiDecode for ICompoundRateOracleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <UnderlyingYieldBearingProtocolIdCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ICompoundRateOracleCalls::UnderlyingYieldBearingProtocolId(
                    decoded,
                ));
            }
            if let Ok(decoded) = <CtokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICompoundRateOracleCalls::Ctoken(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICompoundRateOracleCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <GetApyFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICompoundRateOracleCalls::GetApyFrom(decoded));
            }
            if let Ok(decoded) =
                <GetApyFromToCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICompoundRateOracleCalls::GetApyFromTo(decoded));
            }
            if let Ok(decoded) =
                <GetBlockSlopeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICompoundRateOracleCalls::GetBlockSlope(decoded));
            }
            if let Ok(decoded) =
                <GetCurrentRateInRayCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICompoundRateOracleCalls::GetCurrentRateInRay(decoded));
            }
            if let Ok(decoded) =
                <GetLastRateSlopeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICompoundRateOracleCalls::GetLastRateSlope(decoded));
            }
            if let Ok(decoded) =
                <GetRateFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICompoundRateOracleCalls::GetRateFrom(decoded));
            }
            if let Ok(decoded) =
                <GetRateFromToCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICompoundRateOracleCalls::GetRateFromTo(decoded));
            }
            if let Ok(decoded) =
                <IncreaseObservationCardinalityNextCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ICompoundRateOracleCalls::IncreaseObservationCardinalityNext(decoded));
            }
            if let Ok(decoded) =
                <MinSecondsSinceLastUpdateCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ICompoundRateOracleCalls::MinSecondsSinceLastUpdate(decoded));
            }
            if let Ok(decoded) =
                <SetMinSecondsSinceLastUpdateCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ICompoundRateOracleCalls::SetMinSecondsSinceLastUpdate(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <UnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICompoundRateOracleCalls::Underlying(decoded));
            }
            if let Ok(decoded) =
                <VariableFactorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICompoundRateOracleCalls::VariableFactor(decoded));
            }
            if let Ok(decoded) =
                <VariableFactorNoCacheCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICompoundRateOracleCalls::VariableFactorNoCache(decoded));
            }
            if let Ok(decoded) =
                <WriteOracleEntryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICompoundRateOracleCalls::WriteOracleEntry(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ICompoundRateOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ICompoundRateOracleCalls::UnderlyingYieldBearingProtocolId(element) => {
                    element.encode()
                }
                ICompoundRateOracleCalls::Ctoken(element) => element.encode(),
                ICompoundRateOracleCalls::Decimals(element) => element.encode(),
                ICompoundRateOracleCalls::GetApyFrom(element) => element.encode(),
                ICompoundRateOracleCalls::GetApyFromTo(element) => element.encode(),
                ICompoundRateOracleCalls::GetBlockSlope(element) => element.encode(),
                ICompoundRateOracleCalls::GetCurrentRateInRay(element) => element.encode(),
                ICompoundRateOracleCalls::GetLastRateSlope(element) => element.encode(),
                ICompoundRateOracleCalls::GetRateFrom(element) => element.encode(),
                ICompoundRateOracleCalls::GetRateFromTo(element) => element.encode(),
                ICompoundRateOracleCalls::IncreaseObservationCardinalityNext(element) => {
                    element.encode()
                }
                ICompoundRateOracleCalls::MinSecondsSinceLastUpdate(element) => element.encode(),
                ICompoundRateOracleCalls::SetMinSecondsSinceLastUpdate(element) => element.encode(),
                ICompoundRateOracleCalls::Underlying(element) => element.encode(),
                ICompoundRateOracleCalls::VariableFactor(element) => element.encode(),
                ICompoundRateOracleCalls::VariableFactorNoCache(element) => element.encode(),
                ICompoundRateOracleCalls::WriteOracleEntry(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ICompoundRateOracleCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ICompoundRateOracleCalls::UnderlyingYieldBearingProtocolId(element) => {
                    element.fmt(f)
                }
                ICompoundRateOracleCalls::Ctoken(element) => element.fmt(f),
                ICompoundRateOracleCalls::Decimals(element) => element.fmt(f),
                ICompoundRateOracleCalls::GetApyFrom(element) => element.fmt(f),
                ICompoundRateOracleCalls::GetApyFromTo(element) => element.fmt(f),
                ICompoundRateOracleCalls::GetBlockSlope(element) => element.fmt(f),
                ICompoundRateOracleCalls::GetCurrentRateInRay(element) => element.fmt(f),
                ICompoundRateOracleCalls::GetLastRateSlope(element) => element.fmt(f),
                ICompoundRateOracleCalls::GetRateFrom(element) => element.fmt(f),
                ICompoundRateOracleCalls::GetRateFromTo(element) => element.fmt(f),
                ICompoundRateOracleCalls::IncreaseObservationCardinalityNext(element) => {
                    element.fmt(f)
                }
                ICompoundRateOracleCalls::MinSecondsSinceLastUpdate(element) => element.fmt(f),
                ICompoundRateOracleCalls::SetMinSecondsSinceLastUpdate(element) => element.fmt(f),
                ICompoundRateOracleCalls::Underlying(element) => element.fmt(f),
                ICompoundRateOracleCalls::VariableFactor(element) => element.fmt(f),
                ICompoundRateOracleCalls::VariableFactorNoCache(element) => element.fmt(f),
                ICompoundRateOracleCalls::WriteOracleEntry(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<UnderlyingYieldBearingProtocolIdCall> for ICompoundRateOracleCalls {
        fn from(var: UnderlyingYieldBearingProtocolIdCall) -> Self {
            ICompoundRateOracleCalls::UnderlyingYieldBearingProtocolId(var)
        }
    }
    impl ::std::convert::From<CtokenCall> for ICompoundRateOracleCalls {
        fn from(var: CtokenCall) -> Self {
            ICompoundRateOracleCalls::Ctoken(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for ICompoundRateOracleCalls {
        fn from(var: DecimalsCall) -> Self {
            ICompoundRateOracleCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<GetApyFromCall> for ICompoundRateOracleCalls {
        fn from(var: GetApyFromCall) -> Self {
            ICompoundRateOracleCalls::GetApyFrom(var)
        }
    }
    impl ::std::convert::From<GetApyFromToCall> for ICompoundRateOracleCalls {
        fn from(var: GetApyFromToCall) -> Self {
            ICompoundRateOracleCalls::GetApyFromTo(var)
        }
    }
    impl ::std::convert::From<GetBlockSlopeCall> for ICompoundRateOracleCalls {
        fn from(var: GetBlockSlopeCall) -> Self {
            ICompoundRateOracleCalls::GetBlockSlope(var)
        }
    }
    impl ::std::convert::From<GetCurrentRateInRayCall> for ICompoundRateOracleCalls {
        fn from(var: GetCurrentRateInRayCall) -> Self {
            ICompoundRateOracleCalls::GetCurrentRateInRay(var)
        }
    }
    impl ::std::convert::From<GetLastRateSlopeCall> for ICompoundRateOracleCalls {
        fn from(var: GetLastRateSlopeCall) -> Self {
            ICompoundRateOracleCalls::GetLastRateSlope(var)
        }
    }
    impl ::std::convert::From<GetRateFromCall> for ICompoundRateOracleCalls {
        fn from(var: GetRateFromCall) -> Self {
            ICompoundRateOracleCalls::GetRateFrom(var)
        }
    }
    impl ::std::convert::From<GetRateFromToCall> for ICompoundRateOracleCalls {
        fn from(var: GetRateFromToCall) -> Self {
            ICompoundRateOracleCalls::GetRateFromTo(var)
        }
    }
    impl ::std::convert::From<IncreaseObservationCardinalityNextCall> for ICompoundRateOracleCalls {
        fn from(var: IncreaseObservationCardinalityNextCall) -> Self {
            ICompoundRateOracleCalls::IncreaseObservationCardinalityNext(var)
        }
    }
    impl ::std::convert::From<MinSecondsSinceLastUpdateCall> for ICompoundRateOracleCalls {
        fn from(var: MinSecondsSinceLastUpdateCall) -> Self {
            ICompoundRateOracleCalls::MinSecondsSinceLastUpdate(var)
        }
    }
    impl ::std::convert::From<SetMinSecondsSinceLastUpdateCall> for ICompoundRateOracleCalls {
        fn from(var: SetMinSecondsSinceLastUpdateCall) -> Self {
            ICompoundRateOracleCalls::SetMinSecondsSinceLastUpdate(var)
        }
    }
    impl ::std::convert::From<UnderlyingCall> for ICompoundRateOracleCalls {
        fn from(var: UnderlyingCall) -> Self {
            ICompoundRateOracleCalls::Underlying(var)
        }
    }
    impl ::std::convert::From<VariableFactorCall> for ICompoundRateOracleCalls {
        fn from(var: VariableFactorCall) -> Self {
            ICompoundRateOracleCalls::VariableFactor(var)
        }
    }
    impl ::std::convert::From<VariableFactorNoCacheCall> for ICompoundRateOracleCalls {
        fn from(var: VariableFactorNoCacheCall) -> Self {
            ICompoundRateOracleCalls::VariableFactorNoCache(var)
        }
    }
    impl ::std::convert::From<WriteOracleEntryCall> for ICompoundRateOracleCalls {
        fn from(var: WriteOracleEntryCall) -> Self {
            ICompoundRateOracleCalls::WriteOracleEntry(var)
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
    #[doc = "Container type for all return fields from the `ctoken` function with signature `ctoken()` and selector `[204, 243, 235, 159]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CtokenReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `decimals` function with signature `decimals()` and selector `[49, 60, 229, 103]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DecimalsReturn(pub u8);
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
