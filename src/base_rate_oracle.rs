pub use base_rate_oracle::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod base_rate_oracle {
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
    #[doc = "BaseRateOracle was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static BASERATEORACLE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"type\":\"error\",\"name\":\"AavePoolGetReserveNormalizedIncomeReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"AavePoolGetReserveNormalizedVariableDebtReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CTokenExchangeRateReturnedZero\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"unlocked\",\"type\":\"bool\",\"components\":[]}],\"type\":\"error\",\"name\":\"CanOnlyTradeIfUnlocked\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CannotLiquidate\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CannotSettleBeforeMaturity\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"DebugError\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"amount0\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"amount1\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"ExpectedOppositeSigns\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint160\",\"name\":\"sqrtPriceX96\",\"type\":\"uint160\",\"components\":[]}],\"type\":\"error\",\"name\":\"ExpectedSqrtPriceZeroBeforeInit\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"IRSNotionalAmountSpecifiedMustBeNonZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidMarginDelta\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"LidoGetPooledEthBySharesReturnedZero\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"type\":\"error\",\"name\":\"LiquidityDeltaMustBePositiveInBurn\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"type\":\"error\",\"name\":\"LiquidityDeltaMustBePositiveInMint\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginLessThanMinimum\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tick\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"variableTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"cumulativeFeeIncurred\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenDeltaUnbalanced\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginRequirementNotMet\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginRequirementNotMetFCM\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"requested\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"available\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"NotEnoughFunds\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OOO\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyFCM\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyMarginEngine\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyOwnerCanUpdatePosition\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyVAMM\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMathUD60x18__Exp2InputTooBig\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMathUD60x18__FromUintOverflow\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMathUD60x18__LogInputTooSmall\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"prod1\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMath__MulDivFixedPointOverflow\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"prod1\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"denominator\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMath__MulDivOverflow\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PositionNetZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PositionNotSettled\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"RocketPoolGetEthValueReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"WithdrawalExceedsCurrentMargin\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"closeToOrBeyondMaturity\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_minSecondsSinceLastUpdate\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MinSecondsSinceLastUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"blockTimestampScaled\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"source\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint16\",\"name\":\"index\",\"type\":\"uint16\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint32\",\"name\":\"blockTimestamp\",\"type\":\"uint32\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"observedValue\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint16\",\"name\":\"cardinality\",\"type\":\"uint16\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint16\",\"name\":\"cardinalityNext\",\"type\":\"uint16\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"OracleBufferUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"observationCardinalityNextNew\",\"type\":\"uint16\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RateCardinalityNext\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ONE_IN_WAD\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"UNDERLYING_YIELD_BEARING_PROTOCOL_ID\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"yieldBearingProtocolID\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"currentBlockSlope\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"timeChange\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"blockChange\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"from\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getApyFrom\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"apyFromToWad\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"from\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"to\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getApyFromTo\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"apyFromToWad\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getBlockSlope\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"blockChange\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"timeChange\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCurrentRateInRay\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"currentRate\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLastRateSlope\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"rateChange\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"timeChange\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLastUpdatedRate\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"timestamp\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"rate\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_from\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRateFrom\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_from\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_to\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRateFromTo\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"rateCardinalityNext\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"increaseObservationCardinalityNext\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"beforeOrAtRateValueRay\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"apyFromBeforeOrAtToAtOrAfterWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"timeDeltaBeforeOrAtToQueriedTimeWad\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"interpolateRateValue\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"rateValueRay\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lastUpdatedBlock\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"timestamp\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"number\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"minSecondsSinceLastUpdate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"observations\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"blockTimestamp\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint216\",\"name\":\"observedValue\",\"type\":\"uint216\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"initialized\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"oracleVars\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"rateIndex\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"rateCardinality\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"rateCardinalityNext\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_minSecondsSinceLastUpdate\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setMinSecondsSinceLastUpdate\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"settlementRateCache\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"underlying\",\"outputs\":[{\"internalType\":\"contract IERC20Minimal\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"termStartTimestampInWeiSeconds\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"termEndTimestampInWeiSeconds\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"variableFactor\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"resultWad\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"termStartTimestampInWeiSeconds\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"termEndTimestampInWeiSeconds\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"variableFactorNoCache\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"resultWad\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"writeOracleEntry\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    pub struct BaseRateOracle<M>(ethers::contract::Contract<M>);
    impl<M> Clone for BaseRateOracle<M> {
        fn clone(&self) -> Self {
            BaseRateOracle(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for BaseRateOracle<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for BaseRateOracle<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(BaseRateOracle))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> BaseRateOracle<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), BASERATEORACLE_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `ONE_IN_WAD` (0xc330c98d) function"]
        pub fn one_in_wad(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([195, 48, 201, 141], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `UNDERLYING_YIELD_BEARING_PROTOCOL_ID` (0x22ff6568) function"]
        pub fn underlying_yield_bearing_protocol_id(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([34, 255, 101, 104], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `currentBlockSlope` (0x24b18b17) function"]
        pub fn current_block_slope(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, (u32, ethers::core::types::U256)> {
            self.0
                .method_hash([36, 177, 139, 23], ())
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
        #[doc = "Calls the contract's `getLastUpdatedRate` (0x8a6b8c5d) function"]
        pub fn get_last_updated_rate(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, (u32, ethers::core::types::U256)> {
            self.0
                .method_hash([138, 107, 140, 93], ())
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
        #[doc = "Calls the contract's `interpolateRateValue` (0x54124c64) function"]
        pub fn interpolate_rate_value(
            &self,
            before_or_at_rate_value_ray: ethers::core::types::U256,
            apy_from_before_or_at_to_at_or_after_wad: ethers::core::types::U256,
            time_delta_before_or_at_to_queried_time_wad: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [84, 18, 76, 100],
                    (
                        before_or_at_rate_value_ray,
                        apy_from_before_or_at_to_at_or_after_wad,
                        time_delta_before_or_at_to_queried_time_wad,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lastUpdatedBlock` (0xf90ce5ba) function"]
        pub fn last_updated_block(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, (u32, ethers::core::types::U256)> {
            self.0
                .method_hash([249, 12, 229, 186], ())
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
        #[doc = "Calls the contract's `observations` (0x252c09d7) function"]
        pub fn observations(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, (u32, ethers::core::types::U256, bool)>
        {
            self.0
                .method_hash([37, 44, 9, 215], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `oracleVars` (0xc7db359b) function"]
        pub fn oracle_vars(&self) -> ethers::contract::builders::ContractCall<M, (u16, u16, u16)> {
            self.0
                .method_hash([199, 219, 53, 155], ())
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
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
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
        #[doc = "Calls the contract's `settlementRateCache` (0x1195082e) function"]
        pub fn settlement_rate_cache(
            &self,
            p0: u32,
            p1: u32,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([17, 149, 8, 46], (p0, p1))
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
            term_start_timestamp_in_wei_seconds: ethers::core::types::U256,
            term_end_timestamp_in_wei_seconds: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [37, 242, 88, 221],
                    (
                        term_start_timestamp_in_wei_seconds,
                        term_end_timestamp_in_wei_seconds,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `variableFactorNoCache` (0x41453528) function"]
        pub fn variable_factor_no_cache(
            &self,
            term_start_timestamp_in_wei_seconds: ethers::core::types::U256,
            term_end_timestamp_in_wei_seconds: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [65, 69, 53, 40],
                    (
                        term_start_timestamp_in_wei_seconds,
                        term_end_timestamp_in_wei_seconds,
                    ),
                )
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
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RateCardinalityNext` event"]
        pub fn rate_cardinality_next_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RateCardinalityNextFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, BaseRateOracleEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for BaseRateOracle<M> {
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
    #[doc = "Custom Error type `PRBMathUD60x18__Exp2InputTooBig` with signature `PRBMathUD60x18__Exp2InputTooBig(uint256)` and selector `[74, 79, 38, 241]`"]
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
        name = "PRBMathUD60x18__Exp2InputTooBig",
        abi = "PRBMathUD60x18__Exp2InputTooBig(uint256)"
    )]
    pub struct PRBMathUD60x18__Exp2InputTooBig {
        pub x: ethers::core::types::U256,
    }
    #[doc = "Custom Error type `PRBMathUD60x18__FromUintOverflow` with signature `PRBMathUD60x18__FromUintOverflow(uint256)` and selector `[52, 146, 255, 217]`"]
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
        name = "PRBMathUD60x18__FromUintOverflow",
        abi = "PRBMathUD60x18__FromUintOverflow(uint256)"
    )]
    pub struct PRBMathUD60x18__FromUintOverflow {
        pub x: ethers::core::types::U256,
    }
    #[doc = "Custom Error type `PRBMathUD60x18__LogInputTooSmall` with signature `PRBMathUD60x18__LogInputTooSmall(uint256)` and selector `[216, 133, 4, 220]`"]
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
        name = "PRBMathUD60x18__LogInputTooSmall",
        abi = "PRBMathUD60x18__LogInputTooSmall(uint256)"
    )]
    pub struct PRBMathUD60x18__LogInputTooSmall {
        pub x: ethers::core::types::U256,
    }
    #[doc = "Custom Error type `PRBMath__MulDivFixedPointOverflow` with signature `PRBMath__MulDivFixedPointOverflow(uint256)` and selector `[211, 27, 52, 2]`"]
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
        name = "PRBMath__MulDivFixedPointOverflow",
        abi = "PRBMath__MulDivFixedPointOverflow(uint256)"
    )]
    pub struct PRBMath__MulDivFixedPointOverflow {
        pub prod_1: ethers::core::types::U256,
    }
    #[doc = "Custom Error type `PRBMath__MulDivOverflow` with signature `PRBMath__MulDivOverflow(uint256,uint256)` and selector `[119, 60, 193, 140]`"]
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
        name = "PRBMath__MulDivOverflow",
        abi = "PRBMath__MulDivOverflow(uint256,uint256)"
    )]
    pub struct PRBMath__MulDivOverflow {
        pub prod_1: ethers::core::types::U256,
        pub denominator: ethers::core::types::U256,
    }
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
    pub enum BaseRateOracleErrors {
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
        PRBMathUD60x18__Exp2InputTooBig(PRBMathUD60x18__Exp2InputTooBig),
        PRBMathUD60x18__FromUintOverflow(PRBMathUD60x18__FromUintOverflow),
        PRBMathUD60x18__LogInputTooSmall(PRBMathUD60x18__LogInputTooSmall),
        PRBMath__MulDivFixedPointOverflow(PRBMath__MulDivFixedPointOverflow),
        PRBMath__MulDivOverflow(PRBMath__MulDivOverflow),
        PositionNetZero(PositionNetZero),
        PositionNotSettled(PositionNotSettled),
        RocketPoolGetEthValueReturnedZero(RocketPoolGetEthValueReturnedZero),
        WithdrawalExceedsCurrentMargin(WithdrawalExceedsCurrentMargin),
        closeToOrBeyondMaturity(closeToOrBeyondMaturity),
    }
    impl ethers::core::abi::AbiDecode for BaseRateOracleErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok (decoded) = < AavePoolGetReserveNormalizedIncomeReturnedZero as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (BaseRateOracleErrors :: AavePoolGetReserveNormalizedIncomeReturnedZero (decoded)) }
            if let Ok (decoded) = < AavePoolGetReserveNormalizedVariableDebtReturnedZero as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (BaseRateOracleErrors :: AavePoolGetReserveNormalizedVariableDebtReturnedZero (decoded)) }
            if let Ok(decoded) =
                <CTokenExchangeRateReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BaseRateOracleErrors::CTokenExchangeRateReturnedZero(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <CanOnlyTradeIfUnlocked as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRateOracleErrors::CanOnlyTradeIfUnlocked(decoded));
            }
            if let Ok(decoded) =
                <CannotLiquidate as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRateOracleErrors::CannotLiquidate(decoded));
            }
            if let Ok(decoded) =
                <CannotSettleBeforeMaturity as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRateOracleErrors::CannotSettleBeforeMaturity(decoded));
            }
            if let Ok(decoded) = <DebugError as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRateOracleErrors::DebugError(decoded));
            }
            if let Ok(decoded) =
                <ExpectedOppositeSigns as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRateOracleErrors::ExpectedOppositeSigns(decoded));
            }
            if let Ok(decoded) =
                <ExpectedSqrtPriceZeroBeforeInit as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BaseRateOracleErrors::ExpectedSqrtPriceZeroBeforeInit(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <IRSNotionalAmountSpecifiedMustBeNonZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BaseRateOracleErrors::IRSNotionalAmountSpecifiedMustBeNonZero(decoded));
            }
            if let Ok(decoded) =
                <InvalidMarginDelta as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRateOracleErrors::InvalidMarginDelta(decoded));
            }
            if let Ok(decoded) =
                <LidoGetPooledEthBySharesReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BaseRateOracleErrors::LidoGetPooledEthBySharesReturnedZero(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <LiquidityDeltaMustBePositiveInBurn as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BaseRateOracleErrors::LiquidityDeltaMustBePositiveInBurn(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <LiquidityDeltaMustBePositiveInMint as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BaseRateOracleErrors::LiquidityDeltaMustBePositiveInMint(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <MarginLessThanMinimum as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRateOracleErrors::MarginLessThanMinimum(decoded));
            }
            if let Ok(decoded) =
                <MarginRequirementNotMet as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRateOracleErrors::MarginRequirementNotMet(decoded));
            }
            if let Ok(decoded) =
                <MarginRequirementNotMetFCM as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRateOracleErrors::MarginRequirementNotMetFCM(decoded));
            }
            if let Ok(decoded) =
                <NotEnoughFunds as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRateOracleErrors::NotEnoughFunds(decoded));
            }
            if let Ok(decoded) = <OOO as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(BaseRateOracleErrors::OOO(decoded));
            }
            if let Ok(decoded) = <OnlyFCM as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(BaseRateOracleErrors::OnlyFCM(decoded));
            }
            if let Ok(decoded) =
                <OnlyMarginEngine as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRateOracleErrors::OnlyMarginEngine(decoded));
            }
            if let Ok(decoded) =
                <OnlyOwnerCanUpdatePosition as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRateOracleErrors::OnlyOwnerCanUpdatePosition(decoded));
            }
            if let Ok(decoded) = <OnlyVAMM as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(BaseRateOracleErrors::OnlyVAMM(decoded));
            }
            if let Ok(decoded) =
                <PRBMathUD60x18__Exp2InputTooBig as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BaseRateOracleErrors::PRBMathUD60x18__Exp2InputTooBig(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <PRBMathUD60x18__FromUintOverflow as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BaseRateOracleErrors::PRBMathUD60x18__FromUintOverflow(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <PRBMathUD60x18__LogInputTooSmall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BaseRateOracleErrors::PRBMathUD60x18__LogInputTooSmall(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <PRBMath__MulDivFixedPointOverflow as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BaseRateOracleErrors::PRBMath__MulDivFixedPointOverflow(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <PRBMath__MulDivOverflow as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRateOracleErrors::PRBMath__MulDivOverflow(decoded));
            }
            if let Ok(decoded) =
                <PositionNetZero as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRateOracleErrors::PositionNetZero(decoded));
            }
            if let Ok(decoded) =
                <PositionNotSettled as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRateOracleErrors::PositionNotSettled(decoded));
            }
            if let Ok(decoded) =
                <RocketPoolGetEthValueReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BaseRateOracleErrors::RocketPoolGetEthValueReturnedZero(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <WithdrawalExceedsCurrentMargin as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BaseRateOracleErrors::WithdrawalExceedsCurrentMargin(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <closeToOrBeyondMaturity as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRateOracleErrors::closeToOrBeyondMaturity(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for BaseRateOracleErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                BaseRateOracleErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(element) => {
                    element.encode()
                }
                BaseRateOracleErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(
                    element,
                ) => element.encode(),
                BaseRateOracleErrors::CTokenExchangeRateReturnedZero(element) => element.encode(),
                BaseRateOracleErrors::CanOnlyTradeIfUnlocked(element) => element.encode(),
                BaseRateOracleErrors::CannotLiquidate(element) => element.encode(),
                BaseRateOracleErrors::CannotSettleBeforeMaturity(element) => element.encode(),
                BaseRateOracleErrors::DebugError(element) => element.encode(),
                BaseRateOracleErrors::ExpectedOppositeSigns(element) => element.encode(),
                BaseRateOracleErrors::ExpectedSqrtPriceZeroBeforeInit(element) => element.encode(),
                BaseRateOracleErrors::IRSNotionalAmountSpecifiedMustBeNonZero(element) => {
                    element.encode()
                }
                BaseRateOracleErrors::InvalidMarginDelta(element) => element.encode(),
                BaseRateOracleErrors::LidoGetPooledEthBySharesReturnedZero(element) => {
                    element.encode()
                }
                BaseRateOracleErrors::LiquidityDeltaMustBePositiveInBurn(element) => {
                    element.encode()
                }
                BaseRateOracleErrors::LiquidityDeltaMustBePositiveInMint(element) => {
                    element.encode()
                }
                BaseRateOracleErrors::MarginLessThanMinimum(element) => element.encode(),
                BaseRateOracleErrors::MarginRequirementNotMet(element) => element.encode(),
                BaseRateOracleErrors::MarginRequirementNotMetFCM(element) => element.encode(),
                BaseRateOracleErrors::NotEnoughFunds(element) => element.encode(),
                BaseRateOracleErrors::OOO(element) => element.encode(),
                BaseRateOracleErrors::OnlyFCM(element) => element.encode(),
                BaseRateOracleErrors::OnlyMarginEngine(element) => element.encode(),
                BaseRateOracleErrors::OnlyOwnerCanUpdatePosition(element) => element.encode(),
                BaseRateOracleErrors::OnlyVAMM(element) => element.encode(),
                BaseRateOracleErrors::PRBMathUD60x18__Exp2InputTooBig(element) => element.encode(),
                BaseRateOracleErrors::PRBMathUD60x18__FromUintOverflow(element) => element.encode(),
                BaseRateOracleErrors::PRBMathUD60x18__LogInputTooSmall(element) => element.encode(),
                BaseRateOracleErrors::PRBMath__MulDivFixedPointOverflow(element) => {
                    element.encode()
                }
                BaseRateOracleErrors::PRBMath__MulDivOverflow(element) => element.encode(),
                BaseRateOracleErrors::PositionNetZero(element) => element.encode(),
                BaseRateOracleErrors::PositionNotSettled(element) => element.encode(),
                BaseRateOracleErrors::RocketPoolGetEthValueReturnedZero(element) => {
                    element.encode()
                }
                BaseRateOracleErrors::WithdrawalExceedsCurrentMargin(element) => element.encode(),
                BaseRateOracleErrors::closeToOrBeyondMaturity(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for BaseRateOracleErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                BaseRateOracleErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(element) => {
                    element.fmt(f)
                }
                BaseRateOracleErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(
                    element,
                ) => element.fmt(f),
                BaseRateOracleErrors::CTokenExchangeRateReturnedZero(element) => element.fmt(f),
                BaseRateOracleErrors::CanOnlyTradeIfUnlocked(element) => element.fmt(f),
                BaseRateOracleErrors::CannotLiquidate(element) => element.fmt(f),
                BaseRateOracleErrors::CannotSettleBeforeMaturity(element) => element.fmt(f),
                BaseRateOracleErrors::DebugError(element) => element.fmt(f),
                BaseRateOracleErrors::ExpectedOppositeSigns(element) => element.fmt(f),
                BaseRateOracleErrors::ExpectedSqrtPriceZeroBeforeInit(element) => element.fmt(f),
                BaseRateOracleErrors::IRSNotionalAmountSpecifiedMustBeNonZero(element) => {
                    element.fmt(f)
                }
                BaseRateOracleErrors::InvalidMarginDelta(element) => element.fmt(f),
                BaseRateOracleErrors::LidoGetPooledEthBySharesReturnedZero(element) => {
                    element.fmt(f)
                }
                BaseRateOracleErrors::LiquidityDeltaMustBePositiveInBurn(element) => element.fmt(f),
                BaseRateOracleErrors::LiquidityDeltaMustBePositiveInMint(element) => element.fmt(f),
                BaseRateOracleErrors::MarginLessThanMinimum(element) => element.fmt(f),
                BaseRateOracleErrors::MarginRequirementNotMet(element) => element.fmt(f),
                BaseRateOracleErrors::MarginRequirementNotMetFCM(element) => element.fmt(f),
                BaseRateOracleErrors::NotEnoughFunds(element) => element.fmt(f),
                BaseRateOracleErrors::OOO(element) => element.fmt(f),
                BaseRateOracleErrors::OnlyFCM(element) => element.fmt(f),
                BaseRateOracleErrors::OnlyMarginEngine(element) => element.fmt(f),
                BaseRateOracleErrors::OnlyOwnerCanUpdatePosition(element) => element.fmt(f),
                BaseRateOracleErrors::OnlyVAMM(element) => element.fmt(f),
                BaseRateOracleErrors::PRBMathUD60x18__Exp2InputTooBig(element) => element.fmt(f),
                BaseRateOracleErrors::PRBMathUD60x18__FromUintOverflow(element) => element.fmt(f),
                BaseRateOracleErrors::PRBMathUD60x18__LogInputTooSmall(element) => element.fmt(f),
                BaseRateOracleErrors::PRBMath__MulDivFixedPointOverflow(element) => element.fmt(f),
                BaseRateOracleErrors::PRBMath__MulDivOverflow(element) => element.fmt(f),
                BaseRateOracleErrors::PositionNetZero(element) => element.fmt(f),
                BaseRateOracleErrors::PositionNotSettled(element) => element.fmt(f),
                BaseRateOracleErrors::RocketPoolGetEthValueReturnedZero(element) => element.fmt(f),
                BaseRateOracleErrors::WithdrawalExceedsCurrentMargin(element) => element.fmt(f),
                BaseRateOracleErrors::closeToOrBeyondMaturity(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AavePoolGetReserveNormalizedIncomeReturnedZero> for BaseRateOracleErrors {
        fn from(var: AavePoolGetReserveNormalizedIncomeReturnedZero) -> Self {
            BaseRateOracleErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(var)
        }
    }
    impl ::std::convert::From<AavePoolGetReserveNormalizedVariableDebtReturnedZero>
        for BaseRateOracleErrors
    {
        fn from(var: AavePoolGetReserveNormalizedVariableDebtReturnedZero) -> Self {
            BaseRateOracleErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(var)
        }
    }
    impl ::std::convert::From<CTokenExchangeRateReturnedZero> for BaseRateOracleErrors {
        fn from(var: CTokenExchangeRateReturnedZero) -> Self {
            BaseRateOracleErrors::CTokenExchangeRateReturnedZero(var)
        }
    }
    impl ::std::convert::From<CanOnlyTradeIfUnlocked> for BaseRateOracleErrors {
        fn from(var: CanOnlyTradeIfUnlocked) -> Self {
            BaseRateOracleErrors::CanOnlyTradeIfUnlocked(var)
        }
    }
    impl ::std::convert::From<CannotLiquidate> for BaseRateOracleErrors {
        fn from(var: CannotLiquidate) -> Self {
            BaseRateOracleErrors::CannotLiquidate(var)
        }
    }
    impl ::std::convert::From<CannotSettleBeforeMaturity> for BaseRateOracleErrors {
        fn from(var: CannotSettleBeforeMaturity) -> Self {
            BaseRateOracleErrors::CannotSettleBeforeMaturity(var)
        }
    }
    impl ::std::convert::From<DebugError> for BaseRateOracleErrors {
        fn from(var: DebugError) -> Self {
            BaseRateOracleErrors::DebugError(var)
        }
    }
    impl ::std::convert::From<ExpectedOppositeSigns> for BaseRateOracleErrors {
        fn from(var: ExpectedOppositeSigns) -> Self {
            BaseRateOracleErrors::ExpectedOppositeSigns(var)
        }
    }
    impl ::std::convert::From<ExpectedSqrtPriceZeroBeforeInit> for BaseRateOracleErrors {
        fn from(var: ExpectedSqrtPriceZeroBeforeInit) -> Self {
            BaseRateOracleErrors::ExpectedSqrtPriceZeroBeforeInit(var)
        }
    }
    impl ::std::convert::From<IRSNotionalAmountSpecifiedMustBeNonZero> for BaseRateOracleErrors {
        fn from(var: IRSNotionalAmountSpecifiedMustBeNonZero) -> Self {
            BaseRateOracleErrors::IRSNotionalAmountSpecifiedMustBeNonZero(var)
        }
    }
    impl ::std::convert::From<InvalidMarginDelta> for BaseRateOracleErrors {
        fn from(var: InvalidMarginDelta) -> Self {
            BaseRateOracleErrors::InvalidMarginDelta(var)
        }
    }
    impl ::std::convert::From<LidoGetPooledEthBySharesReturnedZero> for BaseRateOracleErrors {
        fn from(var: LidoGetPooledEthBySharesReturnedZero) -> Self {
            BaseRateOracleErrors::LidoGetPooledEthBySharesReturnedZero(var)
        }
    }
    impl ::std::convert::From<LiquidityDeltaMustBePositiveInBurn> for BaseRateOracleErrors {
        fn from(var: LiquidityDeltaMustBePositiveInBurn) -> Self {
            BaseRateOracleErrors::LiquidityDeltaMustBePositiveInBurn(var)
        }
    }
    impl ::std::convert::From<LiquidityDeltaMustBePositiveInMint> for BaseRateOracleErrors {
        fn from(var: LiquidityDeltaMustBePositiveInMint) -> Self {
            BaseRateOracleErrors::LiquidityDeltaMustBePositiveInMint(var)
        }
    }
    impl ::std::convert::From<MarginLessThanMinimum> for BaseRateOracleErrors {
        fn from(var: MarginLessThanMinimum) -> Self {
            BaseRateOracleErrors::MarginLessThanMinimum(var)
        }
    }
    impl ::std::convert::From<MarginRequirementNotMet> for BaseRateOracleErrors {
        fn from(var: MarginRequirementNotMet) -> Self {
            BaseRateOracleErrors::MarginRequirementNotMet(var)
        }
    }
    impl ::std::convert::From<MarginRequirementNotMetFCM> for BaseRateOracleErrors {
        fn from(var: MarginRequirementNotMetFCM) -> Self {
            BaseRateOracleErrors::MarginRequirementNotMetFCM(var)
        }
    }
    impl ::std::convert::From<NotEnoughFunds> for BaseRateOracleErrors {
        fn from(var: NotEnoughFunds) -> Self {
            BaseRateOracleErrors::NotEnoughFunds(var)
        }
    }
    impl ::std::convert::From<OOO> for BaseRateOracleErrors {
        fn from(var: OOO) -> Self {
            BaseRateOracleErrors::OOO(var)
        }
    }
    impl ::std::convert::From<OnlyFCM> for BaseRateOracleErrors {
        fn from(var: OnlyFCM) -> Self {
            BaseRateOracleErrors::OnlyFCM(var)
        }
    }
    impl ::std::convert::From<OnlyMarginEngine> for BaseRateOracleErrors {
        fn from(var: OnlyMarginEngine) -> Self {
            BaseRateOracleErrors::OnlyMarginEngine(var)
        }
    }
    impl ::std::convert::From<OnlyOwnerCanUpdatePosition> for BaseRateOracleErrors {
        fn from(var: OnlyOwnerCanUpdatePosition) -> Self {
            BaseRateOracleErrors::OnlyOwnerCanUpdatePosition(var)
        }
    }
    impl ::std::convert::From<OnlyVAMM> for BaseRateOracleErrors {
        fn from(var: OnlyVAMM) -> Self {
            BaseRateOracleErrors::OnlyVAMM(var)
        }
    }
    impl ::std::convert::From<PRBMathUD60x18__Exp2InputTooBig> for BaseRateOracleErrors {
        fn from(var: PRBMathUD60x18__Exp2InputTooBig) -> Self {
            BaseRateOracleErrors::PRBMathUD60x18__Exp2InputTooBig(var)
        }
    }
    impl ::std::convert::From<PRBMathUD60x18__FromUintOverflow> for BaseRateOracleErrors {
        fn from(var: PRBMathUD60x18__FromUintOverflow) -> Self {
            BaseRateOracleErrors::PRBMathUD60x18__FromUintOverflow(var)
        }
    }
    impl ::std::convert::From<PRBMathUD60x18__LogInputTooSmall> for BaseRateOracleErrors {
        fn from(var: PRBMathUD60x18__LogInputTooSmall) -> Self {
            BaseRateOracleErrors::PRBMathUD60x18__LogInputTooSmall(var)
        }
    }
    impl ::std::convert::From<PRBMath__MulDivFixedPointOverflow> for BaseRateOracleErrors {
        fn from(var: PRBMath__MulDivFixedPointOverflow) -> Self {
            BaseRateOracleErrors::PRBMath__MulDivFixedPointOverflow(var)
        }
    }
    impl ::std::convert::From<PRBMath__MulDivOverflow> for BaseRateOracleErrors {
        fn from(var: PRBMath__MulDivOverflow) -> Self {
            BaseRateOracleErrors::PRBMath__MulDivOverflow(var)
        }
    }
    impl ::std::convert::From<PositionNetZero> for BaseRateOracleErrors {
        fn from(var: PositionNetZero) -> Self {
            BaseRateOracleErrors::PositionNetZero(var)
        }
    }
    impl ::std::convert::From<PositionNotSettled> for BaseRateOracleErrors {
        fn from(var: PositionNotSettled) -> Self {
            BaseRateOracleErrors::PositionNotSettled(var)
        }
    }
    impl ::std::convert::From<RocketPoolGetEthValueReturnedZero> for BaseRateOracleErrors {
        fn from(var: RocketPoolGetEthValueReturnedZero) -> Self {
            BaseRateOracleErrors::RocketPoolGetEthValueReturnedZero(var)
        }
    }
    impl ::std::convert::From<WithdrawalExceedsCurrentMargin> for BaseRateOracleErrors {
        fn from(var: WithdrawalExceedsCurrentMargin) -> Self {
            BaseRateOracleErrors::WithdrawalExceedsCurrentMargin(var)
        }
    }
    impl ::std::convert::From<closeToOrBeyondMaturity> for BaseRateOracleErrors {
        fn from(var: closeToOrBeyondMaturity) -> Self {
            BaseRateOracleErrors::closeToOrBeyondMaturity(var)
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
    pub enum BaseRateOracleEvents {
        MinSecondsSinceLastUpdateFilter(MinSecondsSinceLastUpdateFilter),
        OracleBufferUpdateFilter(OracleBufferUpdateFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        RateCardinalityNextFilter(RateCardinalityNextFilter),
    }
    impl ethers::contract::EthLogDecode for BaseRateOracleEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = MinSecondsSinceLastUpdateFilter::decode_log(log) {
                return Ok(BaseRateOracleEvents::MinSecondsSinceLastUpdateFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = OracleBufferUpdateFilter::decode_log(log) {
                return Ok(BaseRateOracleEvents::OracleBufferUpdateFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(BaseRateOracleEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = RateCardinalityNextFilter::decode_log(log) {
                return Ok(BaseRateOracleEvents::RateCardinalityNextFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for BaseRateOracleEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                BaseRateOracleEvents::MinSecondsSinceLastUpdateFilter(element) => element.fmt(f),
                BaseRateOracleEvents::OracleBufferUpdateFilter(element) => element.fmt(f),
                BaseRateOracleEvents::OwnershipTransferredFilter(element) => element.fmt(f),
                BaseRateOracleEvents::RateCardinalityNextFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `ONE_IN_WAD` function with signature `ONE_IN_WAD()` and selector `[195, 48, 201, 141]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "ONE_IN_WAD", abi = "ONE_IN_WAD()")]
    pub struct OneInWadCall;
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
    #[doc = "Container type for all input parameters for the `currentBlockSlope` function with signature `currentBlockSlope()` and selector `[36, 177, 139, 23]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "currentBlockSlope", abi = "currentBlockSlope()")]
    pub struct CurrentBlockSlopeCall;
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
    #[doc = "Container type for all input parameters for the `getLastUpdatedRate` function with signature `getLastUpdatedRate()` and selector `[138, 107, 140, 93]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getLastUpdatedRate", abi = "getLastUpdatedRate()")]
    pub struct GetLastUpdatedRateCall;
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
    #[doc = "Container type for all input parameters for the `interpolateRateValue` function with signature `interpolateRateValue(uint256,uint256,uint256)` and selector `[84, 18, 76, 100]`"]
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
        name = "interpolateRateValue",
        abi = "interpolateRateValue(uint256,uint256,uint256)"
    )]
    pub struct InterpolateRateValueCall {
        pub before_or_at_rate_value_ray: ethers::core::types::U256,
        pub apy_from_before_or_at_to_at_or_after_wad: ethers::core::types::U256,
        pub time_delta_before_or_at_to_queried_time_wad: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `lastUpdatedBlock` function with signature `lastUpdatedBlock()` and selector `[249, 12, 229, 186]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "lastUpdatedBlock", abi = "lastUpdatedBlock()")]
    pub struct LastUpdatedBlockCall;
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
    #[doc = "Container type for all input parameters for the `observations` function with signature `observations(uint256)` and selector `[37, 44, 9, 215]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "observations", abi = "observations(uint256)")]
    pub struct ObservationsCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `oracleVars` function with signature `oracleVars()` and selector `[199, 219, 53, 155]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "oracleVars", abi = "oracleVars()")]
    pub struct OracleVarsCall;
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
    #[doc = "Container type for all input parameters for the `settlementRateCache` function with signature `settlementRateCache(uint32,uint32)` and selector `[17, 149, 8, 46]`"]
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
        name = "settlementRateCache",
        abi = "settlementRateCache(uint32,uint32)"
    )]
    pub struct SettlementRateCacheCall(pub u32, pub u32);
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
        pub term_start_timestamp_in_wei_seconds: ethers::core::types::U256,
        pub term_end_timestamp_in_wei_seconds: ethers::core::types::U256,
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
        pub term_start_timestamp_in_wei_seconds: ethers::core::types::U256,
        pub term_end_timestamp_in_wei_seconds: ethers::core::types::U256,
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
    pub enum BaseRateOracleCalls {
        OneInWad(OneInWadCall),
        UnderlyingYieldBearingProtocolId(UnderlyingYieldBearingProtocolIdCall),
        CurrentBlockSlope(CurrentBlockSlopeCall),
        GetApyFrom(GetApyFromCall),
        GetApyFromTo(GetApyFromToCall),
        GetBlockSlope(GetBlockSlopeCall),
        GetCurrentRateInRay(GetCurrentRateInRayCall),
        GetLastRateSlope(GetLastRateSlopeCall),
        GetLastUpdatedRate(GetLastUpdatedRateCall),
        GetRateFrom(GetRateFromCall),
        GetRateFromTo(GetRateFromToCall),
        IncreaseObservationCardinalityNext(IncreaseObservationCardinalityNextCall),
        InterpolateRateValue(InterpolateRateValueCall),
        LastUpdatedBlock(LastUpdatedBlockCall),
        MinSecondsSinceLastUpdate(MinSecondsSinceLastUpdateCall),
        Observations(ObservationsCall),
        OracleVars(OracleVarsCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetMinSecondsSinceLastUpdate(SetMinSecondsSinceLastUpdateCall),
        SettlementRateCache(SettlementRateCacheCall),
        TransferOwnership(TransferOwnershipCall),
        Underlying(UnderlyingCall),
        VariableFactor(VariableFactorCall),
        VariableFactorNoCache(VariableFactorNoCacheCall),
        WriteOracleEntry(WriteOracleEntryCall),
    }
    impl ethers::core::abi::AbiDecode for BaseRateOracleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <OneInWadCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRateOracleCalls::OneInWad(decoded));
            }
            if let Ok(decoded) =
                <UnderlyingYieldBearingProtocolIdCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BaseRateOracleCalls::UnderlyingYieldBearingProtocolId(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <CurrentBlockSlopeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRateOracleCalls::CurrentBlockSlope(decoded));
            }
            if let Ok(decoded) =
                <GetApyFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRateOracleCalls::GetApyFrom(decoded));
            }
            if let Ok(decoded) =
                <GetApyFromToCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRateOracleCalls::GetApyFromTo(decoded));
            }
            if let Ok(decoded) =
                <GetBlockSlopeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRateOracleCalls::GetBlockSlope(decoded));
            }
            if let Ok(decoded) =
                <GetCurrentRateInRayCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRateOracleCalls::GetCurrentRateInRay(decoded));
            }
            if let Ok(decoded) =
                <GetLastRateSlopeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRateOracleCalls::GetLastRateSlope(decoded));
            }
            if let Ok(decoded) =
                <GetLastUpdatedRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRateOracleCalls::GetLastUpdatedRate(decoded));
            }
            if let Ok(decoded) =
                <GetRateFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRateOracleCalls::GetRateFrom(decoded));
            }
            if let Ok(decoded) =
                <GetRateFromToCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRateOracleCalls::GetRateFromTo(decoded));
            }
            if let Ok(decoded) =
                <IncreaseObservationCardinalityNextCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BaseRateOracleCalls::IncreaseObservationCardinalityNext(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <InterpolateRateValueCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRateOracleCalls::InterpolateRateValue(decoded));
            }
            if let Ok(decoded) =
                <LastUpdatedBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRateOracleCalls::LastUpdatedBlock(decoded));
            }
            if let Ok(decoded) =
                <MinSecondsSinceLastUpdateCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BaseRateOracleCalls::MinSecondsSinceLastUpdate(decoded));
            }
            if let Ok(decoded) =
                <ObservationsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRateOracleCalls::Observations(decoded));
            }
            if let Ok(decoded) =
                <OracleVarsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRateOracleCalls::OracleVars(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRateOracleCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRateOracleCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <SetMinSecondsSinceLastUpdateCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BaseRateOracleCalls::SetMinSecondsSinceLastUpdate(decoded));
            }
            if let Ok(decoded) =
                <SettlementRateCacheCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRateOracleCalls::SettlementRateCache(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRateOracleCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <UnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRateOracleCalls::Underlying(decoded));
            }
            if let Ok(decoded) =
                <VariableFactorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRateOracleCalls::VariableFactor(decoded));
            }
            if let Ok(decoded) =
                <VariableFactorNoCacheCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRateOracleCalls::VariableFactorNoCache(decoded));
            }
            if let Ok(decoded) =
                <WriteOracleEntryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRateOracleCalls::WriteOracleEntry(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for BaseRateOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                BaseRateOracleCalls::OneInWad(element) => element.encode(),
                BaseRateOracleCalls::UnderlyingYieldBearingProtocolId(element) => element.encode(),
                BaseRateOracleCalls::CurrentBlockSlope(element) => element.encode(),
                BaseRateOracleCalls::GetApyFrom(element) => element.encode(),
                BaseRateOracleCalls::GetApyFromTo(element) => element.encode(),
                BaseRateOracleCalls::GetBlockSlope(element) => element.encode(),
                BaseRateOracleCalls::GetCurrentRateInRay(element) => element.encode(),
                BaseRateOracleCalls::GetLastRateSlope(element) => element.encode(),
                BaseRateOracleCalls::GetLastUpdatedRate(element) => element.encode(),
                BaseRateOracleCalls::GetRateFrom(element) => element.encode(),
                BaseRateOracleCalls::GetRateFromTo(element) => element.encode(),
                BaseRateOracleCalls::IncreaseObservationCardinalityNext(element) => {
                    element.encode()
                }
                BaseRateOracleCalls::InterpolateRateValue(element) => element.encode(),
                BaseRateOracleCalls::LastUpdatedBlock(element) => element.encode(),
                BaseRateOracleCalls::MinSecondsSinceLastUpdate(element) => element.encode(),
                BaseRateOracleCalls::Observations(element) => element.encode(),
                BaseRateOracleCalls::OracleVars(element) => element.encode(),
                BaseRateOracleCalls::Owner(element) => element.encode(),
                BaseRateOracleCalls::RenounceOwnership(element) => element.encode(),
                BaseRateOracleCalls::SetMinSecondsSinceLastUpdate(element) => element.encode(),
                BaseRateOracleCalls::SettlementRateCache(element) => element.encode(),
                BaseRateOracleCalls::TransferOwnership(element) => element.encode(),
                BaseRateOracleCalls::Underlying(element) => element.encode(),
                BaseRateOracleCalls::VariableFactor(element) => element.encode(),
                BaseRateOracleCalls::VariableFactorNoCache(element) => element.encode(),
                BaseRateOracleCalls::WriteOracleEntry(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for BaseRateOracleCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                BaseRateOracleCalls::OneInWad(element) => element.fmt(f),
                BaseRateOracleCalls::UnderlyingYieldBearingProtocolId(element) => element.fmt(f),
                BaseRateOracleCalls::CurrentBlockSlope(element) => element.fmt(f),
                BaseRateOracleCalls::GetApyFrom(element) => element.fmt(f),
                BaseRateOracleCalls::GetApyFromTo(element) => element.fmt(f),
                BaseRateOracleCalls::GetBlockSlope(element) => element.fmt(f),
                BaseRateOracleCalls::GetCurrentRateInRay(element) => element.fmt(f),
                BaseRateOracleCalls::GetLastRateSlope(element) => element.fmt(f),
                BaseRateOracleCalls::GetLastUpdatedRate(element) => element.fmt(f),
                BaseRateOracleCalls::GetRateFrom(element) => element.fmt(f),
                BaseRateOracleCalls::GetRateFromTo(element) => element.fmt(f),
                BaseRateOracleCalls::IncreaseObservationCardinalityNext(element) => element.fmt(f),
                BaseRateOracleCalls::InterpolateRateValue(element) => element.fmt(f),
                BaseRateOracleCalls::LastUpdatedBlock(element) => element.fmt(f),
                BaseRateOracleCalls::MinSecondsSinceLastUpdate(element) => element.fmt(f),
                BaseRateOracleCalls::Observations(element) => element.fmt(f),
                BaseRateOracleCalls::OracleVars(element) => element.fmt(f),
                BaseRateOracleCalls::Owner(element) => element.fmt(f),
                BaseRateOracleCalls::RenounceOwnership(element) => element.fmt(f),
                BaseRateOracleCalls::SetMinSecondsSinceLastUpdate(element) => element.fmt(f),
                BaseRateOracleCalls::SettlementRateCache(element) => element.fmt(f),
                BaseRateOracleCalls::TransferOwnership(element) => element.fmt(f),
                BaseRateOracleCalls::Underlying(element) => element.fmt(f),
                BaseRateOracleCalls::VariableFactor(element) => element.fmt(f),
                BaseRateOracleCalls::VariableFactorNoCache(element) => element.fmt(f),
                BaseRateOracleCalls::WriteOracleEntry(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<OneInWadCall> for BaseRateOracleCalls {
        fn from(var: OneInWadCall) -> Self {
            BaseRateOracleCalls::OneInWad(var)
        }
    }
    impl ::std::convert::From<UnderlyingYieldBearingProtocolIdCall> for BaseRateOracleCalls {
        fn from(var: UnderlyingYieldBearingProtocolIdCall) -> Self {
            BaseRateOracleCalls::UnderlyingYieldBearingProtocolId(var)
        }
    }
    impl ::std::convert::From<CurrentBlockSlopeCall> for BaseRateOracleCalls {
        fn from(var: CurrentBlockSlopeCall) -> Self {
            BaseRateOracleCalls::CurrentBlockSlope(var)
        }
    }
    impl ::std::convert::From<GetApyFromCall> for BaseRateOracleCalls {
        fn from(var: GetApyFromCall) -> Self {
            BaseRateOracleCalls::GetApyFrom(var)
        }
    }
    impl ::std::convert::From<GetApyFromToCall> for BaseRateOracleCalls {
        fn from(var: GetApyFromToCall) -> Self {
            BaseRateOracleCalls::GetApyFromTo(var)
        }
    }
    impl ::std::convert::From<GetBlockSlopeCall> for BaseRateOracleCalls {
        fn from(var: GetBlockSlopeCall) -> Self {
            BaseRateOracleCalls::GetBlockSlope(var)
        }
    }
    impl ::std::convert::From<GetCurrentRateInRayCall> for BaseRateOracleCalls {
        fn from(var: GetCurrentRateInRayCall) -> Self {
            BaseRateOracleCalls::GetCurrentRateInRay(var)
        }
    }
    impl ::std::convert::From<GetLastRateSlopeCall> for BaseRateOracleCalls {
        fn from(var: GetLastRateSlopeCall) -> Self {
            BaseRateOracleCalls::GetLastRateSlope(var)
        }
    }
    impl ::std::convert::From<GetLastUpdatedRateCall> for BaseRateOracleCalls {
        fn from(var: GetLastUpdatedRateCall) -> Self {
            BaseRateOracleCalls::GetLastUpdatedRate(var)
        }
    }
    impl ::std::convert::From<GetRateFromCall> for BaseRateOracleCalls {
        fn from(var: GetRateFromCall) -> Self {
            BaseRateOracleCalls::GetRateFrom(var)
        }
    }
    impl ::std::convert::From<GetRateFromToCall> for BaseRateOracleCalls {
        fn from(var: GetRateFromToCall) -> Self {
            BaseRateOracleCalls::GetRateFromTo(var)
        }
    }
    impl ::std::convert::From<IncreaseObservationCardinalityNextCall> for BaseRateOracleCalls {
        fn from(var: IncreaseObservationCardinalityNextCall) -> Self {
            BaseRateOracleCalls::IncreaseObservationCardinalityNext(var)
        }
    }
    impl ::std::convert::From<InterpolateRateValueCall> for BaseRateOracleCalls {
        fn from(var: InterpolateRateValueCall) -> Self {
            BaseRateOracleCalls::InterpolateRateValue(var)
        }
    }
    impl ::std::convert::From<LastUpdatedBlockCall> for BaseRateOracleCalls {
        fn from(var: LastUpdatedBlockCall) -> Self {
            BaseRateOracleCalls::LastUpdatedBlock(var)
        }
    }
    impl ::std::convert::From<MinSecondsSinceLastUpdateCall> for BaseRateOracleCalls {
        fn from(var: MinSecondsSinceLastUpdateCall) -> Self {
            BaseRateOracleCalls::MinSecondsSinceLastUpdate(var)
        }
    }
    impl ::std::convert::From<ObservationsCall> for BaseRateOracleCalls {
        fn from(var: ObservationsCall) -> Self {
            BaseRateOracleCalls::Observations(var)
        }
    }
    impl ::std::convert::From<OracleVarsCall> for BaseRateOracleCalls {
        fn from(var: OracleVarsCall) -> Self {
            BaseRateOracleCalls::OracleVars(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for BaseRateOracleCalls {
        fn from(var: OwnerCall) -> Self {
            BaseRateOracleCalls::Owner(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for BaseRateOracleCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            BaseRateOracleCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<SetMinSecondsSinceLastUpdateCall> for BaseRateOracleCalls {
        fn from(var: SetMinSecondsSinceLastUpdateCall) -> Self {
            BaseRateOracleCalls::SetMinSecondsSinceLastUpdate(var)
        }
    }
    impl ::std::convert::From<SettlementRateCacheCall> for BaseRateOracleCalls {
        fn from(var: SettlementRateCacheCall) -> Self {
            BaseRateOracleCalls::SettlementRateCache(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for BaseRateOracleCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            BaseRateOracleCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<UnderlyingCall> for BaseRateOracleCalls {
        fn from(var: UnderlyingCall) -> Self {
            BaseRateOracleCalls::Underlying(var)
        }
    }
    impl ::std::convert::From<VariableFactorCall> for BaseRateOracleCalls {
        fn from(var: VariableFactorCall) -> Self {
            BaseRateOracleCalls::VariableFactor(var)
        }
    }
    impl ::std::convert::From<VariableFactorNoCacheCall> for BaseRateOracleCalls {
        fn from(var: VariableFactorNoCacheCall) -> Self {
            BaseRateOracleCalls::VariableFactorNoCache(var)
        }
    }
    impl ::std::convert::From<WriteOracleEntryCall> for BaseRateOracleCalls {
        fn from(var: WriteOracleEntryCall) -> Self {
            BaseRateOracleCalls::WriteOracleEntry(var)
        }
    }
    #[doc = "Container type for all return fields from the `ONE_IN_WAD` function with signature `ONE_IN_WAD()` and selector `[195, 48, 201, 141]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct OneInWadReturn(pub ethers::core::types::U256);
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
    #[doc = "Container type for all return fields from the `currentBlockSlope` function with signature `currentBlockSlope()` and selector `[36, 177, 139, 23]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CurrentBlockSlopeReturn {
        pub time_change: u32,
        pub block_change: ethers::core::types::U256,
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
        pub apy_from_to_wad: ethers::core::types::U256,
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
        pub apy_from_to_wad: ethers::core::types::U256,
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
    #[doc = "Container type for all return fields from the `getLastUpdatedRate` function with signature `getLastUpdatedRate()` and selector `[138, 107, 140, 93]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetLastUpdatedRateReturn {
        pub timestamp: u32,
        pub rate: ethers::core::types::U256,
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
    #[doc = "Container type for all return fields from the `interpolateRateValue` function with signature `interpolateRateValue(uint256,uint256,uint256)` and selector `[84, 18, 76, 100]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct InterpolateRateValueReturn {
        pub rate_value_ray: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `lastUpdatedBlock` function with signature `lastUpdatedBlock()` and selector `[249, 12, 229, 186]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LastUpdatedBlockReturn {
        pub timestamp: u32,
        pub number: ethers::core::types::U256,
    }
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
    #[doc = "Container type for all return fields from the `observations` function with signature `observations(uint256)` and selector `[37, 44, 9, 215]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ObservationsReturn {
        pub block_timestamp: u32,
        pub observed_value: ethers::core::types::U256,
        pub initialized: bool,
    }
    #[doc = "Container type for all return fields from the `oracleVars` function with signature `oracleVars()` and selector `[199, 219, 53, 155]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct OracleVarsReturn {
        pub rate_index: u16,
        pub rate_cardinality: u16,
        pub rate_cardinality_next: u16,
    }
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
    #[doc = "Container type for all return fields from the `settlementRateCache` function with signature `settlementRateCache(uint32,uint32)` and selector `[17, 149, 8, 46]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SettlementRateCacheReturn(pub ethers::core::types::U256);
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
        pub result_wad: ethers::core::types::U256,
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
        pub result_wad: ethers::core::types::U256,
    }
}
