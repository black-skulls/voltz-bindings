pub use e2e_setup::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod e2e_setup {
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
    #[doc = "E2ESetup was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static E2ESETUP_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"type\":\"error\",\"name\":\"AavePoolGetReserveNormalizedIncomeReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"AavePoolGetReserveNormalizedVariableDebtReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CTokenExchangeRateReturnedZero\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"unlocked\",\"type\":\"bool\",\"components\":[]}],\"type\":\"error\",\"name\":\"CanOnlyTradeIfUnlocked\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CannotLiquidate\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CannotSettleBeforeMaturity\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"DebugError\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"amount0\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"amount1\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"ExpectedOppositeSigns\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint160\",\"name\":\"sqrtPriceX96\",\"type\":\"uint160\",\"components\":[]}],\"type\":\"error\",\"name\":\"ExpectedSqrtPriceZeroBeforeInit\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"IRSNotionalAmountSpecifiedMustBeNonZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidMarginDelta\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"LidoGetPooledEthBySharesReturnedZero\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"type\":\"error\",\"name\":\"LiquidityDeltaMustBePositiveInBurn\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"type\":\"error\",\"name\":\"LiquidityDeltaMustBePositiveInMint\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginLessThanMinimum\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tick\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"variableTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"cumulativeFeeIncurred\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenDeltaUnbalanced\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginRequirementNotMet\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginRequirementNotMetFCM\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"requested\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"available\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"NotEnoughFunds\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OOO\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyFCM\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyMarginEngine\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyOwnerCanUpdatePosition\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyVAMM\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMathUD60x18__FromUintOverflow\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"prod1\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"denominator\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMath__MulDivOverflow\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PositionNetZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PositionNotSettled\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"RocketPoolGetEthValueReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"WithdrawalExceedsCurrentMargin\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"closeToOrBeyondMaturity\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"FCMAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MEAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"VAMMAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"aaveLendingPool\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"abs\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addPosition\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"variableTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenDeltaUnbalanced\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"cumulativeFeeIncurred\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addSwapSnapshot\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addYBATrader\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allPositions\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allYBATraders\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burnViaAMM\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"cToken\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPositionHistory\",\"outputs\":[{\"internalType\":\"struct E2ESetup.PositionSnapshot[]\",\"name\":\"\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"currentTimestampWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"termStartTimestampWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"termEndTimestampWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"margin\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"marginRequirement\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"estimatedSettlementCashflow\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenBalance\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"variableTokenBalance\",\"type\":\"int256\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPositionSwapsHistory\",\"outputs\":[{\"internalType\":\"struct E2ESetup.SwapSnapshot[]\",\"name\":\"\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"reserveNormalizedIncomeAtSwap\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"swapInitiationTimestampWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"termEndTimestampWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"notional\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isFT\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"fixedRateWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"feePaidInUnderlyingTokens\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"indexAllPositions\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"indexAllYBATraders\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"notional\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceLimitX96\",\"type\":\"uint160\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initiateFullyCollateralisedFixedTakerSwap\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"liquidator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"lowerTickLiquidator\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"upperTickLiquidator\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"liquidatePosition\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"struct IPeriphery.MintOrBurnParams\",\"name\":\"params\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"contract IMarginEngine\",\"name\":\"marginEngine\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"notional\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isMint\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"marginDelta\",\"type\":\"int256\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"mintOrBurnViaPeriphery\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"positionMarginRequirement\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mintViaAMM\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"positionMarginRequirement\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"peripheryAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"positionHistory\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"currentTimestampWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"termStartTimestampWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"termEndTimestampWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"margin\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"marginRequirement\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"estimatedSettlementCashflow\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenBalance\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"variableTokenBalance\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"positionSwapsHistory\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"reserveNormalizedIncomeAtSwap\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"swapInitiationTimestampWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"termEndTimestampWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"notional\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isFT\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"fixedRateWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"feePaidInUnderlyingTokens\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"rateOracleAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_aaveLendingPool\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setAaveLendingPool\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_cToken\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setCToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_FCMAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setFCMAddress\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"intAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"allowIntegration\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setIntegrationApproval\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_MEAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setMEAddress\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"rate\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setNewRate\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_peripheryAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPeripheryAddress\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_rateOracleAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setRateOracleAddress\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_VAMMAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setVAMMAddress\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"settlePositionViaAMM\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"settleYBATrader\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"sizeAllPositions\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"sizeAllYBATraders\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"sizeOfPositionHistory\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"sizeOfPositionSwapsHistory\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct IVAMM.SwapParams\",\"name\":\"params\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"amountSpecified\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceLimitX96\",\"type\":\"uint160\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"swapViaAMM\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"_fixedTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_variableTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_cumulativeFeeIncurred\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_fixedTokenDeltaUnbalanced\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_marginRequirement\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"struct IPeriphery.SwapPeripheryParams\",\"name\":\"params\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"contract IMarginEngine\",\"name\":\"marginEngine\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isFT\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"notional\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceLimitX96\",\"type\":\"uint160\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"marginDelta\",\"type\":\"int256\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"swapViaPeriphery\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"_fixedTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_variableTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_cumulativeFeeIncurred\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_fixedTokenDeltaUnbalanced\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_marginRequirement\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"notionalToUnwind\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceLimitX96\",\"type\":\"uint160\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unwindFullyCollateralisedFixedTakerSwap\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"marginDelta\",\"type\":\"int256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updatePositionMarginViaAMM\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static E2ESETUP_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60806040526000600255600060055534801561001a57600080fd5b50612b578061002a6000396000f3fe60806040526004361061025b5760003560e01c80637a0a98dd11610144578063d1f323c8116100b6578063e9d337b81161007a578063e9d337b814610982578063f08af47e146109a2578063f0f1f570146109c2578063f2354390146109ef578063f32aaeb314610a02578063fc35a80a14610a3057600080fd5b8063d1f323c81461084e578063dc8886841461086e578063e203b492146108a4578063e343590614610942578063e9ae4bc81461096257600080fd5b8063a16c458e11610108578063a16c458e146106b3578063a54b6c88146106f0578063bac0764e14610710578063be6e9a2c1461076d578063c6e34d371461078d578063cdc867931461083857600080fd5b80637a0a98dd1461061d5780637e3ffefd1461064a57806383e345e714610660578063840047eb1461067357806398923f971461069357600080fd5b80633c0ce413116101dd5780634cb71222116101a15780634cb71222146105235780634cccf05f1461056057806351e4569a1461058057806369696dbf146105a057806369e527da146105c057806378d91872146105e057600080fd5b80633c0ce4131461043e5780633e668c881461047b5780633faf68fe1461049b57806340283698146104e3578063447877711461050357600080fd5b80631c893ab3116102245780631c893ab3146103575780631f27dbfc146103845780632d483bec146103c15780632df1bfb5146103e1578063381a0e8b1461041e57600080fd5b80623b80dc1461026057806302b3b4c41461029f5780630a485545146102bf5780631677450f146102fc5780631b5ac4b514610329575b600080fd5b34801561026c57600080fd5b5061029d61027b366004612327565b600a80546001600160a01b0319166001600160a01b0392909216919091179055565b005b3480156102ab57600080fd5b5061029d6102ba366004612356565b610a5d565b3480156102cb57600080fd5b50600a546102df906001600160a01b031681565b6040516001600160a01b0390911681526020015b60405180910390f35b34801561030857600080fd5b5061031c6103173660046123ce565b610dff565b6040516102f39190612413565b34801561033557600080fd5b506103496103443660046124a0565b610fa3565b6040519081526020016102f3565b34801561036357600080fd5b50610349610372366004612327565b60046020526000908152604090205481565b34801561039057600080fd5b5061029d61039f366004612327565b600b80546001600160a01b0319166001600160a01b0392909216919091179055565b3480156103cd57600080fd5b5061029d6103dc3660046124a0565b610fc5565b3480156103ed57600080fd5b5061029d6103fc366004612327565b600c80546001600160a01b0319166001600160a01b0392909216919091179055565b34801561042a57600080fd5b506103496104393660046124b9565b611219565b34801561044a57600080fd5b5061029d610459366004612327565b600d80546001600160a01b0319166001600160a01b0392909216919091179055565b34801561048757600080fd5b50600d546102df906001600160a01b031681565b3480156104a757600080fd5b506104bb6104b636600461256a565b611308565b604080519586526020860194909452928401919091526060830152608082015260a0016102f3565b3480156104ef57600080fd5b5061029d6104fe366004612613565b6114ce565b34801561050f57600080fd5b5061029d61051e366004612327565b611526565b34801561052f57600080fd5b5061029d61053e366004612327565b601080546001600160a01b0319166001600160a01b0392909216919091179055565b34801561056c57600080fd5b5061029d61057b366004612651565b61159e565b34801561058c57600080fd5b5061029d61059b3660046123ce565b611783565b3480156105ac57600080fd5b5061029d6105bb3660046126b2565b6117d9565b3480156105cc57600080fd5b506010546102df906001600160a01b031681565b3480156105ec57600080fd5b5061029d6105fb366004612327565b600e80546001600160a01b0319166001600160a01b0392909216919091179055565b34801561062957600080fd5b506103496106383660046124a0565b60016020526000908152604090205481565b34801561065657600080fd5b5061034960055481565b61034961066e3660046126f4565b611a58565b34801561067f57600080fd5b5061029d61068e366004612327565b611b33565b34801561069f57600080fd5b5061029d6106ae3660046127bc565b611b9e565b3480156106bf57600080fd5b5061029d6106ce366004612327565b600f80546001600160a01b0319166001600160a01b0392909216919091179055565b3480156106fc57600080fd5b5061029d61070b3660046123ce565b611c73565b34801561071c57600080fd5b5061075e61072b3660046124a0565b6000602081905290815260409020546001600160a01b03811690600160a01b8104600290810b91600160b81b9004900b83565b6040516102f393929190612809565b34801561077957600080fd5b50600e546102df906001600160a01b031681565b34801561079957600080fd5b506107fd6107a836600461282e565b6006602052816000526040600020602052806000526040600020600091509150508060000154908060010154908060020154908060030154908060040154908060050154908060060154908060070154905088565b604080519889526020890197909752958701949094526060860192909252608085015260a084015260c083015260e0820152610100016102f3565b34801561084457600080fd5b5061034960025481565b34801561085a57600080fd5b50600c546102df906001600160a01b031681565b34801561087a57600080fd5b506102df6108893660046124a0565b6003602052600090815260409020546001600160a01b031681565b3480156108b057600080fd5b5061090b6108bf36600461282e565b60086020908152600092835260408084209091529082529020805460018201546002830154600384015460048501546005860154600690960154949593949293919260ff909116919087565b6040805197885260208801969096529486019390935260608501919091521515608084015260a083015260c082015260e0016102f3565b34801561094e57600080fd5b5061029d61095d3660046124b9565b611d5f565b34801561096e57600080fd5b5061029d61097d3660046126b2565b611df0565b34801561098e57600080fd5b50600f546102df906001600160a01b031681565b3480156109ae57600080fd5b50600b546102df906001600160a01b031681565b3480156109ce57600080fd5b506103496109dd3660046124a0565b60076020526000908152604090205481565b6104bb6109fd366004612850565b611f35565b348015610a0e57600080fd5b50610a22610a1d3660046123ce565b612035565b6040516102f3929190612900565b348015610a3c57600080fd5b50610349610a4b3660046124a0565b60096020526000908152604090205481565b6040516314a96d9160e31b8152309063a54b6c8890610a8490899089908990600401612809565b600060405180830381600087803b158015610a9e57600080fd5b505af1158015610ab2573d6000803e3d6000fd5b50506040516314a96d9160e31b815230925063a54b6c889150610add90869086908690600401612809565b600060405180830381600087803b158015610af757600080fd5b505af1158015610b0b573d6000803e3d6000fd5b505050506000600a60009054906101000a90046001600160a01b03166001600160a01b0316632495a5996040518163ffffffff1660e01b815260040160206040518083038186803b158015610b5f57600080fd5b505afa158015610b73573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610b97919061298b565b6040516370a0823160e01b81526001600160a01b03898116600483015291909116906370a082319060240160206040518083038186803b158015610bda57600080fd5b505afa158015610bee573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c1291906129a8565b600a5460405163cacdd72360e01b81526001600160a01b039182166004820152600286810b602483015285900b6044820152868216606482015291925088169063cacdd72390608401600060405180830381600087803b158015610c7557600080fd5b505af1158015610c89573d6000803e3d6000fd5b505050506000600a60009054906101000a90046001600160a01b03166001600160a01b0316632495a5996040518163ffffffff1660e01b815260040160206040518083038186803b158015610cdd57600080fd5b505afa158015610cf1573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610d15919061298b565b6040516370a0823160e01b81526001600160a01b038a8116600483015291909116906370a082319060240160206040518083038186803b158015610d5857600080fd5b505afa158015610d6c573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610d9091906129a8565b905080821115610df55760405162461bcd60e51b815260206004820152602560248201527f6c69717569646174696f6e207265776172642073686f756c6420626520706f73604482015264697469766560d81b60648201526084015b60405180910390fd5b5050505050505050565b60606000848484604051602001610e18939291906129c1565b60408051601f1981840301815291815281516020928301206000818152600790935290822054909250908167ffffffffffffffff811115610e5b57610e5b61251d565b604051908082528060200260200182016040528015610ed457816020015b610ec160405180610100016040528060008152602001600081526020016000815260200160008152602001600081526020016000815260200160008152602001600081525090565b815260200190600190039081610e795790505b50905060005b82811015610f9657600084815260066020526040812090610efc836001612a05565b81526020019081526020016000206040518061010001604052908160008201548152602001600182015481526020016002820154815260200160038201548152602001600482015481526020016005820154815260200160068201548152602001600782015481525050828281518110610f7857610f78612a1d565b60200260200101819052508080610f8e90612a33565b915050610eda565b50925050505b9392505050565b600080821215610fbc57610fb682612a4e565b92915050565b5090565b919050565b600d546040805163045fecad60e31b815290516000926001600160a01b0316916322ff6568916004808301926020929190829003018186803b15801561100a57600080fd5b505afa15801561101e573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906110429190612a6b565b90506000600d60009054906101000a90046001600160a01b03166001600160a01b0316636f307dc36040518163ffffffff1660e01b815260040160206040518083038186803b15801561109457600080fd5b505afa1580156110a8573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906110cc919061298b565b90508160ff166001141561114157600f54604051631157b80360e21b81526001600160a01b038381166004830152602482018690529091169063455ee00c90604401600060405180830381600087803b15801561112857600080fd5b505af115801561113c573d6000803e3d6000fd5b505050505b8160ff16600214156111ac57601054604051636d83470760e11b8152600481018590526001600160a01b039091169063db068e0e90602401600060405180830381600087803b15801561119357600080fd5b505af11580156111a7573d6000803e3d6000fd5b505050505b600d60009054906101000a90046001600160a01b03166001600160a01b0316637aa4db136040518163ffffffff1660e01b8152600401600060405180830381600087803b1580156111fc57600080fd5b505af1158015611210573d6000803e3d6000fd5b50505050505050565b6040516314a96d9160e31b8152600090309063a54b6c889061124390889088908890600401612809565b600060405180830381600087803b15801561125d57600080fd5b505af1158015611271573d6000803e3d6000fd5b5050600b546040516322a1d8ed60e21b81526001600160a01b03808a169450638a8763b493506112ad9216908990899089908990600401612a8e565b602060405180830381600087803b1580156112c757600080fd5b505af11580156112db573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906112ff91906129a8565b95945050505050565b8051606082015160808301516040516314a96d9160e31b81526000938493849384938493309363a54b6c889361134093600401612809565b600060405180830381600087803b15801561135a57600080fd5b505af115801561136e573d6000803e3d6000fd5b50508751600b546040805163022769c960e11b81526001600160a01b0392831660048201528b518316602482015260208c01516044820152908b01518216606482015260608b0151600290810b608483015260808c0151900b60a48201529116925063044ed392915060c40160a060405180830381600087803b1580156113f457600080fd5b505af1158015611408573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061142c9190612acb565b8a5160608c015160808d0151604051634cccf05f60e01b81526001600160a01b039093166004840152600291820b6024840152900b6044820152606481018390526084810185905260a48101849052949950929750909550935091503090634cccf05f9060c401600060405180830381600087803b1580156114ad57600080fd5b505af11580156114c1573d6000803e3d6000fd5b5050505091939590929450565b600a5460405163080506d360e31b81526001600160a01b03918216600482015283821660248201528215156044820152908416906340283698906064015b600060405180830381600087803b1580156111fc57600080fd5b6001600160a01b038116600090815260046020526040902054156115475750565b60016005600082825461155a9190612a05565b909155505060058054600090815260036020908152604080832080546001600160a01b039096166001600160a01b0319909616861790559254938252600490522055565b60008686866040516020016115b5939291906129c1565b60408051601f198184030181528282528051602091820120600a546324fb6d1560e21b855292519094506000936001600160a01b03909316926393edb454926004808301939192829003018186803b15801561161057600080fd5b505afa158015611624573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061164891906129a8565b9050600061167e61167461166361165e88610fa3565b6121cf565b61166f61165e8a610fa3565b61221c565b61166f60646121cf565b905060006001905060006040518060e001604052808381526020016116a1612231565b81526020018581526020016116b58a610fa3565b815260200160008a136116c95760016116cc565b60005b1515815260200184815260200187815250905060016009600087815260200190815260200160002060008282546117039190612a05565b90915550506000948552600860209081526040808720600983528188205488528252958690208251815590820151600182015594810151600286015560608101516003860155608081015160048601805460ff191691151591909117905560a0810151600586015560c00151600690940193909355505050505050505050565b61178e838383611c73565b600a5460405163303958f360e01b81526001600160a01b03918216600482015290841660248201819052600284810b604484015283900b60648301529063303958f39060840161150c565b6117e283611526565b600a5460408051632495a59960e01b815290516000926001600160a01b031691632495a599916004808301926020929190829003018186803b15801561182757600080fd5b505afa15801561183b573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061185f919061298b565b600a546040516370a0823160e01b81526001600160a01b0391821660048201529116906370a082319060240160206040518083038186803b1580156118a357600080fd5b505afa1580156118b7573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906118db91906129a8565b600c546040516369696dbf60e01b81526001600160a01b0391821660048201526024810186905284821660448201529192508516906369696dbf906064015b600060405180830381600087803b15801561193457600080fd5b505af1158015611948573d6000803e3d6000fd5b505050506000600a60009054906101000a90046001600160a01b03166001600160a01b0316632495a5996040518163ffffffff1660e01b815260040160206040518083038186803b15801561199c57600080fd5b505afa1580156119b0573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906119d4919061298b565b600a546040516370a0823160e01b81526001600160a01b0391821660048201529116906370a082319060240160206040518083038186803b158015611a1857600080fd5b505afa158015611a2c573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611a5091906129a8565b505050505050565b6000611a6d8383602001518460400151611c73565b600e54604080516383e345e760e01b81526001600160a01b0392831660048201528451831660248201526020850151600290810b60448301529185015190910b6064820152606084015160848201526080840151151560a482015260a084015160c4820152908416906383e345e790349060e4016020604051808303818588803b158015611afa57600080fd5b505af1158015611b0e573d6000803e3d6000fd5b50505050506040513d601f19601f82011682018060405250810190610f9c91906129a8565b611b3c81611526565b600c5460405163840047eb60e01b81526001600160a01b0391821660048201529082169063840047eb90602401600060405180830381600087803b158015611b8357600080fd5b505af1158015611b97573d6000803e3d6000fd5b5050505050565b6040516314a96d9160e31b8152309063a54b6c8890611bc590879087908790600401612809565b600060405180830381600087803b158015611bdf57600080fd5b505af1158015611bf3573d6000803e3d6000fd5b5050600a546040516318defb9f60e11b81526001600160a01b03918216600482015290871660248201819052600287810b604484015286900b60648301526084820185905292506331bdf73e915060a4015b600060405180830381600087803b158015611c5f57600080fd5b505af1158015610df5573d6000803e3d6000fd5b6000838383604051602001611c8a939291906129c1565b60408051601f1981840301815291815281516020928301206000818152600190935291205490915015611cbd5750505050565b600160026000828254611cd09190612a05565b9091555050604080516060810182526001600160a01b039586168152600294850b602080830191825294860b82840190815286546000908152808752848120935184549351925199166001600160b81b031990931692909217600160a01b62ffffff928316021762ffffff60b81b1916600160b81b919098160296909617905592549084526001909152912055565b6040516314a96d9160e31b8152309063a54b6c8890611d8690879087908790600401612809565b600060405180830381600087803b158015611da057600080fd5b505af1158015611db4573d6000803e3d6000fd5b5050600b5460405163292a60d560e01b81526001600160a01b03808916945063292a60d59350611c459216908890889088908890600401612a8e565b611df983611526565b600a5460408051632495a59960e01b815290516000926001600160a01b031691632495a599916004808301926020929190829003018186803b158015611e3e57600080fd5b505afa158015611e52573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611e76919061298b565b600a546040516370a0823160e01b81526001600160a01b0391821660048201529116906370a082319060240160206040518083038186803b158015611eba57600080fd5b505afa158015611ece573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611ef291906129a8565b600c54604051631d35c97960e31b81526001600160a01b03918216600482015260248101869052848216604482015291925085169063e9ae4bc89060640161191a565b6000806000806000611f508787608001518860a00151611c73565b600e5460408051630f23543960e41b81526001600160a01b0392831660048201528851831660248201526020890151151560448201529088015160648201526060880151821660848201526080880151600290810b60a483015260a0890151900b60c482015260c088015160e48201529088169063f23543909034906101040160a0604051808303818588803b158015611fe957600080fd5b505af1158015611ffd573d6000803e3d6000fd5b50505050506040513d601f19601f820116820180604052508101906120229190612acb565b939b929a50909850965090945092505050565b606060008085858560405160200161204f939291906129c1565b60408051601f1981840301815291815281516020928301206000818152600990935290822054909250908167ffffffffffffffff8111156120925761209261251d565b60405190808252806020026020018201604052801561210557816020015b6120f26040518060e001604052806000815260200160008152602001600081526020016000815260200160001515815260200160008152602001600081525090565b8152602001906001900390816120b05790505b50905060005b828110156121c25760008481526008602052604081209061212d836001612a05565b81526020808201929092526040908101600020815160e0810183528154815260018201549381019390935260028101549183019190915260038101546060830152600481015460ff1615156080830152600581015460a08301526006015460c082015282518390839081106121a4576121a4612a1d565b602002602001018190525080806121ba90612a33565b91505061210b565b5097909650945050505050565b60007812725dd1d243aba0e75fe645cc4873f9e65afe688c928e1f2182111561220e57604051633492ffd960e01b815260048101839052602401610dec565b50670de0b6b3a76400000290565b6000610f9c83670de0b6b3a764000084612241565b600061223c426121cf565b905090565b60008080600019858709858702925082811083820303915050806000141561227c5783828161227257612272612b0b565b0492505050610f9c565b8381106122a657604051631dcf306360e21b81526004810182905260248101859052604401610dec565b60008486880960026001871981018816978890046003810283188082028403028082028403028082028403028082028403028082028403029081029092039091026000889003889004909101858311909403939093029303949094049190911702949350505050565b6001600160a01b038116811461232457600080fd5b50565b60006020828403121561233957600080fd5b8135610f9c8161230f565b8035600281900b8114610fc057600080fd5b60008060008060008060c0878903121561236f57600080fd5b863561237a8161230f565b955061238860208801612344565b945061239660408801612344565b935060608701356123a68161230f565b92506123b460808801612344565b91506123c260a08801612344565b90509295509295509295565b6000806000606084860312156123e357600080fd5b83356123ee8161230f565b92506123fc60208501612344565b915061240a60408501612344565b90509250925092565b602080825282518282018190526000919060409081850190868401855b828110156124935781518051855286810151878601528581015186860152606080820151908601526080808201519086015260a0808201519086015260c0808201519086015260e090810151908501526101009093019290850190600101612430565b5091979650505050505050565b6000602082840312156124b257600080fd5b5035919050565b600080600080608085870312156124cf57600080fd5b84356124da8161230f565b93506124e860208601612344565b92506124f660408601612344565b915060608501356001600160801b038116811461251257600080fd5b939692955090935050565b634e487b7160e01b600052604160045260246000fd5b60405160e0810167ffffffffffffffff8111828210171561256457634e487b7160e01b600052604160045260246000fd5b60405290565b600060a0828403121561257c57600080fd5b60405160a0810181811067ffffffffffffffff821117156125ad57634e487b7160e01b600052604160045260246000fd5b60405282356125bb8161230f565b81526020838101359082015260408301356125d58161230f565b60408201526125e660608401612344565b60608201526125f760808401612344565b60808201529392505050565b80358015158114610fc057600080fd5b60008060006060848603121561262857600080fd5b83356126338161230f565b925060208401356126438161230f565b915061240a60408501612603565b60008060008060008060c0878903121561266a57600080fd5b86356126758161230f565b955061268360208801612344565b945061269160408801612344565b9350606087013592506080870135915060a087013590509295509295509295565b6000806000606084860312156126c757600080fd5b83356126d28161230f565b92506020840135915060408401356126e98161230f565b809150509250925092565b60008082840360e081121561270857600080fd5b83356127138161230f565b925060c0601f198201121561272757600080fd5b5060405160c0810181811067ffffffffffffffff8211171561275957634e487b7160e01b600052604160045260246000fd5b604052602084013561276a8161230f565b815261277860408501612344565b602082015261278960608501612344565b6040820152608084013560608201526127a460a08501612603565b608082015260c0939093013560a08401525092909150565b600080600080608085870312156127d257600080fd5b84356127dd8161230f565b93506127eb60208601612344565b92506127f960408601612344565b9396929550929360600135925050565b6001600160a01b03939093168352600291820b6020840152900b604082015260600190565b6000806040838503121561284157600080fd5b50508035926020909101359150565b60008082840361010081121561286557600080fd5b83356128708161230f565b925060e0601f198201121561288457600080fd5b5061288d612533565b602084013561289b8161230f565b81526128a960408501612603565b60208201526060840135604082015260808401356128c68161230f565b60608201526128d760a08501612344565b60808201526128e860c08501612344565b60a082015260e0939093013560c08401525092909150565b6040808252835182820181905260009190606090818501906020808901865b838110156129775781518051865283810151848701528781015188870152868101518787015260808082015115159087015260a0808201519087015260c0908101519086015260e0909401939082019060010161291f565b505095909501959095525092949350505050565b60006020828403121561299d57600080fd5b8151610f9c8161230f565b6000602082840312156129ba57600080fd5b5051919050565b60609390931b6bffffffffffffffffffffffff1916835260e891821b6014840152901b6017820152601a0190565b634e487b7160e01b600052601160045260246000fd5b60008219821115612a1857612a186129ef565b500190565b634e487b7160e01b600052603260045260246000fd5b6000600019821415612a4757612a476129ef565b5060010190565b6000600160ff1b821415612a6457612a646129ef565b5060000390565b600060208284031215612a7d57600080fd5b815160ff81168114610f9c57600080fd5b6001600160a01b039586168152939094166020840152600291820b6040840152900b60608201526001600160801b03909116608082015260a00190565b600080600080600060a08688031215612ae357600080fd5b5050835160208501516040860151606087015160809097015192989197509594509092509050565b634e487b7160e01b600052601260045260246000fdfea26469706673582212205778cae145cc21cae4bde8f1525b1d686b96f0f4ab011621dd8780a9b3bd0bcc64736f6c63430008090033" . parse () . expect ("invalid bytecode")
        });
    pub struct E2ESetup<M>(ethers::contract::Contract<M>);
    impl<M> Clone for E2ESetup<M> {
        fn clone(&self) -> Self {
            E2ESetup(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for E2ESetup<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for E2ESetup<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(E2ESetup))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> E2ESetup<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), E2ESETUP_ABI.clone(), client).into()
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
                E2ESETUP_ABI.clone(),
                E2ESETUP_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `FCMAddress` (0xd1f323c8) function"]
        pub fn fcm_address(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([209, 243, 35, 200], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MEAddress` (0x0a485545) function"]
        pub fn me_address(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([10, 72, 85, 69], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `VAMMAddress` (0xf08af47e) function"]
        pub fn vamm_address(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([240, 138, 244, 126], ())
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
        #[doc = "Calls the contract's `abs` (0x1b5ac4b5) function"]
        pub fn abs(
            &self,
            value: I256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([27, 90, 196, 181], value)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addPosition` (0xa54b6c88) function"]
        pub fn add_position(
            &self,
            owner: ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([165, 75, 108, 136], (owner, tick_lower, tick_upper))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addSwapSnapshot` (0x4cccf05f) function"]
        pub fn add_swap_snapshot(
            &self,
            owner: ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
            variable_token_delta: I256,
            fixed_token_delta_unbalanced: I256,
            cumulative_fee_incurred: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [76, 204, 240, 95],
                    (
                        owner,
                        tick_lower,
                        tick_upper,
                        variable_token_delta,
                        fixed_token_delta_unbalanced,
                        cumulative_fee_incurred,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addYBATrader` (0x44787771) function"]
        pub fn add_yba_trader(
            &self,
            trader: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([68, 120, 119, 113], trader)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allPositions` (0xbac0764e) function"]
        pub fn all_positions(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, (ethers::core::types::Address, i32, i32)>
        {
            self.0
                .method_hash([186, 192, 118, 78], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allYBATraders` (0xdc888684) function"]
        pub fn all_yba_traders(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([220, 136, 134, 132], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `burnViaAMM` (0xe3435906) function"]
        pub fn burn_via_amm(
            &self,
            recipient: ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
            amount: u128,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [227, 67, 89, 6],
                    (recipient, tick_lower, tick_upper, amount),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `cToken` (0x69e527da) function"]
        pub fn c_token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([105, 229, 39, 218], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPositionHistory` (0x1677450f) function"]
        pub fn get_position_history(
            &self,
            owner: ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<PositionSnapshot>>
        {
            self.0
                .method_hash([22, 119, 69, 15], (owner, tick_lower, tick_upper))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPositionSwapsHistory` (0xf32aaeb3) function"]
        pub fn get_position_swaps_history(
            &self,
            owner: ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (::std::vec::Vec<SwapSnapshot>, ethers::core::types::U256),
        > {
            self.0
                .method_hash([243, 42, 174, 179], (owner, tick_lower, tick_upper))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `indexAllPositions` (0x7a0a98dd) function"]
        pub fn index_all_positions(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([122, 10, 152, 221], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `indexAllYBATraders` (0x1c893ab3) function"]
        pub fn index_all_yba_traders(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([28, 137, 58, 179], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initiateFullyCollateralisedFixedTakerSwap` (0x69696dbf) function"]
        pub fn initiate_fully_collateralised_fixed_taker_swap(
            &self,
            trader: ethers::core::types::Address,
            notional: ethers::core::types::U256,
            sqrt_price_limit_x96: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [105, 105, 109, 191],
                    (trader, notional, sqrt_price_limit_x96),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liquidatePosition` (0x02b3b4c4) function"]
        pub fn liquidate_position(
            &self,
            liquidator: ethers::core::types::Address,
            lower_tick_liquidator: i32,
            upper_tick_liquidator: i32,
            owner: ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [2, 179, 180, 196],
                    (
                        liquidator,
                        lower_tick_liquidator,
                        upper_tick_liquidator,
                        owner,
                        tick_lower,
                        tick_upper,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mintOrBurnViaPeriphery` (0x83e345e7) function"]
        pub fn mint_or_burn_via_periphery(
            &self,
            trader: ethers::core::types::Address,
            params: MintOrBurnParams,
        ) -> ethers::contract::builders::ContractCall<M, I256> {
            self.0
                .method_hash([131, 227, 69, 231], (trader, params))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mintViaAMM` (0x381a0e8b) function"]
        pub fn mint_via_amm(
            &self,
            recipient: ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
            amount: u128,
        ) -> ethers::contract::builders::ContractCall<M, I256> {
            self.0
                .method_hash(
                    [56, 26, 14, 139],
                    (recipient, tick_lower, tick_upper, amount),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `peripheryAddress` (0xbe6e9a2c) function"]
        pub fn periphery_address(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([190, 110, 154, 44], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `positionHistory` (0xc6e34d37) function"]
        pub fn position_history(
            &self,
            p0: [u8; 32],
            p1: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                I256,
                ethers::core::types::U256,
                I256,
                I256,
                I256,
            ),
        > {
            self.0
                .method_hash([198, 227, 77, 55], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `positionSwapsHistory` (0xe203b492) function"]
        pub fn position_swaps_history(
            &self,
            p0: [u8; 32],
            p1: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                bool,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([226, 3, 180, 146], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rateOracleAddress` (0x3e668c88) function"]
        pub fn rate_oracle_address(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([62, 102, 140, 136], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setAaveLendingPool` (0xa16c458e) function"]
        pub fn set_aave_lending_pool(
            &self,
            aave_lending_pool: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([161, 108, 69, 142], aave_lending_pool)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setCToken` (0x4cb71222) function"]
        pub fn set_c_token(
            &self,
            c_token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([76, 183, 18, 34], c_token)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setFCMAddress` (0x2df1bfb5) function"]
        pub fn set_fcm_address(
            &self,
            fcm_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([45, 241, 191, 181], fcm_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setIntegrationApproval` (0x40283698) function"]
        pub fn set_integration_approval(
            &self,
            recipient: ethers::core::types::Address,
            int_address: ethers::core::types::Address,
            allow_integration: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [64, 40, 54, 152],
                    (recipient, int_address, allow_integration),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setMEAddress` (0x003b80dc) function"]
        pub fn set_me_address(
            &self,
            me_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([0, 59, 128, 220], me_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setNewRate` (0x2d483bec) function"]
        pub fn set_new_rate(
            &self,
            rate: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([45, 72, 59, 236], rate)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPeripheryAddress` (0x78d91872) function"]
        pub fn set_periphery_address(
            &self,
            periphery_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([120, 217, 24, 114], periphery_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setRateOracleAddress` (0x3c0ce413) function"]
        pub fn set_rate_oracle_address(
            &self,
            rate_oracle_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 12, 228, 19], rate_oracle_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setVAMMAddress` (0x1f27dbfc) function"]
        pub fn set_vamm_address(
            &self,
            vamm_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([31, 39, 219, 252], vamm_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `settlePositionViaAMM` (0x51e4569a) function"]
        pub fn settle_position_via_amm(
            &self,
            recipient: ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([81, 228, 86, 154], (recipient, tick_lower, tick_upper))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `settleYBATrader` (0x840047eb) function"]
        pub fn settle_yba_trader(
            &self,
            trader: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 0, 71, 235], trader)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sizeAllPositions` (0xcdc86793) function"]
        pub fn size_all_positions(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([205, 200, 103, 147], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sizeAllYBATraders` (0x7e3ffefd) function"]
        pub fn size_all_yba_traders(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([126, 63, 254, 253], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sizeOfPositionHistory` (0xf0f1f570) function"]
        pub fn size_of_position_history(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([240, 241, 245, 112], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sizeOfPositionSwapsHistory` (0xfc35a80a) function"]
        pub fn size_of_position_swaps_history(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([252, 53, 168, 10], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapViaAMM` (0x3faf68fe) function"]
        pub fn swap_via_amm(
            &self,
            params: SwapParams,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (I256, I256, ethers::core::types::U256, I256, I256),
        > {
            self.0
                .method_hash([63, 175, 104, 254], (params,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapViaPeriphery` (0xf2354390) function"]
        pub fn swap_via_periphery(
            &self,
            trader: ethers::core::types::Address,
            params: SwapPeripheryParams,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (I256, I256, ethers::core::types::U256, I256, I256),
        > {
            self.0
                .method_hash([242, 53, 67, 144], (trader, params))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unwindFullyCollateralisedFixedTakerSwap` (0xe9ae4bc8) function"]
        pub fn unwind_fully_collateralised_fixed_taker_swap(
            &self,
            trader: ethers::core::types::Address,
            notional_to_unwind: ethers::core::types::U256,
            sqrt_price_limit_x96: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [233, 174, 75, 200],
                    (trader, notional_to_unwind, sqrt_price_limit_x96),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updatePositionMarginViaAMM` (0x98923f97) function"]
        pub fn update_position_margin_via_amm(
            &self,
            owner: ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
            margin_delta: I256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [152, 146, 63, 151],
                    (owner, tick_lower, tick_upper, margin_delta),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for E2ESetup<M> {
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
    pub enum E2ESetupErrors {
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
        PRBMathUD60x18__FromUintOverflow(PRBMathUD60x18__FromUintOverflow),
        PRBMath__MulDivOverflow(PRBMath__MulDivOverflow),
        PositionNetZero(PositionNetZero),
        PositionNotSettled(PositionNotSettled),
        RocketPoolGetEthValueReturnedZero(RocketPoolGetEthValueReturnedZero),
        WithdrawalExceedsCurrentMargin(WithdrawalExceedsCurrentMargin),
        closeToOrBeyondMaturity(closeToOrBeyondMaturity),
    }
    impl ethers::core::abi::AbiDecode for E2ESetupErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok (decoded) = < AavePoolGetReserveNormalizedIncomeReturnedZero as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (E2ESetupErrors :: AavePoolGetReserveNormalizedIncomeReturnedZero (decoded)) }
            if let Ok (decoded) = < AavePoolGetReserveNormalizedVariableDebtReturnedZero as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (E2ESetupErrors :: AavePoolGetReserveNormalizedVariableDebtReturnedZero (decoded)) }
            if let Ok(decoded) =
                <CTokenExchangeRateReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(E2ESetupErrors::CTokenExchangeRateReturnedZero(decoded));
            }
            if let Ok(decoded) =
                <CanOnlyTradeIfUnlocked as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupErrors::CanOnlyTradeIfUnlocked(decoded));
            }
            if let Ok(decoded) =
                <CannotLiquidate as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupErrors::CannotLiquidate(decoded));
            }
            if let Ok(decoded) =
                <CannotSettleBeforeMaturity as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupErrors::CannotSettleBeforeMaturity(decoded));
            }
            if let Ok(decoded) = <DebugError as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupErrors::DebugError(decoded));
            }
            if let Ok(decoded) =
                <ExpectedOppositeSigns as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupErrors::ExpectedOppositeSigns(decoded));
            }
            if let Ok(decoded) =
                <ExpectedSqrtPriceZeroBeforeInit as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(E2ESetupErrors::ExpectedSqrtPriceZeroBeforeInit(decoded));
            }
            if let Ok(decoded) =
                <IRSNotionalAmountSpecifiedMustBeNonZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(E2ESetupErrors::IRSNotionalAmountSpecifiedMustBeNonZero(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <InvalidMarginDelta as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupErrors::InvalidMarginDelta(decoded));
            }
            if let Ok(decoded) =
                <LidoGetPooledEthBySharesReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(E2ESetupErrors::LidoGetPooledEthBySharesReturnedZero(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <LiquidityDeltaMustBePositiveInBurn as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(E2ESetupErrors::LiquidityDeltaMustBePositiveInBurn(decoded));
            }
            if let Ok(decoded) =
                <LiquidityDeltaMustBePositiveInMint as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(E2ESetupErrors::LiquidityDeltaMustBePositiveInMint(decoded));
            }
            if let Ok(decoded) =
                <MarginLessThanMinimum as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupErrors::MarginLessThanMinimum(decoded));
            }
            if let Ok(decoded) =
                <MarginRequirementNotMet as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupErrors::MarginRequirementNotMet(decoded));
            }
            if let Ok(decoded) =
                <MarginRequirementNotMetFCM as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupErrors::MarginRequirementNotMetFCM(decoded));
            }
            if let Ok(decoded) =
                <NotEnoughFunds as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupErrors::NotEnoughFunds(decoded));
            }
            if let Ok(decoded) = <OOO as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(E2ESetupErrors::OOO(decoded));
            }
            if let Ok(decoded) = <OnlyFCM as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(E2ESetupErrors::OnlyFCM(decoded));
            }
            if let Ok(decoded) =
                <OnlyMarginEngine as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupErrors::OnlyMarginEngine(decoded));
            }
            if let Ok(decoded) =
                <OnlyOwnerCanUpdatePosition as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupErrors::OnlyOwnerCanUpdatePosition(decoded));
            }
            if let Ok(decoded) = <OnlyVAMM as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(E2ESetupErrors::OnlyVAMM(decoded));
            }
            if let Ok(decoded) =
                <PRBMathUD60x18__FromUintOverflow as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(E2ESetupErrors::PRBMathUD60x18__FromUintOverflow(decoded));
            }
            if let Ok(decoded) =
                <PRBMath__MulDivOverflow as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupErrors::PRBMath__MulDivOverflow(decoded));
            }
            if let Ok(decoded) =
                <PositionNetZero as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupErrors::PositionNetZero(decoded));
            }
            if let Ok(decoded) =
                <PositionNotSettled as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupErrors::PositionNotSettled(decoded));
            }
            if let Ok(decoded) =
                <RocketPoolGetEthValueReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(E2ESetupErrors::RocketPoolGetEthValueReturnedZero(decoded));
            }
            if let Ok(decoded) =
                <WithdrawalExceedsCurrentMargin as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(E2ESetupErrors::WithdrawalExceedsCurrentMargin(decoded));
            }
            if let Ok(decoded) =
                <closeToOrBeyondMaturity as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupErrors::closeToOrBeyondMaturity(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for E2ESetupErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                E2ESetupErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(element) => {
                    element.encode()
                }
                E2ESetupErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(element) => {
                    element.encode()
                }
                E2ESetupErrors::CTokenExchangeRateReturnedZero(element) => element.encode(),
                E2ESetupErrors::CanOnlyTradeIfUnlocked(element) => element.encode(),
                E2ESetupErrors::CannotLiquidate(element) => element.encode(),
                E2ESetupErrors::CannotSettleBeforeMaturity(element) => element.encode(),
                E2ESetupErrors::DebugError(element) => element.encode(),
                E2ESetupErrors::ExpectedOppositeSigns(element) => element.encode(),
                E2ESetupErrors::ExpectedSqrtPriceZeroBeforeInit(element) => element.encode(),
                E2ESetupErrors::IRSNotionalAmountSpecifiedMustBeNonZero(element) => {
                    element.encode()
                }
                E2ESetupErrors::InvalidMarginDelta(element) => element.encode(),
                E2ESetupErrors::LidoGetPooledEthBySharesReturnedZero(element) => element.encode(),
                E2ESetupErrors::LiquidityDeltaMustBePositiveInBurn(element) => element.encode(),
                E2ESetupErrors::LiquidityDeltaMustBePositiveInMint(element) => element.encode(),
                E2ESetupErrors::MarginLessThanMinimum(element) => element.encode(),
                E2ESetupErrors::MarginRequirementNotMet(element) => element.encode(),
                E2ESetupErrors::MarginRequirementNotMetFCM(element) => element.encode(),
                E2ESetupErrors::NotEnoughFunds(element) => element.encode(),
                E2ESetupErrors::OOO(element) => element.encode(),
                E2ESetupErrors::OnlyFCM(element) => element.encode(),
                E2ESetupErrors::OnlyMarginEngine(element) => element.encode(),
                E2ESetupErrors::OnlyOwnerCanUpdatePosition(element) => element.encode(),
                E2ESetupErrors::OnlyVAMM(element) => element.encode(),
                E2ESetupErrors::PRBMathUD60x18__FromUintOverflow(element) => element.encode(),
                E2ESetupErrors::PRBMath__MulDivOverflow(element) => element.encode(),
                E2ESetupErrors::PositionNetZero(element) => element.encode(),
                E2ESetupErrors::PositionNotSettled(element) => element.encode(),
                E2ESetupErrors::RocketPoolGetEthValueReturnedZero(element) => element.encode(),
                E2ESetupErrors::WithdrawalExceedsCurrentMargin(element) => element.encode(),
                E2ESetupErrors::closeToOrBeyondMaturity(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for E2ESetupErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                E2ESetupErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(element) => {
                    element.fmt(f)
                }
                E2ESetupErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(element) => {
                    element.fmt(f)
                }
                E2ESetupErrors::CTokenExchangeRateReturnedZero(element) => element.fmt(f),
                E2ESetupErrors::CanOnlyTradeIfUnlocked(element) => element.fmt(f),
                E2ESetupErrors::CannotLiquidate(element) => element.fmt(f),
                E2ESetupErrors::CannotSettleBeforeMaturity(element) => element.fmt(f),
                E2ESetupErrors::DebugError(element) => element.fmt(f),
                E2ESetupErrors::ExpectedOppositeSigns(element) => element.fmt(f),
                E2ESetupErrors::ExpectedSqrtPriceZeroBeforeInit(element) => element.fmt(f),
                E2ESetupErrors::IRSNotionalAmountSpecifiedMustBeNonZero(element) => element.fmt(f),
                E2ESetupErrors::InvalidMarginDelta(element) => element.fmt(f),
                E2ESetupErrors::LidoGetPooledEthBySharesReturnedZero(element) => element.fmt(f),
                E2ESetupErrors::LiquidityDeltaMustBePositiveInBurn(element) => element.fmt(f),
                E2ESetupErrors::LiquidityDeltaMustBePositiveInMint(element) => element.fmt(f),
                E2ESetupErrors::MarginLessThanMinimum(element) => element.fmt(f),
                E2ESetupErrors::MarginRequirementNotMet(element) => element.fmt(f),
                E2ESetupErrors::MarginRequirementNotMetFCM(element) => element.fmt(f),
                E2ESetupErrors::NotEnoughFunds(element) => element.fmt(f),
                E2ESetupErrors::OOO(element) => element.fmt(f),
                E2ESetupErrors::OnlyFCM(element) => element.fmt(f),
                E2ESetupErrors::OnlyMarginEngine(element) => element.fmt(f),
                E2ESetupErrors::OnlyOwnerCanUpdatePosition(element) => element.fmt(f),
                E2ESetupErrors::OnlyVAMM(element) => element.fmt(f),
                E2ESetupErrors::PRBMathUD60x18__FromUintOverflow(element) => element.fmt(f),
                E2ESetupErrors::PRBMath__MulDivOverflow(element) => element.fmt(f),
                E2ESetupErrors::PositionNetZero(element) => element.fmt(f),
                E2ESetupErrors::PositionNotSettled(element) => element.fmt(f),
                E2ESetupErrors::RocketPoolGetEthValueReturnedZero(element) => element.fmt(f),
                E2ESetupErrors::WithdrawalExceedsCurrentMargin(element) => element.fmt(f),
                E2ESetupErrors::closeToOrBeyondMaturity(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AavePoolGetReserveNormalizedIncomeReturnedZero> for E2ESetupErrors {
        fn from(var: AavePoolGetReserveNormalizedIncomeReturnedZero) -> Self {
            E2ESetupErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(var)
        }
    }
    impl ::std::convert::From<AavePoolGetReserveNormalizedVariableDebtReturnedZero> for E2ESetupErrors {
        fn from(var: AavePoolGetReserveNormalizedVariableDebtReturnedZero) -> Self {
            E2ESetupErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(var)
        }
    }
    impl ::std::convert::From<CTokenExchangeRateReturnedZero> for E2ESetupErrors {
        fn from(var: CTokenExchangeRateReturnedZero) -> Self {
            E2ESetupErrors::CTokenExchangeRateReturnedZero(var)
        }
    }
    impl ::std::convert::From<CanOnlyTradeIfUnlocked> for E2ESetupErrors {
        fn from(var: CanOnlyTradeIfUnlocked) -> Self {
            E2ESetupErrors::CanOnlyTradeIfUnlocked(var)
        }
    }
    impl ::std::convert::From<CannotLiquidate> for E2ESetupErrors {
        fn from(var: CannotLiquidate) -> Self {
            E2ESetupErrors::CannotLiquidate(var)
        }
    }
    impl ::std::convert::From<CannotSettleBeforeMaturity> for E2ESetupErrors {
        fn from(var: CannotSettleBeforeMaturity) -> Self {
            E2ESetupErrors::CannotSettleBeforeMaturity(var)
        }
    }
    impl ::std::convert::From<DebugError> for E2ESetupErrors {
        fn from(var: DebugError) -> Self {
            E2ESetupErrors::DebugError(var)
        }
    }
    impl ::std::convert::From<ExpectedOppositeSigns> for E2ESetupErrors {
        fn from(var: ExpectedOppositeSigns) -> Self {
            E2ESetupErrors::ExpectedOppositeSigns(var)
        }
    }
    impl ::std::convert::From<ExpectedSqrtPriceZeroBeforeInit> for E2ESetupErrors {
        fn from(var: ExpectedSqrtPriceZeroBeforeInit) -> Self {
            E2ESetupErrors::ExpectedSqrtPriceZeroBeforeInit(var)
        }
    }
    impl ::std::convert::From<IRSNotionalAmountSpecifiedMustBeNonZero> for E2ESetupErrors {
        fn from(var: IRSNotionalAmountSpecifiedMustBeNonZero) -> Self {
            E2ESetupErrors::IRSNotionalAmountSpecifiedMustBeNonZero(var)
        }
    }
    impl ::std::convert::From<InvalidMarginDelta> for E2ESetupErrors {
        fn from(var: InvalidMarginDelta) -> Self {
            E2ESetupErrors::InvalidMarginDelta(var)
        }
    }
    impl ::std::convert::From<LidoGetPooledEthBySharesReturnedZero> for E2ESetupErrors {
        fn from(var: LidoGetPooledEthBySharesReturnedZero) -> Self {
            E2ESetupErrors::LidoGetPooledEthBySharesReturnedZero(var)
        }
    }
    impl ::std::convert::From<LiquidityDeltaMustBePositiveInBurn> for E2ESetupErrors {
        fn from(var: LiquidityDeltaMustBePositiveInBurn) -> Self {
            E2ESetupErrors::LiquidityDeltaMustBePositiveInBurn(var)
        }
    }
    impl ::std::convert::From<LiquidityDeltaMustBePositiveInMint> for E2ESetupErrors {
        fn from(var: LiquidityDeltaMustBePositiveInMint) -> Self {
            E2ESetupErrors::LiquidityDeltaMustBePositiveInMint(var)
        }
    }
    impl ::std::convert::From<MarginLessThanMinimum> for E2ESetupErrors {
        fn from(var: MarginLessThanMinimum) -> Self {
            E2ESetupErrors::MarginLessThanMinimum(var)
        }
    }
    impl ::std::convert::From<MarginRequirementNotMet> for E2ESetupErrors {
        fn from(var: MarginRequirementNotMet) -> Self {
            E2ESetupErrors::MarginRequirementNotMet(var)
        }
    }
    impl ::std::convert::From<MarginRequirementNotMetFCM> for E2ESetupErrors {
        fn from(var: MarginRequirementNotMetFCM) -> Self {
            E2ESetupErrors::MarginRequirementNotMetFCM(var)
        }
    }
    impl ::std::convert::From<NotEnoughFunds> for E2ESetupErrors {
        fn from(var: NotEnoughFunds) -> Self {
            E2ESetupErrors::NotEnoughFunds(var)
        }
    }
    impl ::std::convert::From<OOO> for E2ESetupErrors {
        fn from(var: OOO) -> Self {
            E2ESetupErrors::OOO(var)
        }
    }
    impl ::std::convert::From<OnlyFCM> for E2ESetupErrors {
        fn from(var: OnlyFCM) -> Self {
            E2ESetupErrors::OnlyFCM(var)
        }
    }
    impl ::std::convert::From<OnlyMarginEngine> for E2ESetupErrors {
        fn from(var: OnlyMarginEngine) -> Self {
            E2ESetupErrors::OnlyMarginEngine(var)
        }
    }
    impl ::std::convert::From<OnlyOwnerCanUpdatePosition> for E2ESetupErrors {
        fn from(var: OnlyOwnerCanUpdatePosition) -> Self {
            E2ESetupErrors::OnlyOwnerCanUpdatePosition(var)
        }
    }
    impl ::std::convert::From<OnlyVAMM> for E2ESetupErrors {
        fn from(var: OnlyVAMM) -> Self {
            E2ESetupErrors::OnlyVAMM(var)
        }
    }
    impl ::std::convert::From<PRBMathUD60x18__FromUintOverflow> for E2ESetupErrors {
        fn from(var: PRBMathUD60x18__FromUintOverflow) -> Self {
            E2ESetupErrors::PRBMathUD60x18__FromUintOverflow(var)
        }
    }
    impl ::std::convert::From<PRBMath__MulDivOverflow> for E2ESetupErrors {
        fn from(var: PRBMath__MulDivOverflow) -> Self {
            E2ESetupErrors::PRBMath__MulDivOverflow(var)
        }
    }
    impl ::std::convert::From<PositionNetZero> for E2ESetupErrors {
        fn from(var: PositionNetZero) -> Self {
            E2ESetupErrors::PositionNetZero(var)
        }
    }
    impl ::std::convert::From<PositionNotSettled> for E2ESetupErrors {
        fn from(var: PositionNotSettled) -> Self {
            E2ESetupErrors::PositionNotSettled(var)
        }
    }
    impl ::std::convert::From<RocketPoolGetEthValueReturnedZero> for E2ESetupErrors {
        fn from(var: RocketPoolGetEthValueReturnedZero) -> Self {
            E2ESetupErrors::RocketPoolGetEthValueReturnedZero(var)
        }
    }
    impl ::std::convert::From<WithdrawalExceedsCurrentMargin> for E2ESetupErrors {
        fn from(var: WithdrawalExceedsCurrentMargin) -> Self {
            E2ESetupErrors::WithdrawalExceedsCurrentMargin(var)
        }
    }
    impl ::std::convert::From<closeToOrBeyondMaturity> for E2ESetupErrors {
        fn from(var: closeToOrBeyondMaturity) -> Self {
            E2ESetupErrors::closeToOrBeyondMaturity(var)
        }
    }
    #[doc = "Container type for all input parameters for the `FCMAddress` function with signature `FCMAddress()` and selector `[209, 243, 35, 200]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "FCMAddress", abi = "FCMAddress()")]
    pub struct FcmaddressCall;
    #[doc = "Container type for all input parameters for the `MEAddress` function with signature `MEAddress()` and selector `[10, 72, 85, 69]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "MEAddress", abi = "MEAddress()")]
    pub struct MeaddressCall;
    #[doc = "Container type for all input parameters for the `VAMMAddress` function with signature `VAMMAddress()` and selector `[240, 138, 244, 126]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "VAMMAddress", abi = "VAMMAddress()")]
    pub struct VammaddressCall;
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
    #[doc = "Container type for all input parameters for the `abs` function with signature `abs(int256)` and selector `[27, 90, 196, 181]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "abs", abi = "abs(int256)")]
    pub struct AbsCall {
        pub value: I256,
    }
    #[doc = "Container type for all input parameters for the `addPosition` function with signature `addPosition(address,int24,int24)` and selector `[165, 75, 108, 136]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "addPosition", abi = "addPosition(address,int24,int24)")]
    pub struct AddPositionCall {
        pub owner: ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
    }
    #[doc = "Container type for all input parameters for the `addSwapSnapshot` function with signature `addSwapSnapshot(address,int24,int24,int256,int256,uint256)` and selector `[76, 204, 240, 95]`"]
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
        name = "addSwapSnapshot",
        abi = "addSwapSnapshot(address,int24,int24,int256,int256,uint256)"
    )]
    pub struct AddSwapSnapshotCall {
        pub owner: ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub variable_token_delta: I256,
        pub fixed_token_delta_unbalanced: I256,
        pub cumulative_fee_incurred: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `addYBATrader` function with signature `addYBATrader(address)` and selector `[68, 120, 119, 113]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "addYBATrader", abi = "addYBATrader(address)")]
    pub struct AddYBATraderCall {
        pub trader: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `allPositions` function with signature `allPositions(uint256)` and selector `[186, 192, 118, 78]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "allPositions", abi = "allPositions(uint256)")]
    pub struct AllPositionsCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `allYBATraders` function with signature `allYBATraders(uint256)` and selector `[220, 136, 134, 132]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "allYBATraders", abi = "allYBATraders(uint256)")]
    pub struct AllYBATradersCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `burnViaAMM` function with signature `burnViaAMM(address,int24,int24,uint128)` and selector `[227, 67, 89, 6]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "burnViaAMM", abi = "burnViaAMM(address,int24,int24,uint128)")]
    pub struct BurnViaAMMCall {
        pub recipient: ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub amount: u128,
    }
    #[doc = "Container type for all input parameters for the `cToken` function with signature `cToken()` and selector `[105, 229, 39, 218]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "cToken", abi = "cToken()")]
    pub struct CtokenCall;
    #[doc = "Container type for all input parameters for the `getPositionHistory` function with signature `getPositionHistory(address,int24,int24)` and selector `[22, 119, 69, 15]`"]
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
        name = "getPositionHistory",
        abi = "getPositionHistory(address,int24,int24)"
    )]
    pub struct GetPositionHistoryCall {
        pub owner: ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
    }
    #[doc = "Container type for all input parameters for the `getPositionSwapsHistory` function with signature `getPositionSwapsHistory(address,int24,int24)` and selector `[243, 42, 174, 179]`"]
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
        name = "getPositionSwapsHistory",
        abi = "getPositionSwapsHistory(address,int24,int24)"
    )]
    pub struct GetPositionSwapsHistoryCall {
        pub owner: ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
    }
    #[doc = "Container type for all input parameters for the `indexAllPositions` function with signature `indexAllPositions(bytes32)` and selector `[122, 10, 152, 221]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "indexAllPositions", abi = "indexAllPositions(bytes32)")]
    pub struct IndexAllPositionsCall(pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `indexAllYBATraders` function with signature `indexAllYBATraders(address)` and selector `[28, 137, 58, 179]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "indexAllYBATraders", abi = "indexAllYBATraders(address)")]
    pub struct IndexAllYBATradersCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `initiateFullyCollateralisedFixedTakerSwap` function with signature `initiateFullyCollateralisedFixedTakerSwap(address,uint256,uint160)` and selector `[105, 105, 109, 191]`"]
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
        name = "initiateFullyCollateralisedFixedTakerSwap",
        abi = "initiateFullyCollateralisedFixedTakerSwap(address,uint256,uint160)"
    )]
    pub struct InitiateFullyCollateralisedFixedTakerSwapCall {
        pub trader: ethers::core::types::Address,
        pub notional: ethers::core::types::U256,
        pub sqrt_price_limit_x96: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `liquidatePosition` function with signature `liquidatePosition(address,int24,int24,address,int24,int24)` and selector `[2, 179, 180, 196]`"]
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
        abi = "liquidatePosition(address,int24,int24,address,int24,int24)"
    )]
    pub struct LiquidatePositionCall {
        pub liquidator: ethers::core::types::Address,
        pub lower_tick_liquidator: i32,
        pub upper_tick_liquidator: i32,
        pub owner: ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
    }
    #[doc = "Container type for all input parameters for the `mintOrBurnViaPeriphery` function with signature `mintOrBurnViaPeriphery(address,(address,int24,int24,uint256,bool,int256))` and selector `[131, 227, 69, 231]`"]
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
        name = "mintOrBurnViaPeriphery",
        abi = "mintOrBurnViaPeriphery(address,(address,int24,int24,uint256,bool,int256))"
    )]
    pub struct MintOrBurnViaPeripheryCall {
        pub trader: ethers::core::types::Address,
        pub params: MintOrBurnParams,
    }
    #[doc = "Container type for all input parameters for the `mintViaAMM` function with signature `mintViaAMM(address,int24,int24,uint128)` and selector `[56, 26, 14, 139]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "mintViaAMM", abi = "mintViaAMM(address,int24,int24,uint128)")]
    pub struct MintViaAMMCall {
        pub recipient: ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub amount: u128,
    }
    #[doc = "Container type for all input parameters for the `peripheryAddress` function with signature `peripheryAddress()` and selector `[190, 110, 154, 44]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "peripheryAddress", abi = "peripheryAddress()")]
    pub struct PeripheryAddressCall;
    #[doc = "Container type for all input parameters for the `positionHistory` function with signature `positionHistory(bytes32,uint256)` and selector `[198, 227, 77, 55]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "positionHistory", abi = "positionHistory(bytes32,uint256)")]
    pub struct PositionHistoryCall(pub [u8; 32], pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `positionSwapsHistory` function with signature `positionSwapsHistory(bytes32,uint256)` and selector `[226, 3, 180, 146]`"]
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
        name = "positionSwapsHistory",
        abi = "positionSwapsHistory(bytes32,uint256)"
    )]
    pub struct PositionSwapsHistoryCall(pub [u8; 32], pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `rateOracleAddress` function with signature `rateOracleAddress()` and selector `[62, 102, 140, 136]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "rateOracleAddress", abi = "rateOracleAddress()")]
    pub struct RateOracleAddressCall;
    #[doc = "Container type for all input parameters for the `setAaveLendingPool` function with signature `setAaveLendingPool(address)` and selector `[161, 108, 69, 142]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setAaveLendingPool", abi = "setAaveLendingPool(address)")]
    pub struct SetAaveLendingPoolCall {
        pub aave_lending_pool: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setCToken` function with signature `setCToken(address)` and selector `[76, 183, 18, 34]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setCToken", abi = "setCToken(address)")]
    pub struct SetCTokenCall {
        pub c_token: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setFCMAddress` function with signature `setFCMAddress(address)` and selector `[45, 241, 191, 181]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setFCMAddress", abi = "setFCMAddress(address)")]
    pub struct SetFCMAddressCall {
        pub fcm_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setIntegrationApproval` function with signature `setIntegrationApproval(address,address,bool)` and selector `[64, 40, 54, 152]`"]
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
        name = "setIntegrationApproval",
        abi = "setIntegrationApproval(address,address,bool)"
    )]
    pub struct SetIntegrationApprovalCall {
        pub recipient: ethers::core::types::Address,
        pub int_address: ethers::core::types::Address,
        pub allow_integration: bool,
    }
    #[doc = "Container type for all input parameters for the `setMEAddress` function with signature `setMEAddress(address)` and selector `[0, 59, 128, 220]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setMEAddress", abi = "setMEAddress(address)")]
    pub struct SetMEAddressCall {
        pub me_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setNewRate` function with signature `setNewRate(uint256)` and selector `[45, 72, 59, 236]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setNewRate", abi = "setNewRate(uint256)")]
    pub struct SetNewRateCall {
        pub rate: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setPeripheryAddress` function with signature `setPeripheryAddress(address)` and selector `[120, 217, 24, 114]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setPeripheryAddress", abi = "setPeripheryAddress(address)")]
    pub struct SetPeripheryAddressCall {
        pub periphery_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setRateOracleAddress` function with signature `setRateOracleAddress(address)` and selector `[60, 12, 228, 19]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setRateOracleAddress", abi = "setRateOracleAddress(address)")]
    pub struct SetRateOracleAddressCall {
        pub rate_oracle_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setVAMMAddress` function with signature `setVAMMAddress(address)` and selector `[31, 39, 219, 252]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setVAMMAddress", abi = "setVAMMAddress(address)")]
    pub struct SetVAMMAddressCall {
        pub vamm_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `settlePositionViaAMM` function with signature `settlePositionViaAMM(address,int24,int24)` and selector `[81, 228, 86, 154]`"]
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
        name = "settlePositionViaAMM",
        abi = "settlePositionViaAMM(address,int24,int24)"
    )]
    pub struct SettlePositionViaAMMCall {
        pub recipient: ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
    }
    #[doc = "Container type for all input parameters for the `settleYBATrader` function with signature `settleYBATrader(address)` and selector `[132, 0, 71, 235]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "settleYBATrader", abi = "settleYBATrader(address)")]
    pub struct SettleYBATraderCall {
        pub trader: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `sizeAllPositions` function with signature `sizeAllPositions()` and selector `[205, 200, 103, 147]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "sizeAllPositions", abi = "sizeAllPositions()")]
    pub struct SizeAllPositionsCall;
    #[doc = "Container type for all input parameters for the `sizeAllYBATraders` function with signature `sizeAllYBATraders()` and selector `[126, 63, 254, 253]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "sizeAllYBATraders", abi = "sizeAllYBATraders()")]
    pub struct SizeAllYBATradersCall;
    #[doc = "Container type for all input parameters for the `sizeOfPositionHistory` function with signature `sizeOfPositionHistory(bytes32)` and selector `[240, 241, 245, 112]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "sizeOfPositionHistory", abi = "sizeOfPositionHistory(bytes32)")]
    pub struct SizeOfPositionHistoryCall(pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `sizeOfPositionSwapsHistory` function with signature `sizeOfPositionSwapsHistory(bytes32)` and selector `[252, 53, 168, 10]`"]
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
        name = "sizeOfPositionSwapsHistory",
        abi = "sizeOfPositionSwapsHistory(bytes32)"
    )]
    pub struct SizeOfPositionSwapsHistoryCall(pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `swapViaAMM` function with signature `swapViaAMM((address,int256,uint160,int24,int24))` and selector `[63, 175, 104, 254]`"]
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
        name = "swapViaAMM",
        abi = "swapViaAMM((address,int256,uint160,int24,int24))"
    )]
    pub struct SwapViaAMMCall {
        pub params: SwapParams,
    }
    #[doc = "Container type for all input parameters for the `swapViaPeriphery` function with signature `swapViaPeriphery(address,(address,bool,uint256,uint160,int24,int24,int256))` and selector `[242, 53, 67, 144]`"]
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
        name = "swapViaPeriphery",
        abi = "swapViaPeriphery(address,(address,bool,uint256,uint160,int24,int24,int256))"
    )]
    pub struct SwapViaPeripheryCall {
        pub trader: ethers::core::types::Address,
        pub params: SwapPeripheryParams,
    }
    #[doc = "Container type for all input parameters for the `unwindFullyCollateralisedFixedTakerSwap` function with signature `unwindFullyCollateralisedFixedTakerSwap(address,uint256,uint160)` and selector `[233, 174, 75, 200]`"]
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
        name = "unwindFullyCollateralisedFixedTakerSwap",
        abi = "unwindFullyCollateralisedFixedTakerSwap(address,uint256,uint160)"
    )]
    pub struct UnwindFullyCollateralisedFixedTakerSwapCall {
        pub trader: ethers::core::types::Address,
        pub notional_to_unwind: ethers::core::types::U256,
        pub sqrt_price_limit_x96: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `updatePositionMarginViaAMM` function with signature `updatePositionMarginViaAMM(address,int24,int24,int256)` and selector `[152, 146, 63, 151]`"]
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
        name = "updatePositionMarginViaAMM",
        abi = "updatePositionMarginViaAMM(address,int24,int24,int256)"
    )]
    pub struct UpdatePositionMarginViaAMMCall {
        pub owner: ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub margin_delta: I256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum E2ESetupCalls {
        Fcmaddress(FcmaddressCall),
        Meaddress(MeaddressCall),
        Vammaddress(VammaddressCall),
        AaveLendingPool(AaveLendingPoolCall),
        Abs(AbsCall),
        AddPosition(AddPositionCall),
        AddSwapSnapshot(AddSwapSnapshotCall),
        AddYBATrader(AddYBATraderCall),
        AllPositions(AllPositionsCall),
        AllYBATraders(AllYBATradersCall),
        BurnViaAMM(BurnViaAMMCall),
        Ctoken(CtokenCall),
        GetPositionHistory(GetPositionHistoryCall),
        GetPositionSwapsHistory(GetPositionSwapsHistoryCall),
        IndexAllPositions(IndexAllPositionsCall),
        IndexAllYBATraders(IndexAllYBATradersCall),
        InitiateFullyCollateralisedFixedTakerSwap(InitiateFullyCollateralisedFixedTakerSwapCall),
        LiquidatePosition(LiquidatePositionCall),
        MintOrBurnViaPeriphery(MintOrBurnViaPeripheryCall),
        MintViaAMM(MintViaAMMCall),
        PeripheryAddress(PeripheryAddressCall),
        PositionHistory(PositionHistoryCall),
        PositionSwapsHistory(PositionSwapsHistoryCall),
        RateOracleAddress(RateOracleAddressCall),
        SetAaveLendingPool(SetAaveLendingPoolCall),
        SetCToken(SetCTokenCall),
        SetFCMAddress(SetFCMAddressCall),
        SetIntegrationApproval(SetIntegrationApprovalCall),
        SetMEAddress(SetMEAddressCall),
        SetNewRate(SetNewRateCall),
        SetPeripheryAddress(SetPeripheryAddressCall),
        SetRateOracleAddress(SetRateOracleAddressCall),
        SetVAMMAddress(SetVAMMAddressCall),
        SettlePositionViaAMM(SettlePositionViaAMMCall),
        SettleYBATrader(SettleYBATraderCall),
        SizeAllPositions(SizeAllPositionsCall),
        SizeAllYBATraders(SizeAllYBATradersCall),
        SizeOfPositionHistory(SizeOfPositionHistoryCall),
        SizeOfPositionSwapsHistory(SizeOfPositionSwapsHistoryCall),
        SwapViaAMM(SwapViaAMMCall),
        SwapViaPeriphery(SwapViaPeripheryCall),
        UnwindFullyCollateralisedFixedTakerSwap(UnwindFullyCollateralisedFixedTakerSwapCall),
        UpdatePositionMarginViaAMM(UpdatePositionMarginViaAMMCall),
    }
    impl ethers::core::abi::AbiDecode for E2ESetupCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <FcmaddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupCalls::Fcmaddress(decoded));
            }
            if let Ok(decoded) =
                <MeaddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupCalls::Meaddress(decoded));
            }
            if let Ok(decoded) =
                <VammaddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupCalls::Vammaddress(decoded));
            }
            if let Ok(decoded) =
                <AaveLendingPoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupCalls::AaveLendingPool(decoded));
            }
            if let Ok(decoded) = <AbsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(E2ESetupCalls::Abs(decoded));
            }
            if let Ok(decoded) =
                <AddPositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupCalls::AddPosition(decoded));
            }
            if let Ok(decoded) =
                <AddSwapSnapshotCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupCalls::AddSwapSnapshot(decoded));
            }
            if let Ok(decoded) =
                <AddYBATraderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupCalls::AddYBATrader(decoded));
            }
            if let Ok(decoded) =
                <AllPositionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupCalls::AllPositions(decoded));
            }
            if let Ok(decoded) =
                <AllYBATradersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupCalls::AllYBATraders(decoded));
            }
            if let Ok(decoded) =
                <BurnViaAMMCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupCalls::BurnViaAMM(decoded));
            }
            if let Ok(decoded) = <CtokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupCalls::Ctoken(decoded));
            }
            if let Ok(decoded) =
                <GetPositionHistoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupCalls::GetPositionHistory(decoded));
            }
            if let Ok(decoded) =
                <GetPositionSwapsHistoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupCalls::GetPositionSwapsHistory(decoded));
            }
            if let Ok(decoded) =
                <IndexAllPositionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupCalls::IndexAllPositions(decoded));
            }
            if let Ok(decoded) =
                <IndexAllYBATradersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupCalls::IndexAllYBATraders(decoded));
            }
            if let Ok (decoded) = < InitiateFullyCollateralisedFixedTakerSwapCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (E2ESetupCalls :: InitiateFullyCollateralisedFixedTakerSwap (decoded)) }
            if let Ok(decoded) =
                <LiquidatePositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupCalls::LiquidatePosition(decoded));
            }
            if let Ok(decoded) =
                <MintOrBurnViaPeripheryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupCalls::MintOrBurnViaPeriphery(decoded));
            }
            if let Ok(decoded) =
                <MintViaAMMCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupCalls::MintViaAMM(decoded));
            }
            if let Ok(decoded) =
                <PeripheryAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupCalls::PeripheryAddress(decoded));
            }
            if let Ok(decoded) =
                <PositionHistoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupCalls::PositionHistory(decoded));
            }
            if let Ok(decoded) =
                <PositionSwapsHistoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupCalls::PositionSwapsHistory(decoded));
            }
            if let Ok(decoded) =
                <RateOracleAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupCalls::RateOracleAddress(decoded));
            }
            if let Ok(decoded) =
                <SetAaveLendingPoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupCalls::SetAaveLendingPool(decoded));
            }
            if let Ok(decoded) =
                <SetCTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupCalls::SetCToken(decoded));
            }
            if let Ok(decoded) =
                <SetFCMAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupCalls::SetFCMAddress(decoded));
            }
            if let Ok(decoded) =
                <SetIntegrationApprovalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupCalls::SetIntegrationApproval(decoded));
            }
            if let Ok(decoded) =
                <SetMEAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupCalls::SetMEAddress(decoded));
            }
            if let Ok(decoded) =
                <SetNewRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupCalls::SetNewRate(decoded));
            }
            if let Ok(decoded) =
                <SetPeripheryAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupCalls::SetPeripheryAddress(decoded));
            }
            if let Ok(decoded) =
                <SetRateOracleAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupCalls::SetRateOracleAddress(decoded));
            }
            if let Ok(decoded) =
                <SetVAMMAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupCalls::SetVAMMAddress(decoded));
            }
            if let Ok(decoded) =
                <SettlePositionViaAMMCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupCalls::SettlePositionViaAMM(decoded));
            }
            if let Ok(decoded) =
                <SettleYBATraderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupCalls::SettleYBATrader(decoded));
            }
            if let Ok(decoded) =
                <SizeAllPositionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupCalls::SizeAllPositions(decoded));
            }
            if let Ok(decoded) =
                <SizeAllYBATradersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupCalls::SizeAllYBATraders(decoded));
            }
            if let Ok(decoded) =
                <SizeOfPositionHistoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupCalls::SizeOfPositionHistory(decoded));
            }
            if let Ok(decoded) =
                <SizeOfPositionSwapsHistoryCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(E2ESetupCalls::SizeOfPositionSwapsHistory(decoded));
            }
            if let Ok(decoded) =
                <SwapViaAMMCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupCalls::SwapViaAMM(decoded));
            }
            if let Ok(decoded) =
                <SwapViaPeripheryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(E2ESetupCalls::SwapViaPeriphery(decoded));
            }
            if let Ok (decoded) = < UnwindFullyCollateralisedFixedTakerSwapCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (E2ESetupCalls :: UnwindFullyCollateralisedFixedTakerSwap (decoded)) }
            if let Ok(decoded) =
                <UpdatePositionMarginViaAMMCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(E2ESetupCalls::UpdatePositionMarginViaAMM(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for E2ESetupCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                E2ESetupCalls::Fcmaddress(element) => element.encode(),
                E2ESetupCalls::Meaddress(element) => element.encode(),
                E2ESetupCalls::Vammaddress(element) => element.encode(),
                E2ESetupCalls::AaveLendingPool(element) => element.encode(),
                E2ESetupCalls::Abs(element) => element.encode(),
                E2ESetupCalls::AddPosition(element) => element.encode(),
                E2ESetupCalls::AddSwapSnapshot(element) => element.encode(),
                E2ESetupCalls::AddYBATrader(element) => element.encode(),
                E2ESetupCalls::AllPositions(element) => element.encode(),
                E2ESetupCalls::AllYBATraders(element) => element.encode(),
                E2ESetupCalls::BurnViaAMM(element) => element.encode(),
                E2ESetupCalls::Ctoken(element) => element.encode(),
                E2ESetupCalls::GetPositionHistory(element) => element.encode(),
                E2ESetupCalls::GetPositionSwapsHistory(element) => element.encode(),
                E2ESetupCalls::IndexAllPositions(element) => element.encode(),
                E2ESetupCalls::IndexAllYBATraders(element) => element.encode(),
                E2ESetupCalls::InitiateFullyCollateralisedFixedTakerSwap(element) => {
                    element.encode()
                }
                E2ESetupCalls::LiquidatePosition(element) => element.encode(),
                E2ESetupCalls::MintOrBurnViaPeriphery(element) => element.encode(),
                E2ESetupCalls::MintViaAMM(element) => element.encode(),
                E2ESetupCalls::PeripheryAddress(element) => element.encode(),
                E2ESetupCalls::PositionHistory(element) => element.encode(),
                E2ESetupCalls::PositionSwapsHistory(element) => element.encode(),
                E2ESetupCalls::RateOracleAddress(element) => element.encode(),
                E2ESetupCalls::SetAaveLendingPool(element) => element.encode(),
                E2ESetupCalls::SetCToken(element) => element.encode(),
                E2ESetupCalls::SetFCMAddress(element) => element.encode(),
                E2ESetupCalls::SetIntegrationApproval(element) => element.encode(),
                E2ESetupCalls::SetMEAddress(element) => element.encode(),
                E2ESetupCalls::SetNewRate(element) => element.encode(),
                E2ESetupCalls::SetPeripheryAddress(element) => element.encode(),
                E2ESetupCalls::SetRateOracleAddress(element) => element.encode(),
                E2ESetupCalls::SetVAMMAddress(element) => element.encode(),
                E2ESetupCalls::SettlePositionViaAMM(element) => element.encode(),
                E2ESetupCalls::SettleYBATrader(element) => element.encode(),
                E2ESetupCalls::SizeAllPositions(element) => element.encode(),
                E2ESetupCalls::SizeAllYBATraders(element) => element.encode(),
                E2ESetupCalls::SizeOfPositionHistory(element) => element.encode(),
                E2ESetupCalls::SizeOfPositionSwapsHistory(element) => element.encode(),
                E2ESetupCalls::SwapViaAMM(element) => element.encode(),
                E2ESetupCalls::SwapViaPeriphery(element) => element.encode(),
                E2ESetupCalls::UnwindFullyCollateralisedFixedTakerSwap(element) => element.encode(),
                E2ESetupCalls::UpdatePositionMarginViaAMM(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for E2ESetupCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                E2ESetupCalls::Fcmaddress(element) => element.fmt(f),
                E2ESetupCalls::Meaddress(element) => element.fmt(f),
                E2ESetupCalls::Vammaddress(element) => element.fmt(f),
                E2ESetupCalls::AaveLendingPool(element) => element.fmt(f),
                E2ESetupCalls::Abs(element) => element.fmt(f),
                E2ESetupCalls::AddPosition(element) => element.fmt(f),
                E2ESetupCalls::AddSwapSnapshot(element) => element.fmt(f),
                E2ESetupCalls::AddYBATrader(element) => element.fmt(f),
                E2ESetupCalls::AllPositions(element) => element.fmt(f),
                E2ESetupCalls::AllYBATraders(element) => element.fmt(f),
                E2ESetupCalls::BurnViaAMM(element) => element.fmt(f),
                E2ESetupCalls::Ctoken(element) => element.fmt(f),
                E2ESetupCalls::GetPositionHistory(element) => element.fmt(f),
                E2ESetupCalls::GetPositionSwapsHistory(element) => element.fmt(f),
                E2ESetupCalls::IndexAllPositions(element) => element.fmt(f),
                E2ESetupCalls::IndexAllYBATraders(element) => element.fmt(f),
                E2ESetupCalls::InitiateFullyCollateralisedFixedTakerSwap(element) => element.fmt(f),
                E2ESetupCalls::LiquidatePosition(element) => element.fmt(f),
                E2ESetupCalls::MintOrBurnViaPeriphery(element) => element.fmt(f),
                E2ESetupCalls::MintViaAMM(element) => element.fmt(f),
                E2ESetupCalls::PeripheryAddress(element) => element.fmt(f),
                E2ESetupCalls::PositionHistory(element) => element.fmt(f),
                E2ESetupCalls::PositionSwapsHistory(element) => element.fmt(f),
                E2ESetupCalls::RateOracleAddress(element) => element.fmt(f),
                E2ESetupCalls::SetAaveLendingPool(element) => element.fmt(f),
                E2ESetupCalls::SetCToken(element) => element.fmt(f),
                E2ESetupCalls::SetFCMAddress(element) => element.fmt(f),
                E2ESetupCalls::SetIntegrationApproval(element) => element.fmt(f),
                E2ESetupCalls::SetMEAddress(element) => element.fmt(f),
                E2ESetupCalls::SetNewRate(element) => element.fmt(f),
                E2ESetupCalls::SetPeripheryAddress(element) => element.fmt(f),
                E2ESetupCalls::SetRateOracleAddress(element) => element.fmt(f),
                E2ESetupCalls::SetVAMMAddress(element) => element.fmt(f),
                E2ESetupCalls::SettlePositionViaAMM(element) => element.fmt(f),
                E2ESetupCalls::SettleYBATrader(element) => element.fmt(f),
                E2ESetupCalls::SizeAllPositions(element) => element.fmt(f),
                E2ESetupCalls::SizeAllYBATraders(element) => element.fmt(f),
                E2ESetupCalls::SizeOfPositionHistory(element) => element.fmt(f),
                E2ESetupCalls::SizeOfPositionSwapsHistory(element) => element.fmt(f),
                E2ESetupCalls::SwapViaAMM(element) => element.fmt(f),
                E2ESetupCalls::SwapViaPeriphery(element) => element.fmt(f),
                E2ESetupCalls::UnwindFullyCollateralisedFixedTakerSwap(element) => element.fmt(f),
                E2ESetupCalls::UpdatePositionMarginViaAMM(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<FcmaddressCall> for E2ESetupCalls {
        fn from(var: FcmaddressCall) -> Self {
            E2ESetupCalls::Fcmaddress(var)
        }
    }
    impl ::std::convert::From<MeaddressCall> for E2ESetupCalls {
        fn from(var: MeaddressCall) -> Self {
            E2ESetupCalls::Meaddress(var)
        }
    }
    impl ::std::convert::From<VammaddressCall> for E2ESetupCalls {
        fn from(var: VammaddressCall) -> Self {
            E2ESetupCalls::Vammaddress(var)
        }
    }
    impl ::std::convert::From<AaveLendingPoolCall> for E2ESetupCalls {
        fn from(var: AaveLendingPoolCall) -> Self {
            E2ESetupCalls::AaveLendingPool(var)
        }
    }
    impl ::std::convert::From<AbsCall> for E2ESetupCalls {
        fn from(var: AbsCall) -> Self {
            E2ESetupCalls::Abs(var)
        }
    }
    impl ::std::convert::From<AddPositionCall> for E2ESetupCalls {
        fn from(var: AddPositionCall) -> Self {
            E2ESetupCalls::AddPosition(var)
        }
    }
    impl ::std::convert::From<AddSwapSnapshotCall> for E2ESetupCalls {
        fn from(var: AddSwapSnapshotCall) -> Self {
            E2ESetupCalls::AddSwapSnapshot(var)
        }
    }
    impl ::std::convert::From<AddYBATraderCall> for E2ESetupCalls {
        fn from(var: AddYBATraderCall) -> Self {
            E2ESetupCalls::AddYBATrader(var)
        }
    }
    impl ::std::convert::From<AllPositionsCall> for E2ESetupCalls {
        fn from(var: AllPositionsCall) -> Self {
            E2ESetupCalls::AllPositions(var)
        }
    }
    impl ::std::convert::From<AllYBATradersCall> for E2ESetupCalls {
        fn from(var: AllYBATradersCall) -> Self {
            E2ESetupCalls::AllYBATraders(var)
        }
    }
    impl ::std::convert::From<BurnViaAMMCall> for E2ESetupCalls {
        fn from(var: BurnViaAMMCall) -> Self {
            E2ESetupCalls::BurnViaAMM(var)
        }
    }
    impl ::std::convert::From<CtokenCall> for E2ESetupCalls {
        fn from(var: CtokenCall) -> Self {
            E2ESetupCalls::Ctoken(var)
        }
    }
    impl ::std::convert::From<GetPositionHistoryCall> for E2ESetupCalls {
        fn from(var: GetPositionHistoryCall) -> Self {
            E2ESetupCalls::GetPositionHistory(var)
        }
    }
    impl ::std::convert::From<GetPositionSwapsHistoryCall> for E2ESetupCalls {
        fn from(var: GetPositionSwapsHistoryCall) -> Self {
            E2ESetupCalls::GetPositionSwapsHistory(var)
        }
    }
    impl ::std::convert::From<IndexAllPositionsCall> for E2ESetupCalls {
        fn from(var: IndexAllPositionsCall) -> Self {
            E2ESetupCalls::IndexAllPositions(var)
        }
    }
    impl ::std::convert::From<IndexAllYBATradersCall> for E2ESetupCalls {
        fn from(var: IndexAllYBATradersCall) -> Self {
            E2ESetupCalls::IndexAllYBATraders(var)
        }
    }
    impl ::std::convert::From<InitiateFullyCollateralisedFixedTakerSwapCall> for E2ESetupCalls {
        fn from(var: InitiateFullyCollateralisedFixedTakerSwapCall) -> Self {
            E2ESetupCalls::InitiateFullyCollateralisedFixedTakerSwap(var)
        }
    }
    impl ::std::convert::From<LiquidatePositionCall> for E2ESetupCalls {
        fn from(var: LiquidatePositionCall) -> Self {
            E2ESetupCalls::LiquidatePosition(var)
        }
    }
    impl ::std::convert::From<MintOrBurnViaPeripheryCall> for E2ESetupCalls {
        fn from(var: MintOrBurnViaPeripheryCall) -> Self {
            E2ESetupCalls::MintOrBurnViaPeriphery(var)
        }
    }
    impl ::std::convert::From<MintViaAMMCall> for E2ESetupCalls {
        fn from(var: MintViaAMMCall) -> Self {
            E2ESetupCalls::MintViaAMM(var)
        }
    }
    impl ::std::convert::From<PeripheryAddressCall> for E2ESetupCalls {
        fn from(var: PeripheryAddressCall) -> Self {
            E2ESetupCalls::PeripheryAddress(var)
        }
    }
    impl ::std::convert::From<PositionHistoryCall> for E2ESetupCalls {
        fn from(var: PositionHistoryCall) -> Self {
            E2ESetupCalls::PositionHistory(var)
        }
    }
    impl ::std::convert::From<PositionSwapsHistoryCall> for E2ESetupCalls {
        fn from(var: PositionSwapsHistoryCall) -> Self {
            E2ESetupCalls::PositionSwapsHistory(var)
        }
    }
    impl ::std::convert::From<RateOracleAddressCall> for E2ESetupCalls {
        fn from(var: RateOracleAddressCall) -> Self {
            E2ESetupCalls::RateOracleAddress(var)
        }
    }
    impl ::std::convert::From<SetAaveLendingPoolCall> for E2ESetupCalls {
        fn from(var: SetAaveLendingPoolCall) -> Self {
            E2ESetupCalls::SetAaveLendingPool(var)
        }
    }
    impl ::std::convert::From<SetCTokenCall> for E2ESetupCalls {
        fn from(var: SetCTokenCall) -> Self {
            E2ESetupCalls::SetCToken(var)
        }
    }
    impl ::std::convert::From<SetFCMAddressCall> for E2ESetupCalls {
        fn from(var: SetFCMAddressCall) -> Self {
            E2ESetupCalls::SetFCMAddress(var)
        }
    }
    impl ::std::convert::From<SetIntegrationApprovalCall> for E2ESetupCalls {
        fn from(var: SetIntegrationApprovalCall) -> Self {
            E2ESetupCalls::SetIntegrationApproval(var)
        }
    }
    impl ::std::convert::From<SetMEAddressCall> for E2ESetupCalls {
        fn from(var: SetMEAddressCall) -> Self {
            E2ESetupCalls::SetMEAddress(var)
        }
    }
    impl ::std::convert::From<SetNewRateCall> for E2ESetupCalls {
        fn from(var: SetNewRateCall) -> Self {
            E2ESetupCalls::SetNewRate(var)
        }
    }
    impl ::std::convert::From<SetPeripheryAddressCall> for E2ESetupCalls {
        fn from(var: SetPeripheryAddressCall) -> Self {
            E2ESetupCalls::SetPeripheryAddress(var)
        }
    }
    impl ::std::convert::From<SetRateOracleAddressCall> for E2ESetupCalls {
        fn from(var: SetRateOracleAddressCall) -> Self {
            E2ESetupCalls::SetRateOracleAddress(var)
        }
    }
    impl ::std::convert::From<SetVAMMAddressCall> for E2ESetupCalls {
        fn from(var: SetVAMMAddressCall) -> Self {
            E2ESetupCalls::SetVAMMAddress(var)
        }
    }
    impl ::std::convert::From<SettlePositionViaAMMCall> for E2ESetupCalls {
        fn from(var: SettlePositionViaAMMCall) -> Self {
            E2ESetupCalls::SettlePositionViaAMM(var)
        }
    }
    impl ::std::convert::From<SettleYBATraderCall> for E2ESetupCalls {
        fn from(var: SettleYBATraderCall) -> Self {
            E2ESetupCalls::SettleYBATrader(var)
        }
    }
    impl ::std::convert::From<SizeAllPositionsCall> for E2ESetupCalls {
        fn from(var: SizeAllPositionsCall) -> Self {
            E2ESetupCalls::SizeAllPositions(var)
        }
    }
    impl ::std::convert::From<SizeAllYBATradersCall> for E2ESetupCalls {
        fn from(var: SizeAllYBATradersCall) -> Self {
            E2ESetupCalls::SizeAllYBATraders(var)
        }
    }
    impl ::std::convert::From<SizeOfPositionHistoryCall> for E2ESetupCalls {
        fn from(var: SizeOfPositionHistoryCall) -> Self {
            E2ESetupCalls::SizeOfPositionHistory(var)
        }
    }
    impl ::std::convert::From<SizeOfPositionSwapsHistoryCall> for E2ESetupCalls {
        fn from(var: SizeOfPositionSwapsHistoryCall) -> Self {
            E2ESetupCalls::SizeOfPositionSwapsHistory(var)
        }
    }
    impl ::std::convert::From<SwapViaAMMCall> for E2ESetupCalls {
        fn from(var: SwapViaAMMCall) -> Self {
            E2ESetupCalls::SwapViaAMM(var)
        }
    }
    impl ::std::convert::From<SwapViaPeripheryCall> for E2ESetupCalls {
        fn from(var: SwapViaPeripheryCall) -> Self {
            E2ESetupCalls::SwapViaPeriphery(var)
        }
    }
    impl ::std::convert::From<UnwindFullyCollateralisedFixedTakerSwapCall> for E2ESetupCalls {
        fn from(var: UnwindFullyCollateralisedFixedTakerSwapCall) -> Self {
            E2ESetupCalls::UnwindFullyCollateralisedFixedTakerSwap(var)
        }
    }
    impl ::std::convert::From<UpdatePositionMarginViaAMMCall> for E2ESetupCalls {
        fn from(var: UpdatePositionMarginViaAMMCall) -> Self {
            E2ESetupCalls::UpdatePositionMarginViaAMM(var)
        }
    }
    #[doc = "Container type for all return fields from the `FCMAddress` function with signature `FCMAddress()` and selector `[209, 243, 35, 200]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct FcmaddressReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `MEAddress` function with signature `MEAddress()` and selector `[10, 72, 85, 69]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MeaddressReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `VAMMAddress` function with signature `VAMMAddress()` and selector `[240, 138, 244, 126]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct VammaddressReturn(pub ethers::core::types::Address);
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
    #[doc = "Container type for all return fields from the `abs` function with signature `abs(int256)` and selector `[27, 90, 196, 181]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct AbsReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `allPositions` function with signature `allPositions(uint256)` and selector `[186, 192, 118, 78]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct AllPositionsReturn {
        pub owner: ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
    }
    #[doc = "Container type for all return fields from the `allYBATraders` function with signature `allYBATraders(uint256)` and selector `[220, 136, 134, 132]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct AllYBATradersReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `cToken` function with signature `cToken()` and selector `[105, 229, 39, 218]`"]
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
    #[doc = "Container type for all return fields from the `getPositionHistory` function with signature `getPositionHistory(address,int24,int24)` and selector `[22, 119, 69, 15]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetPositionHistoryReturn(pub ::std::vec::Vec<PositionSnapshot>);
    #[doc = "Container type for all return fields from the `getPositionSwapsHistory` function with signature `getPositionSwapsHistory(address,int24,int24)` and selector `[243, 42, 174, 179]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetPositionSwapsHistoryReturn(
        pub ::std::vec::Vec<SwapSnapshot>,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all return fields from the `indexAllPositions` function with signature `indexAllPositions(bytes32)` and selector `[122, 10, 152, 221]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IndexAllPositionsReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `indexAllYBATraders` function with signature `indexAllYBATraders(address)` and selector `[28, 137, 58, 179]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IndexAllYBATradersReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `mintOrBurnViaPeriphery` function with signature `mintOrBurnViaPeriphery(address,(address,int24,int24,uint256,bool,int256))` and selector `[131, 227, 69, 231]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MintOrBurnViaPeripheryReturn {
        pub position_margin_requirement: I256,
    }
    #[doc = "Container type for all return fields from the `mintViaAMM` function with signature `mintViaAMM(address,int24,int24,uint128)` and selector `[56, 26, 14, 139]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MintViaAMMReturn {
        pub position_margin_requirement: I256,
    }
    #[doc = "Container type for all return fields from the `peripheryAddress` function with signature `peripheryAddress()` and selector `[190, 110, 154, 44]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PeripheryAddressReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `positionHistory` function with signature `positionHistory(bytes32,uint256)` and selector `[198, 227, 77, 55]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PositionHistoryReturn {
        pub current_timestamp_wad: ethers::core::types::U256,
        pub term_start_timestamp_wad: ethers::core::types::U256,
        pub term_end_timestamp_wad: ethers::core::types::U256,
        pub margin: I256,
        pub margin_requirement: ethers::core::types::U256,
        pub estimated_settlement_cashflow: I256,
        pub fixed_token_balance: I256,
        pub variable_token_balance: I256,
    }
    #[doc = "Container type for all return fields from the `positionSwapsHistory` function with signature `positionSwapsHistory(bytes32,uint256)` and selector `[226, 3, 180, 146]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PositionSwapsHistoryReturn {
        pub reserve_normalized_income_at_swap: ethers::core::types::U256,
        pub swap_initiation_timestamp_wad: ethers::core::types::U256,
        pub term_end_timestamp_wad: ethers::core::types::U256,
        pub notional: ethers::core::types::U256,
        pub is_ft: bool,
        pub fixed_rate_wad: ethers::core::types::U256,
        pub fee_paid_in_underlying_tokens: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `rateOracleAddress` function with signature `rateOracleAddress()` and selector `[62, 102, 140, 136]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct RateOracleAddressReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `sizeAllPositions` function with signature `sizeAllPositions()` and selector `[205, 200, 103, 147]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SizeAllPositionsReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `sizeAllYBATraders` function with signature `sizeAllYBATraders()` and selector `[126, 63, 254, 253]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SizeAllYBATradersReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `sizeOfPositionHistory` function with signature `sizeOfPositionHistory(bytes32)` and selector `[240, 241, 245, 112]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SizeOfPositionHistoryReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `sizeOfPositionSwapsHistory` function with signature `sizeOfPositionSwapsHistory(bytes32)` and selector `[252, 53, 168, 10]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SizeOfPositionSwapsHistoryReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `swapViaAMM` function with signature `swapViaAMM((address,int256,uint160,int24,int24))` and selector `[63, 175, 104, 254]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SwapViaAMMReturn {
        pub fixed_token_delta: I256,
        pub variable_token_delta: I256,
        pub cumulative_fee_incurred: ethers::core::types::U256,
        pub fixed_token_delta_unbalanced: I256,
        pub margin_requirement: I256,
    }
    #[doc = "Container type for all return fields from the `swapViaPeriphery` function with signature `swapViaPeriphery(address,(address,bool,uint256,uint160,int24,int24,int256))` and selector `[242, 53, 67, 144]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SwapViaPeripheryReturn {
        pub fixed_token_delta: I256,
        pub variable_token_delta: I256,
        pub cumulative_fee_incurred: ethers::core::types::U256,
        pub fixed_token_delta_unbalanced: I256,
        pub margin_requirement: I256,
    }
    #[doc = "`PositionSnapshot(uint256,uint256,uint256,int256,uint256,int256,int256,int256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct PositionSnapshot {
        pub current_timestamp_wad: ethers::core::types::U256,
        pub term_start_timestamp_wad: ethers::core::types::U256,
        pub term_end_timestamp_wad: ethers::core::types::U256,
        pub margin: I256,
        pub margin_requirement: ethers::core::types::U256,
        pub estimated_settlement_cashflow: I256,
        pub fixed_token_balance: I256,
        pub variable_token_balance: I256,
    }
    #[doc = "`SwapSnapshot(uint256,uint256,uint256,uint256,bool,uint256,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SwapSnapshot {
        pub reserve_normalized_income_at_swap: ethers::core::types::U256,
        pub swap_initiation_timestamp_wad: ethers::core::types::U256,
        pub term_end_timestamp_wad: ethers::core::types::U256,
        pub notional: ethers::core::types::U256,
        pub is_ft: bool,
        pub fixed_rate_wad: ethers::core::types::U256,
        pub fee_paid_in_underlying_tokens: ethers::core::types::U256,
    }
}
