pub use margin_engine::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod margin_engine {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    pub use super::super::shared_types::*;
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "MarginEngine was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static MARGINENGINE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"AavePoolGetReserveNormalizedIncomeReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"AavePoolGetReserveNormalizedVariableDebtReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CTokenExchangeRateReturnedZero\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"unlocked\",\"type\":\"bool\",\"components\":[]}],\"type\":\"error\",\"name\":\"CanOnlyTradeIfUnlocked\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CannotLiquidate\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CannotSettleBeforeMaturity\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"DebugError\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"amount0\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"amount1\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"ExpectedOppositeSigns\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint160\",\"name\":\"sqrtPriceX96\",\"type\":\"uint160\",\"components\":[]}],\"type\":\"error\",\"name\":\"ExpectedSqrtPriceZeroBeforeInit\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"IRSNotionalAmountSpecifiedMustBeNonZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidMarginDelta\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"LidoGetPooledEthBySharesReturnedZero\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"type\":\"error\",\"name\":\"LiquidityDeltaMustBePositiveInBurn\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"type\":\"error\",\"name\":\"LiquidityDeltaMustBePositiveInMint\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginLessThanMinimum\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tick\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"variableTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"cumulativeFeeIncurred\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenDeltaUnbalanced\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginRequirementNotMet\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginRequirementNotMetFCM\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"requested\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"available\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"NotEnoughFunds\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OOO\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyFCM\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyMarginEngine\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyOwnerCanUpdatePosition\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyVAMM\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PRBMathSD59x18__DivInputTooSmall\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"rAbs\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMathSD59x18__DivOverflow\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"x\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMathSD59x18__Exp2InputTooBig\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"x\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMathSD59x18__ExpInputTooBig\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"x\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMathSD59x18__FromIntOverflow\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"x\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMathSD59x18__FromIntUnderflow\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PRBMathSD59x18__MulInputTooSmall\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"rAbs\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMathSD59x18__MulOverflow\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"x\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMathSD59x18__SqrtNegativeInput\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"x\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMathSD59x18__SqrtOverflow\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMathUD60x18__FromUintOverflow\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"prod1\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMath__MulDivFixedPointOverflow\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"prod1\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"denominator\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMath__MulDivOverflow\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PositionNetZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PositionNotSettled\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"RocketPoolGetEthValueReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"WithdrawalExceedsCurrentMargin\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"closeToOrBeyondMaturity\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AdminChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"beacon\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"BeaconUpgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"cacheMaxAgeInSeconds\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"CacheMaxAgeSetting\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract IFCM\",\"name\":\"fcm\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"FCMSetting\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"HistoricalApy\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"secondsAgo\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"HistoricalApyWindowSetting\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"version\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Initialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"__isAlpha\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"IsAlpha\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"liquidatorRewardWad\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"LiquidatorRewardSetting\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"struct IMarginEngine.MarginCalculatorParameters\",\"name\":\"marginCalculatorParameters\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"apyUpperMultiplierWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"apyLowerMultiplierWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"sigmaSquaredWad\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"alphaWad\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"betaWad\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"xiUpperWad\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"xiLowerWad\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"tMaxWad\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"etaIMWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"etaLMWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap1\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap2\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap3\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap4\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap5\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap6\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap7\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"minMarginToIncentiviseLiquidators\",\"type\":\"uint256\",\"components\":[]}],\"indexed\":false}],\"type\":\"event\",\"name\":\"MarginCalculatorParametersSetting\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[],\"indexed\":true},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"liquidator\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"notionalUnwound\",\"type\":\"int256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"liquidatorReward\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PositionLiquidation\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[],\"indexed\":true},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[],\"indexed\":true},{\"internalType\":\"int256\",\"name\":\"marginDelta\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PositionMarginUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[],\"indexed\":true},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[],\"indexed\":true},{\"internalType\":\"int256\",\"name\":\"settlementCashflow\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PositionSettlement\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[],\"indexed\":true},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint128\",\"name\":\"_liquidity\",\"type\":\"uint128\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"margin\",\"type\":\"int256\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"fixedTokenBalance\",\"type\":\"int256\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"variableTokenBalance\",\"type\":\"int256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"accumulatedFees\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PositionUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ProtocolCollection\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"cacheMaxAgeInSeconds\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RateOracle\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract IRateOracle\",\"name\":\"rateOracle\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"RateOracleSetting\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"implementation\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Upgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract IVAMM\",\"name\":\"vamm\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"VAMMSetting\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MAX_CACHE_MAX_AGE_IN_SECONDS\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MAX_FIXED_RATE_WAD\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MAX_LIQUIDATOR_REWARD_WAD\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MAX_LOOKBACK_WINDOW_IN_SECONDS\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MIN_LOOKBACK_WINDOW_IN_SECONDS\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ONE\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ONE_UINT\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"SECONDS_IN_YEAR\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"cacheMaxAgeInSeconds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"collectProtocol\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"factory\",\"outputs\":[{\"internalType\":\"contract IFactory\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"fcm\",\"outputs\":[{\"internalType\":\"contract IFCM\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"getHistoricalApy\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getHistoricalApyReadOnly\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"_tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"_tickUpper\",\"type\":\"int24\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"getPosition\",\"outputs\":[{\"internalType\":\"struct Position.Info\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bool\",\"name\":\"isSettled\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"_liquidity\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"margin\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenGrowthInsideLastX128\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"variableTokenGrowthInsideLastX128\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenBalance\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"variableTokenBalance\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"feeGrowthInsideLastX128\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"rewardPerAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"accumulatedFees\",\"type\":\"uint256\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"_tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"_tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"_isLM\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"getPositionMarginRequirement\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract IERC20Minimal\",\"name\":\"__underlyingToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IRateOracle\",\"name\":\"__rateOracle\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"__termStartTimestampWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"__termEndTimestampWad\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isAlpha\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"_tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"_tickUpper\",\"type\":\"int24\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"liquidatePosition\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"liquidatorRewardWad\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lookbackWindowInSeconds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"marginEngineParameters\",\"outputs\":[{\"internalType\":\"struct IMarginEngine.MarginCalculatorParameters\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"apyUpperMultiplierWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"apyLowerMultiplierWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"sigmaSquaredWad\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"alphaWad\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"betaWad\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"xiUpperWad\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"xiLowerWad\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"tMaxWad\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"etaIMWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"etaLMWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap1\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap2\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap3\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap4\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap5\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap6\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap7\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"minMarginToIncentiviseLiquidators\",\"type\":\"uint256\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"paused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"proxiableUUID\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"rateOracle\",\"outputs\":[{\"internalType\":\"contract IRateOracle\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_newCacheMaxAgeInSeconds\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setCacheMaxAgeInSeconds\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract IFCM\",\"name\":\"_newFCM\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setFCM\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"__isAlpha\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setIsAlpha\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_newLiquidatorRewardWad\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLiquidatorReward\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_newSecondsAgo\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLookbackWindowInSeconds\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IMarginEngine.MarginCalculatorParameters\",\"name\":\"_marginCalculatorParameters\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"apyUpperMultiplierWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"apyLowerMultiplierWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"sigmaSquaredWad\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"alphaWad\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"betaWad\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"xiUpperWad\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"xiLowerWad\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"tMaxWad\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"etaIMWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"etaLMWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap1\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap2\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap3\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap4\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap5\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap6\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap7\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"minMarginToIncentiviseLiquidators\",\"type\":\"uint256\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setMarginCalculatorParameters\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"state\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPausability\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract IRateOracle\",\"name\":\"__rateOracle\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setRateOracle\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract IVAMM\",\"name\":\"_vAMM\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setVAMM\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"_tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"_tickUpper\",\"type\":\"int24\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"settlePosition\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"termEndTimestampWad\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"termStartTimestampWad\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_marginDelta\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferMarginToFCMTrader\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"underlyingToken\",\"outputs\":[{\"internalType\":\"contract IERC20Minimal\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"_tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"_tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_marginDelta\",\"type\":\"int256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updatePositionMargin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IPositionStructs.ModifyPositionParams\",\"name\":\"_params\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"liquidityDelta\",\"type\":\"int128\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updatePositionPostVAMMInducedMintBurn\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"_positionMarginRequirement\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"_tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"_tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_fixedTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_variableTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_cumulativeFeeIncurred\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_fixedTokenDeltaUnbalanced\",\"type\":\"int256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updatePositionPostVAMMInducedSwap\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"_positionMarginRequirement\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newImplementation\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"upgradeTo\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newImplementation\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"upgradeToAndCall\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"vamm\",\"outputs\":[{\"internalType\":\"contract IVAMM\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MARGINENGINE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60a0604052306080523480156200001557600080fd5b50606554610100900460ff1615808015620000375750606554600160ff909116105b8062000067575062000054306200014160201b620024561760201c565b15801562000067575060655460ff166001145b620000cf5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b606482015260840160405180910390fd5b6065805460ff191660011790558015620000f3576065805461ff0019166101001790555b80156200013a576065805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b5062000150565b6001600160a01b03163b151590565b6080516161046200018860003960008181610cf901528181610d3901528181610dd901528181610e190152610eac01526161046000f3fe6080604052600436106102935760003560e01c80639cbff1881161015a578063cf3c99bd116100c1578063e9e441bb1161007a578063e9e441bb14610818578063eb990c5914610838578063efcfc3f914610858578063f121610514610878578063f2fde38b14610898578063f907bd6d146108b857600080fd5b8063cf3c99bd1461077f578063d50d88111461079f578063e087caf1146107b4578063e098372c146107c9578063e3f08374146107e7578063e6e306c9146107fc57600080fd5b8063c09617ae11610113578063c09617ae146106e1578063c2ee3a0814610409578063c326189214610701578063c45a015514610721578063c7607a9c1461073f578063cd41b3d51461075f57600080fd5b80639cbff18814610637578063a1ea6a201461064c578063a725b9651461066c578063b5c22d491461068c578063b623f519146106ac578063bfb5607d146106c157600080fd5b8063652c30b7116101fe57806387e16303116101b757806387e163031461058557806388428752146105a15780638da5cb5b146105b95780639209e9ba146105d757806393edb4541461060457806398f4b1b21461061957600080fd5b8063652c30b7146104255780636938217f1461043a578063715018a614610519578063754e2a8f1461052e5780637717797f1461054e57806386b127ee1461056e57600080fd5b806352d1902d1161025057806352d1902d14610370578063534d3375146103855780635c975abb146103a55780635dcc9391146103d45780635f6a3e0c146103f357806363f573811461040957600080fd5b80630d211954146102985780631656503e146102ba57806322d23b21146102ed5780632495a5991461031f5780633659cfe61461033d5780634f1ef2861461035d575b600080fd5b3480156102a457600080fd5b506102b86102b33660046155db565b6108d0565b005b3480156102c657600080fd5b506102da6102d536600461561c565b61097a565b6040519081526020015b60405180910390f35b3480156102f957600080fd5b506004546001600160a01b03165b6040516001600160a01b0390911681526020016102e4565b34801561032b57600080fd5b506001546001600160a01b0316610307565b34801561034957600080fd5b506102b8610358366004615667565b610cee565b6102b861036b3660046156f5565b610dce565b34801561037c57600080fd5b506102da610e9f565b34801561039157600080fd5b506102b86103a0366004615667565b610f52565b3480156103b157600080fd5b50601f546103c490610100900460ff1681565b60405190151581526020016102e4565b3480156103e057600080fd5b506102da6a1a1601fc4ea7109e00000081565b3480156103ff57600080fd5b506102da610e1081565b34801561041557600080fd5b506102da670de0b6b3a764000081565b34801561043157600080fd5b506002546102da565b34801561044657600080fd5b5061044f610fa4565b6040516102e49190815181526020808301519082015260408083015190820152606080830151908201526080808301519082015260a0808301519082015260c0808301519082015260e08083015190820152610100808301519082015261012080830151908201526101408083015190820152610160808301519082015261018080830151908201526101a080830151908201526101c080830151908201526101e08083015190820152610200808301519082015261022091820151918101919091526102400190565b34801561052557600080fd5b506102b86110da565b34801561053a57600080fd5b506102b861054936600461579d565b6110ee565b34801561055a57600080fd5b506102b86105693660046157c9565b6111e2565b34801561057a57600080fd5b506102da6212750081565b34801561059157600080fd5b506102da670429d069189e000081565b3480156105ad57600080fd5b50601f5460ff166103c4565b3480156105c557600080fd5b506098546001600160a01b0316610307565b3480156105e357600080fd5b506105f76105f236600461561c565b61151a565b6040516102e4919061581a565b34801561061057600080fd5b506003546102da565b34801561062557600080fd5b50600c546001600160a01b0316610307565b34801561064357600080fd5b506007546102da565b34801561065857600080fd5b506102b861066736600461589a565b611621565b34801561067857600080fd5b506102b861068736600461561c565b6116d0565b34801561069857600080fd5b506102b86106a73660046158b3565b6118e1565b3480156106b857600080fd5b50600a546102da565b3480156106cd57600080fd5b506102da6106dc36600461598f565b611a6a565b3480156106ed57600080fd5b506102da6106fc366004615a1b565b611ba7565b34801561070d57600080fd5b506102b861071c36600461589a565b611deb565b34801561072d57600080fd5b50600b546001600160a01b0316610307565b34801561074b57600080fd5b506102b861075a366004615667565b611e65565b34801561076b57600080fd5b506102b861077a3660046155db565b611eb7565b34801561078b57600080fd5b506102b861079a366004615667565b611f06565b3480156107ab57600080fd5b506102da611f58565b3480156107c057600080fd5b506000546102da565b3480156107d557600080fd5b506006546001600160a01b0316610307565b3480156107f357600080fd5b506102da611f85565b34801561080857600080fd5b506102da67d02ab486cedc000081565b34801561082457600080fd5b506102b861083336600461589a565b611fe8565b34801561084457600080fd5b506102b8610853366004615a89565b612066565b34801561086457600080fd5b506102b861087336600461579d565b6122d9565b34801561088457600080fd5b506102da610893366004615acf565b612343565b3480156108a457600080fd5b506102b86108b3366004615667565b6123e0565b3480156108c457600080fd5b506102da6312cc030081565b6006546001600160a01b031633146108fb57604051633dec6c6960e11b815260040160405180910390fd5b601f80548215156101000261ff001990911617905560048054604051630348465560e21b81526001600160a01b0390911691630d2119549161094591859101901515815260200190565b600060405180830381600087803b15801561095f57600080fd5b505af1158015610973573d6000803e3d6000fd5b5050505050565b601f54600090610100900460ff16156109ae5760405162461bcd60e51b81526004016109a590615b2b565b60405180910390fd5b6109b9600354612465565b156109d7576040516314eb7aa760e21b815260040160405180910390fd5b6109e1838361248d565b60006109f0600586868661254e565b90506109ff81858560006125be565b6000610a0c8286866127bd565b50905080610a2d5760405163bf87c7d560e01b815260040160405180910390fd5b6007820154610a9757600080836005015412610a4d578260050154610a5b565b8260050154610a5b90615b61565b9050600083600101541315610a8d57610a83610a7d84600101546000546127e4565b826127f0565b6007840155610a95565b600060078401555b505b815460009061010090046001600160801b031615610b97576006548354604051631f2f089360e01b81526001600160a01b038a8116600483015260028a810b602484015289900b60448301526101009092046001600160801b03166064820152911690631f2f089390608401602060405180830381600087803b158015610b1d57600080fd5b505af1158015610b31573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610b559190615b7e565b508254610b7a90610b739061010090046001600160801b0316615b97565b8490612805565b610b8a83600101546000546127e4565b610b949082615bc7565b90505b6000610ba584898989612912565b90508015610beb5760008112610bc857610bc38185600701546127e4565b610bde565b610bde610bd482615b61565b85600701546127e4565b610be89083615bc7565b91505b8115610c2457610c0d610bfd83612aa2565b610c0690615b61565b8590612aeb565b600154610c24906001600160a01b03163384612b08565b6040805133815260208101839052908101839052600287810b919089900b906001600160a01b038b16907f743fc9c78420f1cdcbbcb2ed0928d77e4a043cf392481c7e9edbf27bf7a3cea19060600160405180910390a48560020b8760020b896001600160a01b03166000805160206160688339815191528760000160019054906101000a90046001600160801b0316886001015489600401548a600501548b60080154604051610cd9959493929190615bdf565b60405180910390a450925050505b9392505050565b306001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161415610d375760405162461bcd60e51b81526004016109a590615c0d565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316610d80600080516020616088833981519152546001600160a01b031690565b6001600160a01b031614610da65760405162461bcd60e51b81526004016109a590615c59565b610daf81612b6b565b60408051600080825260208201909252610dcb91839190612b73565b50565b306001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161415610e175760405162461bcd60e51b81526004016109a590615c0d565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316610e60600080516020616088833981519152546001600160a01b031690565b6001600160a01b031614610e865760405162461bcd60e51b81526004016109a590615c59565b610e8f82612b6b565b610e9b82826001612b73565b5050565b6000306001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614610f3f5760405162461bcd60e51b815260206004820152603860248201527f555550535570677261646561626c653a206d757374206e6f742062652063616c60448201527f6c6564207468726f7567682064656c656761746563616c6c000000000000000060648201526084016109a5565b5060008051602061608883398151915290565b610f5a612ced565b600480546001600160a01b0319166001600160a01b0383169081179091556040517fb637eb25e9652bee83990e0c20b043e658d22a8b4739422ebd3862bf9ff53dc590600090a250565b6110326040518061024001604052806000815260200160008152602001600081526020016000815260200160008152602001600081526020016000815260200160008152602001600081526020016000815260200160008152602001600081526020016000815260200160008152602001600081526020016000815260200160008152602001600081525090565b506040805161024081018252600d548152600e546020820152600f54918101919091526010546060820152601154608082015260125460a082015260135460c082015260145460e0820152601554610100820152601654610120820152601754610140820152601854610160820152601954610180820152601a546101a0820152601b546101c0820152601c546101e0820152601d54610200820152601e5461022082015290565b6110e2612ced565b6110ec6000612d47565b565b601f54610100900460ff16156111165760405162461bcd60e51b81526004016109a590615b2b565b61111e612ced565b801561119a57600654604051630867377160e41b8152600481018390526001600160a01b0390911690638673771090602401600060405180830381600087803b15801561116a57600080fd5b505af115801561117e573d6000803e3d6000fd5b505060015461119a92506001600160a01b031690508383612b08565b60408051338152602081018390526001600160a01b038416917fb78dfa45a5ff63131f1605e70c83cda5d70122e260919864eeee2ae01e3e459f910160405180910390a25050565b601f54610100900460ff161561120a5760405162461bcd60e51b81526004016109a590615b2b565b808061122957604051638acc6d7f60e01b815260040160405180910390fd5b6001600160a01b0385166112645760405162461bcd60e51b815260206004820152600260248201526104f360f41b60448201526064016109a5565b61126e848461248d565b600061127d600587878761254e565b601f5490915060ff161561135157600b5460408051633bd5670d60e11b815290516000926001600160a01b0316916377aace1a916004808301926020929190829003018186803b1580156112d057600080fd5b505afa1580156112e4573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906113089190615ca5565b9050336001600160a01b0382161461134f5760405162461bcd60e51b815260206004820152600a6024820152697070687279206f6e6c7960b01b60448201526064016109a5565b505b61135e81868660006125be565b6000831215611442576001600160a01b03861633148015906114005750600b546040516351c4bc1f60e11b81526001600160a01b0388811660048301523360248301529091169063a389783e9060440160206040518083038186803b1580156113c657600080fd5b505afa1580156113da573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906113fe9190615cc2565b155b1561141e57604051637da45ce760e01b815260040160405180910390fd5b6114288184612aeb565b611433818686612d99565b61143d8684612dfc565b611456565b61144c8184612aeb565b6114563384612dfc565b600060078201556040805133815260208101859052600286810b929088900b916001600160a01b038a16917f58fda8ef9050967ebeb4f3bc6baea53d849c21755d63a2abb4507c5db1b118ec910160405180910390a48360020b8560020b876001600160a01b03166000805160206160688339815191528460000160019054906101000a90046001600160801b0316856001015486600401548760050154886008015460405161150a959493929190615bdf565b60405180910390a4505050505050565b61157b60405180610140016040528060001515815260200160006001600160801b0316815260200160008152602001600081526020016000815260200160008152602001600081526020016000815260200160008152602001600081525090565b600061158a600586868661254e565b905061159981858560006125be565b6040805161014081018252825460ff811615158252610100908190046001600160801b031660208301526001840154928201929092526002830154606082015260038301546080820152600483015460a0820152600583015460c0820152600683015460e0820152600783015491810191909152600890910154610120820152949350505050565b611629612ced565b6312cc0300811115801561163f5750610e108110155b6116745760405162461bcd60e51b815260206004820152600660248201526526211027a7a160d11b60448201526064016109a5565b600754611685576007819055611692565b6007819055611692612f56565b7f1615a39c548a63ced5cc405350ce2e18f4f50ce7d4aedb4bfb95b7e4a821a8c06007546040516116c591815260200190565b60405180910390a150565b601f54610100900460ff16156116f85760405162461bcd60e51b81526004016109a590615b2b565b611700612f67565b6003541115611722576040516301730b8160e11b815260040160405180910390fd5b61172c828261248d565b600061173b600585858561254e565b905061174a81848460006125be565b6004818101546005830154600254600354600c546040516325f258dd60e01b8152958601839052602486018290526000956117ed95949392916001600160a01b0316906325f258dd90604401602060405180830381600087803b1580156117b057600080fd5b505af11580156117c4573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906117e89190615b7e565b612f72565b9050611816826004015461180090615b61565b836005015461180e90615b61565b849190612fdb565b6118208282612aeb565b6118298261301c565b8260020b8460020b866001600160a01b03167f970071e0d424aa8ce645bd8034df7db5eec3ce8fb9d83833a84320fb29b2c9fc8460405161186c91815260200190565b60405180910390a48260020b8460020b866001600160a01b03166000805160206160688339815191528560000160019054906101000a90046001600160801b031686600101548760040154886005015489600801546040516118d2959493929190615bdf565b60405180910390a45050505050565b6118e9612ced565b8051600d9081556020820151600e55604080830151600f556060830151601055608083015160115560a083015160125560c083015160135560e08301516014556101008301516015556101208301516016556101408301516017556101608301516018556101808301516019556101a0830151601a556101c0830151601b556101e0830151601c55610200830151601d55610220830151601e55517f96fef58d97876707a1bd5650c97ca391d5558a8c30e1c96363c89486cb9aa8c2916116c5918154815260018201546020820152600282015460408201526003820154606082015260048201546080820152600582015460a0820152600682015460c0820152600782015460e082015260088201546101008201526009820154610120820152600a820154610140820152600b820154610160820152600c820154610180820152600d8201546101a0820152600e8201546101c0820152600f8201546101e082015260108201546102008201526011909101546102208201526102400190565b601f54600090610100900460ff1615611a955760405162461bcd60e51b81526004016109a590615b2b565b6006546001600160a01b03163314611ac057604051633dec6c6960e11b815260040160405180910390fd5b815160208301516040840151600092611adb9260059261254e565b9050611af2818460200151856040015160016125be565b6060830151611b02908290612805565b60008360600151600f0b1315611b2857611b25818460200151856040015161306d565b91505b60006007820155826040015160020b836020015160020b84600001516001600160a01b03166000805160206160688339815191528460000160019054906101000a90046001600160801b03168560010154866004015487600501548860080154604051611b99959493929190615bdf565b60405180910390a450919050565b601f54600090610100900460ff1615611bd25760405162461bcd60e51b81526004016109a590615b2b565b6006546001600160a01b03163314611bfd57604051633dec6c6960e11b815260040160405180910390fd5b6000611c0c60058a8a8a61254e565b9050611c1b81898960006125be565b6000808260050154138015611c305750600086125b80611c4a575060008260050154128015611c4a5750600086135b90508415611c6e57611c6e611c5e86612aa2565b611c6790615b61565b8390612aeb565b611c79828888612fdb565b611c8e611c89838b8b60006130a8565b612aa2565b9250816001015483138015611ca1575080155b15611d6e57600654604080516320283ddb60e21b815290516000926001600160a01b0316916380a0f76c916004808301926060929190829003018186803b158015611ceb57600080fd5b505afa158015611cff573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611d239190615cdf565b60208101516040516343f2832160e01b81526004810187905260029190910b6024820152604481018a9052606481018990526084810188905260a4810187905290915060c4016109a5565b600082600701819055508760020b8960020b8b6001600160a01b03166000805160206160688339815191528560000160019054906101000a90046001600160801b03168660010154876004015488600501548960080154604051611dd6959493929190615bdf565b60405180910390a45050979650505050505050565b611df3612ced565b62127500811115611e305760405162461bcd60e51b815260206004820152600760248201526621a6a09027a7a160c91b60448201526064016109a5565b600a8190556040518181527f03f78e38097f23422a330825158a9cf778080b032c154a45063464b46a328f95906020016116c5565b611e6d612ced565b600680546001600160a01b0319166001600160a01b0383169081179091556040517f8bd432982306c1f9ddd987c98d3842200f8aa7668b030cbdea0a45fd31f5d69c90600090a250565b611ebf612ced565b601f805460ff191682151590811790915560405160ff909116151581527fa201234976cfdc556c03f06ca9366e09441724eae79256ad9da6b5f04cbdb058906020016116c5565b611f0e612ced565b600c80546001600160a01b0319166001600160a01b0383169081179091556040517fbdf78832ed83738bb07ebf2c5671aa6b81e05ca40d52cbdf5cc9f327829a3d7090600090a250565b6000600a5460095442611f6b9190615d57565b1115611f7e57611f7961337f565b905090565b5060085490565b6000600a5460095442611f989190615d57565b1115611f7e57611fa6612f56565b7fa54badf52ad5c1729ee2a0e934e6b23e9a3a037d7a761ccd78b01a8db4bf3f14600854604051611fd991815260200190565b60405180910390a15060085490565b611ff0612ced565b670429d069189e00008111156120315760405162461bcd60e51b815260206004820152600660248201526526291027a7a160d11b60448201526064016109a5565b60008190556040518181527f7c1f79218de766d3f02f194836ae0d52b5b8a3fb34f3d76795d9fe9050fc2ea9906020016116c5565b606554610100900460ff16158080156120865750606554600160ff909116105b806120a05750303b1580156120a0575060655460ff166001145b6121035760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084016109a5565b6065805460ff191660011790558015612126576065805461ff0019166101001790555b6001600160a01b0385166121615760405162461bcd60e51b8152602060048201526002602482015261155560f21b60448201526064016109a5565b6001600160a01b03841661219c5760405162461bcd60e51b8152602060048201526002602482015261524f60f01b60448201526064016109a5565b826121ce5760405162461bcd60e51b8152602060048201526002602482015261545360f01b60448201526064016109a5565b816122005760405162461bcd60e51b8152602060048201526002602482015261544560f01b60448201526064016109a5565b8282116122385760405162461bcd60e51b815260206004820152600660248201526554453c3d545360d01b60448201526064016109a5565b600180546001600160a01b038088166001600160a01b03199283161790925560028590556003849055600c805492871692821692909217909155600b805490911633179055612285613418565b61228d613447565b8015610973576065805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15050505050565b601f54610100900460ff16156123015760405162461bcd60e51b81526004016109a590615b2b565b6004546001600160a01b0316331461232c57604051635d8a367560e01b815260040160405180910390fd5b600154610e9b906001600160a01b03168383612b08565b600080612353600587878761254e565b905061236281868660006125be565b8360020b8560020b876001600160a01b03166000805160206160688339815191528460000160019054906101000a90046001600160801b031685600101548660040154876005015488600801546040516123c0959493929190615bdf565b60405180910390a46123d4818686866130a8565b9150505b949350505050565b6123e8612ced565b6001600160a01b03811661244d5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b60648201526084016109a5565b610dcb81612d47565b6001600160a01b03163b151590565b60008169124bc0ddd92e5600000061247b612f67565b6124859190615bc7565b101592915050565b8060020b8260020b126124c85760405162461bcd60e51b8152602060048201526003602482015262544c5560e81b60448201526064016109a5565b62010deb19600283900b12156125065760405162461bcd60e51b8152602060048201526003602482015262544c4d60e81b60448201526064016109a5565b61251362010deb19615d6e565b60020b8160020b1315610e9b5760405162461bcd60e51b815260206004820152600360248201526254554d60e81b60448201526064016109a5565b600061255a838361248d565b6040516bffffffffffffffffffffffff19606086901b16602082015260e884811b603483015283901b60378201528590600090603a016040516020818303038152906040528051906020012081526020019081526020016000209050949350505050565b835461010090046001600160801b03161561270f57600654604051631e47919f60e11b8152600285810b600483015284900b6024820152600091829182916001600160a01b031690633c8f233e9060440160606040518083038186803b15801561262757600080fd5b505afa15801561263b573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061265f9190615d88565b9194509250905060008061267489868661346e565b909250905060006126858a8561355e565b90506126a8612695600185615db6565b6126a0600185615db6565b8c9190612fdb565b60028a0186905560038a0185905580156126ff576126c7600182615d57565b8a60080160008282546126da9190615bc7565b909155506126ff905060016126ee83612aa2565b6126f89190615db6565b8b90612aeb565b5050506006870155506127b79050565b80156127b757600654604051631e47919f60e11b8152600285810b600483015284900b6024820152600091829182916001600160a01b031690633c8f233e9060440160606040518083038186803b15801561276957600080fd5b505afa15801561277d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906127a19190615d88565b60028a0192909255600389015560068801555050505b50505050565b60008060006127d2611c8987878760016130a8565b60019690960154861396945050505050565b6000610ce783836135f5565b6000610ce783670de0b6b3a7640000846136b8565b6040805161014081018252835460ff8116151582526001600160801b03610100918290041660208301526001850154928201929092526002840154606082015260038401546080820152600484015460a0820152600584015460c0820152600684015460e08201526007840154918101919091526008830154610120820152600f82900b6128d457600081602001516001600160801b0316116128cf5760405162461bcd60e51b815260206004820152600260248201526104e560f41b60448201526064016109a5565b505050565b6128e2816020015183613786565b83546001600160801b03919091166101000270ffffffffffffffffffffffffffffffff0019909116178355505050565b600061291e838361248d565b6005850154156123d857600080600080886005015412905060006040518060a00160405280896001600160a01b031681526020018a6005015481526020018361297e5761297960016c1fa71f3f5f68a90479ee3f8fec615df5565b612995565b6129956b0816769404766de590afe04e6001615e1d565b6001600160a01b03908116825260028a810b6020808501919091528a820b60409485015260065484516333bac73760e11b815286518516600482015291860151602483015293850151831660448201526060850151820b6064820152608085015190910b608482015292935016906367758e6e9060a40160a060405180830381600087803b158015612a2657600080fd5b505af1158015612a3a573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612a5e9190615e48565b50919750919550909350508215612a8b57612a8b612a7b84612aa2565b612a8490615b61565b8a90612aeb565b612a96898587612fdb565b50505050949350505050565b6000600160ff1b8210612ae75760405162461bcd60e51b815260206004820152600d60248201526c746f496e74323536206f666c6f60981b60448201526064016109a5565b5090565b80826001016000828254612aff9190615e88565b90915550505050565b6040516001600160a01b0383166024820152604481018290526128cf90849063a9059cbb60e01b906064015b60408051601f198184030181529190526020810180516001600160e01b03166001600160e01b0319909316929092179091526137b4565b610dcb612ced565b7f4910fdfa16fed3260ed0e7147f7cc6da11a60208b5b9406d12a635614ffd91435460ff1615612ba6576128cf83613835565b826001600160a01b03166352d1902d6040518163ffffffff1660e01b815260040160206040518083038186803b158015612bdf57600080fd5b505afa925050508015612c0f575060408051601f3d908101601f19168201909252612c0c91810190615b7e565b60015b612c725760405162461bcd60e51b815260206004820152602e60248201527f45524331393637557067726164653a206e657720696d706c656d656e7461746960448201526d6f6e206973206e6f74205555505360901b60648201526084016109a5565b6000805160206160888339815191528114612ce15760405162461bcd60e51b815260206004820152602960248201527f45524331393637557067726164653a20756e737570706f727465642070726f786044820152681a58589b195555525160ba1b60648201526084016109a5565b506128cf8383836138d1565b6098546001600160a01b031633146110ec5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016109a5565b609880546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b600354612da4612f67565b10612df157825460ff16612dcb5760405163169b07f760e21b815260040160405180910390fd5b6000836001015412156128cf57604051630a5f871f60e21b815260040160405180910390fd5b6127b783838361306d565b6000811315612e1d57600154610e9b906001600160a01b03168330846138f6565b6001546040516370a0823160e01b81523060048201526000916001600160a01b0316906370a082319060240160206040518083038186803b158015612e6157600080fd5b505afa158015612e75573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612e999190615b7e565b9050600082900381811115612f3f578115612ed257612eb88282615d57565b600154909150612ed2906001600160a01b03168584612b08565b600480546040516318399f4d60e31b81526001600160a01b03878116938201939093526024810184905291169063c1ccfa6890604401600060405180830381600087803b158015612f2257600080fd5b505af1158015612f36573d6000803e3d6000fd5b505050506127b7565b6001546127b7906001600160a01b03168583612b08565b612f5e61337f565b60085542600955565b6000611f794261392e565b600080612f7e8761397b565b90506000612f8b8761397b565b90506000612fa5612f9e600189896139f8565b8490613ab4565b90506000612fb38387613ab4565b90506000612fc18284615e88565b670de0b6b3a764000090059b9a5050505050505050505050565b818117156128cf5781836004016000828254612ff79190615e88565b92505081905550808360050160008282546130129190615e88565b9091555050505050565b805460ff16156130605760405162461bcd60e51b815260206004820152600f60248201526e185b1c9958591e481cd95d1d1b1959608a1b60448201526064016109a5565b805460ff19166001179055565b600061307f611c8985858560006130a8565b905080846001015413610ce757604051631ad3ffc960e21b8152600481018290526024016109a5565b60006130b4848461248d565b600654604080516320283ddb60e21b815290516000926001600160a01b0316916380a0f76c916004808301926060929190829003018186803b1580156130f957600080fd5b505afa15801561310d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906131319190615cdf565b6020810151600c546002546003546040516325f258dd60e01b81526004810192909252602482015292935090916000916001600160a01b0316906325f258dd90604401602060405180830381600087803b15801561318e57600080fd5b505af11580156131a2573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906131c69190615b7e565b885490915061010090046001600160801b031615613362576132136040518060a00160405280600060020b8152602001600081526020016000815260200160008152602001600081525090565b8760020b8360020b12613239578660020b8360020b12613233578661323b565b8261323b565b875b600290810b8252600090819089810b9086900b12156132795782518b5461327391908b9061010090046001600160801b031687613b79565b90925090505b808b600501546132899190615e88565b602084015260048b015461329e908390615e88565b604084015260028a810b9086900b13156132db5782518b546132d191908c9061010090046001600160801b031687613b79565b90925090506132e2565b5060009050805b808b600501546132f29190615e88565b606084015260048b0154613307908390615e88565b608084015260408301516020840151600091613323918b613c25565b9050600061333a856080015186606001518c613c25565b905080821115613353575096506123d895505050505050565b97506123d89650505050505050565b6133758860040154896005015487613c25565b93505050506123d8565b600080600754426133909190615d57565b600c546040516393556dbd60e01b8152600481018390524260248201529192506000916001600160a01b03909116906393556dbd9060440160206040518083038186803b1580156133e057600080fd5b505afa1580156133f4573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610ce79190615b7e565b606554610100900460ff1661343f5760405162461bcd60e51b81526004016109a590615ec9565b6110ec613cc8565b606554610100900460ff166110ec5760405162461bcd60e51b81526004016109a590615ec9565b6040805161014081018252845460ff8116151582526001600160801b036101009182900416602083015260018601549282019290925260028501546060820181905260038601546080830152600486015460a0830152600586015460c0830152600686015460e0830152600786015492820192909252600885015461012082015260009182919082906135019087615db6565b905061351f8183602001516001600160801b0316600160801b613cf8565b935060008260800151866135339190615db6565b90506135518184602001516001600160801b0316600160801b613cf8565b9350505050935093915050565b6040805161014081018252835460ff8116151582526001600160801b036101009182900416602083018190526001860154938301939093526002850154606083015260038501546080830152600485015460a0830152600585015460c0830152600685015460e0830181905260078601549183019190915260088501546101208301526000926123d891850390600160801b613d2c565b60008080600019848609848602925082811083820303915050670de0b6b3a764000081106136395760405163698d9a0160e11b8152600481018290526024016109a5565b600080670de0b6b3a76400008688099150506706f05b59d3b1ffff8111826136735780670de0b6b3a76400008504019450505050506136b2565b620400008285030493909111909103600160ee1b02919091177faccb18165bd6fe31ae1cf318dc5b51eee0e1ba569b88cd74c1773b91fac10669020190505b92915050565b6000808060001985870985870292508281108382030391505080600014156136f3578382816136e9576136e9615f14565b0492505050610ce7565b83811061371d57604051631dcf306360e21b815260048101829052602481018590526044016109a5565b60008486880960026001871981018816978890046003810283188082028403028082028403028082028403028082028403028082028403029081029092039091026000889003889004909101858311909403939093029303949094049190911702949350505050565b60008082600f0b12156137aa5760008290036137a28185615f2a565b9150506136b2565b610ce78284615f4a565b60006137e083836040518060400160405280600781526020016629aa261032b93960c91b815250613dd6565b8051909150156128cf57808060200190518101906137fe9190615cc2565b6128cf5760405162461bcd60e51b815260206004820152600860248201526714d5130819985a5b60c21b60448201526064016109a5565b6001600160a01b0381163b6138a25760405162461bcd60e51b815260206004820152602d60248201527f455243313936373a206e657720696d706c656d656e746174696f6e206973206e60448201526c1bdd08184818dbdb9d1c9858dd609a1b60648201526084016109a5565b60008051602061608883398151915280546001600160a01b0319166001600160a01b0392909216919091179055565b6138da83613e84565b6000825111806138e75750805b156128cf576127b78383613ec4565b6040516001600160a01b03808516602483015283166044820152606481018290526127b79085906323b872dd60e01b90608401612b34565b60007812725dd1d243aba0e75fe645cc4873f9e65afe688c928e1f2182111561396d57604051633492ffd960e01b8152600481018390526024016109a5565b50670de0b6b3a76400000290565b60007809392ee8e921d5d073aff322e62439fcf32d7f344649470f8f198212156139bb5760405163e608e18b60e01b8152600481018390526024016109a5565b7809392ee8e921d5d073aff322e62439fcf32d7f344649470f9082131561396d576040516371f72a3160e01b8152600481018390526024016109a5565b6000828211613a195760405162461bcd60e51b81526004016109a590615f6c565b6000613a23612f67565b905083811015613a5d5760405162461bcd60e51b8152602060048201526005602482015264422e543c5360d81b60448201526064016109a5565b60008580613a6b5750838210155b15613a8157613a7a8585615d57565b9050613a8e565b613a8b8583615d57565b90505b613aaa68056bc75e2d63100000613aa483613fb8565b906127f0565b9695505050505050565b6000600160ff1b831480613acb5750600160ff1b82145b15613ae957604051630d01a11b60e21b815260040160405180910390fd5b60008060008512613afa5784613aff565b846000035b915060008412613b0f5783613b14565b836000035b90506000613b2283836135f5565b90506001600160ff1b03811115613b4f5760405163bf79e8d960e01b8152600481018290526024016109a5565b600019808713908613808218600114613b685782613b6d565b826000035b98975050505050505050565b6000808460020b8660020b1415613b9557506000905080613c1c565b6000613ba087613fcf565b90506000613bad87613fcf565b90506000613bd783838a60020b8c60020b12613bc95789614367565b613bd28a615b97565b614367565b90506000613c0184848b60020b8d60020b12613bfb57613bf68b615b97565b6143a9565b8a6143a9565b9050613c148282896002546003546143dd565b955093505050505b94509492505050565b6000613c32848484614469565b90506000808412613c4b57613c4684614533565b613c5c565b613c5c613c5785615b61565b614533565b90506000613ca1613c80613c6e612f67565b600354613c7b9190615d57565b613fb8565b613c9b86613c9057601554613c94565b6016545b85906127e4565b906127e4565b905080831015613caf578092505b601e54831015613cbf57601e5492505b50509392505050565b606554610100900460ff16613cef5760405162461bcd60e51b81526004016109a590615ec9565b6110ec33612d47565b600080841215613d2557613d15613d0e85615b61565b8484613d2c565b613d1e90615b61565b9050610ce7565b6123d88484845b600080806000198587098587029250828110838203039150508060001415613d9c5760008411613d915760405162461bcd60e51b815260206004820152601060248201526f4469766973696f6e206279207a65726f60801b60448201526064016109a5565b508290049050610ce7565b80841161371d5760405162461bcd60e51b81526020600482015260086024820152676f766572666c6f7760c01b60448201526064016109a5565b6060833b613e155760405162461bcd60e51b815260206004820152600c60248201526b1b9bdb8b58dbdb9d1c9858dd60a21b60448201526064016109a5565b600080856001600160a01b0316600086604051613e329190615fb6565b60006040518083038185875af1925050503d8060008114613e6f576040519150601f19603f3d011682016040523d82523d6000602084013e613e74565b606091505b5091509150613aaa828286614575565b613e8d81613835565b6040516001600160a01b038216907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b90600090a250565b60606001600160a01b0383163b613f2c5760405162461bcd60e51b815260206004820152602660248201527f416464726573733a2064656c65676174652063616c6c20746f206e6f6e2d636f6044820152651b9d1c9858dd60d21b60648201526084016109a5565b600080846001600160a01b031684604051613f479190615fb6565b600060405180830381855af49150503d8060008114613f82576040519150601f19603f3d011682016040523d82523d6000602084013e613f87565b606091505b5091509150613faf82826040518060600160405280602781526020016160a8602791396145ae565b95945050505050565b60006136b2826a1a1601fc4ea7109e0000006127f0565b60008060008360020b12613fe6578260020b613ff3565b8260020b613ff390615b61565b905061400262010deb19615d6e565b60020b8111156140385760405162461bcd60e51b81526020600482015260016024820152601560fa1b60448201526064016109a5565b60006001821661404c57600160801b61405e565b6ffffcb933bd6fad37aa2d162d1a5940015b70ffffffffffffffffffffffffffffffffff169050600282161561409d576080614098826ffff97272373d413259a46990580e213a615fd2565b901c90505b60048216156140c75760806140c2826ffff2e50f5f656932ef12357cf3c7fdcc615fd2565b901c90505b60088216156140f15760806140ec826fffe5caca7e10e4e61c3624eaa0941cd0615fd2565b901c90505b601082161561411b576080614116826fffcb9843d60f6159c9db58835c926644615fd2565b901c90505b6020821615614145576080614140826fff973b41fa98c081472e6896dfb254c0615fd2565b901c90505b604082161561416f57608061416a826fff2ea16466c96a3843ec78b326b52861615fd2565b901c90505b6080821615614199576080614194826ffe5dee046a99a2a811c461f1969c3053615fd2565b901c90505b6101008216156141c45760806141bf826ffcbe86c7900a88aedcffc83b479aa3a4615fd2565b901c90505b6102008216156141ef5760806141ea826ff987a7253ac413176f2b074cf7815e54615fd2565b901c90505b61040082161561421a576080614215826ff3392b0822b70005940c7a398e4b70f3615fd2565b901c90505b610800821615614245576080614240826fe7159475a2c29b7443b29c7fa6e889d9615fd2565b901c90505b61100082161561427057608061426b826fd097f3bdfd2022b8845ad8f792aa5825615fd2565b901c90505b61200082161561429b576080614296826fa9f746462d870fdf8a65dc1f90e061e5615fd2565b901c90505b6140008216156142c65760806142c1826f70d869a156d2a1b890bb3df62baf32f7615fd2565b901c90505b6180008216156142f15760806142ec826f31be135f97d08fd981231505542fcfa6615fd2565b901c90505b6201000082161561431d576080614318826f09aa508b5b7a84e1c677de54f3e99bc9615fd2565b901c90505b60008460020b13156143385761433581600019615ff1565b90505b61434764010000000082616005565b15614353576001614356565b60005b6123d89060ff16602083901c615bc7565b60008082600f0b1261438857614383611c8985858560016145c7565b6123d8565b6143a0611c89858561439986615b97565b60006145c7565b6123d890615b61565b60008082600f0b126143c557614383611c8985858560016146d3565b6143a0611c8985856143d686615b97565b60006146d3565b60008282116143fe5760405162461bcd60e51b81526004016109a590615f6c565b8515801561440a575084155b1561441757506000613faf565b60006144228761397b565b9050600061442f8761397b565b905060006144408383898989614743565b905060006144508483898961476f565b670de0b6b3a764000090059a9950505050505050505050565b600080841215801561447c575060008312155b1561448957506000610ce7565b60006144948561397b565b905060006144a18561397b565b905060006144c1836144bc611c8960016002546003546139f8565b613ab4565b9050600086156144ea576144e7836144bc611c8960008b128a6144e2611f85565b6147b0565b90505b60006144f68284615e88565b905060008112156145225761451b61450d82615b61565b670de0b6b3a7640000900490565b9550614527565b600095505b50505050509392505050565b600080821215612ae75760405162461bcd60e51b815260206004820152600d60248201526c0746f55696e74323536203c203609c1b60448201526064016109a5565b60608315614584575081610ce7565b8251156145945782518084602001fd5b8160405162461bcd60e51b81526004016109a59190616019565b606083156145bd575081610ce7565b610ce783836148cc565b6000836001600160a01b0316856001600160a01b031611156145e7579293925b6fffffffffffffffffffffffffffffffff60601b606084901b16600061460d8787615df5565b6001600160a01b031690506000876001600160a01b0316116146665760405162461bcd60e51b8152602060048201526012602482015271073717274526174696f4158393620213e20360741b60448201526064016109a5565b8361469c57866001600160a01b03166146898383896001600160a01b0316613d2c565b8161469657614696615f14565b046146c8565b6146c86146b38383896001600160a01b03166148f6565b886001600160a01b0316808204910615150190565b979650505050505050565b6000836001600160a01b0316856001600160a01b031611156146f3579293925b816147205761471b836001600160801b03168686036001600160a01b0316600160601b613d2c565b613faf565b613faf836001600160801b03168686036001600160a01b0316600160601b6148f6565b600061474f8585613ab4565b61476561475e600086866139f8565b8890613ab4565b613aaa9190615e88565b60008282116147905760405162461bcd60e51b81526004016109a590615f6c565b6147a661479f600185856139f8565b8590614964565b613faf9086615db6565b600c546002546000918291670de0b6b3a7640000916001600160a01b0316906341453528906147dd612f67565b6040516001600160e01b031960e085901b1681526004810192909252602482015260440160206040518083038186803b15801561481957600080fd5b505afa15801561482d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906148519190615b7e565b61485b9190615bc7565b905060006148698487614a2a565b90508461488e5761488b8661488057600e54614884565b600d545b82906127e4565b90505b670de0b6b3a76400006148c2670de0b6b3a76400006148b1613c94613c6e612f67565b6148bb9190615bc7565b84906127e4565b613aaa9190615d57565b8151156148dc5781518083602001fd5b8060405162461bcd60e51b81526004016109a59190616019565b6000614903848484613d2c565b90506000828061491557614915615f14565b8486091115610ce757600019811061495a5760405162461bcd60e51b81526020600482015260086024820152676f766572666c6f7760c01b60448201526064016109a5565b80613faf8161604c565b6000600160ff1b83148061497b5750600160ff1b82145b156149995760405163b3c754a360e01b815260040160405180910390fd5b600080600085126149aa57846149af565b846000035b9150600084126149bf57836149c4565b836000035b905060006149db83670de0b6b3a7640000846136b8565b90506001600160ff1b03811115614a0857604051637cb4bef560e01b8152600481018290526024016109a5565b600019808713908613808218600114614a215782613b6d565b613b6d83615b61565b600080614a35614b35565b600f54601054919250600091614a519160029190911b90614964565b90506000614a92614a6a84670de0b6b3a7640000615db6565b600f54601154614a8c919082908b90614a869060021b8a613ab4565b90613ab4565b90614964565b90506000614ac286614aa657601354614aaa565b6012545b614a866001614abb8787831b615e88565b901b614be7565b905085614ad557614ad281615b61565b90505b6000614b1782614ae58587615e88565b614aef9190615e88565b601154614a869060021b614a8c614b0e8a670de0b6b3a7640000615db6565b600f5490613ab4565b905060008112614b275780613b6d565b600098975050505050505050565b600080614b40612f67565b9050600354811115614b7c5760405162461bcd60e51b815260206004820152600560248201526410d50f115560da1b60448201526064016109a5565b601154614bb05760405162461bcd60e51b8152602060048201526002602482015261042360f41b60448201526064016109a5565b614be1614bdc600d60040154614bc590615b61565b614a86600d6007015485600354614a8c9190615db6565b614c5d565b91505090565b600080821215614c0d5760405163608c83ff60e11b8152600481018390526024016109a5565b7809392ee8e921d5d073aff322e62439fcf32d7f344649470f90821315614c4a57604051632c482c3960e01b8152600481018390526024016109a5565b6136b2670de0b6b3a76400008302614ccd565b600068023f2fa8f6da5b9d3119821215614c7957506000919050565b680736ea4425c11ac6318212614ca5576040516399bb754160e01b8152600481018390526024016109a5565b6714057b7ef767814f8202610ce7670de0b6b3a76400006706f05b59d3b20000830105614e3e565b600081614cdc57506000919050565b50600181600160801b8110614cf65760409190911b9060801c5b680100000000000000008110614d115760209190911b9060401c5b6401000000008110614d285760109190911b9060201c5b620100008110614d3d5760089190911b9060101c5b6101008110614d515760049190911b9060081c5b60108110614d645760029190911b9060041c5b60048110614d7457600182901b91505b6001828481614d8557614d85615f14565b048301901c91506001828481614d9d57614d9d615f14565b048301901c91506001828481614db557614db5615f14565b048301901c91506001828481614dcd57614dcd615f14565b048301901c91506001828481614de557614de5615f14565b048301901c91506001828481614dfd57614dfd615f14565b048301901c91506001828481614e1557614e15615f14565b048301901c91506000828481614e2d57614e2d615f14565b04905080831015610ce757826123d8565b600080821215614e925768033dd1780914b9711419821215614e6257506000919050565b614e6e82600003614e3e565b6ec097ce7bc90715b34b9f100000000081614e8b57614e8b615f14565b0592915050565b680a688906bd8b0000008212614ebe5760405163e69458f960e01b8152600481018390526024016109a5565b670de0b6b3a7640000604083901b04610ce781600160bf1b678000000000000000821615614ef55768016a09e667f3bcc9090260401c5b674000000000000000821615614f14576801306fe0a31b7152df0260401c5b672000000000000000821615614f33576801172b83c7d517adce0260401c5b671000000000000000821615614f525768010b5586cf9890f62a0260401c5b670800000000000000821615614f71576801059b0d31585743ae0260401c5b670400000000000000821615614f9057680102c9a3e778060ee70260401c5b670200000000000000821615614faf5768010163da9fb33356d80260401c5b670100000000000000821615614fce57680100b1afa5abcbed610260401c5b6680000000000000821615614fec5768010058c86da1c09ea20260401c5b664000000000000082161561500a576801002c605e2e8cec500260401c5b662000000000000082161561502857680100162f3904051fa10260401c5b6610000000000000821615615046576801000b175effdc76ba0260401c5b660800000000000082161561506457680100058ba01fb9f96d0260401c5b66040000000000008216156150825768010002c5cc37da94920260401c5b66020000000000008216156150a0576801000162e525ee05470260401c5b66010000000000008216156150be5768010000b17255775c040260401c5b658000000000008216156150db576801000058b91b5bc9ae0260401c5b654000000000008216156150f857680100002c5c89d5ec6d0260401c5b652000000000008216156151155768010000162e43f4f8310260401c5b6510000000000082161561513257680100000b1721bcfc9a0260401c5b6508000000000082161561514f5768010000058b90cf1e6e0260401c5b6504000000000082161561516c576801000002c5c863b73f0260401c5b6502000000000082161561518957680100000162e430e5a20260401c5b650100000000008216156151a6576801000000b1721835510260401c5b6480000000008216156151c257680100000058b90c0b490260401c5b6440000000008216156151de5768010000002c5c8601cc0260401c5b6420000000008216156151fa576801000000162e42fff00260401c5b6410000000008216156152165768010000000b17217fbb0260401c5b640800000000821615615232576801000000058b90bfce0260401c5b64040000000082161561524e57680100000002c5c85fe30260401c5b64020000000082161561526a5768010000000162e42ff10260401c5b64010000000082161561528657680100000000b17217f80260401c5b63800000008216156152a15768010000000058b90bfc0260401c5b63400000008216156152bc576801000000002c5c85fe0260401c5b63200000008216156152d757680100000000162e42ff0260401c5b63100000008216156152f2576801000000000b17217f0260401c5b630800000082161561530d57680100000000058b90c00260401c5b63040000008216156153285768010000000002c5c8600260401c5b6302000000821615615343576801000000000162e4300260401c5b630100000082161561535e5768010000000000b172180260401c5b62800000821615615378576801000000000058b90c0260401c5b6240000082161561539257680100000000002c5c860260401c5b622000008216156153ac5768010000000000162e430260401c5b621000008216156153c657680100000000000b17210260401c5b620800008216156153e05768010000000000058b910260401c5b620400008216156153fa576801000000000002c5c80260401c5b6202000082161561541457680100000000000162e40260401c5b6201000082161561542e576801000000000000b1720260401c5b61800082161561544757680100000000000058b90260401c5b6140008216156154605768010000000000002c5d0260401c5b612000821615615479576801000000000000162e0260401c5b6110008216156154925768010000000000000b170260401c5b6108008216156154ab576801000000000000058c0260401c5b6104008216156154c457680100000000000002c60260401c5b6102008216156154dd57680100000000000001630260401c5b6101008216156154f657680100000000000000b10260401c5b608082161561550e57680100000000000000590260401c5b6040821615615526576801000000000000002c0260401c5b602082161561553e57680100000000000000160260401c5b6010821615615556576801000000000000000b0260401c5b600882161561556e57680100000000000000060260401c5b600482161561558657680100000000000000030260401c5b600282161561559e57680100000000000000010260401c5b60018216156155b657680100000000000000010260401c5b670de0b6b3a76400000260409190911c60bf031c90565b8015158114610dcb57600080fd5b6000602082840312156155ed57600080fd5b8135610ce7816155cd565b6001600160a01b0381168114610dcb57600080fd5b8060020b8114610dcb57600080fd5b60008060006060848603121561563157600080fd5b833561563c816155f8565b9250602084013561564c8161560d565b9150604084013561565c8161560d565b809150509250925092565b60006020828403121561567957600080fd5b8135610ce7816155f8565b634e487b7160e01b600052604160045260246000fd5b604051610240810167ffffffffffffffff811182821017156156be576156be615684565b60405290565b604051601f8201601f1916810167ffffffffffffffff811182821017156156ed576156ed615684565b604052919050565b6000806040838503121561570857600080fd5b8235615713816155f8565b915060208381013567ffffffffffffffff8082111561573157600080fd5b818601915086601f83011261574557600080fd5b81358181111561575757615757615684565b615769601f8201601f191685016156c4565b9150808252878482850101111561577f57600080fd5b80848401858401376000848284010152508093505050509250929050565b600080604083850312156157b057600080fd5b82356157bb816155f8565b946020939093013593505050565b600080600080608085870312156157df57600080fd5b84356157ea816155f8565b935060208501356157fa8161560d565b9250604085013561580a8161560d565b9396929550929360600135925050565b8151151581526101408101602083015161583f60208401826001600160801b03169052565b5060408301516040830152606083015160608301526080830151608083015260a083015160a083015260c083015160c083015260e083015160e083015261010080840151818401525061012080840151818401525092915050565b6000602082840312156158ac57600080fd5b5035919050565b600061024082840312156158c657600080fd5b6158ce61569a565b823581526020808401359082015260408084013590820152606080840135908201526080808401359082015260a0808401359082015260c0808401359082015260e08084013590820152610100808401359082015261012080840135908201526101408084013590820152610160808401359082015261018080840135908201526101a080840135908201526101c080840135908201526101e080840135908201526102008084013590820152610220928301359281019290925250919050565b6000608082840312156159a157600080fd5b6040516080810181811067ffffffffffffffff821117156159c4576159c4615684565b60405282356159d2816155f8565b815260208301356159e28161560d565b602082015260408301356159f58161560d565b60408201526060830135600f81900b8114615a0f57600080fd5b60608201529392505050565b600080600080600080600060e0888a031215615a3657600080fd5b8735615a41816155f8565b96506020880135615a518161560d565b95506040880135615a618161560d565b969995985095966060810135965060808101359560a0820135955060c0909101359350915050565b60008060008060808587031215615a9f57600080fd5b8435615aaa816155f8565b93506020850135615aba816155f8565b93969395505050506040820135916060013590565b60008060008060808587031215615ae557600080fd5b8435615af0816155f8565b93506020850135615b008161560d565b92506040850135615b108161560d565b91506060850135615b20816155cd565b939692955090935050565b60208082526006908201526514185d5cd95960d21b604082015260600190565b634e487b7160e01b600052601160045260246000fd5b6000600160ff1b821415615b7757615b77615b4b565b5060000390565b600060208284031215615b9057600080fd5b5051919050565b600081600f0b6f7fffffffffffffffffffffffffffffff19811415615bbe57615bbe615b4b565b60000392915050565b60008219821115615bda57615bda615b4b565b500190565b6001600160801b03959095168552602085019390935260408401919091526060830152608082015260a00190565b6020808252602c908201527f46756e6374696f6e206d7573742062652063616c6c6564207468726f7567682060408201526b19195b1959d85d1958d85b1b60a21b606082015260800190565b6020808252602c908201527f46756e6374696f6e206d7573742062652063616c6c6564207468726f7567682060408201526b6163746976652070726f787960a01b606082015260800190565b600060208284031215615cb757600080fd5b8151610ce7816155f8565b600060208284031215615cd457600080fd5b8151610ce7816155cd565b600060608284031215615cf157600080fd5b6040516060810181811067ffffffffffffffff82111715615d1457615d14615684565b6040528251615d22816155f8565b81526020830151615d328161560d565b6020820152604083015160ff81168114615d4b57600080fd5b60408201529392505050565b600082821015615d6957615d69615b4b565b500390565b60008160020b627fffff19811415615bbe57615bbe615b4b565b600080600060608486031215615d9d57600080fd5b8351925060208401519150604084015190509250925092565b60008083128015600160ff1b850184121615615dd457615dd4615b4b565b6001600160ff1b0384018313811615615def57615def615b4b565b50500390565b60006001600160a01b0383811690831681811015615e1557615e15615b4b565b039392505050565b60006001600160a01b03828116848216808303821115615e3f57615e3f615b4b565b01949350505050565b600080600080600060a08688031215615e6057600080fd5b5050835160208501516040860151606087015160809097015192989197509594509092509050565b600080821280156001600160ff1b0384900385131615615eaa57615eaa615b4b565b600160ff1b8390038412811615615ec357615ec3615b4b565b50500190565b6020808252602b908201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960408201526a6e697469616c697a696e6760a81b606082015260800190565b634e487b7160e01b600052601260045260246000fd5b60006001600160801b0383811690831681811015615e1557615e15615b4b565b60006001600160801b03808316818516808303821115615e3f57615e3f615b4b565b602080825260049082015263453c3d5360e01b604082015260600190565b60005b83811015615fa5578181015183820152602001615f8d565b838111156127b75750506000910152565b60008251615fc8818460208701615f8a565b9190910192915050565b6000816000190483118215151615615fec57615fec615b4b565b500290565b60008261600057616000615f14565b500490565b60008261601457616014615f14565b500690565b6020815260008251806020840152616038816040850160208701615f8a565b601f01601f19169190910160400192915050565b600060001982141561606057616060615b4b565b506001019056fe4a0dd77e6cb2be1847de991681f679a59bbe3e047ecb337a8426980861f82c0f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc416464726573733a206c6f772d6c6576656c2064656c65676174652063616c6c206661696c6564a2646970667358221220b58bea741356d5182bf1d9e1164d2bcf75af8d90bfc8ca446c0a018aa9b0b96c64736f6c63430008090033" . parse () . expect ("invalid bytecode")
        });
    pub struct MarginEngine<M>(ethers::contract::Contract<M>);
    impl<M> Clone for MarginEngine<M> {
        fn clone(&self) -> Self {
            MarginEngine(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for MarginEngine<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for MarginEngine<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MarginEngine))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> MarginEngine<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), MARGINENGINE_ABI.clone(), client).into()
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
                MARGINENGINE_ABI.clone(),
                MARGINENGINE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `MAX_CACHE_MAX_AGE_IN_SECONDS` (0x86b127ee) function"]
        pub fn max_cache_max_age_in_seconds(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([134, 177, 39, 238], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAX_FIXED_RATE_WAD` (0xe6e306c9) function"]
        pub fn max_fixed_rate_wad(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([230, 227, 6, 201], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAX_LIQUIDATOR_REWARD_WAD` (0x87e16303) function"]
        pub fn max_liquidator_reward_wad(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([135, 225, 99, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAX_LOOKBACK_WINDOW_IN_SECONDS` (0xf907bd6d) function"]
        pub fn max_lookback_window_in_seconds(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([249, 7, 189, 109], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MIN_LOOKBACK_WINDOW_IN_SECONDS` (0x5f6a3e0c) function"]
        pub fn min_lookback_window_in_seconds(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([95, 106, 62, 12], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ONE` (0xc2ee3a08) function"]
        pub fn one(&self) -> ethers::contract::builders::ContractCall<M, I256> {
            self.0
                .method_hash([194, 238, 58, 8], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ONE_UINT` (0x63f57381) function"]
        pub fn one_uint(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([99, 245, 115, 129], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `SECONDS_IN_YEAR` (0x5dcc9391) function"]
        pub fn seconds_in_year(&self) -> ethers::contract::builders::ContractCall<M, I256> {
            self.0
                .method_hash([93, 204, 147, 145], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `cacheMaxAgeInSeconds` (0xb623f519) function"]
        pub fn cache_max_age_in_seconds(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([182, 35, 245, 25], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `collectProtocol` (0x754e2a8f) function"]
        pub fn collect_protocol(
            &self,
            recipient: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([117, 78, 42, 143], (recipient, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `factory` (0xc45a0155) function"]
        pub fn factory(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([196, 90, 1, 85], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fcm` (0x22d23b21) function"]
        pub fn fcm(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([34, 210, 59, 33], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getHistoricalApy` (0xe3f08374) function"]
        pub fn get_historical_apy(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([227, 240, 131, 116], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getHistoricalApyReadOnly` (0xd50d8811) function"]
        pub fn get_historical_apy_read_only(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([213, 13, 136, 17], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPosition` (0x9209e9ba) function"]
        pub fn get_position(
            &self,
            owner: ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
        ) -> ethers::contract::builders::ContractCall<M, Info> {
            self.0
                .method_hash([146, 9, 233, 186], (owner, tick_lower, tick_upper))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPositionMarginRequirement` (0xf1216105) function"]
        pub fn get_position_margin_requirement(
            &self,
            recipient: ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
            is_lm: bool,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([241, 33, 97, 5], (recipient, tick_lower, tick_upper, is_lm))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0xeb990c59) function"]
        pub fn initialize(
            &self,
            underlying_token: ethers::core::types::Address,
            rate_oracle: ethers::core::types::Address,
            term_start_timestamp_wad: ethers::core::types::U256,
            term_end_timestamp_wad: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [235, 153, 12, 89],
                    (
                        underlying_token,
                        rate_oracle,
                        term_start_timestamp_wad,
                        term_end_timestamp_wad,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isAlpha` (0x88428752) function"]
        pub fn is_alpha(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([136, 66, 135, 82], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liquidatePosition` (0x1656503e) function"]
        pub fn liquidate_position(
            &self,
            owner: ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([22, 86, 80, 62], (owner, tick_lower, tick_upper))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liquidatorRewardWad` (0xe087caf1) function"]
        pub fn liquidator_reward_wad(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([224, 135, 202, 241], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lookbackWindowInSeconds` (0x9cbff188) function"]
        pub fn lookback_window_in_seconds(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([156, 191, 241, 136], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `marginEngineParameters` (0x6938217f) function"]
        pub fn margin_engine_parameters(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, MarginCalculatorParameters> {
            self.0
                .method_hash([105, 56, 33, 127], ())
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
        #[doc = "Calls the contract's `proxiableUUID` (0x52d1902d) function"]
        pub fn proxiable_uuid(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([82, 209, 144, 45], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rateOracle` (0x98f4b1b2) function"]
        pub fn rate_oracle(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([152, 244, 177, 178], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setCacheMaxAgeInSeconds` (0xc3261892) function"]
        pub fn set_cache_max_age_in_seconds(
            &self,
            new_cache_max_age_in_seconds: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([195, 38, 24, 146], new_cache_max_age_in_seconds)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setFCM` (0x534d3375) function"]
        pub fn set_fcm(
            &self,
            new_fcm: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([83, 77, 51, 117], new_fcm)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setIsAlpha` (0xcd41b3d5) function"]
        pub fn set_is_alpha(
            &self,
            is_alpha: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([205, 65, 179, 213], is_alpha)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLiquidatorReward` (0xe9e441bb) function"]
        pub fn set_liquidator_reward(
            &self,
            new_liquidator_reward_wad: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([233, 228, 65, 187], new_liquidator_reward_wad)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLookbackWindowInSeconds` (0xa1ea6a20) function"]
        pub fn set_lookback_window_in_seconds(
            &self,
            new_seconds_ago: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([161, 234, 106, 32], new_seconds_ago)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setMarginCalculatorParameters` (0xb5c22d49) function"]
        pub fn set_margin_calculator_parameters(
            &self,
            margin_calculator_parameters: MarginCalculatorParameters,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([181, 194, 45, 73], (margin_calculator_parameters,))
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
        #[doc = "Calls the contract's `setRateOracle` (0xcf3c99bd) function"]
        pub fn set_rate_oracle(
            &self,
            rate_oracle: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([207, 60, 153, 189], rate_oracle)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setVAMM` (0xc7607a9c) function"]
        pub fn set_vamm(
            &self,
            v_amm: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([199, 96, 122, 156], v_amm)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `settlePosition` (0xa725b965) function"]
        pub fn settle_position(
            &self,
            owner: ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([167, 37, 185, 101], (owner, tick_lower, tick_upper))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `termEndTimestampWad` (0x93edb454) function"]
        pub fn term_end_timestamp_wad(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([147, 237, 180, 84], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `termStartTimestampWad` (0x652c30b7) function"]
        pub fn term_start_timestamp_wad(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([101, 44, 48, 183], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferMarginToFCMTrader` (0xefcfc3f9) function"]
        pub fn transfer_margin_to_fcm_trader(
            &self,
            account: ethers::core::types::Address,
            margin_delta: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([239, 207, 195, 249], (account, margin_delta))
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
        #[doc = "Calls the contract's `underlyingToken` (0x2495a599) function"]
        pub fn underlying_token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([36, 149, 165, 153], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updatePositionMargin` (0x7717797f) function"]
        pub fn update_position_margin(
            &self,
            owner: ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
            margin_delta: I256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [119, 23, 121, 127],
                    (owner, tick_lower, tick_upper, margin_delta),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updatePositionPostVAMMInducedMintBurn` (0xbfb5607d) function"]
        pub fn update_position_post_vamm_induced_mint_burn(
            &self,
            params: ModifyPositionParams,
        ) -> ethers::contract::builders::ContractCall<M, I256> {
            self.0
                .method_hash([191, 181, 96, 125], (params,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updatePositionPostVAMMInducedSwap` (0xc09617ae) function"]
        pub fn update_position_post_vamm_induced_swap(
            &self,
            owner: ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
            fixed_token_delta: I256,
            variable_token_delta: I256,
            cumulative_fee_incurred: ethers::core::types::U256,
            fixed_token_delta_unbalanced: I256,
        ) -> ethers::contract::builders::ContractCall<M, I256> {
            self.0
                .method_hash(
                    [192, 150, 23, 174],
                    (
                        owner,
                        tick_lower,
                        tick_upper,
                        fixed_token_delta,
                        variable_token_delta,
                        cumulative_fee_incurred,
                        fixed_token_delta_unbalanced,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `upgradeTo` (0x3659cfe6) function"]
        pub fn upgrade_to(
            &self,
            new_implementation: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 89, 207, 230], new_implementation)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `upgradeToAndCall` (0x4f1ef286) function"]
        pub fn upgrade_to_and_call(
            &self,
            new_implementation: ethers::core::types::Address,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([79, 30, 242, 134], (new_implementation, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `vamm` (0xe098372c) function"]
        pub fn vamm(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([224, 152, 55, 44], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `AdminChanged` event"]
        pub fn admin_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AdminChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `BeaconUpgraded` event"]
        pub fn beacon_upgraded_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, BeaconUpgradedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `CacheMaxAgeSetting` event"]
        pub fn cache_max_age_setting_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CacheMaxAgeSettingFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `FCMSetting` event"]
        pub fn fcm_setting_filter(&self) -> ethers::contract::builders::Event<M, FcmsettingFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `HistoricalApy` event"]
        pub fn historical_apy_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, HistoricalApyFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `HistoricalApyWindowSetting` event"]
        pub fn historical_apy_window_setting_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, HistoricalApyWindowSettingFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Initialized` event"]
        pub fn initialized_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, InitializedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `IsAlpha` event"]
        pub fn is_alpha_filter(&self) -> ethers::contract::builders::Event<M, IsAlphaFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LiquidatorRewardSetting` event"]
        pub fn liquidator_reward_setting_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LiquidatorRewardSettingFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `MarginCalculatorParametersSetting` event"]
        pub fn margin_calculator_parameters_setting_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, MarginCalculatorParametersSettingFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PositionLiquidation` event"]
        pub fn position_liquidation_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PositionLiquidationFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PositionMarginUpdate` event"]
        pub fn position_margin_update_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PositionMarginUpdateFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PositionSettlement` event"]
        pub fn position_settlement_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PositionSettlementFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PositionUpdate` event"]
        pub fn position_update_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PositionUpdateFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ProtocolCollection` event"]
        pub fn protocol_collection_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ProtocolCollectionFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RateOracle` event"]
        pub fn rate_oracle_filter(&self) -> ethers::contract::builders::Event<M, RateOracleFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RateOracleSetting` event"]
        pub fn rate_oracle_setting_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RateOracleSettingFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Upgraded` event"]
        pub fn upgraded_filter(&self) -> ethers::contract::builders::Event<M, UpgradedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `VAMMSetting` event"]
        pub fn vamm_setting_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, VammsettingFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, MarginEngineEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for MarginEngine<M> {
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
    #[doc = "Custom Error type `PRBMathSD59x18__DivInputTooSmall` with signature `PRBMathSD59x18__DivInputTooSmall()` and selector `[179, 199, 84, 163]`"]
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
        name = "PRBMathSD59x18__DivInputTooSmall",
        abi = "PRBMathSD59x18__DivInputTooSmall()"
    )]
    pub struct PRBMathSD59x18__DivInputTooSmall;
    #[doc = "Custom Error type `PRBMathSD59x18__DivOverflow` with signature `PRBMathSD59x18__DivOverflow(uint256)` and selector `[124, 180, 190, 245]`"]
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
        name = "PRBMathSD59x18__DivOverflow",
        abi = "PRBMathSD59x18__DivOverflow(uint256)"
    )]
    pub struct PRBMathSD59x18__DivOverflow {
        pub r_abs: ethers::core::types::U256,
    }
    #[doc = "Custom Error type `PRBMathSD59x18__Exp2InputTooBig` with signature `PRBMathSD59x18__Exp2InputTooBig(int256)` and selector `[230, 148, 88, 249]`"]
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
        name = "PRBMathSD59x18__Exp2InputTooBig",
        abi = "PRBMathSD59x18__Exp2InputTooBig(int256)"
    )]
    pub struct PRBMathSD59x18__Exp2InputTooBig {
        pub x: I256,
    }
    #[doc = "Custom Error type `PRBMathSD59x18__ExpInputTooBig` with signature `PRBMathSD59x18__ExpInputTooBig(int256)` and selector `[153, 187, 117, 65]`"]
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
        name = "PRBMathSD59x18__ExpInputTooBig",
        abi = "PRBMathSD59x18__ExpInputTooBig(int256)"
    )]
    pub struct PRBMathSD59x18__ExpInputTooBig {
        pub x: I256,
    }
    #[doc = "Custom Error type `PRBMathSD59x18__FromIntOverflow` with signature `PRBMathSD59x18__FromIntOverflow(int256)` and selector `[113, 247, 42, 49]`"]
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
        name = "PRBMathSD59x18__FromIntOverflow",
        abi = "PRBMathSD59x18__FromIntOverflow(int256)"
    )]
    pub struct PRBMathSD59x18__FromIntOverflow {
        pub x: I256,
    }
    #[doc = "Custom Error type `PRBMathSD59x18__FromIntUnderflow` with signature `PRBMathSD59x18__FromIntUnderflow(int256)` and selector `[230, 8, 225, 139]`"]
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
        name = "PRBMathSD59x18__FromIntUnderflow",
        abi = "PRBMathSD59x18__FromIntUnderflow(int256)"
    )]
    pub struct PRBMathSD59x18__FromIntUnderflow {
        pub x: I256,
    }
    #[doc = "Custom Error type `PRBMathSD59x18__MulInputTooSmall` with signature `PRBMathSD59x18__MulInputTooSmall()` and selector `[52, 6, 132, 108]`"]
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
        name = "PRBMathSD59x18__MulInputTooSmall",
        abi = "PRBMathSD59x18__MulInputTooSmall()"
    )]
    pub struct PRBMathSD59x18__MulInputTooSmall;
    #[doc = "Custom Error type `PRBMathSD59x18__MulOverflow` with signature `PRBMathSD59x18__MulOverflow(uint256)` and selector `[191, 121, 232, 217]`"]
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
        name = "PRBMathSD59x18__MulOverflow",
        abi = "PRBMathSD59x18__MulOverflow(uint256)"
    )]
    pub struct PRBMathSD59x18__MulOverflow {
        pub r_abs: ethers::core::types::U256,
    }
    #[doc = "Custom Error type `PRBMathSD59x18__SqrtNegativeInput` with signature `PRBMathSD59x18__SqrtNegativeInput(int256)` and selector `[193, 25, 7, 254]`"]
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
        name = "PRBMathSD59x18__SqrtNegativeInput",
        abi = "PRBMathSD59x18__SqrtNegativeInput(int256)"
    )]
    pub struct PRBMathSD59x18__SqrtNegativeInput {
        pub x: I256,
    }
    #[doc = "Custom Error type `PRBMathSD59x18__SqrtOverflow` with signature `PRBMathSD59x18__SqrtOverflow(int256)` and selector `[44, 72, 44, 57]`"]
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
        name = "PRBMathSD59x18__SqrtOverflow",
        abi = "PRBMathSD59x18__SqrtOverflow(int256)"
    )]
    pub struct PRBMathSD59x18__SqrtOverflow {
        pub x: I256,
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
    pub enum MarginEngineErrors {
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
        PRBMathSD59x18__DivInputTooSmall(PRBMathSD59x18__DivInputTooSmall),
        PRBMathSD59x18__DivOverflow(PRBMathSD59x18__DivOverflow),
        PRBMathSD59x18__Exp2InputTooBig(PRBMathSD59x18__Exp2InputTooBig),
        PRBMathSD59x18__ExpInputTooBig(PRBMathSD59x18__ExpInputTooBig),
        PRBMathSD59x18__FromIntOverflow(PRBMathSD59x18__FromIntOverflow),
        PRBMathSD59x18__FromIntUnderflow(PRBMathSD59x18__FromIntUnderflow),
        PRBMathSD59x18__MulInputTooSmall(PRBMathSD59x18__MulInputTooSmall),
        PRBMathSD59x18__MulOverflow(PRBMathSD59x18__MulOverflow),
        PRBMathSD59x18__SqrtNegativeInput(PRBMathSD59x18__SqrtNegativeInput),
        PRBMathSD59x18__SqrtOverflow(PRBMathSD59x18__SqrtOverflow),
        PRBMathUD60x18__FromUintOverflow(PRBMathUD60x18__FromUintOverflow),
        PRBMath__MulDivFixedPointOverflow(PRBMath__MulDivFixedPointOverflow),
        PRBMath__MulDivOverflow(PRBMath__MulDivOverflow),
        PositionNetZero(PositionNetZero),
        PositionNotSettled(PositionNotSettled),
        RocketPoolGetEthValueReturnedZero(RocketPoolGetEthValueReturnedZero),
        WithdrawalExceedsCurrentMargin(WithdrawalExceedsCurrentMargin),
        closeToOrBeyondMaturity(closeToOrBeyondMaturity),
    }
    impl ethers::core::abi::AbiDecode for MarginEngineErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok (decoded) = < AavePoolGetReserveNormalizedIncomeReturnedZero as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (MarginEngineErrors :: AavePoolGetReserveNormalizedIncomeReturnedZero (decoded)) }
            if let Ok (decoded) = < AavePoolGetReserveNormalizedVariableDebtReturnedZero as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (MarginEngineErrors :: AavePoolGetReserveNormalizedVariableDebtReturnedZero (decoded)) }
            if let Ok(decoded) =
                <CTokenExchangeRateReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MarginEngineErrors::CTokenExchangeRateReturnedZero(decoded));
            }
            if let Ok(decoded) =
                <CanOnlyTradeIfUnlocked as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineErrors::CanOnlyTradeIfUnlocked(decoded));
            }
            if let Ok(decoded) =
                <CannotLiquidate as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineErrors::CannotLiquidate(decoded));
            }
            if let Ok(decoded) =
                <CannotSettleBeforeMaturity as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineErrors::CannotSettleBeforeMaturity(decoded));
            }
            if let Ok(decoded) = <DebugError as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineErrors::DebugError(decoded));
            }
            if let Ok(decoded) =
                <ExpectedOppositeSigns as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineErrors::ExpectedOppositeSigns(decoded));
            }
            if let Ok(decoded) =
                <ExpectedSqrtPriceZeroBeforeInit as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MarginEngineErrors::ExpectedSqrtPriceZeroBeforeInit(decoded));
            }
            if let Ok(decoded) =
                <IRSNotionalAmountSpecifiedMustBeNonZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MarginEngineErrors::IRSNotionalAmountSpecifiedMustBeNonZero(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <InvalidMarginDelta as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineErrors::InvalidMarginDelta(decoded));
            }
            if let Ok(decoded) =
                <LidoGetPooledEthBySharesReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MarginEngineErrors::LidoGetPooledEthBySharesReturnedZero(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <LiquidityDeltaMustBePositiveInBurn as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MarginEngineErrors::LiquidityDeltaMustBePositiveInBurn(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <LiquidityDeltaMustBePositiveInMint as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MarginEngineErrors::LiquidityDeltaMustBePositiveInMint(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <MarginLessThanMinimum as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineErrors::MarginLessThanMinimum(decoded));
            }
            if let Ok(decoded) =
                <MarginRequirementNotMet as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineErrors::MarginRequirementNotMet(decoded));
            }
            if let Ok(decoded) =
                <MarginRequirementNotMetFCM as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineErrors::MarginRequirementNotMetFCM(decoded));
            }
            if let Ok(decoded) =
                <NotEnoughFunds as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineErrors::NotEnoughFunds(decoded));
            }
            if let Ok(decoded) = <OOO as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(MarginEngineErrors::OOO(decoded));
            }
            if let Ok(decoded) = <OnlyFCM as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(MarginEngineErrors::OnlyFCM(decoded));
            }
            if let Ok(decoded) =
                <OnlyMarginEngine as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineErrors::OnlyMarginEngine(decoded));
            }
            if let Ok(decoded) =
                <OnlyOwnerCanUpdatePosition as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineErrors::OnlyOwnerCanUpdatePosition(decoded));
            }
            if let Ok(decoded) = <OnlyVAMM as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(MarginEngineErrors::OnlyVAMM(decoded));
            }
            if let Ok(decoded) =
                <PRBMathSD59x18__DivInputTooSmall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MarginEngineErrors::PRBMathSD59x18__DivInputTooSmall(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <PRBMathSD59x18__DivOverflow as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineErrors::PRBMathSD59x18__DivOverflow(decoded));
            }
            if let Ok(decoded) =
                <PRBMathSD59x18__Exp2InputTooBig as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MarginEngineErrors::PRBMathSD59x18__Exp2InputTooBig(decoded));
            }
            if let Ok(decoded) =
                <PRBMathSD59x18__ExpInputTooBig as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MarginEngineErrors::PRBMathSD59x18__ExpInputTooBig(decoded));
            }
            if let Ok(decoded) =
                <PRBMathSD59x18__FromIntOverflow as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MarginEngineErrors::PRBMathSD59x18__FromIntOverflow(decoded));
            }
            if let Ok(decoded) =
                <PRBMathSD59x18__FromIntUnderflow as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MarginEngineErrors::PRBMathSD59x18__FromIntUnderflow(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <PRBMathSD59x18__MulInputTooSmall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MarginEngineErrors::PRBMathSD59x18__MulInputTooSmall(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <PRBMathSD59x18__MulOverflow as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineErrors::PRBMathSD59x18__MulOverflow(decoded));
            }
            if let Ok(decoded) =
                <PRBMathSD59x18__SqrtNegativeInput as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MarginEngineErrors::PRBMathSD59x18__SqrtNegativeInput(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <PRBMathSD59x18__SqrtOverflow as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MarginEngineErrors::PRBMathSD59x18__SqrtOverflow(decoded));
            }
            if let Ok(decoded) =
                <PRBMathUD60x18__FromUintOverflow as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MarginEngineErrors::PRBMathUD60x18__FromUintOverflow(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <PRBMath__MulDivFixedPointOverflow as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MarginEngineErrors::PRBMath__MulDivFixedPointOverflow(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <PRBMath__MulDivOverflow as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineErrors::PRBMath__MulDivOverflow(decoded));
            }
            if let Ok(decoded) =
                <PositionNetZero as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineErrors::PositionNetZero(decoded));
            }
            if let Ok(decoded) =
                <PositionNotSettled as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineErrors::PositionNotSettled(decoded));
            }
            if let Ok(decoded) =
                <RocketPoolGetEthValueReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MarginEngineErrors::RocketPoolGetEthValueReturnedZero(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <WithdrawalExceedsCurrentMargin as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MarginEngineErrors::WithdrawalExceedsCurrentMargin(decoded));
            }
            if let Ok(decoded) =
                <closeToOrBeyondMaturity as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineErrors::closeToOrBeyondMaturity(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MarginEngineErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                MarginEngineErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(element) => {
                    element.encode()
                }
                MarginEngineErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(
                    element,
                ) => element.encode(),
                MarginEngineErrors::CTokenExchangeRateReturnedZero(element) => element.encode(),
                MarginEngineErrors::CanOnlyTradeIfUnlocked(element) => element.encode(),
                MarginEngineErrors::CannotLiquidate(element) => element.encode(),
                MarginEngineErrors::CannotSettleBeforeMaturity(element) => element.encode(),
                MarginEngineErrors::DebugError(element) => element.encode(),
                MarginEngineErrors::ExpectedOppositeSigns(element) => element.encode(),
                MarginEngineErrors::ExpectedSqrtPriceZeroBeforeInit(element) => element.encode(),
                MarginEngineErrors::IRSNotionalAmountSpecifiedMustBeNonZero(element) => {
                    element.encode()
                }
                MarginEngineErrors::InvalidMarginDelta(element) => element.encode(),
                MarginEngineErrors::LidoGetPooledEthBySharesReturnedZero(element) => {
                    element.encode()
                }
                MarginEngineErrors::LiquidityDeltaMustBePositiveInBurn(element) => element.encode(),
                MarginEngineErrors::LiquidityDeltaMustBePositiveInMint(element) => element.encode(),
                MarginEngineErrors::MarginLessThanMinimum(element) => element.encode(),
                MarginEngineErrors::MarginRequirementNotMet(element) => element.encode(),
                MarginEngineErrors::MarginRequirementNotMetFCM(element) => element.encode(),
                MarginEngineErrors::NotEnoughFunds(element) => element.encode(),
                MarginEngineErrors::OOO(element) => element.encode(),
                MarginEngineErrors::OnlyFCM(element) => element.encode(),
                MarginEngineErrors::OnlyMarginEngine(element) => element.encode(),
                MarginEngineErrors::OnlyOwnerCanUpdatePosition(element) => element.encode(),
                MarginEngineErrors::OnlyVAMM(element) => element.encode(),
                MarginEngineErrors::PRBMathSD59x18__DivInputTooSmall(element) => element.encode(),
                MarginEngineErrors::PRBMathSD59x18__DivOverflow(element) => element.encode(),
                MarginEngineErrors::PRBMathSD59x18__Exp2InputTooBig(element) => element.encode(),
                MarginEngineErrors::PRBMathSD59x18__ExpInputTooBig(element) => element.encode(),
                MarginEngineErrors::PRBMathSD59x18__FromIntOverflow(element) => element.encode(),
                MarginEngineErrors::PRBMathSD59x18__FromIntUnderflow(element) => element.encode(),
                MarginEngineErrors::PRBMathSD59x18__MulInputTooSmall(element) => element.encode(),
                MarginEngineErrors::PRBMathSD59x18__MulOverflow(element) => element.encode(),
                MarginEngineErrors::PRBMathSD59x18__SqrtNegativeInput(element) => element.encode(),
                MarginEngineErrors::PRBMathSD59x18__SqrtOverflow(element) => element.encode(),
                MarginEngineErrors::PRBMathUD60x18__FromUintOverflow(element) => element.encode(),
                MarginEngineErrors::PRBMath__MulDivFixedPointOverflow(element) => element.encode(),
                MarginEngineErrors::PRBMath__MulDivOverflow(element) => element.encode(),
                MarginEngineErrors::PositionNetZero(element) => element.encode(),
                MarginEngineErrors::PositionNotSettled(element) => element.encode(),
                MarginEngineErrors::RocketPoolGetEthValueReturnedZero(element) => element.encode(),
                MarginEngineErrors::WithdrawalExceedsCurrentMargin(element) => element.encode(),
                MarginEngineErrors::closeToOrBeyondMaturity(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MarginEngineErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MarginEngineErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(element) => {
                    element.fmt(f)
                }
                MarginEngineErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(
                    element,
                ) => element.fmt(f),
                MarginEngineErrors::CTokenExchangeRateReturnedZero(element) => element.fmt(f),
                MarginEngineErrors::CanOnlyTradeIfUnlocked(element) => element.fmt(f),
                MarginEngineErrors::CannotLiquidate(element) => element.fmt(f),
                MarginEngineErrors::CannotSettleBeforeMaturity(element) => element.fmt(f),
                MarginEngineErrors::DebugError(element) => element.fmt(f),
                MarginEngineErrors::ExpectedOppositeSigns(element) => element.fmt(f),
                MarginEngineErrors::ExpectedSqrtPriceZeroBeforeInit(element) => element.fmt(f),
                MarginEngineErrors::IRSNotionalAmountSpecifiedMustBeNonZero(element) => {
                    element.fmt(f)
                }
                MarginEngineErrors::InvalidMarginDelta(element) => element.fmt(f),
                MarginEngineErrors::LidoGetPooledEthBySharesReturnedZero(element) => element.fmt(f),
                MarginEngineErrors::LiquidityDeltaMustBePositiveInBurn(element) => element.fmt(f),
                MarginEngineErrors::LiquidityDeltaMustBePositiveInMint(element) => element.fmt(f),
                MarginEngineErrors::MarginLessThanMinimum(element) => element.fmt(f),
                MarginEngineErrors::MarginRequirementNotMet(element) => element.fmt(f),
                MarginEngineErrors::MarginRequirementNotMetFCM(element) => element.fmt(f),
                MarginEngineErrors::NotEnoughFunds(element) => element.fmt(f),
                MarginEngineErrors::OOO(element) => element.fmt(f),
                MarginEngineErrors::OnlyFCM(element) => element.fmt(f),
                MarginEngineErrors::OnlyMarginEngine(element) => element.fmt(f),
                MarginEngineErrors::OnlyOwnerCanUpdatePosition(element) => element.fmt(f),
                MarginEngineErrors::OnlyVAMM(element) => element.fmt(f),
                MarginEngineErrors::PRBMathSD59x18__DivInputTooSmall(element) => element.fmt(f),
                MarginEngineErrors::PRBMathSD59x18__DivOverflow(element) => element.fmt(f),
                MarginEngineErrors::PRBMathSD59x18__Exp2InputTooBig(element) => element.fmt(f),
                MarginEngineErrors::PRBMathSD59x18__ExpInputTooBig(element) => element.fmt(f),
                MarginEngineErrors::PRBMathSD59x18__FromIntOverflow(element) => element.fmt(f),
                MarginEngineErrors::PRBMathSD59x18__FromIntUnderflow(element) => element.fmt(f),
                MarginEngineErrors::PRBMathSD59x18__MulInputTooSmall(element) => element.fmt(f),
                MarginEngineErrors::PRBMathSD59x18__MulOverflow(element) => element.fmt(f),
                MarginEngineErrors::PRBMathSD59x18__SqrtNegativeInput(element) => element.fmt(f),
                MarginEngineErrors::PRBMathSD59x18__SqrtOverflow(element) => element.fmt(f),
                MarginEngineErrors::PRBMathUD60x18__FromUintOverflow(element) => element.fmt(f),
                MarginEngineErrors::PRBMath__MulDivFixedPointOverflow(element) => element.fmt(f),
                MarginEngineErrors::PRBMath__MulDivOverflow(element) => element.fmt(f),
                MarginEngineErrors::PositionNetZero(element) => element.fmt(f),
                MarginEngineErrors::PositionNotSettled(element) => element.fmt(f),
                MarginEngineErrors::RocketPoolGetEthValueReturnedZero(element) => element.fmt(f),
                MarginEngineErrors::WithdrawalExceedsCurrentMargin(element) => element.fmt(f),
                MarginEngineErrors::closeToOrBeyondMaturity(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AavePoolGetReserveNormalizedIncomeReturnedZero> for MarginEngineErrors {
        fn from(var: AavePoolGetReserveNormalizedIncomeReturnedZero) -> Self {
            MarginEngineErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(var)
        }
    }
    impl ::std::convert::From<AavePoolGetReserveNormalizedVariableDebtReturnedZero>
        for MarginEngineErrors
    {
        fn from(var: AavePoolGetReserveNormalizedVariableDebtReturnedZero) -> Self {
            MarginEngineErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(var)
        }
    }
    impl ::std::convert::From<CTokenExchangeRateReturnedZero> for MarginEngineErrors {
        fn from(var: CTokenExchangeRateReturnedZero) -> Self {
            MarginEngineErrors::CTokenExchangeRateReturnedZero(var)
        }
    }
    impl ::std::convert::From<CanOnlyTradeIfUnlocked> for MarginEngineErrors {
        fn from(var: CanOnlyTradeIfUnlocked) -> Self {
            MarginEngineErrors::CanOnlyTradeIfUnlocked(var)
        }
    }
    impl ::std::convert::From<CannotLiquidate> for MarginEngineErrors {
        fn from(var: CannotLiquidate) -> Self {
            MarginEngineErrors::CannotLiquidate(var)
        }
    }
    impl ::std::convert::From<CannotSettleBeforeMaturity> for MarginEngineErrors {
        fn from(var: CannotSettleBeforeMaturity) -> Self {
            MarginEngineErrors::CannotSettleBeforeMaturity(var)
        }
    }
    impl ::std::convert::From<DebugError> for MarginEngineErrors {
        fn from(var: DebugError) -> Self {
            MarginEngineErrors::DebugError(var)
        }
    }
    impl ::std::convert::From<ExpectedOppositeSigns> for MarginEngineErrors {
        fn from(var: ExpectedOppositeSigns) -> Self {
            MarginEngineErrors::ExpectedOppositeSigns(var)
        }
    }
    impl ::std::convert::From<ExpectedSqrtPriceZeroBeforeInit> for MarginEngineErrors {
        fn from(var: ExpectedSqrtPriceZeroBeforeInit) -> Self {
            MarginEngineErrors::ExpectedSqrtPriceZeroBeforeInit(var)
        }
    }
    impl ::std::convert::From<IRSNotionalAmountSpecifiedMustBeNonZero> for MarginEngineErrors {
        fn from(var: IRSNotionalAmountSpecifiedMustBeNonZero) -> Self {
            MarginEngineErrors::IRSNotionalAmountSpecifiedMustBeNonZero(var)
        }
    }
    impl ::std::convert::From<InvalidMarginDelta> for MarginEngineErrors {
        fn from(var: InvalidMarginDelta) -> Self {
            MarginEngineErrors::InvalidMarginDelta(var)
        }
    }
    impl ::std::convert::From<LidoGetPooledEthBySharesReturnedZero> for MarginEngineErrors {
        fn from(var: LidoGetPooledEthBySharesReturnedZero) -> Self {
            MarginEngineErrors::LidoGetPooledEthBySharesReturnedZero(var)
        }
    }
    impl ::std::convert::From<LiquidityDeltaMustBePositiveInBurn> for MarginEngineErrors {
        fn from(var: LiquidityDeltaMustBePositiveInBurn) -> Self {
            MarginEngineErrors::LiquidityDeltaMustBePositiveInBurn(var)
        }
    }
    impl ::std::convert::From<LiquidityDeltaMustBePositiveInMint> for MarginEngineErrors {
        fn from(var: LiquidityDeltaMustBePositiveInMint) -> Self {
            MarginEngineErrors::LiquidityDeltaMustBePositiveInMint(var)
        }
    }
    impl ::std::convert::From<MarginLessThanMinimum> for MarginEngineErrors {
        fn from(var: MarginLessThanMinimum) -> Self {
            MarginEngineErrors::MarginLessThanMinimum(var)
        }
    }
    impl ::std::convert::From<MarginRequirementNotMet> for MarginEngineErrors {
        fn from(var: MarginRequirementNotMet) -> Self {
            MarginEngineErrors::MarginRequirementNotMet(var)
        }
    }
    impl ::std::convert::From<MarginRequirementNotMetFCM> for MarginEngineErrors {
        fn from(var: MarginRequirementNotMetFCM) -> Self {
            MarginEngineErrors::MarginRequirementNotMetFCM(var)
        }
    }
    impl ::std::convert::From<NotEnoughFunds> for MarginEngineErrors {
        fn from(var: NotEnoughFunds) -> Self {
            MarginEngineErrors::NotEnoughFunds(var)
        }
    }
    impl ::std::convert::From<OOO> for MarginEngineErrors {
        fn from(var: OOO) -> Self {
            MarginEngineErrors::OOO(var)
        }
    }
    impl ::std::convert::From<OnlyFCM> for MarginEngineErrors {
        fn from(var: OnlyFCM) -> Self {
            MarginEngineErrors::OnlyFCM(var)
        }
    }
    impl ::std::convert::From<OnlyMarginEngine> for MarginEngineErrors {
        fn from(var: OnlyMarginEngine) -> Self {
            MarginEngineErrors::OnlyMarginEngine(var)
        }
    }
    impl ::std::convert::From<OnlyOwnerCanUpdatePosition> for MarginEngineErrors {
        fn from(var: OnlyOwnerCanUpdatePosition) -> Self {
            MarginEngineErrors::OnlyOwnerCanUpdatePosition(var)
        }
    }
    impl ::std::convert::From<OnlyVAMM> for MarginEngineErrors {
        fn from(var: OnlyVAMM) -> Self {
            MarginEngineErrors::OnlyVAMM(var)
        }
    }
    impl ::std::convert::From<PRBMathSD59x18__DivInputTooSmall> for MarginEngineErrors {
        fn from(var: PRBMathSD59x18__DivInputTooSmall) -> Self {
            MarginEngineErrors::PRBMathSD59x18__DivInputTooSmall(var)
        }
    }
    impl ::std::convert::From<PRBMathSD59x18__DivOverflow> for MarginEngineErrors {
        fn from(var: PRBMathSD59x18__DivOverflow) -> Self {
            MarginEngineErrors::PRBMathSD59x18__DivOverflow(var)
        }
    }
    impl ::std::convert::From<PRBMathSD59x18__Exp2InputTooBig> for MarginEngineErrors {
        fn from(var: PRBMathSD59x18__Exp2InputTooBig) -> Self {
            MarginEngineErrors::PRBMathSD59x18__Exp2InputTooBig(var)
        }
    }
    impl ::std::convert::From<PRBMathSD59x18__ExpInputTooBig> for MarginEngineErrors {
        fn from(var: PRBMathSD59x18__ExpInputTooBig) -> Self {
            MarginEngineErrors::PRBMathSD59x18__ExpInputTooBig(var)
        }
    }
    impl ::std::convert::From<PRBMathSD59x18__FromIntOverflow> for MarginEngineErrors {
        fn from(var: PRBMathSD59x18__FromIntOverflow) -> Self {
            MarginEngineErrors::PRBMathSD59x18__FromIntOverflow(var)
        }
    }
    impl ::std::convert::From<PRBMathSD59x18__FromIntUnderflow> for MarginEngineErrors {
        fn from(var: PRBMathSD59x18__FromIntUnderflow) -> Self {
            MarginEngineErrors::PRBMathSD59x18__FromIntUnderflow(var)
        }
    }
    impl ::std::convert::From<PRBMathSD59x18__MulInputTooSmall> for MarginEngineErrors {
        fn from(var: PRBMathSD59x18__MulInputTooSmall) -> Self {
            MarginEngineErrors::PRBMathSD59x18__MulInputTooSmall(var)
        }
    }
    impl ::std::convert::From<PRBMathSD59x18__MulOverflow> for MarginEngineErrors {
        fn from(var: PRBMathSD59x18__MulOverflow) -> Self {
            MarginEngineErrors::PRBMathSD59x18__MulOverflow(var)
        }
    }
    impl ::std::convert::From<PRBMathSD59x18__SqrtNegativeInput> for MarginEngineErrors {
        fn from(var: PRBMathSD59x18__SqrtNegativeInput) -> Self {
            MarginEngineErrors::PRBMathSD59x18__SqrtNegativeInput(var)
        }
    }
    impl ::std::convert::From<PRBMathSD59x18__SqrtOverflow> for MarginEngineErrors {
        fn from(var: PRBMathSD59x18__SqrtOverflow) -> Self {
            MarginEngineErrors::PRBMathSD59x18__SqrtOverflow(var)
        }
    }
    impl ::std::convert::From<PRBMathUD60x18__FromUintOverflow> for MarginEngineErrors {
        fn from(var: PRBMathUD60x18__FromUintOverflow) -> Self {
            MarginEngineErrors::PRBMathUD60x18__FromUintOverflow(var)
        }
    }
    impl ::std::convert::From<PRBMath__MulDivFixedPointOverflow> for MarginEngineErrors {
        fn from(var: PRBMath__MulDivFixedPointOverflow) -> Self {
            MarginEngineErrors::PRBMath__MulDivFixedPointOverflow(var)
        }
    }
    impl ::std::convert::From<PRBMath__MulDivOverflow> for MarginEngineErrors {
        fn from(var: PRBMath__MulDivOverflow) -> Self {
            MarginEngineErrors::PRBMath__MulDivOverflow(var)
        }
    }
    impl ::std::convert::From<PositionNetZero> for MarginEngineErrors {
        fn from(var: PositionNetZero) -> Self {
            MarginEngineErrors::PositionNetZero(var)
        }
    }
    impl ::std::convert::From<PositionNotSettled> for MarginEngineErrors {
        fn from(var: PositionNotSettled) -> Self {
            MarginEngineErrors::PositionNotSettled(var)
        }
    }
    impl ::std::convert::From<RocketPoolGetEthValueReturnedZero> for MarginEngineErrors {
        fn from(var: RocketPoolGetEthValueReturnedZero) -> Self {
            MarginEngineErrors::RocketPoolGetEthValueReturnedZero(var)
        }
    }
    impl ::std::convert::From<WithdrawalExceedsCurrentMargin> for MarginEngineErrors {
        fn from(var: WithdrawalExceedsCurrentMargin) -> Self {
            MarginEngineErrors::WithdrawalExceedsCurrentMargin(var)
        }
    }
    impl ::std::convert::From<closeToOrBeyondMaturity> for MarginEngineErrors {
        fn from(var: closeToOrBeyondMaturity) -> Self {
            MarginEngineErrors::closeToOrBeyondMaturity(var)
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
    #[ethevent(name = "AdminChanged", abi = "AdminChanged(address,address)")]
    pub struct AdminChangedFilter {
        pub previous_admin: ethers::core::types::Address,
        pub new_admin: ethers::core::types::Address,
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
    #[ethevent(name = "BeaconUpgraded", abi = "BeaconUpgraded(address)")]
    pub struct BeaconUpgradedFilter {
        #[ethevent(indexed)]
        pub beacon: ethers::core::types::Address,
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
    #[ethevent(name = "CacheMaxAgeSetting", abi = "CacheMaxAgeSetting(uint256)")]
    pub struct CacheMaxAgeSettingFilter {
        pub cache_max_age_in_seconds: ethers::core::types::U256,
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
    #[ethevent(name = "FCMSetting", abi = "FCMSetting(address)")]
    pub struct FcmsettingFilter {
        #[ethevent(indexed)]
        pub fcm: ethers::core::types::Address,
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
    #[ethevent(name = "HistoricalApy", abi = "HistoricalApy(uint256)")]
    pub struct HistoricalApyFilter {
        pub value: ethers::core::types::U256,
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
        name = "HistoricalApyWindowSetting",
        abi = "HistoricalApyWindowSetting(uint256)"
    )]
    pub struct HistoricalApyWindowSettingFilter {
        pub seconds_ago: ethers::core::types::U256,
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
    #[ethevent(name = "IsAlpha", abi = "IsAlpha(bool)")]
    pub struct IsAlphaFilter {
        pub is_alpha: bool,
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
        name = "LiquidatorRewardSetting",
        abi = "LiquidatorRewardSetting(uint256)"
    )]
    pub struct LiquidatorRewardSettingFilter {
        pub liquidator_reward_wad: ethers::core::types::U256,
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
        name = "MarginCalculatorParametersSetting",
        abi = "MarginCalculatorParametersSetting((uint256,uint256,int256,int256,int256,int256,int256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256))"
    )]
    pub struct MarginCalculatorParametersSettingFilter {
        pub margin_calculator_parameters: MarginCalculatorParameters,
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
    #[ethevent(
        name = "PositionLiquidation",
        abi = "PositionLiquidation(address,int24,int24,address,int256,uint256)"
    )]
    pub struct PositionLiquidationFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub tick_lower: i32,
        #[ethevent(indexed)]
        pub tick_upper: i32,
        pub liquidator: ethers::core::types::Address,
        pub notional_unwound: I256,
        pub liquidator_reward: ethers::core::types::U256,
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
        name = "PositionMarginUpdate",
        abi = "PositionMarginUpdate(address,address,int24,int24,int256)"
    )]
    pub struct PositionMarginUpdateFilter {
        pub sender: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub tick_lower: i32,
        #[ethevent(indexed)]
        pub tick_upper: i32,
        pub margin_delta: I256,
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
        name = "PositionSettlement",
        abi = "PositionSettlement(address,int24,int24,int256)"
    )]
    pub struct PositionSettlementFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub tick_lower: i32,
        #[ethevent(indexed)]
        pub tick_upper: i32,
        pub settlement_cashflow: I256,
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
        name = "PositionUpdate",
        abi = "PositionUpdate(address,int24,int24,uint128,int256,int256,int256,uint256)"
    )]
    pub struct PositionUpdateFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub tick_lower: i32,
        #[ethevent(indexed)]
        pub tick_upper: i32,
        pub liquidity: u128,
        pub margin: I256,
        pub fixed_token_balance: I256,
        pub variable_token_balance: I256,
        pub accumulated_fees: ethers::core::types::U256,
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
        name = "ProtocolCollection",
        abi = "ProtocolCollection(address,address,uint256)"
    )]
    pub struct ProtocolCollectionFilter {
        pub sender: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub recipient: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
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
    #[ethevent(name = "RateOracle", abi = "RateOracle(uint256)")]
    pub struct RateOracleFilter {
        pub cache_max_age_in_seconds: ethers::core::types::U256,
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
    #[ethevent(name = "RateOracleSetting", abi = "RateOracleSetting(address)")]
    pub struct RateOracleSettingFilter {
        #[ethevent(indexed)]
        pub rate_oracle: ethers::core::types::Address,
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
    #[ethevent(name = "Upgraded", abi = "Upgraded(address)")]
    pub struct UpgradedFilter {
        #[ethevent(indexed)]
        pub implementation: ethers::core::types::Address,
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
    #[ethevent(name = "VAMMSetting", abi = "VAMMSetting(address)")]
    pub struct VammsettingFilter {
        #[ethevent(indexed)]
        pub vamm: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MarginEngineEvents {
        AdminChangedFilter(AdminChangedFilter),
        BeaconUpgradedFilter(BeaconUpgradedFilter),
        CacheMaxAgeSettingFilter(CacheMaxAgeSettingFilter),
        FcmsettingFilter(FcmsettingFilter),
        HistoricalApyFilter(HistoricalApyFilter),
        HistoricalApyWindowSettingFilter(HistoricalApyWindowSettingFilter),
        InitializedFilter(InitializedFilter),
        IsAlphaFilter(IsAlphaFilter),
        LiquidatorRewardSettingFilter(LiquidatorRewardSettingFilter),
        MarginCalculatorParametersSettingFilter(MarginCalculatorParametersSettingFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PositionLiquidationFilter(PositionLiquidationFilter),
        PositionMarginUpdateFilter(PositionMarginUpdateFilter),
        PositionSettlementFilter(PositionSettlementFilter),
        PositionUpdateFilter(PositionUpdateFilter),
        ProtocolCollectionFilter(ProtocolCollectionFilter),
        RateOracleFilter(RateOracleFilter),
        RateOracleSettingFilter(RateOracleSettingFilter),
        UpgradedFilter(UpgradedFilter),
        VammsettingFilter(VammsettingFilter),
    }
    impl ethers::contract::EthLogDecode for MarginEngineEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AdminChangedFilter::decode_log(log) {
                return Ok(MarginEngineEvents::AdminChangedFilter(decoded));
            }
            if let Ok(decoded) = BeaconUpgradedFilter::decode_log(log) {
                return Ok(MarginEngineEvents::BeaconUpgradedFilter(decoded));
            }
            if let Ok(decoded) = CacheMaxAgeSettingFilter::decode_log(log) {
                return Ok(MarginEngineEvents::CacheMaxAgeSettingFilter(decoded));
            }
            if let Ok(decoded) = FcmsettingFilter::decode_log(log) {
                return Ok(MarginEngineEvents::FcmsettingFilter(decoded));
            }
            if let Ok(decoded) = HistoricalApyFilter::decode_log(log) {
                return Ok(MarginEngineEvents::HistoricalApyFilter(decoded));
            }
            if let Ok(decoded) = HistoricalApyWindowSettingFilter::decode_log(log) {
                return Ok(MarginEngineEvents::HistoricalApyWindowSettingFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(MarginEngineEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = IsAlphaFilter::decode_log(log) {
                return Ok(MarginEngineEvents::IsAlphaFilter(decoded));
            }
            if let Ok(decoded) = LiquidatorRewardSettingFilter::decode_log(log) {
                return Ok(MarginEngineEvents::LiquidatorRewardSettingFilter(decoded));
            }
            if let Ok(decoded) = MarginCalculatorParametersSettingFilter::decode_log(log) {
                return Ok(MarginEngineEvents::MarginCalculatorParametersSettingFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(MarginEngineEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PositionLiquidationFilter::decode_log(log) {
                return Ok(MarginEngineEvents::PositionLiquidationFilter(decoded));
            }
            if let Ok(decoded) = PositionMarginUpdateFilter::decode_log(log) {
                return Ok(MarginEngineEvents::PositionMarginUpdateFilter(decoded));
            }
            if let Ok(decoded) = PositionSettlementFilter::decode_log(log) {
                return Ok(MarginEngineEvents::PositionSettlementFilter(decoded));
            }
            if let Ok(decoded) = PositionUpdateFilter::decode_log(log) {
                return Ok(MarginEngineEvents::PositionUpdateFilter(decoded));
            }
            if let Ok(decoded) = ProtocolCollectionFilter::decode_log(log) {
                return Ok(MarginEngineEvents::ProtocolCollectionFilter(decoded));
            }
            if let Ok(decoded) = RateOracleFilter::decode_log(log) {
                return Ok(MarginEngineEvents::RateOracleFilter(decoded));
            }
            if let Ok(decoded) = RateOracleSettingFilter::decode_log(log) {
                return Ok(MarginEngineEvents::RateOracleSettingFilter(decoded));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(MarginEngineEvents::UpgradedFilter(decoded));
            }
            if let Ok(decoded) = VammsettingFilter::decode_log(log) {
                return Ok(MarginEngineEvents::VammsettingFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for MarginEngineEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MarginEngineEvents::AdminChangedFilter(element) => element.fmt(f),
                MarginEngineEvents::BeaconUpgradedFilter(element) => element.fmt(f),
                MarginEngineEvents::CacheMaxAgeSettingFilter(element) => element.fmt(f),
                MarginEngineEvents::FcmsettingFilter(element) => element.fmt(f),
                MarginEngineEvents::HistoricalApyFilter(element) => element.fmt(f),
                MarginEngineEvents::HistoricalApyWindowSettingFilter(element) => element.fmt(f),
                MarginEngineEvents::InitializedFilter(element) => element.fmt(f),
                MarginEngineEvents::IsAlphaFilter(element) => element.fmt(f),
                MarginEngineEvents::LiquidatorRewardSettingFilter(element) => element.fmt(f),
                MarginEngineEvents::MarginCalculatorParametersSettingFilter(element) => {
                    element.fmt(f)
                }
                MarginEngineEvents::OwnershipTransferredFilter(element) => element.fmt(f),
                MarginEngineEvents::PositionLiquidationFilter(element) => element.fmt(f),
                MarginEngineEvents::PositionMarginUpdateFilter(element) => element.fmt(f),
                MarginEngineEvents::PositionSettlementFilter(element) => element.fmt(f),
                MarginEngineEvents::PositionUpdateFilter(element) => element.fmt(f),
                MarginEngineEvents::ProtocolCollectionFilter(element) => element.fmt(f),
                MarginEngineEvents::RateOracleFilter(element) => element.fmt(f),
                MarginEngineEvents::RateOracleSettingFilter(element) => element.fmt(f),
                MarginEngineEvents::UpgradedFilter(element) => element.fmt(f),
                MarginEngineEvents::VammsettingFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `MAX_CACHE_MAX_AGE_IN_SECONDS` function with signature `MAX_CACHE_MAX_AGE_IN_SECONDS()` and selector `[134, 177, 39, 238]`"]
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
        name = "MAX_CACHE_MAX_AGE_IN_SECONDS",
        abi = "MAX_CACHE_MAX_AGE_IN_SECONDS()"
    )]
    pub struct MaxCacheMaxAgeInSecondsCall;
    #[doc = "Container type for all input parameters for the `MAX_FIXED_RATE_WAD` function with signature `MAX_FIXED_RATE_WAD()` and selector `[230, 227, 6, 201]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "MAX_FIXED_RATE_WAD", abi = "MAX_FIXED_RATE_WAD()")]
    pub struct MaxFixedRateWadCall;
    #[doc = "Container type for all input parameters for the `MAX_LIQUIDATOR_REWARD_WAD` function with signature `MAX_LIQUIDATOR_REWARD_WAD()` and selector `[135, 225, 99, 3]`"]
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
        name = "MAX_LIQUIDATOR_REWARD_WAD",
        abi = "MAX_LIQUIDATOR_REWARD_WAD()"
    )]
    pub struct MaxLiquidatorRewardWadCall;
    #[doc = "Container type for all input parameters for the `MAX_LOOKBACK_WINDOW_IN_SECONDS` function with signature `MAX_LOOKBACK_WINDOW_IN_SECONDS()` and selector `[249, 7, 189, 109]`"]
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
        name = "MAX_LOOKBACK_WINDOW_IN_SECONDS",
        abi = "MAX_LOOKBACK_WINDOW_IN_SECONDS()"
    )]
    pub struct MaxLookbackWindowInSecondsCall;
    #[doc = "Container type for all input parameters for the `MIN_LOOKBACK_WINDOW_IN_SECONDS` function with signature `MIN_LOOKBACK_WINDOW_IN_SECONDS()` and selector `[95, 106, 62, 12]`"]
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
        name = "MIN_LOOKBACK_WINDOW_IN_SECONDS",
        abi = "MIN_LOOKBACK_WINDOW_IN_SECONDS()"
    )]
    pub struct MinLookbackWindowInSecondsCall;
    #[doc = "Container type for all input parameters for the `ONE` function with signature `ONE()` and selector `[194, 238, 58, 8]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "ONE", abi = "ONE()")]
    pub struct OneCall;
    #[doc = "Container type for all input parameters for the `ONE_UINT` function with signature `ONE_UINT()` and selector `[99, 245, 115, 129]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "ONE_UINT", abi = "ONE_UINT()")]
    pub struct OneUintCall;
    #[doc = "Container type for all input parameters for the `SECONDS_IN_YEAR` function with signature `SECONDS_IN_YEAR()` and selector `[93, 204, 147, 145]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "SECONDS_IN_YEAR", abi = "SECONDS_IN_YEAR()")]
    pub struct SecondsInYearCall;
    #[doc = "Container type for all input parameters for the `cacheMaxAgeInSeconds` function with signature `cacheMaxAgeInSeconds()` and selector `[182, 35, 245, 25]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "cacheMaxAgeInSeconds", abi = "cacheMaxAgeInSeconds()")]
    pub struct CacheMaxAgeInSecondsCall;
    #[doc = "Container type for all input parameters for the `collectProtocol` function with signature `collectProtocol(address,uint256)` and selector `[117, 78, 42, 143]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "collectProtocol", abi = "collectProtocol(address,uint256)")]
    pub struct CollectProtocolCall {
        pub recipient: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `factory` function with signature `factory()` and selector `[196, 90, 1, 85]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "factory", abi = "factory()")]
    pub struct FactoryCall;
    #[doc = "Container type for all input parameters for the `fcm` function with signature `fcm()` and selector `[34, 210, 59, 33]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "fcm", abi = "fcm()")]
    pub struct FcmCall;
    #[doc = "Container type for all input parameters for the `getHistoricalApy` function with signature `getHistoricalApy()` and selector `[227, 240, 131, 116]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getHistoricalApy", abi = "getHistoricalApy()")]
    pub struct GetHistoricalApyCall;
    #[doc = "Container type for all input parameters for the `getHistoricalApyReadOnly` function with signature `getHistoricalApyReadOnly()` and selector `[213, 13, 136, 17]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getHistoricalApyReadOnly", abi = "getHistoricalApyReadOnly()")]
    pub struct GetHistoricalApyReadOnlyCall;
    #[doc = "Container type for all input parameters for the `getPosition` function with signature `getPosition(address,int24,int24)` and selector `[146, 9, 233, 186]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getPosition", abi = "getPosition(address,int24,int24)")]
    pub struct GetPositionCall {
        pub owner: ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
    }
    #[doc = "Container type for all input parameters for the `getPositionMarginRequirement` function with signature `getPositionMarginRequirement(address,int24,int24,bool)` and selector `[241, 33, 97, 5]`"]
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
        name = "getPositionMarginRequirement",
        abi = "getPositionMarginRequirement(address,int24,int24,bool)"
    )]
    pub struct GetPositionMarginRequirementCall {
        pub recipient: ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub is_lm: bool,
    }
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize(address,address,uint256,uint256)` and selector `[235, 153, 12, 89]`"]
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
        name = "initialize",
        abi = "initialize(address,address,uint256,uint256)"
    )]
    pub struct InitializeCall {
        pub underlying_token: ethers::core::types::Address,
        pub rate_oracle: ethers::core::types::Address,
        pub term_start_timestamp_wad: ethers::core::types::U256,
        pub term_end_timestamp_wad: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `isAlpha` function with signature `isAlpha()` and selector `[136, 66, 135, 82]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "isAlpha", abi = "isAlpha()")]
    pub struct IsAlphaCall;
    #[doc = "Container type for all input parameters for the `liquidatePosition` function with signature `liquidatePosition(address,int24,int24)` and selector `[22, 86, 80, 62]`"]
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
        name = "liquidatePosition",
        abi = "liquidatePosition(address,int24,int24)"
    )]
    pub struct LiquidatePositionCall {
        pub owner: ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
    }
    #[doc = "Container type for all input parameters for the `liquidatorRewardWad` function with signature `liquidatorRewardWad()` and selector `[224, 135, 202, 241]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "liquidatorRewardWad", abi = "liquidatorRewardWad()")]
    pub struct LiquidatorRewardWadCall;
    #[doc = "Container type for all input parameters for the `lookbackWindowInSeconds` function with signature `lookbackWindowInSeconds()` and selector `[156, 191, 241, 136]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "lookbackWindowInSeconds", abi = "lookbackWindowInSeconds()")]
    pub struct LookbackWindowInSecondsCall;
    #[doc = "Container type for all input parameters for the `marginEngineParameters` function with signature `marginEngineParameters()` and selector `[105, 56, 33, 127]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "marginEngineParameters", abi = "marginEngineParameters()")]
    pub struct MarginEngineParametersCall;
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
    #[doc = "Container type for all input parameters for the `proxiableUUID` function with signature `proxiableUUID()` and selector `[82, 209, 144, 45]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "proxiableUUID", abi = "proxiableUUID()")]
    pub struct ProxiableUUIDCall;
    #[doc = "Container type for all input parameters for the `rateOracle` function with signature `rateOracle()` and selector `[152, 244, 177, 178]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "rateOracle", abi = "rateOracle()")]
    pub struct RateOracleCall;
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
    #[doc = "Container type for all input parameters for the `setCacheMaxAgeInSeconds` function with signature `setCacheMaxAgeInSeconds(uint256)` and selector `[195, 38, 24, 146]`"]
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
        name = "setCacheMaxAgeInSeconds",
        abi = "setCacheMaxAgeInSeconds(uint256)"
    )]
    pub struct SetCacheMaxAgeInSecondsCall {
        pub new_cache_max_age_in_seconds: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setFCM` function with signature `setFCM(address)` and selector `[83, 77, 51, 117]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setFCM", abi = "setFCM(address)")]
    pub struct SetFCMCall {
        pub new_fcm: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setIsAlpha` function with signature `setIsAlpha(bool)` and selector `[205, 65, 179, 213]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setIsAlpha", abi = "setIsAlpha(bool)")]
    pub struct SetIsAlphaCall {
        pub is_alpha: bool,
    }
    #[doc = "Container type for all input parameters for the `setLiquidatorReward` function with signature `setLiquidatorReward(uint256)` and selector `[233, 228, 65, 187]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setLiquidatorReward", abi = "setLiquidatorReward(uint256)")]
    pub struct SetLiquidatorRewardCall {
        pub new_liquidator_reward_wad: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setLookbackWindowInSeconds` function with signature `setLookbackWindowInSeconds(uint256)` and selector `[161, 234, 106, 32]`"]
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
        name = "setLookbackWindowInSeconds",
        abi = "setLookbackWindowInSeconds(uint256)"
    )]
    pub struct SetLookbackWindowInSecondsCall {
        pub new_seconds_ago: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setMarginCalculatorParameters` function with signature `setMarginCalculatorParameters((uint256,uint256,int256,int256,int256,int256,int256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256))` and selector `[181, 194, 45, 73]`"]
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
        name = "setMarginCalculatorParameters",
        abi = "setMarginCalculatorParameters((uint256,uint256,int256,int256,int256,int256,int256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256))"
    )]
    pub struct SetMarginCalculatorParametersCall {
        pub margin_calculator_parameters: MarginCalculatorParameters,
    }
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
    #[doc = "Container type for all input parameters for the `setRateOracle` function with signature `setRateOracle(address)` and selector `[207, 60, 153, 189]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setRateOracle", abi = "setRateOracle(address)")]
    pub struct SetRateOracleCall {
        pub rate_oracle: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setVAMM` function with signature `setVAMM(address)` and selector `[199, 96, 122, 156]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setVAMM", abi = "setVAMM(address)")]
    pub struct SetVAMMCall {
        pub v_amm: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `settlePosition` function with signature `settlePosition(address,int24,int24)` and selector `[167, 37, 185, 101]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "settlePosition", abi = "settlePosition(address,int24,int24)")]
    pub struct SettlePositionCall {
        pub owner: ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
    }
    #[doc = "Container type for all input parameters for the `termEndTimestampWad` function with signature `termEndTimestampWad()` and selector `[147, 237, 180, 84]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "termEndTimestampWad", abi = "termEndTimestampWad()")]
    pub struct TermEndTimestampWadCall;
    #[doc = "Container type for all input parameters for the `termStartTimestampWad` function with signature `termStartTimestampWad()` and selector `[101, 44, 48, 183]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "termStartTimestampWad", abi = "termStartTimestampWad()")]
    pub struct TermStartTimestampWadCall;
    #[doc = "Container type for all input parameters for the `transferMarginToFCMTrader` function with signature `transferMarginToFCMTrader(address,uint256)` and selector `[239, 207, 195, 249]`"]
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
        name = "transferMarginToFCMTrader",
        abi = "transferMarginToFCMTrader(address,uint256)"
    )]
    pub struct TransferMarginToFCMTraderCall {
        pub account: ethers::core::types::Address,
        pub margin_delta: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `underlyingToken` function with signature `underlyingToken()` and selector `[36, 149, 165, 153]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "underlyingToken", abi = "underlyingToken()")]
    pub struct UnderlyingTokenCall;
    #[doc = "Container type for all input parameters for the `updatePositionMargin` function with signature `updatePositionMargin(address,int24,int24,int256)` and selector `[119, 23, 121, 127]`"]
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
        name = "updatePositionMargin",
        abi = "updatePositionMargin(address,int24,int24,int256)"
    )]
    pub struct UpdatePositionMarginCall {
        pub owner: ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub margin_delta: I256,
    }
    #[doc = "Container type for all input parameters for the `updatePositionPostVAMMInducedMintBurn` function with signature `updatePositionPostVAMMInducedMintBurn((address,int24,int24,int128))` and selector `[191, 181, 96, 125]`"]
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
        name = "updatePositionPostVAMMInducedMintBurn",
        abi = "updatePositionPostVAMMInducedMintBurn((address,int24,int24,int128))"
    )]
    pub struct UpdatePositionPostVAMMInducedMintBurnCall {
        pub params: ModifyPositionParams,
    }
    #[doc = "Container type for all input parameters for the `updatePositionPostVAMMInducedSwap` function with signature `updatePositionPostVAMMInducedSwap(address,int24,int24,int256,int256,uint256,int256)` and selector `[192, 150, 23, 174]`"]
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
        name = "updatePositionPostVAMMInducedSwap",
        abi = "updatePositionPostVAMMInducedSwap(address,int24,int24,int256,int256,uint256,int256)"
    )]
    pub struct UpdatePositionPostVAMMInducedSwapCall {
        pub owner: ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub fixed_token_delta: I256,
        pub variable_token_delta: I256,
        pub cumulative_fee_incurred: ethers::core::types::U256,
        pub fixed_token_delta_unbalanced: I256,
    }
    #[doc = "Container type for all input parameters for the `upgradeTo` function with signature `upgradeTo(address)` and selector `[54, 89, 207, 230]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "upgradeTo", abi = "upgradeTo(address)")]
    pub struct UpgradeToCall {
        pub new_implementation: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `upgradeToAndCall` function with signature `upgradeToAndCall(address,bytes)` and selector `[79, 30, 242, 134]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "upgradeToAndCall", abi = "upgradeToAndCall(address,bytes)")]
    pub struct UpgradeToAndCallCall {
        pub new_implementation: ethers::core::types::Address,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `vamm` function with signature `vamm()` and selector `[224, 152, 55, 44]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "vamm", abi = "vamm()")]
    pub struct VammCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MarginEngineCalls {
        MaxCacheMaxAgeInSeconds(MaxCacheMaxAgeInSecondsCall),
        MaxFixedRateWad(MaxFixedRateWadCall),
        MaxLiquidatorRewardWad(MaxLiquidatorRewardWadCall),
        MaxLookbackWindowInSeconds(MaxLookbackWindowInSecondsCall),
        MinLookbackWindowInSeconds(MinLookbackWindowInSecondsCall),
        One(OneCall),
        OneUint(OneUintCall),
        SecondsInYear(SecondsInYearCall),
        CacheMaxAgeInSeconds(CacheMaxAgeInSecondsCall),
        CollectProtocol(CollectProtocolCall),
        Factory(FactoryCall),
        Fcm(FcmCall),
        GetHistoricalApy(GetHistoricalApyCall),
        GetHistoricalApyReadOnly(GetHistoricalApyReadOnlyCall),
        GetPosition(GetPositionCall),
        GetPositionMarginRequirement(GetPositionMarginRequirementCall),
        Initialize(InitializeCall),
        IsAlpha(IsAlphaCall),
        LiquidatePosition(LiquidatePositionCall),
        LiquidatorRewardWad(LiquidatorRewardWadCall),
        LookbackWindowInSeconds(LookbackWindowInSecondsCall),
        MarginEngineParameters(MarginEngineParametersCall),
        Owner(OwnerCall),
        Paused(PausedCall),
        ProxiableUUID(ProxiableUUIDCall),
        RateOracle(RateOracleCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetCacheMaxAgeInSeconds(SetCacheMaxAgeInSecondsCall),
        SetFCM(SetFCMCall),
        SetIsAlpha(SetIsAlphaCall),
        SetLiquidatorReward(SetLiquidatorRewardCall),
        SetLookbackWindowInSeconds(SetLookbackWindowInSecondsCall),
        SetMarginCalculatorParameters(SetMarginCalculatorParametersCall),
        SetPausability(SetPausabilityCall),
        SetRateOracle(SetRateOracleCall),
        SetVAMM(SetVAMMCall),
        SettlePosition(SettlePositionCall),
        TermEndTimestampWad(TermEndTimestampWadCall),
        TermStartTimestampWad(TermStartTimestampWadCall),
        TransferMarginToFCMTrader(TransferMarginToFCMTraderCall),
        TransferOwnership(TransferOwnershipCall),
        UnderlyingToken(UnderlyingTokenCall),
        UpdatePositionMargin(UpdatePositionMarginCall),
        UpdatePositionPostVAMMInducedMintBurn(UpdatePositionPostVAMMInducedMintBurnCall),
        UpdatePositionPostVAMMInducedSwap(UpdatePositionPostVAMMInducedSwapCall),
        UpgradeTo(UpgradeToCall),
        UpgradeToAndCall(UpgradeToAndCallCall),
        Vamm(VammCall),
    }
    impl ethers::core::abi::AbiDecode for MarginEngineCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <MaxCacheMaxAgeInSecondsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineCalls::MaxCacheMaxAgeInSeconds(decoded));
            }
            if let Ok(decoded) =
                <MaxFixedRateWadCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineCalls::MaxFixedRateWad(decoded));
            }
            if let Ok(decoded) =
                <MaxLiquidatorRewardWadCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineCalls::MaxLiquidatorRewardWad(decoded));
            }
            if let Ok(decoded) =
                <MaxLookbackWindowInSecondsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MarginEngineCalls::MaxLookbackWindowInSeconds(decoded));
            }
            if let Ok(decoded) =
                <MinLookbackWindowInSecondsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MarginEngineCalls::MinLookbackWindowInSeconds(decoded));
            }
            if let Ok(decoded) = <OneCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(MarginEngineCalls::One(decoded));
            }
            if let Ok(decoded) =
                <OneUintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineCalls::OneUint(decoded));
            }
            if let Ok(decoded) =
                <SecondsInYearCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineCalls::SecondsInYear(decoded));
            }
            if let Ok(decoded) =
                <CacheMaxAgeInSecondsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineCalls::CacheMaxAgeInSeconds(decoded));
            }
            if let Ok(decoded) =
                <CollectProtocolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineCalls::CollectProtocol(decoded));
            }
            if let Ok(decoded) =
                <FactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineCalls::Factory(decoded));
            }
            if let Ok(decoded) = <FcmCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(MarginEngineCalls::Fcm(decoded));
            }
            if let Ok(decoded) =
                <GetHistoricalApyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineCalls::GetHistoricalApy(decoded));
            }
            if let Ok(decoded) =
                <GetHistoricalApyReadOnlyCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MarginEngineCalls::GetHistoricalApyReadOnly(decoded));
            }
            if let Ok(decoded) =
                <GetPositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineCalls::GetPosition(decoded));
            }
            if let Ok(decoded) =
                <GetPositionMarginRequirementCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MarginEngineCalls::GetPositionMarginRequirement(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineCalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <IsAlphaCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineCalls::IsAlpha(decoded));
            }
            if let Ok(decoded) =
                <LiquidatePositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineCalls::LiquidatePosition(decoded));
            }
            if let Ok(decoded) =
                <LiquidatorRewardWadCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineCalls::LiquidatorRewardWad(decoded));
            }
            if let Ok(decoded) =
                <LookbackWindowInSecondsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineCalls::LookbackWindowInSeconds(decoded));
            }
            if let Ok(decoded) =
                <MarginEngineParametersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineCalls::MarginEngineParameters(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineCalls::Owner(decoded));
            }
            if let Ok(decoded) = <PausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineCalls::Paused(decoded));
            }
            if let Ok(decoded) =
                <ProxiableUUIDCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineCalls::ProxiableUUID(decoded));
            }
            if let Ok(decoded) =
                <RateOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineCalls::RateOracle(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <SetCacheMaxAgeInSecondsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineCalls::SetCacheMaxAgeInSeconds(decoded));
            }
            if let Ok(decoded) = <SetFCMCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineCalls::SetFCM(decoded));
            }
            if let Ok(decoded) =
                <SetIsAlphaCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineCalls::SetIsAlpha(decoded));
            }
            if let Ok(decoded) =
                <SetLiquidatorRewardCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineCalls::SetLiquidatorReward(decoded));
            }
            if let Ok(decoded) =
                <SetLookbackWindowInSecondsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MarginEngineCalls::SetLookbackWindowInSeconds(decoded));
            }
            if let Ok(decoded) =
                <SetMarginCalculatorParametersCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MarginEngineCalls::SetMarginCalculatorParameters(decoded));
            }
            if let Ok(decoded) =
                <SetPausabilityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineCalls::SetPausability(decoded));
            }
            if let Ok(decoded) =
                <SetRateOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineCalls::SetRateOracle(decoded));
            }
            if let Ok(decoded) =
                <SetVAMMCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineCalls::SetVAMM(decoded));
            }
            if let Ok(decoded) =
                <SettlePositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineCalls::SettlePosition(decoded));
            }
            if let Ok(decoded) =
                <TermEndTimestampWadCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineCalls::TermEndTimestampWad(decoded));
            }
            if let Ok(decoded) =
                <TermStartTimestampWadCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineCalls::TermStartTimestampWad(decoded));
            }
            if let Ok(decoded) =
                <TransferMarginToFCMTraderCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MarginEngineCalls::TransferMarginToFCMTrader(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <UnderlyingTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineCalls::UnderlyingToken(decoded));
            }
            if let Ok(decoded) =
                <UpdatePositionMarginCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineCalls::UpdatePositionMargin(decoded));
            }
            if let Ok(decoded) =
                <UpdatePositionPostVAMMInducedMintBurnCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MarginEngineCalls::UpdatePositionPostVAMMInducedMintBurn(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <UpdatePositionPostVAMMInducedSwapCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MarginEngineCalls::UpdatePositionPostVAMMInducedSwap(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <UpgradeToCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineCalls::UpgradeTo(decoded));
            }
            if let Ok(decoded) =
                <UpgradeToAndCallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineCalls::UpgradeToAndCall(decoded));
            }
            if let Ok(decoded) = <VammCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(MarginEngineCalls::Vamm(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MarginEngineCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MarginEngineCalls::MaxCacheMaxAgeInSeconds(element) => element.encode(),
                MarginEngineCalls::MaxFixedRateWad(element) => element.encode(),
                MarginEngineCalls::MaxLiquidatorRewardWad(element) => element.encode(),
                MarginEngineCalls::MaxLookbackWindowInSeconds(element) => element.encode(),
                MarginEngineCalls::MinLookbackWindowInSeconds(element) => element.encode(),
                MarginEngineCalls::One(element) => element.encode(),
                MarginEngineCalls::OneUint(element) => element.encode(),
                MarginEngineCalls::SecondsInYear(element) => element.encode(),
                MarginEngineCalls::CacheMaxAgeInSeconds(element) => element.encode(),
                MarginEngineCalls::CollectProtocol(element) => element.encode(),
                MarginEngineCalls::Factory(element) => element.encode(),
                MarginEngineCalls::Fcm(element) => element.encode(),
                MarginEngineCalls::GetHistoricalApy(element) => element.encode(),
                MarginEngineCalls::GetHistoricalApyReadOnly(element) => element.encode(),
                MarginEngineCalls::GetPosition(element) => element.encode(),
                MarginEngineCalls::GetPositionMarginRequirement(element) => element.encode(),
                MarginEngineCalls::Initialize(element) => element.encode(),
                MarginEngineCalls::IsAlpha(element) => element.encode(),
                MarginEngineCalls::LiquidatePosition(element) => element.encode(),
                MarginEngineCalls::LiquidatorRewardWad(element) => element.encode(),
                MarginEngineCalls::LookbackWindowInSeconds(element) => element.encode(),
                MarginEngineCalls::MarginEngineParameters(element) => element.encode(),
                MarginEngineCalls::Owner(element) => element.encode(),
                MarginEngineCalls::Paused(element) => element.encode(),
                MarginEngineCalls::ProxiableUUID(element) => element.encode(),
                MarginEngineCalls::RateOracle(element) => element.encode(),
                MarginEngineCalls::RenounceOwnership(element) => element.encode(),
                MarginEngineCalls::SetCacheMaxAgeInSeconds(element) => element.encode(),
                MarginEngineCalls::SetFCM(element) => element.encode(),
                MarginEngineCalls::SetIsAlpha(element) => element.encode(),
                MarginEngineCalls::SetLiquidatorReward(element) => element.encode(),
                MarginEngineCalls::SetLookbackWindowInSeconds(element) => element.encode(),
                MarginEngineCalls::SetMarginCalculatorParameters(element) => element.encode(),
                MarginEngineCalls::SetPausability(element) => element.encode(),
                MarginEngineCalls::SetRateOracle(element) => element.encode(),
                MarginEngineCalls::SetVAMM(element) => element.encode(),
                MarginEngineCalls::SettlePosition(element) => element.encode(),
                MarginEngineCalls::TermEndTimestampWad(element) => element.encode(),
                MarginEngineCalls::TermStartTimestampWad(element) => element.encode(),
                MarginEngineCalls::TransferMarginToFCMTrader(element) => element.encode(),
                MarginEngineCalls::TransferOwnership(element) => element.encode(),
                MarginEngineCalls::UnderlyingToken(element) => element.encode(),
                MarginEngineCalls::UpdatePositionMargin(element) => element.encode(),
                MarginEngineCalls::UpdatePositionPostVAMMInducedMintBurn(element) => {
                    element.encode()
                }
                MarginEngineCalls::UpdatePositionPostVAMMInducedSwap(element) => element.encode(),
                MarginEngineCalls::UpgradeTo(element) => element.encode(),
                MarginEngineCalls::UpgradeToAndCall(element) => element.encode(),
                MarginEngineCalls::Vamm(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MarginEngineCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MarginEngineCalls::MaxCacheMaxAgeInSeconds(element) => element.fmt(f),
                MarginEngineCalls::MaxFixedRateWad(element) => element.fmt(f),
                MarginEngineCalls::MaxLiquidatorRewardWad(element) => element.fmt(f),
                MarginEngineCalls::MaxLookbackWindowInSeconds(element) => element.fmt(f),
                MarginEngineCalls::MinLookbackWindowInSeconds(element) => element.fmt(f),
                MarginEngineCalls::One(element) => element.fmt(f),
                MarginEngineCalls::OneUint(element) => element.fmt(f),
                MarginEngineCalls::SecondsInYear(element) => element.fmt(f),
                MarginEngineCalls::CacheMaxAgeInSeconds(element) => element.fmt(f),
                MarginEngineCalls::CollectProtocol(element) => element.fmt(f),
                MarginEngineCalls::Factory(element) => element.fmt(f),
                MarginEngineCalls::Fcm(element) => element.fmt(f),
                MarginEngineCalls::GetHistoricalApy(element) => element.fmt(f),
                MarginEngineCalls::GetHistoricalApyReadOnly(element) => element.fmt(f),
                MarginEngineCalls::GetPosition(element) => element.fmt(f),
                MarginEngineCalls::GetPositionMarginRequirement(element) => element.fmt(f),
                MarginEngineCalls::Initialize(element) => element.fmt(f),
                MarginEngineCalls::IsAlpha(element) => element.fmt(f),
                MarginEngineCalls::LiquidatePosition(element) => element.fmt(f),
                MarginEngineCalls::LiquidatorRewardWad(element) => element.fmt(f),
                MarginEngineCalls::LookbackWindowInSeconds(element) => element.fmt(f),
                MarginEngineCalls::MarginEngineParameters(element) => element.fmt(f),
                MarginEngineCalls::Owner(element) => element.fmt(f),
                MarginEngineCalls::Paused(element) => element.fmt(f),
                MarginEngineCalls::ProxiableUUID(element) => element.fmt(f),
                MarginEngineCalls::RateOracle(element) => element.fmt(f),
                MarginEngineCalls::RenounceOwnership(element) => element.fmt(f),
                MarginEngineCalls::SetCacheMaxAgeInSeconds(element) => element.fmt(f),
                MarginEngineCalls::SetFCM(element) => element.fmt(f),
                MarginEngineCalls::SetIsAlpha(element) => element.fmt(f),
                MarginEngineCalls::SetLiquidatorReward(element) => element.fmt(f),
                MarginEngineCalls::SetLookbackWindowInSeconds(element) => element.fmt(f),
                MarginEngineCalls::SetMarginCalculatorParameters(element) => element.fmt(f),
                MarginEngineCalls::SetPausability(element) => element.fmt(f),
                MarginEngineCalls::SetRateOracle(element) => element.fmt(f),
                MarginEngineCalls::SetVAMM(element) => element.fmt(f),
                MarginEngineCalls::SettlePosition(element) => element.fmt(f),
                MarginEngineCalls::TermEndTimestampWad(element) => element.fmt(f),
                MarginEngineCalls::TermStartTimestampWad(element) => element.fmt(f),
                MarginEngineCalls::TransferMarginToFCMTrader(element) => element.fmt(f),
                MarginEngineCalls::TransferOwnership(element) => element.fmt(f),
                MarginEngineCalls::UnderlyingToken(element) => element.fmt(f),
                MarginEngineCalls::UpdatePositionMargin(element) => element.fmt(f),
                MarginEngineCalls::UpdatePositionPostVAMMInducedMintBurn(element) => element.fmt(f),
                MarginEngineCalls::UpdatePositionPostVAMMInducedSwap(element) => element.fmt(f),
                MarginEngineCalls::UpgradeTo(element) => element.fmt(f),
                MarginEngineCalls::UpgradeToAndCall(element) => element.fmt(f),
                MarginEngineCalls::Vamm(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<MaxCacheMaxAgeInSecondsCall> for MarginEngineCalls {
        fn from(var: MaxCacheMaxAgeInSecondsCall) -> Self {
            MarginEngineCalls::MaxCacheMaxAgeInSeconds(var)
        }
    }
    impl ::std::convert::From<MaxFixedRateWadCall> for MarginEngineCalls {
        fn from(var: MaxFixedRateWadCall) -> Self {
            MarginEngineCalls::MaxFixedRateWad(var)
        }
    }
    impl ::std::convert::From<MaxLiquidatorRewardWadCall> for MarginEngineCalls {
        fn from(var: MaxLiquidatorRewardWadCall) -> Self {
            MarginEngineCalls::MaxLiquidatorRewardWad(var)
        }
    }
    impl ::std::convert::From<MaxLookbackWindowInSecondsCall> for MarginEngineCalls {
        fn from(var: MaxLookbackWindowInSecondsCall) -> Self {
            MarginEngineCalls::MaxLookbackWindowInSeconds(var)
        }
    }
    impl ::std::convert::From<MinLookbackWindowInSecondsCall> for MarginEngineCalls {
        fn from(var: MinLookbackWindowInSecondsCall) -> Self {
            MarginEngineCalls::MinLookbackWindowInSeconds(var)
        }
    }
    impl ::std::convert::From<OneCall> for MarginEngineCalls {
        fn from(var: OneCall) -> Self {
            MarginEngineCalls::One(var)
        }
    }
    impl ::std::convert::From<OneUintCall> for MarginEngineCalls {
        fn from(var: OneUintCall) -> Self {
            MarginEngineCalls::OneUint(var)
        }
    }
    impl ::std::convert::From<SecondsInYearCall> for MarginEngineCalls {
        fn from(var: SecondsInYearCall) -> Self {
            MarginEngineCalls::SecondsInYear(var)
        }
    }
    impl ::std::convert::From<CacheMaxAgeInSecondsCall> for MarginEngineCalls {
        fn from(var: CacheMaxAgeInSecondsCall) -> Self {
            MarginEngineCalls::CacheMaxAgeInSeconds(var)
        }
    }
    impl ::std::convert::From<CollectProtocolCall> for MarginEngineCalls {
        fn from(var: CollectProtocolCall) -> Self {
            MarginEngineCalls::CollectProtocol(var)
        }
    }
    impl ::std::convert::From<FactoryCall> for MarginEngineCalls {
        fn from(var: FactoryCall) -> Self {
            MarginEngineCalls::Factory(var)
        }
    }
    impl ::std::convert::From<FcmCall> for MarginEngineCalls {
        fn from(var: FcmCall) -> Self {
            MarginEngineCalls::Fcm(var)
        }
    }
    impl ::std::convert::From<GetHistoricalApyCall> for MarginEngineCalls {
        fn from(var: GetHistoricalApyCall) -> Self {
            MarginEngineCalls::GetHistoricalApy(var)
        }
    }
    impl ::std::convert::From<GetHistoricalApyReadOnlyCall> for MarginEngineCalls {
        fn from(var: GetHistoricalApyReadOnlyCall) -> Self {
            MarginEngineCalls::GetHistoricalApyReadOnly(var)
        }
    }
    impl ::std::convert::From<GetPositionCall> for MarginEngineCalls {
        fn from(var: GetPositionCall) -> Self {
            MarginEngineCalls::GetPosition(var)
        }
    }
    impl ::std::convert::From<GetPositionMarginRequirementCall> for MarginEngineCalls {
        fn from(var: GetPositionMarginRequirementCall) -> Self {
            MarginEngineCalls::GetPositionMarginRequirement(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for MarginEngineCalls {
        fn from(var: InitializeCall) -> Self {
            MarginEngineCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<IsAlphaCall> for MarginEngineCalls {
        fn from(var: IsAlphaCall) -> Self {
            MarginEngineCalls::IsAlpha(var)
        }
    }
    impl ::std::convert::From<LiquidatePositionCall> for MarginEngineCalls {
        fn from(var: LiquidatePositionCall) -> Self {
            MarginEngineCalls::LiquidatePosition(var)
        }
    }
    impl ::std::convert::From<LiquidatorRewardWadCall> for MarginEngineCalls {
        fn from(var: LiquidatorRewardWadCall) -> Self {
            MarginEngineCalls::LiquidatorRewardWad(var)
        }
    }
    impl ::std::convert::From<LookbackWindowInSecondsCall> for MarginEngineCalls {
        fn from(var: LookbackWindowInSecondsCall) -> Self {
            MarginEngineCalls::LookbackWindowInSeconds(var)
        }
    }
    impl ::std::convert::From<MarginEngineParametersCall> for MarginEngineCalls {
        fn from(var: MarginEngineParametersCall) -> Self {
            MarginEngineCalls::MarginEngineParameters(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for MarginEngineCalls {
        fn from(var: OwnerCall) -> Self {
            MarginEngineCalls::Owner(var)
        }
    }
    impl ::std::convert::From<PausedCall> for MarginEngineCalls {
        fn from(var: PausedCall) -> Self {
            MarginEngineCalls::Paused(var)
        }
    }
    impl ::std::convert::From<ProxiableUUIDCall> for MarginEngineCalls {
        fn from(var: ProxiableUUIDCall) -> Self {
            MarginEngineCalls::ProxiableUUID(var)
        }
    }
    impl ::std::convert::From<RateOracleCall> for MarginEngineCalls {
        fn from(var: RateOracleCall) -> Self {
            MarginEngineCalls::RateOracle(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for MarginEngineCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            MarginEngineCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<SetCacheMaxAgeInSecondsCall> for MarginEngineCalls {
        fn from(var: SetCacheMaxAgeInSecondsCall) -> Self {
            MarginEngineCalls::SetCacheMaxAgeInSeconds(var)
        }
    }
    impl ::std::convert::From<SetFCMCall> for MarginEngineCalls {
        fn from(var: SetFCMCall) -> Self {
            MarginEngineCalls::SetFCM(var)
        }
    }
    impl ::std::convert::From<SetIsAlphaCall> for MarginEngineCalls {
        fn from(var: SetIsAlphaCall) -> Self {
            MarginEngineCalls::SetIsAlpha(var)
        }
    }
    impl ::std::convert::From<SetLiquidatorRewardCall> for MarginEngineCalls {
        fn from(var: SetLiquidatorRewardCall) -> Self {
            MarginEngineCalls::SetLiquidatorReward(var)
        }
    }
    impl ::std::convert::From<SetLookbackWindowInSecondsCall> for MarginEngineCalls {
        fn from(var: SetLookbackWindowInSecondsCall) -> Self {
            MarginEngineCalls::SetLookbackWindowInSeconds(var)
        }
    }
    impl ::std::convert::From<SetMarginCalculatorParametersCall> for MarginEngineCalls {
        fn from(var: SetMarginCalculatorParametersCall) -> Self {
            MarginEngineCalls::SetMarginCalculatorParameters(var)
        }
    }
    impl ::std::convert::From<SetPausabilityCall> for MarginEngineCalls {
        fn from(var: SetPausabilityCall) -> Self {
            MarginEngineCalls::SetPausability(var)
        }
    }
    impl ::std::convert::From<SetRateOracleCall> for MarginEngineCalls {
        fn from(var: SetRateOracleCall) -> Self {
            MarginEngineCalls::SetRateOracle(var)
        }
    }
    impl ::std::convert::From<SetVAMMCall> for MarginEngineCalls {
        fn from(var: SetVAMMCall) -> Self {
            MarginEngineCalls::SetVAMM(var)
        }
    }
    impl ::std::convert::From<SettlePositionCall> for MarginEngineCalls {
        fn from(var: SettlePositionCall) -> Self {
            MarginEngineCalls::SettlePosition(var)
        }
    }
    impl ::std::convert::From<TermEndTimestampWadCall> for MarginEngineCalls {
        fn from(var: TermEndTimestampWadCall) -> Self {
            MarginEngineCalls::TermEndTimestampWad(var)
        }
    }
    impl ::std::convert::From<TermStartTimestampWadCall> for MarginEngineCalls {
        fn from(var: TermStartTimestampWadCall) -> Self {
            MarginEngineCalls::TermStartTimestampWad(var)
        }
    }
    impl ::std::convert::From<TransferMarginToFCMTraderCall> for MarginEngineCalls {
        fn from(var: TransferMarginToFCMTraderCall) -> Self {
            MarginEngineCalls::TransferMarginToFCMTrader(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for MarginEngineCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            MarginEngineCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<UnderlyingTokenCall> for MarginEngineCalls {
        fn from(var: UnderlyingTokenCall) -> Self {
            MarginEngineCalls::UnderlyingToken(var)
        }
    }
    impl ::std::convert::From<UpdatePositionMarginCall> for MarginEngineCalls {
        fn from(var: UpdatePositionMarginCall) -> Self {
            MarginEngineCalls::UpdatePositionMargin(var)
        }
    }
    impl ::std::convert::From<UpdatePositionPostVAMMInducedMintBurnCall> for MarginEngineCalls {
        fn from(var: UpdatePositionPostVAMMInducedMintBurnCall) -> Self {
            MarginEngineCalls::UpdatePositionPostVAMMInducedMintBurn(var)
        }
    }
    impl ::std::convert::From<UpdatePositionPostVAMMInducedSwapCall> for MarginEngineCalls {
        fn from(var: UpdatePositionPostVAMMInducedSwapCall) -> Self {
            MarginEngineCalls::UpdatePositionPostVAMMInducedSwap(var)
        }
    }
    impl ::std::convert::From<UpgradeToCall> for MarginEngineCalls {
        fn from(var: UpgradeToCall) -> Self {
            MarginEngineCalls::UpgradeTo(var)
        }
    }
    impl ::std::convert::From<UpgradeToAndCallCall> for MarginEngineCalls {
        fn from(var: UpgradeToAndCallCall) -> Self {
            MarginEngineCalls::UpgradeToAndCall(var)
        }
    }
    impl ::std::convert::From<VammCall> for MarginEngineCalls {
        fn from(var: VammCall) -> Self {
            MarginEngineCalls::Vamm(var)
        }
    }
    #[doc = "Container type for all return fields from the `MAX_CACHE_MAX_AGE_IN_SECONDS` function with signature `MAX_CACHE_MAX_AGE_IN_SECONDS()` and selector `[134, 177, 39, 238]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MaxCacheMaxAgeInSecondsReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `MAX_FIXED_RATE_WAD` function with signature `MAX_FIXED_RATE_WAD()` and selector `[230, 227, 6, 201]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MaxFixedRateWadReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `MAX_LIQUIDATOR_REWARD_WAD` function with signature `MAX_LIQUIDATOR_REWARD_WAD()` and selector `[135, 225, 99, 3]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MaxLiquidatorRewardWadReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `MAX_LOOKBACK_WINDOW_IN_SECONDS` function with signature `MAX_LOOKBACK_WINDOW_IN_SECONDS()` and selector `[249, 7, 189, 109]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MaxLookbackWindowInSecondsReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `MIN_LOOKBACK_WINDOW_IN_SECONDS` function with signature `MIN_LOOKBACK_WINDOW_IN_SECONDS()` and selector `[95, 106, 62, 12]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MinLookbackWindowInSecondsReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `ONE` function with signature `ONE()` and selector `[194, 238, 58, 8]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct OneReturn(pub I256);
    #[doc = "Container type for all return fields from the `ONE_UINT` function with signature `ONE_UINT()` and selector `[99, 245, 115, 129]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct OneUintReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `SECONDS_IN_YEAR` function with signature `SECONDS_IN_YEAR()` and selector `[93, 204, 147, 145]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SecondsInYearReturn(pub I256);
    #[doc = "Container type for all return fields from the `cacheMaxAgeInSeconds` function with signature `cacheMaxAgeInSeconds()` and selector `[182, 35, 245, 25]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CacheMaxAgeInSecondsReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `factory` function with signature `factory()` and selector `[196, 90, 1, 85]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct FactoryReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `fcm` function with signature `fcm()` and selector `[34, 210, 59, 33]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct FcmReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getHistoricalApy` function with signature `getHistoricalApy()` and selector `[227, 240, 131, 116]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetHistoricalApyReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getHistoricalApyReadOnly` function with signature `getHistoricalApyReadOnly()` and selector `[213, 13, 136, 17]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetHistoricalApyReadOnlyReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getPosition` function with signature `getPosition(address,int24,int24)` and selector `[146, 9, 233, 186]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetPositionReturn(pub Info);
    #[doc = "Container type for all return fields from the `getPositionMarginRequirement` function with signature `getPositionMarginRequirement(address,int24,int24,bool)` and selector `[241, 33, 97, 5]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetPositionMarginRequirementReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `isAlpha` function with signature `isAlpha()` and selector `[136, 66, 135, 82]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsAlphaReturn(pub bool);
    #[doc = "Container type for all return fields from the `liquidatePosition` function with signature `liquidatePosition(address,int24,int24)` and selector `[22, 86, 80, 62]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LiquidatePositionReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `liquidatorRewardWad` function with signature `liquidatorRewardWad()` and selector `[224, 135, 202, 241]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LiquidatorRewardWadReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `lookbackWindowInSeconds` function with signature `lookbackWindowInSeconds()` and selector `[156, 191, 241, 136]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LookbackWindowInSecondsReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `marginEngineParameters` function with signature `marginEngineParameters()` and selector `[105, 56, 33, 127]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MarginEngineParametersReturn(pub MarginCalculatorParameters);
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
    #[doc = "Container type for all return fields from the `proxiableUUID` function with signature `proxiableUUID()` and selector `[82, 209, 144, 45]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ProxiableUUIDReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `rateOracle` function with signature `rateOracle()` and selector `[152, 244, 177, 178]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct RateOracleReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `termEndTimestampWad` function with signature `termEndTimestampWad()` and selector `[147, 237, 180, 84]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TermEndTimestampWadReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `termStartTimestampWad` function with signature `termStartTimestampWad()` and selector `[101, 44, 48, 183]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TermStartTimestampWadReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `underlyingToken` function with signature `underlyingToken()` and selector `[36, 149, 165, 153]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct UnderlyingTokenReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `updatePositionPostVAMMInducedMintBurn` function with signature `updatePositionPostVAMMInducedMintBurn((address,int24,int24,int128))` and selector `[191, 181, 96, 125]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct UpdatePositionPostVAMMInducedMintBurnReturn {
        pub position_margin_requirement: I256,
    }
    #[doc = "Container type for all return fields from the `updatePositionPostVAMMInducedSwap` function with signature `updatePositionPostVAMMInducedSwap(address,int24,int24,int256,int256,uint256,int256)` and selector `[192, 150, 23, 174]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct UpdatePositionPostVAMMInducedSwapReturn {
        pub position_margin_requirement: I256,
    }
    #[doc = "Container type for all return fields from the `vamm` function with signature `vamm()` and selector `[224, 152, 55, 44]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct VammReturn(pub ethers::core::types::Address);
}
