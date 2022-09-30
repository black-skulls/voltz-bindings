pub use i_margin_engine::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_margin_engine {
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
    #[doc = "IMarginEngine was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IMARGINENGINE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"type\":\"error\",\"name\":\"AavePoolGetReserveNormalizedIncomeReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"AavePoolGetReserveNormalizedVariableDebtReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CTokenExchangeRateReturnedZero\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"unlocked\",\"type\":\"bool\",\"components\":[]}],\"type\":\"error\",\"name\":\"CanOnlyTradeIfUnlocked\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CannotLiquidate\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CannotSettleBeforeMaturity\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"DebugError\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"amount0\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"amount1\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"ExpectedOppositeSigns\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint160\",\"name\":\"sqrtPriceX96\",\"type\":\"uint160\",\"components\":[]}],\"type\":\"error\",\"name\":\"ExpectedSqrtPriceZeroBeforeInit\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"IRSNotionalAmountSpecifiedMustBeNonZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidMarginDelta\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"LidoGetPooledEthBySharesReturnedZero\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"type\":\"error\",\"name\":\"LiquidityDeltaMustBePositiveInBurn\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"type\":\"error\",\"name\":\"LiquidityDeltaMustBePositiveInMint\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginLessThanMinimum\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tick\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"variableTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"cumulativeFeeIncurred\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenDeltaUnbalanced\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginRequirementNotMet\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginRequirementNotMetFCM\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"requested\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"available\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"NotEnoughFunds\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OOO\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyFCM\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyMarginEngine\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyOwnerCanUpdatePosition\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyVAMM\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PositionNetZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PositionNotSettled\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"RocketPoolGetEthValueReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"WithdrawalExceedsCurrentMargin\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"closeToOrBeyondMaturity\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"cacheMaxAgeInSeconds\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"CacheMaxAgeSetting\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract IFCM\",\"name\":\"fcm\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"FCMSetting\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"HistoricalApy\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"secondsAgo\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"HistoricalApyWindowSetting\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"__isAlpha\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"IsAlpha\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"liquidatorRewardWad\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"LiquidatorRewardSetting\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"struct IMarginEngine.MarginCalculatorParameters\",\"name\":\"marginCalculatorParameters\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"apyUpperMultiplierWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"apyLowerMultiplierWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"sigmaSquaredWad\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"alphaWad\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"betaWad\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"xiUpperWad\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"xiLowerWad\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"tMaxWad\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"etaIMWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"etaLMWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap1\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap2\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap3\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap4\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap5\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap6\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap7\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"minMarginToIncentiviseLiquidators\",\"type\":\"uint256\",\"components\":[]}],\"indexed\":false}],\"type\":\"event\",\"name\":\"MarginCalculatorParametersSetting\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[],\"indexed\":true},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"liquidator\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"notionalUnwound\",\"type\":\"int256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"liquidatorReward\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PositionLiquidation\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[],\"indexed\":true},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[],\"indexed\":true},{\"internalType\":\"int256\",\"name\":\"marginDelta\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PositionMarginUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[],\"indexed\":true},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[],\"indexed\":true},{\"internalType\":\"int256\",\"name\":\"settlementCashflow\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PositionSettlement\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[],\"indexed\":true},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint128\",\"name\":\"_liquidity\",\"type\":\"uint128\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"margin\",\"type\":\"int256\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"fixedTokenBalance\",\"type\":\"int256\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"variableTokenBalance\",\"type\":\"int256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"accumulatedFees\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PositionUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ProtocolCollection\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"cacheMaxAgeInSeconds\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RateOracle\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract IRateOracle\",\"name\":\"rateOracle\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"RateOracleSetting\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract IVAMM\",\"name\":\"vamm\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"VAMMSetting\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"cacheMaxAgeInSeconds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"collectProtocol\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"factory\",\"outputs\":[{\"internalType\":\"contract IFactory\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"fcm\",\"outputs\":[{\"internalType\":\"contract IFCM\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"getHistoricalApy\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getHistoricalApyReadOnly\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"_tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"_tickUpper\",\"type\":\"int24\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"getPosition\",\"outputs\":[{\"internalType\":\"struct Position.Info\",\"name\":\"position\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bool\",\"name\":\"isSettled\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"_liquidity\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"margin\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenGrowthInsideLastX128\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"variableTokenGrowthInsideLastX128\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenBalance\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"variableTokenBalance\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"feeGrowthInsideLastX128\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"rewardPerAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"accumulatedFees\",\"type\":\"uint256\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"_tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"_tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"_isLM\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"getPositionMarginRequirement\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract IERC20Minimal\",\"name\":\"__underlyingToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IRateOracle\",\"name\":\"__rateOracle\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"__termStartTimestampWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"__termEndTimestampWad\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isAlpha\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"_tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"_tickUpper\",\"type\":\"int24\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"liquidatePosition\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"liquidatorRewardWad\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lookbackWindowInSeconds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"marginEngineParameters\",\"outputs\":[{\"internalType\":\"struct IMarginEngine.MarginCalculatorParameters\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"apyUpperMultiplierWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"apyLowerMultiplierWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"sigmaSquaredWad\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"alphaWad\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"betaWad\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"xiUpperWad\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"xiLowerWad\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"tMaxWad\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"etaIMWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"etaLMWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap1\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap2\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap3\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap4\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap5\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap6\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap7\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"minMarginToIncentiviseLiquidators\",\"type\":\"uint256\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"rateOracle\",\"outputs\":[{\"internalType\":\"contract IRateOracle\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_cacheMaxAgeInSeconds\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setCacheMaxAgeInSeconds\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract IFCM\",\"name\":\"_newFCM\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setFCM\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"__isAlpha\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setIsAlpha\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_liquidatorRewardWad\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLiquidatorReward\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_secondsAgo\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLookbackWindowInSeconds\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IMarginEngine.MarginCalculatorParameters\",\"name\":\"_marginCalculatorParameters\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"apyUpperMultiplierWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"apyLowerMultiplierWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"sigmaSquaredWad\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"alphaWad\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"betaWad\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"xiUpperWad\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"xiLowerWad\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"tMaxWad\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"etaIMWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"etaLMWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap1\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap2\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap3\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap4\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap5\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap6\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gap7\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"minMarginToIncentiviseLiquidators\",\"type\":\"uint256\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setMarginCalculatorParameters\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"state\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPausability\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract IRateOracle\",\"name\":\"__rateOracle\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setRateOracle\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract IVAMM\",\"name\":\"_vAMM\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setVAMM\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"_tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"_tickUpper\",\"type\":\"int24\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"settlePosition\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"termEndTimestampWad\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"termStartTimestampWad\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_marginDelta\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferMarginToFCMTrader\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"underlyingToken\",\"outputs\":[{\"internalType\":\"contract IERC20Minimal\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"_tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"_tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"marginDelta\",\"type\":\"int256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updatePositionMargin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IPositionStructs.ModifyPositionParams\",\"name\":\"_params\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"liquidityDelta\",\"type\":\"int128\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updatePositionPostVAMMInducedMintBurn\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"_positionMarginRequirement\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"_tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"_tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_fixedTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_variableTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_cumulativeFeeIncurred\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_fixedTokenDeltaUnbalanced\",\"type\":\"int256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updatePositionPostVAMMInducedSwap\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"_positionMarginRequirement\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"vamm\",\"outputs\":[{\"internalType\":\"contract IVAMM\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct IMarginEngine<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IMarginEngine<M> {
        fn clone(&self) -> Self {
            IMarginEngine(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IMarginEngine<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IMarginEngine<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IMarginEngine))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IMarginEngine<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IMARGINENGINE_ABI.clone(), client)
                .into()
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
        #[doc = "Calls the contract's `rateOracle` (0x98f4b1b2) function"]
        pub fn rate_oracle(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([152, 244, 177, 178], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setCacheMaxAgeInSeconds` (0xc3261892) function"]
        pub fn set_cache_max_age_in_seconds(
            &self,
            cache_max_age_in_seconds: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([195, 38, 24, 146], cache_max_age_in_seconds)
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
            liquidator_reward_wad: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([233, 228, 65, 187], liquidator_reward_wad)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLookbackWindowInSeconds` (0xa1ea6a20) function"]
        pub fn set_lookback_window_in_seconds(
            &self,
            seconds_ago: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([161, 234, 106, 32], seconds_ago)
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
        #[doc = "Calls the contract's `vamm` (0xe098372c) function"]
        pub fn vamm(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([224, 152, 55, 44], ())
                .expect("method not found (this should never happen)")
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
        #[doc = "Gets the contract's `VAMMSetting` event"]
        pub fn vamm_setting_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, VammsettingFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, IMarginEngineEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IMarginEngine<M> {
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
    pub enum IMarginEngineErrors {
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
    impl ethers::core::abi::AbiDecode for IMarginEngineErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok (decoded) = < AavePoolGetReserveNormalizedIncomeReturnedZero as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (IMarginEngineErrors :: AavePoolGetReserveNormalizedIncomeReturnedZero (decoded)) }
            if let Ok (decoded) = < AavePoolGetReserveNormalizedVariableDebtReturnedZero as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (IMarginEngineErrors :: AavePoolGetReserveNormalizedVariableDebtReturnedZero (decoded)) }
            if let Ok(decoded) =
                <CTokenExchangeRateReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IMarginEngineErrors::CTokenExchangeRateReturnedZero(decoded));
            }
            if let Ok(decoded) =
                <CanOnlyTradeIfUnlocked as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineErrors::CanOnlyTradeIfUnlocked(decoded));
            }
            if let Ok(decoded) =
                <CannotLiquidate as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineErrors::CannotLiquidate(decoded));
            }
            if let Ok(decoded) =
                <CannotSettleBeforeMaturity as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineErrors::CannotSettleBeforeMaturity(decoded));
            }
            if let Ok(decoded) = <DebugError as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineErrors::DebugError(decoded));
            }
            if let Ok(decoded) =
                <ExpectedOppositeSigns as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineErrors::ExpectedOppositeSigns(decoded));
            }
            if let Ok(decoded) =
                <ExpectedSqrtPriceZeroBeforeInit as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IMarginEngineErrors::ExpectedSqrtPriceZeroBeforeInit(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <IRSNotionalAmountSpecifiedMustBeNonZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IMarginEngineErrors::IRSNotionalAmountSpecifiedMustBeNonZero(decoded));
            }
            if let Ok(decoded) =
                <InvalidMarginDelta as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineErrors::InvalidMarginDelta(decoded));
            }
            if let Ok(decoded) =
                <LidoGetPooledEthBySharesReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IMarginEngineErrors::LidoGetPooledEthBySharesReturnedZero(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <LiquidityDeltaMustBePositiveInBurn as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IMarginEngineErrors::LiquidityDeltaMustBePositiveInBurn(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <LiquidityDeltaMustBePositiveInMint as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IMarginEngineErrors::LiquidityDeltaMustBePositiveInMint(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <MarginLessThanMinimum as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineErrors::MarginLessThanMinimum(decoded));
            }
            if let Ok(decoded) =
                <MarginRequirementNotMet as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineErrors::MarginRequirementNotMet(decoded));
            }
            if let Ok(decoded) =
                <MarginRequirementNotMetFCM as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineErrors::MarginRequirementNotMetFCM(decoded));
            }
            if let Ok(decoded) =
                <NotEnoughFunds as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineErrors::NotEnoughFunds(decoded));
            }
            if let Ok(decoded) = <OOO as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IMarginEngineErrors::OOO(decoded));
            }
            if let Ok(decoded) = <OnlyFCM as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IMarginEngineErrors::OnlyFCM(decoded));
            }
            if let Ok(decoded) =
                <OnlyMarginEngine as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineErrors::OnlyMarginEngine(decoded));
            }
            if let Ok(decoded) =
                <OnlyOwnerCanUpdatePosition as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineErrors::OnlyOwnerCanUpdatePosition(decoded));
            }
            if let Ok(decoded) = <OnlyVAMM as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IMarginEngineErrors::OnlyVAMM(decoded));
            }
            if let Ok(decoded) =
                <PositionNetZero as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineErrors::PositionNetZero(decoded));
            }
            if let Ok(decoded) =
                <PositionNotSettled as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineErrors::PositionNotSettled(decoded));
            }
            if let Ok(decoded) =
                <RocketPoolGetEthValueReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IMarginEngineErrors::RocketPoolGetEthValueReturnedZero(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <WithdrawalExceedsCurrentMargin as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IMarginEngineErrors::WithdrawalExceedsCurrentMargin(decoded));
            }
            if let Ok(decoded) =
                <closeToOrBeyondMaturity as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineErrors::closeToOrBeyondMaturity(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IMarginEngineErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                IMarginEngineErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(element) => {
                    element.encode()
                }
                IMarginEngineErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(
                    element,
                ) => element.encode(),
                IMarginEngineErrors::CTokenExchangeRateReturnedZero(element) => element.encode(),
                IMarginEngineErrors::CanOnlyTradeIfUnlocked(element) => element.encode(),
                IMarginEngineErrors::CannotLiquidate(element) => element.encode(),
                IMarginEngineErrors::CannotSettleBeforeMaturity(element) => element.encode(),
                IMarginEngineErrors::DebugError(element) => element.encode(),
                IMarginEngineErrors::ExpectedOppositeSigns(element) => element.encode(),
                IMarginEngineErrors::ExpectedSqrtPriceZeroBeforeInit(element) => element.encode(),
                IMarginEngineErrors::IRSNotionalAmountSpecifiedMustBeNonZero(element) => {
                    element.encode()
                }
                IMarginEngineErrors::InvalidMarginDelta(element) => element.encode(),
                IMarginEngineErrors::LidoGetPooledEthBySharesReturnedZero(element) => {
                    element.encode()
                }
                IMarginEngineErrors::LiquidityDeltaMustBePositiveInBurn(element) => {
                    element.encode()
                }
                IMarginEngineErrors::LiquidityDeltaMustBePositiveInMint(element) => {
                    element.encode()
                }
                IMarginEngineErrors::MarginLessThanMinimum(element) => element.encode(),
                IMarginEngineErrors::MarginRequirementNotMet(element) => element.encode(),
                IMarginEngineErrors::MarginRequirementNotMetFCM(element) => element.encode(),
                IMarginEngineErrors::NotEnoughFunds(element) => element.encode(),
                IMarginEngineErrors::OOO(element) => element.encode(),
                IMarginEngineErrors::OnlyFCM(element) => element.encode(),
                IMarginEngineErrors::OnlyMarginEngine(element) => element.encode(),
                IMarginEngineErrors::OnlyOwnerCanUpdatePosition(element) => element.encode(),
                IMarginEngineErrors::OnlyVAMM(element) => element.encode(),
                IMarginEngineErrors::PositionNetZero(element) => element.encode(),
                IMarginEngineErrors::PositionNotSettled(element) => element.encode(),
                IMarginEngineErrors::RocketPoolGetEthValueReturnedZero(element) => element.encode(),
                IMarginEngineErrors::WithdrawalExceedsCurrentMargin(element) => element.encode(),
                IMarginEngineErrors::closeToOrBeyondMaturity(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IMarginEngineErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IMarginEngineErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(element) => {
                    element.fmt(f)
                }
                IMarginEngineErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(
                    element,
                ) => element.fmt(f),
                IMarginEngineErrors::CTokenExchangeRateReturnedZero(element) => element.fmt(f),
                IMarginEngineErrors::CanOnlyTradeIfUnlocked(element) => element.fmt(f),
                IMarginEngineErrors::CannotLiquidate(element) => element.fmt(f),
                IMarginEngineErrors::CannotSettleBeforeMaturity(element) => element.fmt(f),
                IMarginEngineErrors::DebugError(element) => element.fmt(f),
                IMarginEngineErrors::ExpectedOppositeSigns(element) => element.fmt(f),
                IMarginEngineErrors::ExpectedSqrtPriceZeroBeforeInit(element) => element.fmt(f),
                IMarginEngineErrors::IRSNotionalAmountSpecifiedMustBeNonZero(element) => {
                    element.fmt(f)
                }
                IMarginEngineErrors::InvalidMarginDelta(element) => element.fmt(f),
                IMarginEngineErrors::LidoGetPooledEthBySharesReturnedZero(element) => {
                    element.fmt(f)
                }
                IMarginEngineErrors::LiquidityDeltaMustBePositiveInBurn(element) => element.fmt(f),
                IMarginEngineErrors::LiquidityDeltaMustBePositiveInMint(element) => element.fmt(f),
                IMarginEngineErrors::MarginLessThanMinimum(element) => element.fmt(f),
                IMarginEngineErrors::MarginRequirementNotMet(element) => element.fmt(f),
                IMarginEngineErrors::MarginRequirementNotMetFCM(element) => element.fmt(f),
                IMarginEngineErrors::NotEnoughFunds(element) => element.fmt(f),
                IMarginEngineErrors::OOO(element) => element.fmt(f),
                IMarginEngineErrors::OnlyFCM(element) => element.fmt(f),
                IMarginEngineErrors::OnlyMarginEngine(element) => element.fmt(f),
                IMarginEngineErrors::OnlyOwnerCanUpdatePosition(element) => element.fmt(f),
                IMarginEngineErrors::OnlyVAMM(element) => element.fmt(f),
                IMarginEngineErrors::PositionNetZero(element) => element.fmt(f),
                IMarginEngineErrors::PositionNotSettled(element) => element.fmt(f),
                IMarginEngineErrors::RocketPoolGetEthValueReturnedZero(element) => element.fmt(f),
                IMarginEngineErrors::WithdrawalExceedsCurrentMargin(element) => element.fmt(f),
                IMarginEngineErrors::closeToOrBeyondMaturity(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AavePoolGetReserveNormalizedIncomeReturnedZero> for IMarginEngineErrors {
        fn from(var: AavePoolGetReserveNormalizedIncomeReturnedZero) -> Self {
            IMarginEngineErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(var)
        }
    }
    impl ::std::convert::From<AavePoolGetReserveNormalizedVariableDebtReturnedZero>
        for IMarginEngineErrors
    {
        fn from(var: AavePoolGetReserveNormalizedVariableDebtReturnedZero) -> Self {
            IMarginEngineErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(var)
        }
    }
    impl ::std::convert::From<CTokenExchangeRateReturnedZero> for IMarginEngineErrors {
        fn from(var: CTokenExchangeRateReturnedZero) -> Self {
            IMarginEngineErrors::CTokenExchangeRateReturnedZero(var)
        }
    }
    impl ::std::convert::From<CanOnlyTradeIfUnlocked> for IMarginEngineErrors {
        fn from(var: CanOnlyTradeIfUnlocked) -> Self {
            IMarginEngineErrors::CanOnlyTradeIfUnlocked(var)
        }
    }
    impl ::std::convert::From<CannotLiquidate> for IMarginEngineErrors {
        fn from(var: CannotLiquidate) -> Self {
            IMarginEngineErrors::CannotLiquidate(var)
        }
    }
    impl ::std::convert::From<CannotSettleBeforeMaturity> for IMarginEngineErrors {
        fn from(var: CannotSettleBeforeMaturity) -> Self {
            IMarginEngineErrors::CannotSettleBeforeMaturity(var)
        }
    }
    impl ::std::convert::From<DebugError> for IMarginEngineErrors {
        fn from(var: DebugError) -> Self {
            IMarginEngineErrors::DebugError(var)
        }
    }
    impl ::std::convert::From<ExpectedOppositeSigns> for IMarginEngineErrors {
        fn from(var: ExpectedOppositeSigns) -> Self {
            IMarginEngineErrors::ExpectedOppositeSigns(var)
        }
    }
    impl ::std::convert::From<ExpectedSqrtPriceZeroBeforeInit> for IMarginEngineErrors {
        fn from(var: ExpectedSqrtPriceZeroBeforeInit) -> Self {
            IMarginEngineErrors::ExpectedSqrtPriceZeroBeforeInit(var)
        }
    }
    impl ::std::convert::From<IRSNotionalAmountSpecifiedMustBeNonZero> for IMarginEngineErrors {
        fn from(var: IRSNotionalAmountSpecifiedMustBeNonZero) -> Self {
            IMarginEngineErrors::IRSNotionalAmountSpecifiedMustBeNonZero(var)
        }
    }
    impl ::std::convert::From<InvalidMarginDelta> for IMarginEngineErrors {
        fn from(var: InvalidMarginDelta) -> Self {
            IMarginEngineErrors::InvalidMarginDelta(var)
        }
    }
    impl ::std::convert::From<LidoGetPooledEthBySharesReturnedZero> for IMarginEngineErrors {
        fn from(var: LidoGetPooledEthBySharesReturnedZero) -> Self {
            IMarginEngineErrors::LidoGetPooledEthBySharesReturnedZero(var)
        }
    }
    impl ::std::convert::From<LiquidityDeltaMustBePositiveInBurn> for IMarginEngineErrors {
        fn from(var: LiquidityDeltaMustBePositiveInBurn) -> Self {
            IMarginEngineErrors::LiquidityDeltaMustBePositiveInBurn(var)
        }
    }
    impl ::std::convert::From<LiquidityDeltaMustBePositiveInMint> for IMarginEngineErrors {
        fn from(var: LiquidityDeltaMustBePositiveInMint) -> Self {
            IMarginEngineErrors::LiquidityDeltaMustBePositiveInMint(var)
        }
    }
    impl ::std::convert::From<MarginLessThanMinimum> for IMarginEngineErrors {
        fn from(var: MarginLessThanMinimum) -> Self {
            IMarginEngineErrors::MarginLessThanMinimum(var)
        }
    }
    impl ::std::convert::From<MarginRequirementNotMet> for IMarginEngineErrors {
        fn from(var: MarginRequirementNotMet) -> Self {
            IMarginEngineErrors::MarginRequirementNotMet(var)
        }
    }
    impl ::std::convert::From<MarginRequirementNotMetFCM> for IMarginEngineErrors {
        fn from(var: MarginRequirementNotMetFCM) -> Self {
            IMarginEngineErrors::MarginRequirementNotMetFCM(var)
        }
    }
    impl ::std::convert::From<NotEnoughFunds> for IMarginEngineErrors {
        fn from(var: NotEnoughFunds) -> Self {
            IMarginEngineErrors::NotEnoughFunds(var)
        }
    }
    impl ::std::convert::From<OOO> for IMarginEngineErrors {
        fn from(var: OOO) -> Self {
            IMarginEngineErrors::OOO(var)
        }
    }
    impl ::std::convert::From<OnlyFCM> for IMarginEngineErrors {
        fn from(var: OnlyFCM) -> Self {
            IMarginEngineErrors::OnlyFCM(var)
        }
    }
    impl ::std::convert::From<OnlyMarginEngine> for IMarginEngineErrors {
        fn from(var: OnlyMarginEngine) -> Self {
            IMarginEngineErrors::OnlyMarginEngine(var)
        }
    }
    impl ::std::convert::From<OnlyOwnerCanUpdatePosition> for IMarginEngineErrors {
        fn from(var: OnlyOwnerCanUpdatePosition) -> Self {
            IMarginEngineErrors::OnlyOwnerCanUpdatePosition(var)
        }
    }
    impl ::std::convert::From<OnlyVAMM> for IMarginEngineErrors {
        fn from(var: OnlyVAMM) -> Self {
            IMarginEngineErrors::OnlyVAMM(var)
        }
    }
    impl ::std::convert::From<PositionNetZero> for IMarginEngineErrors {
        fn from(var: PositionNetZero) -> Self {
            IMarginEngineErrors::PositionNetZero(var)
        }
    }
    impl ::std::convert::From<PositionNotSettled> for IMarginEngineErrors {
        fn from(var: PositionNotSettled) -> Self {
            IMarginEngineErrors::PositionNotSettled(var)
        }
    }
    impl ::std::convert::From<RocketPoolGetEthValueReturnedZero> for IMarginEngineErrors {
        fn from(var: RocketPoolGetEthValueReturnedZero) -> Self {
            IMarginEngineErrors::RocketPoolGetEthValueReturnedZero(var)
        }
    }
    impl ::std::convert::From<WithdrawalExceedsCurrentMargin> for IMarginEngineErrors {
        fn from(var: WithdrawalExceedsCurrentMargin) -> Self {
            IMarginEngineErrors::WithdrawalExceedsCurrentMargin(var)
        }
    }
    impl ::std::convert::From<closeToOrBeyondMaturity> for IMarginEngineErrors {
        fn from(var: closeToOrBeyondMaturity) -> Self {
            IMarginEngineErrors::closeToOrBeyondMaturity(var)
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
    #[ethevent(name = "VAMMSetting", abi = "VAMMSetting(address)")]
    pub struct VammsettingFilter {
        #[ethevent(indexed)]
        pub vamm: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IMarginEngineEvents {
        CacheMaxAgeSettingFilter(CacheMaxAgeSettingFilter),
        FcmsettingFilter(FcmsettingFilter),
        HistoricalApyFilter(HistoricalApyFilter),
        HistoricalApyWindowSettingFilter(HistoricalApyWindowSettingFilter),
        IsAlphaFilter(IsAlphaFilter),
        LiquidatorRewardSettingFilter(LiquidatorRewardSettingFilter),
        MarginCalculatorParametersSettingFilter(MarginCalculatorParametersSettingFilter),
        PositionLiquidationFilter(PositionLiquidationFilter),
        PositionMarginUpdateFilter(PositionMarginUpdateFilter),
        PositionSettlementFilter(PositionSettlementFilter),
        PositionUpdateFilter(PositionUpdateFilter),
        ProtocolCollectionFilter(ProtocolCollectionFilter),
        RateOracleFilter(RateOracleFilter),
        RateOracleSettingFilter(RateOracleSettingFilter),
        VammsettingFilter(VammsettingFilter),
    }
    impl ethers::contract::EthLogDecode for IMarginEngineEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = CacheMaxAgeSettingFilter::decode_log(log) {
                return Ok(IMarginEngineEvents::CacheMaxAgeSettingFilter(decoded));
            }
            if let Ok(decoded) = FcmsettingFilter::decode_log(log) {
                return Ok(IMarginEngineEvents::FcmsettingFilter(decoded));
            }
            if let Ok(decoded) = HistoricalApyFilter::decode_log(log) {
                return Ok(IMarginEngineEvents::HistoricalApyFilter(decoded));
            }
            if let Ok(decoded) = HistoricalApyWindowSettingFilter::decode_log(log) {
                return Ok(IMarginEngineEvents::HistoricalApyWindowSettingFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = IsAlphaFilter::decode_log(log) {
                return Ok(IMarginEngineEvents::IsAlphaFilter(decoded));
            }
            if let Ok(decoded) = LiquidatorRewardSettingFilter::decode_log(log) {
                return Ok(IMarginEngineEvents::LiquidatorRewardSettingFilter(decoded));
            }
            if let Ok(decoded) = MarginCalculatorParametersSettingFilter::decode_log(log) {
                return Ok(IMarginEngineEvents::MarginCalculatorParametersSettingFilter(decoded));
            }
            if let Ok(decoded) = PositionLiquidationFilter::decode_log(log) {
                return Ok(IMarginEngineEvents::PositionLiquidationFilter(decoded));
            }
            if let Ok(decoded) = PositionMarginUpdateFilter::decode_log(log) {
                return Ok(IMarginEngineEvents::PositionMarginUpdateFilter(decoded));
            }
            if let Ok(decoded) = PositionSettlementFilter::decode_log(log) {
                return Ok(IMarginEngineEvents::PositionSettlementFilter(decoded));
            }
            if let Ok(decoded) = PositionUpdateFilter::decode_log(log) {
                return Ok(IMarginEngineEvents::PositionUpdateFilter(decoded));
            }
            if let Ok(decoded) = ProtocolCollectionFilter::decode_log(log) {
                return Ok(IMarginEngineEvents::ProtocolCollectionFilter(decoded));
            }
            if let Ok(decoded) = RateOracleFilter::decode_log(log) {
                return Ok(IMarginEngineEvents::RateOracleFilter(decoded));
            }
            if let Ok(decoded) = RateOracleSettingFilter::decode_log(log) {
                return Ok(IMarginEngineEvents::RateOracleSettingFilter(decoded));
            }
            if let Ok(decoded) = VammsettingFilter::decode_log(log) {
                return Ok(IMarginEngineEvents::VammsettingFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IMarginEngineEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IMarginEngineEvents::CacheMaxAgeSettingFilter(element) => element.fmt(f),
                IMarginEngineEvents::FcmsettingFilter(element) => element.fmt(f),
                IMarginEngineEvents::HistoricalApyFilter(element) => element.fmt(f),
                IMarginEngineEvents::HistoricalApyWindowSettingFilter(element) => element.fmt(f),
                IMarginEngineEvents::IsAlphaFilter(element) => element.fmt(f),
                IMarginEngineEvents::LiquidatorRewardSettingFilter(element) => element.fmt(f),
                IMarginEngineEvents::MarginCalculatorParametersSettingFilter(element) => {
                    element.fmt(f)
                }
                IMarginEngineEvents::PositionLiquidationFilter(element) => element.fmt(f),
                IMarginEngineEvents::PositionMarginUpdateFilter(element) => element.fmt(f),
                IMarginEngineEvents::PositionSettlementFilter(element) => element.fmt(f),
                IMarginEngineEvents::PositionUpdateFilter(element) => element.fmt(f),
                IMarginEngineEvents::ProtocolCollectionFilter(element) => element.fmt(f),
                IMarginEngineEvents::RateOracleFilter(element) => element.fmt(f),
                IMarginEngineEvents::RateOracleSettingFilter(element) => element.fmt(f),
                IMarginEngineEvents::VammsettingFilter(element) => element.fmt(f),
            }
        }
    }
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
        pub cache_max_age_in_seconds: ethers::core::types::U256,
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
        pub liquidator_reward_wad: ethers::core::types::U256,
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
        pub seconds_ago: ethers::core::types::U256,
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
    pub enum IMarginEngineCalls {
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
        RateOracle(RateOracleCall),
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
        UnderlyingToken(UnderlyingTokenCall),
        UpdatePositionMargin(UpdatePositionMarginCall),
        UpdatePositionPostVAMMInducedMintBurn(UpdatePositionPostVAMMInducedMintBurnCall),
        UpdatePositionPostVAMMInducedSwap(UpdatePositionPostVAMMInducedSwapCall),
        Vamm(VammCall),
    }
    impl ethers::core::abi::AbiDecode for IMarginEngineCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <CacheMaxAgeInSecondsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineCalls::CacheMaxAgeInSeconds(decoded));
            }
            if let Ok(decoded) =
                <CollectProtocolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineCalls::CollectProtocol(decoded));
            }
            if let Ok(decoded) =
                <FactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineCalls::Factory(decoded));
            }
            if let Ok(decoded) = <FcmCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IMarginEngineCalls::Fcm(decoded));
            }
            if let Ok(decoded) =
                <GetHistoricalApyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineCalls::GetHistoricalApy(decoded));
            }
            if let Ok(decoded) =
                <GetHistoricalApyReadOnlyCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IMarginEngineCalls::GetHistoricalApyReadOnly(decoded));
            }
            if let Ok(decoded) =
                <GetPositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineCalls::GetPosition(decoded));
            }
            if let Ok(decoded) =
                <GetPositionMarginRequirementCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IMarginEngineCalls::GetPositionMarginRequirement(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineCalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <IsAlphaCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineCalls::IsAlpha(decoded));
            }
            if let Ok(decoded) =
                <LiquidatePositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineCalls::LiquidatePosition(decoded));
            }
            if let Ok(decoded) =
                <LiquidatorRewardWadCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineCalls::LiquidatorRewardWad(decoded));
            }
            if let Ok(decoded) =
                <LookbackWindowInSecondsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineCalls::LookbackWindowInSeconds(decoded));
            }
            if let Ok(decoded) =
                <MarginEngineParametersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineCalls::MarginEngineParameters(decoded));
            }
            if let Ok(decoded) =
                <RateOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineCalls::RateOracle(decoded));
            }
            if let Ok(decoded) =
                <SetCacheMaxAgeInSecondsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineCalls::SetCacheMaxAgeInSeconds(decoded));
            }
            if let Ok(decoded) = <SetFCMCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineCalls::SetFCM(decoded));
            }
            if let Ok(decoded) =
                <SetIsAlphaCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineCalls::SetIsAlpha(decoded));
            }
            if let Ok(decoded) =
                <SetLiquidatorRewardCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineCalls::SetLiquidatorReward(decoded));
            }
            if let Ok(decoded) =
                <SetLookbackWindowInSecondsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IMarginEngineCalls::SetLookbackWindowInSeconds(decoded));
            }
            if let Ok(decoded) =
                <SetMarginCalculatorParametersCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IMarginEngineCalls::SetMarginCalculatorParameters(decoded));
            }
            if let Ok(decoded) =
                <SetPausabilityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineCalls::SetPausability(decoded));
            }
            if let Ok(decoded) =
                <SetRateOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineCalls::SetRateOracle(decoded));
            }
            if let Ok(decoded) =
                <SetVAMMCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineCalls::SetVAMM(decoded));
            }
            if let Ok(decoded) =
                <SettlePositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineCalls::SettlePosition(decoded));
            }
            if let Ok(decoded) =
                <TermEndTimestampWadCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineCalls::TermEndTimestampWad(decoded));
            }
            if let Ok(decoded) =
                <TermStartTimestampWadCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineCalls::TermStartTimestampWad(decoded));
            }
            if let Ok(decoded) =
                <TransferMarginToFCMTraderCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IMarginEngineCalls::TransferMarginToFCMTrader(decoded));
            }
            if let Ok(decoded) =
                <UnderlyingTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineCalls::UnderlyingToken(decoded));
            }
            if let Ok(decoded) =
                <UpdatePositionMarginCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMarginEngineCalls::UpdatePositionMargin(decoded));
            }
            if let Ok(decoded) =
                <UpdatePositionPostVAMMInducedMintBurnCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IMarginEngineCalls::UpdatePositionPostVAMMInducedMintBurn(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <UpdatePositionPostVAMMInducedSwapCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IMarginEngineCalls::UpdatePositionPostVAMMInducedSwap(
                    decoded,
                ));
            }
            if let Ok(decoded) = <VammCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IMarginEngineCalls::Vamm(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IMarginEngineCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IMarginEngineCalls::CacheMaxAgeInSeconds(element) => element.encode(),
                IMarginEngineCalls::CollectProtocol(element) => element.encode(),
                IMarginEngineCalls::Factory(element) => element.encode(),
                IMarginEngineCalls::Fcm(element) => element.encode(),
                IMarginEngineCalls::GetHistoricalApy(element) => element.encode(),
                IMarginEngineCalls::GetHistoricalApyReadOnly(element) => element.encode(),
                IMarginEngineCalls::GetPosition(element) => element.encode(),
                IMarginEngineCalls::GetPositionMarginRequirement(element) => element.encode(),
                IMarginEngineCalls::Initialize(element) => element.encode(),
                IMarginEngineCalls::IsAlpha(element) => element.encode(),
                IMarginEngineCalls::LiquidatePosition(element) => element.encode(),
                IMarginEngineCalls::LiquidatorRewardWad(element) => element.encode(),
                IMarginEngineCalls::LookbackWindowInSeconds(element) => element.encode(),
                IMarginEngineCalls::MarginEngineParameters(element) => element.encode(),
                IMarginEngineCalls::RateOracle(element) => element.encode(),
                IMarginEngineCalls::SetCacheMaxAgeInSeconds(element) => element.encode(),
                IMarginEngineCalls::SetFCM(element) => element.encode(),
                IMarginEngineCalls::SetIsAlpha(element) => element.encode(),
                IMarginEngineCalls::SetLiquidatorReward(element) => element.encode(),
                IMarginEngineCalls::SetLookbackWindowInSeconds(element) => element.encode(),
                IMarginEngineCalls::SetMarginCalculatorParameters(element) => element.encode(),
                IMarginEngineCalls::SetPausability(element) => element.encode(),
                IMarginEngineCalls::SetRateOracle(element) => element.encode(),
                IMarginEngineCalls::SetVAMM(element) => element.encode(),
                IMarginEngineCalls::SettlePosition(element) => element.encode(),
                IMarginEngineCalls::TermEndTimestampWad(element) => element.encode(),
                IMarginEngineCalls::TermStartTimestampWad(element) => element.encode(),
                IMarginEngineCalls::TransferMarginToFCMTrader(element) => element.encode(),
                IMarginEngineCalls::UnderlyingToken(element) => element.encode(),
                IMarginEngineCalls::UpdatePositionMargin(element) => element.encode(),
                IMarginEngineCalls::UpdatePositionPostVAMMInducedMintBurn(element) => {
                    element.encode()
                }
                IMarginEngineCalls::UpdatePositionPostVAMMInducedSwap(element) => element.encode(),
                IMarginEngineCalls::Vamm(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IMarginEngineCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IMarginEngineCalls::CacheMaxAgeInSeconds(element) => element.fmt(f),
                IMarginEngineCalls::CollectProtocol(element) => element.fmt(f),
                IMarginEngineCalls::Factory(element) => element.fmt(f),
                IMarginEngineCalls::Fcm(element) => element.fmt(f),
                IMarginEngineCalls::GetHistoricalApy(element) => element.fmt(f),
                IMarginEngineCalls::GetHistoricalApyReadOnly(element) => element.fmt(f),
                IMarginEngineCalls::GetPosition(element) => element.fmt(f),
                IMarginEngineCalls::GetPositionMarginRequirement(element) => element.fmt(f),
                IMarginEngineCalls::Initialize(element) => element.fmt(f),
                IMarginEngineCalls::IsAlpha(element) => element.fmt(f),
                IMarginEngineCalls::LiquidatePosition(element) => element.fmt(f),
                IMarginEngineCalls::LiquidatorRewardWad(element) => element.fmt(f),
                IMarginEngineCalls::LookbackWindowInSeconds(element) => element.fmt(f),
                IMarginEngineCalls::MarginEngineParameters(element) => element.fmt(f),
                IMarginEngineCalls::RateOracle(element) => element.fmt(f),
                IMarginEngineCalls::SetCacheMaxAgeInSeconds(element) => element.fmt(f),
                IMarginEngineCalls::SetFCM(element) => element.fmt(f),
                IMarginEngineCalls::SetIsAlpha(element) => element.fmt(f),
                IMarginEngineCalls::SetLiquidatorReward(element) => element.fmt(f),
                IMarginEngineCalls::SetLookbackWindowInSeconds(element) => element.fmt(f),
                IMarginEngineCalls::SetMarginCalculatorParameters(element) => element.fmt(f),
                IMarginEngineCalls::SetPausability(element) => element.fmt(f),
                IMarginEngineCalls::SetRateOracle(element) => element.fmt(f),
                IMarginEngineCalls::SetVAMM(element) => element.fmt(f),
                IMarginEngineCalls::SettlePosition(element) => element.fmt(f),
                IMarginEngineCalls::TermEndTimestampWad(element) => element.fmt(f),
                IMarginEngineCalls::TermStartTimestampWad(element) => element.fmt(f),
                IMarginEngineCalls::TransferMarginToFCMTrader(element) => element.fmt(f),
                IMarginEngineCalls::UnderlyingToken(element) => element.fmt(f),
                IMarginEngineCalls::UpdatePositionMargin(element) => element.fmt(f),
                IMarginEngineCalls::UpdatePositionPostVAMMInducedMintBurn(element) => {
                    element.fmt(f)
                }
                IMarginEngineCalls::UpdatePositionPostVAMMInducedSwap(element) => element.fmt(f),
                IMarginEngineCalls::Vamm(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CacheMaxAgeInSecondsCall> for IMarginEngineCalls {
        fn from(var: CacheMaxAgeInSecondsCall) -> Self {
            IMarginEngineCalls::CacheMaxAgeInSeconds(var)
        }
    }
    impl ::std::convert::From<CollectProtocolCall> for IMarginEngineCalls {
        fn from(var: CollectProtocolCall) -> Self {
            IMarginEngineCalls::CollectProtocol(var)
        }
    }
    impl ::std::convert::From<FactoryCall> for IMarginEngineCalls {
        fn from(var: FactoryCall) -> Self {
            IMarginEngineCalls::Factory(var)
        }
    }
    impl ::std::convert::From<FcmCall> for IMarginEngineCalls {
        fn from(var: FcmCall) -> Self {
            IMarginEngineCalls::Fcm(var)
        }
    }
    impl ::std::convert::From<GetHistoricalApyCall> for IMarginEngineCalls {
        fn from(var: GetHistoricalApyCall) -> Self {
            IMarginEngineCalls::GetHistoricalApy(var)
        }
    }
    impl ::std::convert::From<GetHistoricalApyReadOnlyCall> for IMarginEngineCalls {
        fn from(var: GetHistoricalApyReadOnlyCall) -> Self {
            IMarginEngineCalls::GetHistoricalApyReadOnly(var)
        }
    }
    impl ::std::convert::From<GetPositionCall> for IMarginEngineCalls {
        fn from(var: GetPositionCall) -> Self {
            IMarginEngineCalls::GetPosition(var)
        }
    }
    impl ::std::convert::From<GetPositionMarginRequirementCall> for IMarginEngineCalls {
        fn from(var: GetPositionMarginRequirementCall) -> Self {
            IMarginEngineCalls::GetPositionMarginRequirement(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for IMarginEngineCalls {
        fn from(var: InitializeCall) -> Self {
            IMarginEngineCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<IsAlphaCall> for IMarginEngineCalls {
        fn from(var: IsAlphaCall) -> Self {
            IMarginEngineCalls::IsAlpha(var)
        }
    }
    impl ::std::convert::From<LiquidatePositionCall> for IMarginEngineCalls {
        fn from(var: LiquidatePositionCall) -> Self {
            IMarginEngineCalls::LiquidatePosition(var)
        }
    }
    impl ::std::convert::From<LiquidatorRewardWadCall> for IMarginEngineCalls {
        fn from(var: LiquidatorRewardWadCall) -> Self {
            IMarginEngineCalls::LiquidatorRewardWad(var)
        }
    }
    impl ::std::convert::From<LookbackWindowInSecondsCall> for IMarginEngineCalls {
        fn from(var: LookbackWindowInSecondsCall) -> Self {
            IMarginEngineCalls::LookbackWindowInSeconds(var)
        }
    }
    impl ::std::convert::From<MarginEngineParametersCall> for IMarginEngineCalls {
        fn from(var: MarginEngineParametersCall) -> Self {
            IMarginEngineCalls::MarginEngineParameters(var)
        }
    }
    impl ::std::convert::From<RateOracleCall> for IMarginEngineCalls {
        fn from(var: RateOracleCall) -> Self {
            IMarginEngineCalls::RateOracle(var)
        }
    }
    impl ::std::convert::From<SetCacheMaxAgeInSecondsCall> for IMarginEngineCalls {
        fn from(var: SetCacheMaxAgeInSecondsCall) -> Self {
            IMarginEngineCalls::SetCacheMaxAgeInSeconds(var)
        }
    }
    impl ::std::convert::From<SetFCMCall> for IMarginEngineCalls {
        fn from(var: SetFCMCall) -> Self {
            IMarginEngineCalls::SetFCM(var)
        }
    }
    impl ::std::convert::From<SetIsAlphaCall> for IMarginEngineCalls {
        fn from(var: SetIsAlphaCall) -> Self {
            IMarginEngineCalls::SetIsAlpha(var)
        }
    }
    impl ::std::convert::From<SetLiquidatorRewardCall> for IMarginEngineCalls {
        fn from(var: SetLiquidatorRewardCall) -> Self {
            IMarginEngineCalls::SetLiquidatorReward(var)
        }
    }
    impl ::std::convert::From<SetLookbackWindowInSecondsCall> for IMarginEngineCalls {
        fn from(var: SetLookbackWindowInSecondsCall) -> Self {
            IMarginEngineCalls::SetLookbackWindowInSeconds(var)
        }
    }
    impl ::std::convert::From<SetMarginCalculatorParametersCall> for IMarginEngineCalls {
        fn from(var: SetMarginCalculatorParametersCall) -> Self {
            IMarginEngineCalls::SetMarginCalculatorParameters(var)
        }
    }
    impl ::std::convert::From<SetPausabilityCall> for IMarginEngineCalls {
        fn from(var: SetPausabilityCall) -> Self {
            IMarginEngineCalls::SetPausability(var)
        }
    }
    impl ::std::convert::From<SetRateOracleCall> for IMarginEngineCalls {
        fn from(var: SetRateOracleCall) -> Self {
            IMarginEngineCalls::SetRateOracle(var)
        }
    }
    impl ::std::convert::From<SetVAMMCall> for IMarginEngineCalls {
        fn from(var: SetVAMMCall) -> Self {
            IMarginEngineCalls::SetVAMM(var)
        }
    }
    impl ::std::convert::From<SettlePositionCall> for IMarginEngineCalls {
        fn from(var: SettlePositionCall) -> Self {
            IMarginEngineCalls::SettlePosition(var)
        }
    }
    impl ::std::convert::From<TermEndTimestampWadCall> for IMarginEngineCalls {
        fn from(var: TermEndTimestampWadCall) -> Self {
            IMarginEngineCalls::TermEndTimestampWad(var)
        }
    }
    impl ::std::convert::From<TermStartTimestampWadCall> for IMarginEngineCalls {
        fn from(var: TermStartTimestampWadCall) -> Self {
            IMarginEngineCalls::TermStartTimestampWad(var)
        }
    }
    impl ::std::convert::From<TransferMarginToFCMTraderCall> for IMarginEngineCalls {
        fn from(var: TransferMarginToFCMTraderCall) -> Self {
            IMarginEngineCalls::TransferMarginToFCMTrader(var)
        }
    }
    impl ::std::convert::From<UnderlyingTokenCall> for IMarginEngineCalls {
        fn from(var: UnderlyingTokenCall) -> Self {
            IMarginEngineCalls::UnderlyingToken(var)
        }
    }
    impl ::std::convert::From<UpdatePositionMarginCall> for IMarginEngineCalls {
        fn from(var: UpdatePositionMarginCall) -> Self {
            IMarginEngineCalls::UpdatePositionMargin(var)
        }
    }
    impl ::std::convert::From<UpdatePositionPostVAMMInducedMintBurnCall> for IMarginEngineCalls {
        fn from(var: UpdatePositionPostVAMMInducedMintBurnCall) -> Self {
            IMarginEngineCalls::UpdatePositionPostVAMMInducedMintBurn(var)
        }
    }
    impl ::std::convert::From<UpdatePositionPostVAMMInducedSwapCall> for IMarginEngineCalls {
        fn from(var: UpdatePositionPostVAMMInducedSwapCall) -> Self {
            IMarginEngineCalls::UpdatePositionPostVAMMInducedSwap(var)
        }
    }
    impl ::std::convert::From<VammCall> for IMarginEngineCalls {
        fn from(var: VammCall) -> Self {
            IMarginEngineCalls::Vamm(var)
        }
    }
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
    pub struct GetPositionReturn {
        pub position: Info,
    }
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
