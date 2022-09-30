pub use vamm::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod vamm {
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
    #[doc = "VAMM was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static VAMM_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"AavePoolGetReserveNormalizedIncomeReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"AavePoolGetReserveNormalizedVariableDebtReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CTokenExchangeRateReturnedZero\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"unlocked\",\"type\":\"bool\",\"components\":[]}],\"type\":\"error\",\"name\":\"CanOnlyTradeIfUnlocked\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CannotLiquidate\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CannotSettleBeforeMaturity\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"DebugError\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"amount0\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"amount1\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"ExpectedOppositeSigns\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint160\",\"name\":\"sqrtPriceX96\",\"type\":\"uint160\",\"components\":[]}],\"type\":\"error\",\"name\":\"ExpectedSqrtPriceZeroBeforeInit\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"IRSNotionalAmountSpecifiedMustBeNonZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidMarginDelta\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"LidoGetPooledEthBySharesReturnedZero\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"type\":\"error\",\"name\":\"LiquidityDeltaMustBePositiveInBurn\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"type\":\"error\",\"name\":\"LiquidityDeltaMustBePositiveInMint\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginLessThanMinimum\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tick\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"variableTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"cumulativeFeeIncurred\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenDeltaUnbalanced\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginRequirementNotMet\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginRequirementNotMetFCM\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"requested\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"available\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"NotEnoughFunds\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OOO\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyFCM\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyMarginEngine\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyOwnerCanUpdatePosition\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyVAMM\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PRBMathSD59x18__DivInputTooSmall\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"rAbs\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMathSD59x18__DivOverflow\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"x\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMathSD59x18__FromIntOverflow\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"x\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMathSD59x18__FromIntUnderflow\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PRBMathSD59x18__MulInputTooSmall\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"rAbs\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMathSD59x18__MulOverflow\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMathUD60x18__FromUintOverflow\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"prod1\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMath__MulDivFixedPointOverflow\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"prod1\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"denominator\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMath__MulDivOverflow\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PositionNetZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PositionNotSettled\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"RocketPoolGetEthValueReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"WithdrawalExceedsCurrentMargin\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"closeToOrBeyondMaturity\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AdminChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"beacon\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"BeaconUpgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[],\"indexed\":true},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Burn\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"feeWad\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Fee\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"feeProtocol\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FeeProtocol\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"version\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Initialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"__isAlpha\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"IsAlpha\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[],\"indexed\":true},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Mint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[],\"indexed\":true},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[],\"indexed\":true},{\"internalType\":\"int256\",\"name\":\"desiredNotional\",\"type\":\"int256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceLimitX96\",\"type\":\"uint160\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"cumulativeFeeIncurred\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"fixedTokenDelta\",\"type\":\"int256\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"variableTokenDelta\",\"type\":\"int256\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"fixedTokenDeltaUnbalanced\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Swap\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"implementation\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Upgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint160\",\"name\":\"sqrtPriceX96\",\"type\":\"uint160\",\"components\":[],\"indexed\":false},{\"internalType\":\"int24\",\"name\":\"tick\",\"type\":\"int24\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"VAMMInitialization\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"int24\",\"name\":\"tick\",\"type\":\"int24\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"VAMMPriceChange\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MAX_FEE\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"VOLTZ_PAUSER\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burn\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"positionMarginRequirement\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"permission\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"changePauser\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"computeGrowthInside\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"fixedTokenGrowthInsideX128\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"variableTokenGrowthInsideX128\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"feeGrowthInsideX128\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"factory\",\"outputs\":[{\"internalType\":\"contract IFactory\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"feeGrowthGlobalX128\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"feeWad\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"fixedTokenGrowthGlobalX128\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRateOracle\",\"outputs\":[{\"internalType\":\"contract IRateOracle\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract IMarginEngine\",\"name\":\"__marginEngine\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"__tickSpacing\",\"type\":\"int24\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint160\",\"name\":\"sqrtPriceX96\",\"type\":\"uint160\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initializeVAMM\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isAlpha\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"liquidity\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"\",\"type\":\"uint128\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"marginEngine\",\"outputs\":[{\"internalType\":\"contract IMarginEngine\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"maxLiquidityPerTick\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"\",\"type\":\"uint128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mint\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"positionMarginRequirement\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"paused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"protocolFees\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"proxiableUUID\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"refreshRateOracle\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newFeeWad\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setFee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"feeProtocol\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setFeeProtocol\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"__isAlpha\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setIsAlpha\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"state\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPausability\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IVAMM.SwapParams\",\"name\":\"params\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"amountSpecified\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceLimitX96\",\"type\":\"uint160\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"swap\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"fixedTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"variableTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"cumulativeFeeIncurred\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenDeltaUnbalanced\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"int16\",\"name\":\"wordPosition\",\"type\":\"int16\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"tickBitmap\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"tickSpacing\",\"outputs\":[{\"internalType\":\"int24\",\"name\":\"\",\"type\":\"int24\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"int24\",\"name\":\"tick\",\"type\":\"int24\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ticks\",\"outputs\":[{\"internalType\":\"struct Tick.Info\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint128\",\"name\":\"liquidityGross\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"liquidityNet\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenGrowthOutsideX128\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"variableTokenGrowthOutsideX128\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"feeGrowthOutsideX128\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"initialized\",\"type\":\"bool\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"protocolFeesCollected\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateProtocolFees\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newImplementation\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"upgradeTo\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newImplementation\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"upgradeToAndCall\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"vammVars\",\"outputs\":[{\"internalType\":\"struct IVAMM.VAMMVars\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint160\",\"name\":\"sqrtPriceX96\",\"type\":\"uint160\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tick\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"feeProtocol\",\"type\":\"uint8\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"variableTokenGrowthGlobalX128\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static VAMM_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60a0604052306080523480156200001557600080fd5b50604554610100900460ff1615808015620000375750604554600160ff909116105b8062000067575062000054306200014160201b62002ad01760201c565b15801562000067575060455460ff166001145b620000cf5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b606482015260840160405180910390fd5b6045805460ff191660011790558015620000f3576045805461ff0019166101001790555b80156200013a576045805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b5062000150565b6001600160a01b03163b151590565b6080516161106200018860003960008181610d0201528181610d42015281816110d70152818161111701526111aa01526161106000f3fe6080604052600436106102195760003560e01c806369fe0e2d11610123578063b613a141116100ab578063d0c93a7c1161006f578063d0c93a7c146106d5578063d992d908146106f8578063f2fde38b14610718578063f30dba9314610738578063f3f949901461084e57600080fd5b8063b613a1411461063c578063b8cca34e1461065c578063bc063e1a1461067c578063c45a015514610697578063cd41b3d5146106b557600080fd5b806380a0f76c116100f257806380a0f76c1461053657806386737710146105d157806388428752146105f15780638da5cb5b14610609578063ad23b4161461062757600080fd5b806369fe0e2d146104c557806370cf754a146104e5578063715018a614610503578063778762361461051857600080fd5b80633c8f233e116101a65780634f1ef286116101755780634f1ef286146103fb57806352d1902d1461040e5780635339c296146104235780635c975abb1461045357806367758e6e1461047d57600080fd5b80633c8f233e1461036b578063478de324146103a657806347f75ede146103c65780634e65b408146103e657600080fd5b80631a686502116101ed5780631a686502146102ab5780631ad8b03b146102e25780631b398e06146102f75780631f2f08931461032b5780633659cfe61461034b57600080fd5b80624006e01461021e57806309d7b622146102555780630bd9c1af146102745780630d2119541461028b575b600080fd5b34801561022a57600080fd5b506003546001600160a01b03165b6040516001600160a01b0390911681526020015b60405180910390f35b34801561026157600080fd5b50600a545b60405190815260200161024c565b34801561028057600080fd5b50610289610863565b005b34801561029757600080fd5b506102896102a63660046155f8565b610913565b3480156102b757600080fd5b5060075461010090046001600160801b03165b6040516001600160801b03909116815260200161024c565b3480156102ee57600080fd5b50600954610266565b34801561030357600080fd5b506102667fc02e2d3f2633adb184196f6ae17c8476d7912f8727b7c1cc7da0b7ddac86bc6581565b34801561033757600080fd5b5061026661034636600461563c565b6109d2565b34801561035757600080fd5b506102896103663660046156a0565b610cf7565b34801561037757600080fd5b5061038b6103863660046156bd565b610dd7565b6040805193845260208401929092529082015260600161024c565b3480156103b257600080fd5b506102896103c13660046156f0565b610eb5565b3480156103d257600080fd5b506102896103e13660046156a0565b610ee8565b3480156103f257600080fd5b50600b54610266565b610289610409366004615770565b6110cc565b34801561041a57600080fd5b5061026661119d565b34801561042f57600080fd5b5061026661043e366004615818565b60010b6000908152600e602052604090205490565b34801561045f57600080fd5b5060125461046d9060ff1681565b604051901515815260200161024c565b34801561048957600080fd5b5061049d61049836600461583b565b611250565b604080519586526020860194909452928401919091526060830152608082015260a00161024c565b3480156104d157600080fd5b506102896104e03660046158c6565b61216d565b3480156104f157600080fd5b506004546001600160801b03166102ca565b34801561050f57600080fd5b506102896121f4565b34801561052457600080fd5b506000546001600160a01b0316610238565b34801561054257600080fd5b506105a060408051606081018252600080825260208201819052918101919091525060408051606081018252600f546001600160a01b0381168252600160a01b810460020b6020830152600160b81b900460ff169181019190915290565b6040805182516001600160a01b0316815260208084015160020b908201529181015160ff169082015260600161024c565b3480156105dd57600080fd5b506102896105ec3660046158c6565b612208565b3480156105fd57600080fd5b5060105460ff1661046d565b34801561061557600080fd5b506078546001600160a01b0316610238565b34801561063357600080fd5b50600854610266565b34801561064857600080fd5b506102896106573660046158df565b61227e565b34801561066857600080fd5b5061026661067736600461563c565b61232e565b34801561068857600080fd5b5061026666470de4df82000081565b3480156106a357600080fd5b506005546001600160a01b0316610238565b3480156106c157600080fd5b506102896106d03660046155f8565b612670565b3480156106e157600080fd5b50600c5460405160029190910b815260200161024c565b34801561070457600080fd5b50610289610713366004615902565b6126bf565b34801561072457600080fd5b506102896107333660046156a0565b612a5a565b34801561074457600080fd5b506107f4610753366004615920565b6040805160c081018252600080825260208201819052918101829052606081018290526080810182905260a081019190915250600290810b6000908152600d6020908152604091829020825160c08101845281546001600160801b0381168252600160801b9004600f0b92810192909252600181015492820192909252918101546060830152600381015460808301526004015460ff16151560a082015290565b60405161024c9190600060c0820190506001600160801b0383511682526020830151600f0b602083015260408301516040830152606083015160608301526080830151608083015260a0830151151560a083015292915050565b34801561085a57600080fd5b50600654610266565b61086b612adf565b600360009054906101000a90046001600160a01b03166001600160a01b03166398f4b1b26040518163ffffffff1660e01b815260040160206040518083038186803b1580156108b957600080fd5b505afa1580156108cd573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906108f1919061593b565b600080546001600160a01b0319166001600160a01b0392909216919091179055565b3360009081526011602052604090205460ff166109615760405162461bcd60e51b81526020600482015260076024820152666e6f20726f6c6560c81b60448201526064015b60405180910390fd5b6012805460ff1916821515908117909155600354604051630348465560e21b815260048101929092526001600160a01b031690630d21195490602401600060405180830381600087803b1580156109b757600080fd5b505af11580156109cb573d6000803e3d6000fd5b5050505050565b60105460009060ff1615610abb5760055460408051633bd5670d60e11b815290516000926001600160a01b0316916377aace1a916004808301926020929190829003018186803b158015610a2557600080fd5b505afa158015610a39573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a5d919061593b565b9050336001600160a01b0382161480610a8057506003546001600160a01b031633145b610ab95760405162461bcd60e51b815260206004820152600a6024820152697070687279206f6e6c7960b01b6044820152606401610958565b505b60125460ff1615610ade5760405162461bcd60e51b815260040161095890615958565b60075460ff16610b165760405162461bcd60e51b81526020600482015260036024820152624c4f4b60e81b6044820152606401610958565b6007805460ff191690556001600160801b038216610b525760405163c09d260960e01b81526001600160801b0383166004820152602401610958565b336001600160a01b0386161480610b7357506003546001600160a01b031633145b80610bfc57506005546040516351c4bc1f60e11b81526001600160a01b0387811660048301523360248301529091169063a389783e9060440160206040518083038186803b158015610bc457600080fd5b505afa158015610bd8573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610bfc9190615978565b610c335760405162461bcd60e51b81526020600482015260086024820152674d53206f72204d4560c01b6044820152606401610958565b610c876040518060800160405280876001600160a01b031681526020018660020b81526020018560020b8152602001610c74856001600160801b0316612b39565b610c7d906159ab565b600f0b9052612b82565b604080513381526001600160801b0385166020820152919250600285810b929087900b916001600160a01b038916917ff57f161c6404e7a58d089003a260456719af3caac502550a59509b4c9d8d462891015b60405180910390a46007805460ff19166001179055949350505050565b306001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161415610d405760405162461bcd60e51b8152600401610958906159d2565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316610d89600080516020616094833981519152546001600160a01b031690565b6001600160a01b031614610daf5760405162461bcd60e51b815260040161095890615a1e565b610db881612de0565b60408051600080825260208201909252610dd491839190612de8565b50565b6000806000610de68585612f62565b60408051608081018252600287810b825286810b6020830152600f54600160a01b9004900b91810191909152600a546060820152610e2690600d90613023565b60408051608081018252600288810b825287810b6020830152600f54600160a01b9004900b91810191909152600b546060820152909350610e6990600d9061307c565b60408051608081018252600288810b825287810b6020830152600f54600160a01b9004900b918101919091526008546060820152909250610eac90600d906130c3565b90509250925092565b610ebd612adf565b6001600160a01b03919091166000908152601160205260409020805460ff1916911515919091179055565b6001600160a01b038116610f315760405162461bcd60e51b815260206004820152601060248201526f7a65726f20696e70757420707269636560801b6044820152606401610958565b6c1fa71f3f5f68a90479ee3f8fec6001600160a01b038216108015610f6b57506b0816769404766de590afe04e6001600160a01b03821610155b610f9b5760405162461bcd60e51b81526020600482015260016024820152602960f91b6044820152606401610958565b6003546001600160a01b0316610fea5760405162461bcd60e51b81526020600482015260146024820152731d985b5b481b9bdd081a5b9a5d1a585b1a5e995960621b6044820152606401610958565b600f546001600160a01b03161561102357600f546040516328be1c0f60e21b81526001600160a01b039091166004820152602401610958565b600061102e8261312a565b604080516060810182526001600160a01b038516808252600284900b6020808401829052600093850193909352600f80546001600160b81b0319168317600160a01b62ffffff8816021760ff60b81b191690556007805460ff191660011790558351918252918101919091529192507facf59ced105c47c72de67aa00ab58b6415014ad6018644e3e8d8ca6862ec0dce910160405180910390a15050565b306001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614156111155760405162461bcd60e51b8152600401610958906159d2565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031661115e600080516020616094833981519152546001600160a01b031690565b6001600160a01b0316146111845760405162461bcd60e51b815260040161095890615a1e565b61118d82612de0565b61119982826001612de8565b5050565b6000306001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161461123d5760405162461bcd60e51b815260206004820152603860248201527f555550535570677261646561626c653a206d757374206e6f742062652063616c60448201527f6c6564207468726f7567682064656c656761746563616c6c00000000000000006064820152608401610958565b5060008051602061609483398151915290565b601254600090819081908190819060ff161561127e5760405162461bcd60e51b815260040161095890615958565b611289600254613464565b156112d05760405162461bcd60e51b8152602060048201526017602482015276636c6f7365546f4f724265796f6e644d6174757269747960481b6044820152606401610958565b6112e286606001518760800151612f62565b60408051606081018252600f546001600160a01b0381168252600160a01b810460020b602080840191909152600160b81b90910460ff169282019290925290870151611334908890839060001261348c565b6003546001600160a01b03163314806113e35750600360009054906101000a90046001600160a01b03166001600160a01b03166322d23b216040518163ffffffff1660e01b815260040160206040518083038186803b15801561139657600080fd5b505afa1580156113aa573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906113ce919061593b565b6001600160a01b0316336001600160a01b0316145b6114d65786516001600160a01b031633148061147e575060055487516040516351c4bc1f60e11b81526001600160a01b03918216600482015233602482015291169063a389783e9060440160206040518083038186803b15801561144657600080fd5b505afa15801561145a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061147e9190615978565b6114d65760405162461bcd60e51b815260206004820152602360248201527f6f6e6c792073656e646572206f7220617070726f76656420696e74656772617460448201526234b7b760e91b6064820152608401610958565b6007805460ff1916908190556040805180820182526001600160801b036101009384900481168252600f54600160b81b900460ff1660208084019190915283516101c0810185528c8201518152600081830181905287516001600160a01b039081168388015292880151600290810b6060840152600a546080840152600b5460a0840152855190941660c083015260085460e083015295810186905261012081018690526101408101869052610160810186905261018081018690528554600154935495516325f258dd60e01b815294969591946101a086019491909316926325f258dd926115d19291600401918252602082015260400190565b602060405180830381600087803b1580156115eb57600080fd5b505af11580156115ff573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906116239190615a6a565b815250905060008054906101000a90046001600160a01b03166001600160a01b0316637aa4db136040518163ffffffff1660e01b8152600401600060405180830381600087803b15801561167657600080fd5b505af115801561168a573d6000803e3d6000fd5b50505050600089602001511315611a61575b8051158015906116c6575088604001516001600160a01b031681604001516001600160a01b031614155b15611a5c576116d3615579565b60408201516001600160a01b031681526060820151600c546116fc91600e9160020b60006135ad565b1515604083015260020b602082015261171862010deb19615a83565b60020b816020015160020b131561173f5761173662010deb19615a83565b60020b60208201525b61174c816020015161376a565b81606001906001600160a01b031690816001600160a01b0316815250506118076040518060c0016040528084604001516001600160a01b031681526020018c604001516001600160a01b031684606001516001600160a01b0316116117b55783606001516117bb565b8c604001515b6001600160a01b0316815260c08501516001600160801b031660208201528451604082015260065460608201526080016117f3613b0a565b6002546118009190615a9d565b9052613b1a565b60c085015260a0840152608083019081526001600160a01b0390911660408401525161183290613d43565b82518390611841908390615ab4565b90525060a081015161185290613d43565b826020018181516118639190615ab4565b905250608081015161187490613d43565b61014082015260a081015161188890613d43565b61189190615af3565b61010082015260c0810151610120830180516118ae908390615b10565b905250602083015160ff161561190b57826020015160ff168160c001516118d59190615b3e565b60e0820181905260c0820180516118ed908390615a9d565b90525060e081015161010083018051611907908390615b10565b9052505b60c08201516001600160801b031615611995576119288282613d8c565b6101208501908152608086019190915260a085019190915260e0840191909152516101408301805161195b908390615ab4565b90525061014081015161016083018051611976908390615ab4565b90525061010081015161018083018051611991908390615ab4565b9052505b80606001516001600160a01b031682604001516001600160a01b03161415611a1f57806040015115611a0d5760006119ed826020015184608001518560a001518660e00151600d613e4d90949392919063ffffffff16565b90506119fd8360c0015182613eb1565b6001600160801b031660c0840152505b602081015160020b6060830152611a56565b80600001516001600160a01b031682604001516001600160a01b031614611a5657611a4d826040015161312a565b60020b60608301525b5061169c565b611dcc565b805115801590611a8b575088604001516001600160a01b031681604001516001600160a01b031614155b15611dcc57611a98615579565b60408201516001600160a01b031681526060820151600c54611ac191600e9160020b60016135ad565b1515604083015260020b6020820181905262010deb191315611ae85762010deb1960208201525b611af5816020015161376a565b81606001906001600160a01b031690816001600160a01b031681525050611b5e6040518060c0016040528084604001516001600160a01b031681526020018c604001516001600160a01b031684606001516001600160a01b0316106117b55783606001516117bb565b60c085015260a0840190815260808401919091526001600160a01b03909116604084015251611b8c90613d43565b82518390611b9b908390615b52565b9052506080810151611bac90613d43565b82602001818151611bbd9190615b52565b90525060a0810151611bce90613d43565b611bd790615af3565b6101408201526080810151611beb90613d43565b61010082015260c0810151610120830151611c069190615b10565b610120830152602083015160ff1615611c6657826020015160ff168160c00151611c309190615b3e565b60e0820181905260c082018051611c48908390615a9d565b90525060e081015161010083018051611c62908390615b10565b9052505b60c08201516001600160801b031615611cf057611c838282613d8c565b6101208501908152608086019190915260a085019190915260e08401919091525161014083018051611cb6908390615ab4565b90525061014081015161016083018051611cd1908390615ab4565b90525061010081015161018083018051611cec908390615ab4565b9052505b80606001516001600160a01b031682604001516001600160a01b03161415611d8f57806040015115611d71576000611d48826020015184608001518560a001518660e00151600d613e4d90949392919063ffffffff16565b9050611d618360c0015182611d5c906159ab565b613eb1565b6001600160801b031660c0840152505b60018160200151611d829190615b93565b60020b6060830152611dc6565b80600001516001600160a01b031682604001516001600160a01b031614611dc657611dbd826040015161312a565b60020b60608301525b50611a61565b6040810151600f80546001600160a01b0319166001600160a01b0390921691909117905560208301516060820151600291820b910b14611e2e576060810151600f805462ffffff909216600160a01b0262ffffff60a01b199092169190911790555b8060c001516001600160801b031682600001516001600160801b031614611e845760c0810151600780546001600160801b039092166101000270ffffffffffffffffffffffffffffffff00199092169190911790555b60e081015160085560a0810151600b556080810151600a55610120810151610140820151610160830151610180840151610100850151929b5090995091975090955015611ee85780610100015160096000828254611ee29190615b10565b90915550505b6003546001600160a01b0316331480611f975750600360009054906101000a90046001600160a01b03166001600160a01b03166322d23b216040518163ffffffff1660e01b815260040160206040518083038186803b158015611f4a57600080fd5b505afa158015611f5e573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611f82919061593b565b6001600160a01b0316336001600160a01b0316145b61206d57600354895160608b015160808c015161014085015161016086015161012087015161018088015160405163604b0bd760e11b81526001600160a01b039788166004820152600296870b60248201529490950b60448501526064840192909252608483015260a482015260c481019190915291169063c09617ae9060e401602060405180830381600087803b15801561203257600080fd5b505af1158015612046573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061206a9190615a6a565b93505b600f54604051600160a01b90910460020b81527f3de48b885df0271268324c099733a36a802c1cbb40c7272796b2b28addf04cd29060200160405180910390a1886080015160020b896060015160020b8a600001516001600160a01b03167fa24f288a343811d26ac1ec29998e37b87ff6503cefe399a3c8fb747eb0464e58338d602001518e604001518c8f8f8e60405161214a97969594939291906001600160a01b03978816815260208101969096529390951660408501526060840191909152608083015260a082019290925260c081019190915260e00190565b60405180910390a450506007805460ff1916600117905550939592945090929091565b612175612adf565b66470de4df8200008111156121b85760405162461bcd60e51b81526020600482015260096024820152686665652072616e676560b81b6044820152606401610958565b60068190556040518181527f557809284da7314475b1582804ae28e5f1349efc1fe970ea25d50fce75eb4f43906020015b60405180910390a150565b6121fc612adf565b6122066000613ee8565b565b6003546001600160a01b0316331461223357604051630a0d349f60e21b815260040160405180910390fd5b806009541015612264576009546040516311920a6d60e31b8152610958918391600401918252602082015260400190565b80600960008282546122769190615a9d565b909155505050565b612286612adf565b60ff811615806122a9575060038160ff16101580156122a9575060328160ff1611155b6122e05760405162461bcd60e51b815260206004820152600860248201526750522072616e676560c01b6044820152606401610958565b600f805460ff60b81b1916600160b81b60ff8416908102919091179091556040519081527fe949530fb25dc21f05cb65fe03447f6f68f8e21e3584c72e6e92042b8bc28f79906020016121e9565b60105460009060ff16156124025760055460408051633bd5670d60e11b815290516000926001600160a01b0316916377aace1a916004808301926020929190829003018186803b15801561238157600080fd5b505afa158015612395573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906123b9919061593b565b9050336001600160a01b038216146124005760405162461bcd60e51b815260206004820152600a6024820152697070687279206f6e6c7960b01b6044820152606401610958565b505b60125460ff16156124255760405162461bcd60e51b815260040161095890615958565b612430600254613464565b156124775760405162461bcd60e51b8152602060048201526017602482015276636c6f7365546f4f724265796f6e644d6174757269747960481b6044820152606401610958565b60075460ff166124af5760405162461bcd60e51b81526020600482015260036024820152624c4f4b60e81b6044820152606401610958565b6007805460ff191690556001600160801b0382166124eb57604051633611668d60e21b81526001600160801b0383166004820152602401610958565b336001600160a01b038616148061258057506005546040516351c4bc1f60e11b81526001600160a01b0387811660048301523360248301529091169063a389783e9060440160206040518083038186803b15801561254857600080fd5b505afa15801561255c573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906125809190615978565b6125d85760405162461bcd60e51b8152602060048201526024808201527f6f6e6c79206d73672e73656e646572206f7220617070726f7665642063616e206044820152631b5a5b9d60e21b6064820152608401610958565b6126196040518060800160405280876001600160a01b031681526020018660020b81526020018560020b8152602001610c7d856001600160801b0316612b39565b604080513381526001600160801b0385166020820152919250600285810b929087900b916001600160a01b038916917f712faa344eac6399174fdfa887d9e1451e9b55ce58ee440c91c660229962a5a69101610cda565b612678612adf565b6010805460ff191682151590811790915560405160ff909116151581527fa201234976cfdc556c03f06ca9366e09441724eae79256ad9da6b5f04cbdb058906020016121e9565b604554610100900460ff16158080156126df5750604554600160ff909116105b806126f95750303b1580156126f9575060455460ff166001145b61275c5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608401610958565b6045805460ff19166001179055801561277f576045805461ff0019166101001790555b6001600160a01b0383166127be5760405162461bcd60e51b815260206004820152600660248201526504d45203d20360d41b6044820152606401610958565b60008260020b1380156127d65750614000600283900b125b61280a5760405162461bcd60e51b81526020600482015260056024820152642a29a7a7a160d91b6044820152606401610958565b600380546001600160a01b0319166001600160a01b03851690811790915560408051634c7a58d960e11b815290516398f4b1b291600480820192602092909190829003018186803b15801561285e57600080fd5b505afa158015612872573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612896919061593b565b600080546001600160a01b03929092166001600160a01b03199283161790556005805490911633179055600c805462ffffff841662ffffff1990911617908190556128e39060020b613f3a565b600480546001600160801b0319166001600160801b03929092169190911781556003546040805163652c30b760e01b815290516001600160a01b039092169263652c30b7928282019260209290829003018186803b15801561294457600080fd5b505afa158015612958573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061297c9190615a6a565b600155600354604080516324fb6d1560e21b815290516001600160a01b03909216916393edb45491600480820192602092909190829003018186803b1580156129c457600080fd5b505afa1580156129d8573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906129fc9190615a6a565b600255612a07613fa2565b612a0f613fd1565b8015612a55576045805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b505050565b612a62612adf565b6001600160a01b038116612ac75760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610958565b610dd481613ee8565b6001600160a01b03163b151590565b6078546001600160a01b031633146122065760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610958565b80600f81900b8114612b7d5760405162461bcd60e51b815260206004820152600d60248201526c746f496e74313238206f666c6f60981b6044820152606401610958565b919050565b6000612b9682602001518360400151612f62565b6040805160608082018352600f80546001600160a01b0381168452600160a01b810460020b6020850152600160b81b900460ff1693830193909352840151909160009182910b15612bf057612bea85613ff8565b90925090505b600354600094506001600160a01b03163314612cb2576003546040805163bfb5607d60e01b815287516001600160a01b0390811660048301526020890151600290810b60248401529289015190920b60448201526060880151600f0b606482015291169063bfb5607d90608401602060405180830381600087803b158015612c7757600080fd5b505af1158015612c8b573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612caf9190615a6a565b93505b60008560600151600f0b1215612cf0578115612cd9576020850151612cd990600d906140d7565b8015612cf0576040850151612cf090600d906140d7565b6000805460408051637aa4db1360e01b815290516001600160a01b0390921692637aa4db139260048084019382900301818387803b158015612d3157600080fd5b505af1158015612d45573d6000803e3d6000fd5b505050508460600151600f0b600014612dd857846020015160020b836020015160020b12158015612d835750846040015160020b836020015160020b125b15612dd8576000600760019054906101000a90046001600160801b03169050612db0818760600151613eb1565b600760016101000a8154816001600160801b0302191690836001600160801b03160217905550505b505050919050565b610dd4612adf565b7f4910fdfa16fed3260ed0e7147f7cc6da11a60208b5b9406d12a635614ffd91435460ff1615612e1b57612a5583614110565b826001600160a01b03166352d1902d6040518163ffffffff1660e01b815260040160206040518083038186803b158015612e5457600080fd5b505afa925050508015612e84575060408051601f3d908101601f19168201909252612e8191810190615a6a565b60015b612ee75760405162461bcd60e51b815260206004820152602e60248201527f45524331393637557067726164653a206e657720696d706c656d656e7461746960448201526d6f6e206973206e6f74205555505360901b6064820152608401610958565b6000805160206160948339815191528114612f565760405162461bcd60e51b815260206004820152602960248201527f45524331393637557067726164653a20756e737570706f727465642070726f786044820152681a58589b195555525160ba1b6064820152608401610958565b50612a558383836141ac565b8060020b8260020b12612f9d5760405162461bcd60e51b8152602060048201526003602482015262544c5560e81b6044820152606401610958565b62010deb19600283900b1215612fdb5760405162461bcd60e51b8152602060048201526003602482015262544c4d60e81b6044820152606401610958565b612fe862010deb19615a83565b60020b8160020b13156111995760405162461bcd60e51b815260206004820152600360248201526254554d60e81b6044820152606401610958565b8051600290810b600090815260208481526040808320918501805190940b8352808320855194519186015160608701516001808601549084015496979596939561307395909490939291906141d7565b95945050505050565b8051600290810b6000908152602084815260408083209185018051850b845281842086519151928701516060880151858801549783015496979596929561307395936141d7565b8051600290810b600090815260208481526040808320918501805190940b83528083208551945191860151606087015194959394919361307393909290919061310b90613d43565b6131188760030154613d43565b6131258760030154613d43565b6141d7565b60006b0816769404766de590afe04e6001600160a01b0383161080159061316657506c1fa71f3f5f68a90479ee3f8fec6001600160a01b038316105b6131965760405162461bcd60e51b81526020600482015260016024820152602960f91b6044820152606401610958565b640100000000600160c01b03602083901b166001600160801b03811160071b81811c67ffffffffffffffff811160061b90811c63ffffffff811160051b90811c61ffff811160041b90811c60ff8111600390811b91821c600f811160021b90811c918211600190811b92831c9790881196179094179092171790911717176080811061323157613227607f82615a9d565b83901c9150613242565b61323c81607f615a9d565b83901b91505b60006040613251608084615ab4565b901b9050828302607f1c92508260801c80603f1b8217915083811c935050828302607f1c92508260801c80603e1b8217915083811c935050828302607f1c92508260801c80603d1b8217915083811c935050828302607f1c92508260801c80603c1b8217915083811c935050828302607f1c92508260801c80603b1b8217915083811c935050828302607f1c92508260801c80603a1b8217915083811c935050828302607f1c92508260801c8060391b8217915083811c935050828302607f1c92508260801c8060381b8217915083811c935050828302607f1c92508260801c8060371b8217915083811c935050828302607f1c92508260801c8060361b8217915083811c935050828302607f1c92508260801c8060351b8217915083811c935050828302607f1c92508260801c8060341b8217915083811c935050828302607f1c92508260801c8060331b8217915083811c935050828302607f1c92508260801c8060321b8217915050600081693627a301d71055774c856133d49190615bdb565b9050600060806133f46f028f6481ab7f045a5af012a19d003aaa84615ab4565b901d905060006080613416846fdb2df09e81959a81455e260799a0632f615b52565b901d90508060020b8260020b1461345557886001600160a01b031661343a8261376a565b6001600160a01b0316111561344f5781613457565b80613457565b815b9998505050505050505050565b60008169124bc0ddd92e5600000061347a613b0a565b6134849190615b10565b101592915050565b60208301516134ae57604051631fa907d560e11b815260040160405180910390fd5b60075460ff166134dc57600754604051633cc7822f60e11b815260ff90911615156004820152602401610958565b806135305781600001516001600160a01b031683604001516001600160a01b031610801561352b57506b0816769404766de590afe04e6001600160a01b031683604001516001600160a01b0316115b61357b565b81600001516001600160a01b031683604001516001600160a01b031611801561357b57506c1fa71f3f5f68a90479ee3f8fec6001600160a01b031683604001516001600160a01b0316105b612a555760405162461bcd60e51b815260206004820152600360248201526214d41360ea1b6044820152606401610958565b600080806135bb8587615c60565b905060008660020b1280156135db57506135d58587615c9a565b60020b15155b156135ee57806135ea81615cbc565b9150505b831561369b5760008061360083614243565b90925090506000600160ff831681901b9061361b9082615a9d565b6136259190615b10565b600184900b600090815260208c905260409020548116801515965090915085613665578861365660ff851687615b93565b6136609190615ce0565b613690565b8861366f82614260565b6136799085615d6d565b6136869060ff1687615b93565b6136909190615ce0565b965050505050613760565b6000806136b16136ac846001615d90565b614243565b909250905060006136c9600160ff841681901b615a9d565b600184900b600090815260208c90526040902054901990811680151596509091508561372257886136fb8460ff615d6d565b60ff16613709876001615d90565b6137139190615d90565b61371d9190615ce0565b613759565b888361372d83614383565b6137379190615d6d565b60ff16613745876001615d90565b61374f9190615d90565b6137599190615ce0565b9650505050505b5094509492505050565b60008060008360020b12613781578260020b61378e565b8260020b61378e90615af3565b905061379d62010deb19615a83565b60020b8111156137d35760405162461bcd60e51b81526020600482015260016024820152601560fa1b6044820152606401610958565b6000600182166137e757600160801b6137f9565b6ffffcb933bd6fad37aa2d162d1a5940015b70ffffffffffffffffffffffffffffffffff1690506002821615613838576080613833826ffff97272373d413259a46990580e213a615dd7565b901c90505b600482161561386257608061385d826ffff2e50f5f656932ef12357cf3c7fdcc615dd7565b901c90505b600882161561388c576080613887826fffe5caca7e10e4e61c3624eaa0941cd0615dd7565b901c90505b60108216156138b65760806138b1826fffcb9843d60f6159c9db58835c926644615dd7565b901c90505b60208216156138e05760806138db826fff973b41fa98c081472e6896dfb254c0615dd7565b901c90505b604082161561390a576080613905826fff2ea16466c96a3843ec78b326b52861615dd7565b901c90505b608082161561393457608061392f826ffe5dee046a99a2a811c461f1969c3053615dd7565b901c90505b61010082161561395f57608061395a826ffcbe86c7900a88aedcffc83b479aa3a4615dd7565b901c90505b61020082161561398a576080613985826ff987a7253ac413176f2b074cf7815e54615dd7565b901c90505b6104008216156139b55760806139b0826ff3392b0822b70005940c7a398e4b70f3615dd7565b901c90505b6108008216156139e05760806139db826fe7159475a2c29b7443b29c7fa6e889d9615dd7565b901c90505b611000821615613a0b576080613a06826fd097f3bdfd2022b8845ad8f792aa5825615dd7565b901c90505b612000821615613a36576080613a31826fa9f746462d870fdf8a65dc1f90e061e5615dd7565b901c90505b614000821615613a61576080613a5c826f70d869a156d2a1b890bb3df62baf32f7615dd7565b901c90505b618000821615613a8c576080613a87826f31be135f97d08fd981231505542fcfa6615dd7565b901c90505b62010000821615613ab8576080613ab3826f09aa508b5b7a84e1c677de54f3e99bc9615dd7565b901c90505b60008460020b1315613ad357613ad081600019615b3e565b90505b613ae264010000000082615df6565b15613aee576001613af1565b60005b613b029060ff16602083901c615b10565b949350505050565b6000613b15426144e1565b905090565b6020810151815160608301516000928392839283926001600160a01b039081169216919091101590828112801591840390613bc35782613b7257613b6d886000015189602001518a60400151600161452e565b613b8b565b613b8b886020015189600001518a60400151600161459e565b955085886060015110613ba45787602001519650613c2b565b613bbc886000015189604001518a60600151866146a1565b9650613c2b565b82613be657613be1886000015189602001518a60400151600061459e565b613bff565b613bff886020015189600001518a60400151600061452e565b9450848110613c145787602001519650613c2b565b613c2888600001518960400151838661474e565b96505b60208801516001600160a01b0388811691161460008415613ca757818015613c505750835b613c6e57613c69898b600001518c60400151600161459e565b613c70565b875b9750818015613c7d575083155b613c9b57613c96898b600001518c60400151600061452e565b613c9d565b865b9650869050613d04565b818015613cb15750835b613ccf57613cca8a600001518a8c60400151600161452e565b613cd1565b875b9750818015613cde575083155b613cfc57613cf78a600001518a8c60400151600061459e565b613cfe565b865b96508790505b83158015613d1157508287115b15613d1a578296505b613d35613d26826144e1565b8b60a001518c608001516147fb565b955050505050509193509193565b6000600160ff1b8210613d885760405162461bcd60e51b815260206004820152600d60248201526c746f496e74323536206f666c6f60981b6044820152606401610958565b5090565b600080600080613db28560c00151600160801b8860c001516001600160801b0316614836565b8660e00151613dc19190615b10565b9350613de3856101000151866101400151886101a0015160015460025461494f565b9050613e06856101400151600160801b8860c001516001600160801b03166149d4565b8660a00151613e159190615b52565b9250613e3381600160801b8860c001516001600160801b03166149d4565b8660800151613e429190615b52565b915092959194509250565b600284900b60009081526020869052604081206003810154613e6f9084615a9d565b60038201556001810154613e839086615ab4565b60018201556002810154613e979085615ab4565b600282015554600160801b9004600f0b9695505050505050565b60008082600f0b1215613ed5576000829003613ecd8185615e0a565b915050613ee2565b613edf8284615e32565b90505b92915050565b607880546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b600080613f4b8362010deb19615c9a565b613f599062010deb19615b93565b90506000613f6682615a83565b9050600084613f758484615b93565b613f7f9190615c60565b613f8a906001615e5d565b905061307362ffffff82166001600160801b03615e7b565b604554610100900460ff16613fc95760405162461bcd60e51b815260040161095890615ea1565b612206614a0c565b604554610100900460ff166122065760405162461bcd60e51b815260040161095890615ea1565b60008061400d83602001518460400151612f62565b6020830151600f546060850151600a54600b5460085460045461405296600d969095600160a01b90910460020b949093909290916000906001600160801b0316614a3c565b6040840151600f546060860151600a54600b5460085460045496985061409696600d9695600160a01b900460020b94939291906001906001600160801b0316614a3c565b905081156140b5576020830151600c546140b591600e9160020b614bd2565b80156140d2576040830151600c546140d291600e9160020b614bd2565b915091565b600290810b600090815260209290925260408220828155600181018390559081018290556003810191909155600401805460ff19169055565b6001600160a01b0381163b61417d5760405162461bcd60e51b815260206004820152602d60248201527f455243313936373a206e657720696d706c656d656e746174696f6e206973206e60448201526c1bdd08184818dbdb9d1c9858dd609a1b6064820152608401610958565b60008051602061609483398151915280546001600160a01b0319166001600160a01b0392909216919091179055565b6141b583614c68565b6000825111806141c25750805b15612a55576141d18383614ca8565b50505050565b6000808760020b8660020b126141ee5750826141fb565b6141f88486615ab4565b90505b60008760020b8760020b121561421257508261421f565b61421c8487615ab4565b90505b600061422b8284615b52565b6142359088615ab4565b9a9950505050505050505050565b600281900b60081d600061425961010084615c9a565b9050915091565b60008082116142a15760405162461bcd60e51b815260206004820152600d60248201526c078206d757374206265203e203609c1b6044820152606401610958565b600160801b82106142bf57608091821c916142bc9082615eec565b90505b6801000000000000000082106142e257604091821c916142df9082615eec565b90505b640100000000821061430157602091821c916142fe9082615eec565b90505b62010000821061431e57601091821c9161431b9082615eec565b90505b610100821061433a57600891821c916143379082615eec565b90505b6010821061435557600491821c916143529082615eec565b90505b6004821061437057600291821c9161436d9082615eec565b90505b60028210612b7d57613ee2600182615eec565b60008082116143c45760405162461bcd60e51b815260206004820152600d60248201526c078206d757374206265203e203609c1b6044820152606401610958565b5060ff6001600160801b038216156143e8576143e1608082615d6d565b90506143f0565b608082901c91505b67ffffffffffffffff8216156144125761440b604082615d6d565b905061441a565b604082901c91505b63ffffffff82161561443857614431602082615d6d565b9050614440565b602082901c91505b61ffff82161561445c57614455601082615d6d565b9050614464565b601082901c91505b60ff82161561447f57614478600882615d6d565b9050614487565b600882901c91505b600f8216156144a25761449b600482615d6d565b90506144aa565b600482901c91505b60038216156144c5576144be600282615d6d565b90506144cd565b600282901c91505b6001821615612b7d57613ee2600182615d6d565b60007812725dd1d243aba0e75fe645cc4873f9e65afe688c928e1f2182111561452057604051633492ffd960e01b815260048101839052602401610958565b50670de0b6b3a76400000290565b6000836001600160a01b0316856001600160a01b0316111561454e579293925b8161457b57614576836001600160801b03168686036001600160a01b0316600160601b614836565b613073565b613073836001600160801b03168686036001600160a01b0316600160601b614d93565b6000836001600160a01b0316856001600160a01b031611156145be579293925b600160601b600160e01b03606084901b1660006145db8787615f11565b6001600160a01b031690506000876001600160a01b0316116146345760405162461bcd60e51b8152602060048201526012602482015271073717274526174696f4158393620213e20360741b6044820152606401610958565b8361466a57866001600160a01b03166146578383896001600160a01b0316614836565b8161466457614664615b28565b04614696565b6146966146818383896001600160a01b0316614d93565b886001600160a01b0316808204910615150190565b979650505050505050565b600080856001600160a01b0316116146eb5760405162461bcd60e51b815260206004820152600d60248201526c0737172745058393620213e203609c1b6044820152606401610958565b6000846001600160801b03161161472f5760405162461bcd60e51b815260206004820152600860248201526706c697120213e20360c41b6044820152606401610958565b81614741576145768585856001614e01565b6130738585856001614f1a565b600080856001600160a01b0316116147985760405162461bcd60e51b815260206004820152600d60248201526c0737172745058393620213e203609c1b6044820152606401610958565b6000846001600160801b0316116147dc5760405162461bcd60e51b815260206004820152600860248201526706c697120213e20360c41b6044820152606401610958565b816147ee576145768585856000614f1a565b6130738585856000614e01565b60008061480784615066565b9050600061481e86614819868561507d565b61507d565b9050670de0b6b3a764000081045b9695505050505050565b6000808060001985870985870292508281108382030391505080600014156148a6576000841161489b5760405162461bcd60e51b815260206004820152601060248201526f4469766973696f6e206279207a65726f60801b6044820152606401610958565b508290049050614948565b8084116148e05760405162461bcd60e51b81526020600482015260086024820152676f766572666c6f7760c01b6044820152606401610958565b600084868809851960019081018716968790049682860381900495909211909303600082900391909104909201919091029190911760038402600290811880860282030280860282030280860282030280860282030280860282030280860290910302029150505b9392505050565b60008282116149705760405162461bcd60e51b815260040161095890615f31565b8515801561497c575084155b1561498957506000613073565b600061499487615089565b905060006149a187615089565b905060006149b28383898989615106565b905060006149c284838989615132565b9050670de0b6b3a76400008105614235565b600080841215614a01576149f16149ea85615af3565b8484614836565b6149fa90615af3565b9050614948565b613b02848484614836565b604554610100900460ff16614a335760405162461bcd60e51b815260040161095890615ea1565b61220633613ee8565b600288900b600090815260208a90526040812080546001600160801b031682614a658a83615f4f565b600f0b1215614ab65760405162461bcd60e51b815260206004820152601c60248201527f6e6f7420656e6f756768206c697175696469747920746f206275726e000000006044820152606401610958565b6000614ac2828b613eb1565b9050846001600160801b0316816001600160801b03161115614b0b5760405162461bcd60e51b81526020600482015260026024820152614c4f60f01b6044820152606401610958565b6001600160801b038281161590821615811415945015614b58578a60020b8c60020b13614b48576003830187905560018301899055600283018890555b60048301805460ff191660011790555b82546001600160801b0319166001600160801b03821617835585614b92578254614b8d908b90600160801b9004600f0b615f4f565b614ba9565b8254614ba9908b90600160801b9004600f0b615f95565b83546001600160801b03918216600160801b0291161790925550909a9950505050505050505050565b614bdc8183615c9a565b60020b15614c2c5760405162461bcd60e51b815260206004820152601c60248201527f7469636b206d7573742062652070726f7065726c7920737061636564000000006044820152606401610958565b600080614c3c6136ac8486615c60565b600191820b60009081526020979097526040909620805460ff9097169190911b90951890945550505050565b614c7181614110565b6040516001600160a01b038216907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b90600090a250565b60606001600160a01b0383163b614d105760405162461bcd60e51b815260206004820152602660248201527f416464726573733a2064656c65676174652063616c6c20746f206e6f6e2d636f6044820152651b9d1c9858dd60d21b6064820152608401610958565b600080846001600160a01b031684604051614d2b9190616007565b600060405180830381855af49150503d8060008114614d66576040519150601f19603f3d011682016040523d82523d6000602084013e614d6b565b606091505b509150915061307382826040518060600160405280602781526020016160b460279139615173565b6000614da0848484614836565b905060008280614db257614db2615b28565b8486091115614948576000198110614df75760405162461bcd60e51b81526020600482015260086024820152676f766572666c6f7760c01b6044820152606401610958565b8061307381616023565b60008115614e6b5760006001600160a01b03841115614e3757614e3284600160601b876001600160801b0316614836565b614e4e565b614e4e6001600160801b038616606086901b615b3e565b9050614e598161518c565b614e63908761603e565b915050613b02565b60006001600160a01b03841115614e9957614e9484600160601b876001600160801b0316614d93565b614eb6565b614eb6606085901b6001600160801b038716808204910615150190565b905080866001600160a01b031611614f075760405162461bcd60e51b81526020600482015260146024820152731cdc5c9d14160e4d88084f881c5d5bdd1a595b9d60621b6044820152606401610958565b614e63816001600160a01b038816615a9d565b600082614f28575083613b02565b600160601b600160e01b03606085901b168215614fd35760006001600160a01b03871685614f568282615dd7565b9250614f629083615b3e565b1415614f9d576000614f748284615b10565b9050828110614f9b57614f9183896001600160a01b031683614d93565b9350505050613b02565b505b614fca8286614fb56001600160a01b038b1683615b3e565b614fbf9190615b10565b808204910615150190565b92505050613b02565b60006001600160a01b03871685614fea8282615dd7565b9250614ff69083615b3e565b14801561500257508082115b61503c5760405162461bcd60e51b815260206004820152600b60248201526a64656e6f6d2075666c6f7760a81b6044820152606401610958565b60006150488284615a9d565b9050614f91615061848a6001600160a01b031684614d93565b61518c565b6000613ee2826a1a1601fc4ea7109e0000006151d6565b6000613edf83836151eb565b60007809392ee8e921d5d073aff322e62439fcf32d7f344649470f8f198212156150c95760405163e608e18b60e01b815260048101839052602401610958565b7809392ee8e921d5d073aff322e62439fcf32d7f344649470f90821315614520576040516371f72a3160e01b815260048101839052602401610958565b600061511285856152ad565b61512861512160008686615372565b88906152ad565b61482c9190615b52565b60008282116151535760405162461bcd60e51b815260040161095890615f31565b61516961516260018585615372565b8590615424565b6130739086615ab4565b60608315615182575081614948565b61494883836154ea565b806001600160a01b0381168114612b7d5760405162461bcd60e51b815260206004820152600e60248201526d746f55696e74313630206f666c6f60901b6044820152606401610958565b6000613edf83670de0b6b3a764000084615514565b60008080600019848609848602925082811083820303915050670de0b6b3a7640000811061522f5760405163698d9a0160e11b815260048101829052602401610958565b600080670de0b6b3a76400008688099150506706f05b59d3b1ffff8111826152695780670de0b6b3a7640000850401945050505050613ee2565b620400008285030493909111909103600160ee1b02919091177faccb18165bd6fe31ae1cf318dc5b51eee0e1ba569b88cd74c1773b91fac106690201905092915050565b6000600160ff1b8314806152c45750600160ff1b82145b156152e257604051630d01a11b60e21b815260040160405180910390fd5b600080600085126152f357846152f8565b846000035b915060008412615308578361530d565b836000035b9050600061531b83836151eb565b90506001600160ff1b038111156153485760405163bf79e8d960e01b815260048101829052602401610958565b6000198087139086138082186001146153615782615366565b826000035b98975050505050505050565b60008282116153935760405162461bcd60e51b815260040161095890615f31565b600061539d613b0a565b9050838110156153d75760405162461bcd60e51b8152602060048201526005602482015264422e543c5360d81b6044820152606401610958565b600085806153e55750838210155b156153fb576153f48585615a9d565b9050615408565b6154058583615a9d565b90505b61482c68056bc75e2d6310000061541e83615066565b906151d6565b6000600160ff1b83148061543b5750600160ff1b82145b156154595760405163b3c754a360e01b815260040160405180910390fd5b6000806000851261546a578461546f565b846000035b91506000841261547f5783615484565b836000035b9050600061549b83670de0b6b3a764000084615514565b90506001600160ff1b038111156154c857604051637cb4bef560e01b815260048101829052602401610958565b6000198087139086138082186001146154e15782615366565b61536683615af3565b8151156154fa5781518083602001fd5b8060405162461bcd60e51b81526004016109589190616060565b60008080600019858709858702925082811083820303915050806000141561554f5783828161554557615545615b28565b0492505050614948565b8381106148e057604051631dcf306360e21b81526004810182905260248101859052604401610958565b60405180610160016040528060006001600160a01b03168152602001600060020b815260200160001515815260200160006001600160a01b03168152602001600081526020016000815260200160008152602001600081526020016000815260200160008152602001600081525090565b8015158114610dd457600080fd5b60006020828403121561560a57600080fd5b8135614948816155ea565b6001600160a01b0381168114610dd457600080fd5b8035600281900b8114612b7d57600080fd5b6000806000806080858703121561565257600080fd5b843561565d81615615565b935061566b6020860161562a565b92506156796040860161562a565b915060608501356001600160801b038116811461569557600080fd5b939692955090935050565b6000602082840312156156b257600080fd5b813561494881615615565b600080604083850312156156d057600080fd5b6156d98361562a565b91506156e76020840161562a565b90509250929050565b6000806040838503121561570357600080fd5b823561570e81615615565b9150602083013561571e816155ea565b809150509250929050565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f1916810167ffffffffffffffff8111828210171561576857615768615729565b604052919050565b6000806040838503121561578357600080fd5b823561578e81615615565b915060208381013567ffffffffffffffff808211156157ac57600080fd5b818601915086601f8301126157c057600080fd5b8135818111156157d2576157d2615729565b6157e4601f8201601f1916850161573f565b915080825287848285010111156157fa57600080fd5b80848401858401376000848284010152508093505050509250929050565b60006020828403121561582a57600080fd5b81358060010b811461494857600080fd5b600060a0828403121561584d57600080fd5b60405160a0810181811067ffffffffffffffff8211171561587057615870615729565b604052823561587e81615615565b815260208381013590820152604083013561589881615615565b60408201526158a96060840161562a565b60608201526158ba6080840161562a565b60808201529392505050565b6000602082840312156158d857600080fd5b5035919050565b6000602082840312156158f157600080fd5b813560ff8116811461494857600080fd5b6000806040838503121561591557600080fd5b82356156d981615615565b60006020828403121561593257600080fd5b613edf8261562a565b60006020828403121561594d57600080fd5b815161494881615615565b60208082526006908201526514185d5cd95960d21b604082015260600190565b60006020828403121561598a57600080fd5b8151614948816155ea565b634e487b7160e01b600052601160045260246000fd5b600081600f0b60016001607f1b03198114156159c9576159c9615995565b60000392915050565b6020808252602c908201527f46756e6374696f6e206d7573742062652063616c6c6564207468726f7567682060408201526b19195b1959d85d1958d85b1b60a21b606082015260800190565b6020808252602c908201527f46756e6374696f6e206d7573742062652063616c6c6564207468726f7567682060408201526b6163746976652070726f787960a01b606082015260800190565b600060208284031215615a7c57600080fd5b5051919050565b60008160020b627fffff198114156159c9576159c9615995565b600082821015615aaf57615aaf615995565b500390565b60008083128015600160ff1b850184121615615ad257615ad2615995565b6001600160ff1b0384018313811615615aed57615aed615995565b50500390565b6000600160ff1b821415615b0957615b09615995565b5060000390565b60008219821115615b2357615b23615995565b500190565b634e487b7160e01b600052601260045260246000fd5b600082615b4d57615b4d615b28565b500490565b600080821280156001600160ff1b0384900385131615615b7457615b74615995565b600160ff1b8390038412811615615b8d57615b8d615995565b50500190565b60008160020b8360020b6000811281627fffff1901831281151615615bba57615bba615995565b81627fffff018313811615615bd157615bd1615995565b5090039392505050565b60006001600160ff1b0381841382841380821686840486111615615c0157615c01615995565b600160ff1b6000871282811687830589121615615c2057615c20615995565b60008712925087820587128484161615615c3c57615c3c615995565b87850587128184161615615c5257615c52615995565b505050929093029392505050565b60008160020b8360020b80615c7757615c77615b28565b627fffff19821460001982141615615c9157615c91615995565b90059392505050565b60008260020b80615cad57615cad615b28565b808360020b0791505092915050565b60008160020b627fffff19811415615cd657615cd6615995565b6000190192915050565b60008160020b8360020b627fffff600082136000841383830485118282161615615d0c57615d0c615995565b627fffff196000851282811687830587121615615d2b57615d2b615995565b60008712925085820587128484161615615d4757615d47615995565b85850587128184161615615d5d57615d5d615995565b5050509290910295945050505050565b600060ff821660ff841680821015615d8757615d87615995565b90039392505050565b60008160020b8360020b6000821282627fffff03821381151615615db657615db6615995565b82627fffff19038212811615615dce57615dce615995565b50019392505050565b6000816000190483118215151615615df157615df1615995565b500290565b600082615e0557615e05615b28565b500690565b60006001600160801b0383811690831681811015615e2a57615e2a615995565b039392505050565b60006001600160801b03808316818516808303821115615e5457615e54615995565b01949350505050565b600062ffffff808316818516808303821115615e5457615e54615995565b60006001600160801b0380841680615e9557615e95615b28565b92169190910492915050565b6020808252602b908201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960408201526a6e697469616c697a696e6760a81b606082015260800190565b600060ff821660ff84168060ff03821115615f0957615f09615995565b019392505050565b60006001600160a01b0383811690831681811015615e2a57615e2a615995565b602080825260049082015263453c3d5360e01b604082015260600190565b600081600f0b83600f0b600082128260016001607f1b0303821381151615615f7957615f79615995565b8260016001607f1b0319038212811615615dce57615dce615995565b600081600f0b83600f0b600081128160016001607f1b031901831281151615615fc057615fc0615995565b8160016001607f1b03018313811615615bd157615bd1615995565b60005b83811015615ff6578181015183820152602001615fde565b838111156141d15750506000910152565b60008251616019818460208701615fdb565b9190910192915050565b600060001982141561603757616037615995565b5060010190565b60006001600160a01b03828116848216808303821115615e5457615e54615995565b602081526000825180602084015261607f816040850160208701615fdb565b601f01601f1916919091016040019291505056fe360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc416464726573733a206c6f772d6c6576656c2064656c65676174652063616c6c206661696c6564a264697066735822122029d5e7187c34afa2192e20c96f53058b5eed68b78c7cfa2c860e6e8322b4709b64736f6c63430008090033" . parse () . expect ("invalid bytecode")
        });
    pub struct VAMM<M>(ethers::contract::Contract<M>);
    impl<M> Clone for VAMM<M> {
        fn clone(&self) -> Self {
            VAMM(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for VAMM<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for VAMM<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(VAMM))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> VAMM<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), VAMM_ABI.clone(), client).into()
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
                VAMM_ABI.clone(),
                VAMM_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `MAX_FEE` (0xbc063e1a) function"]
        pub fn max_fee(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([188, 6, 62, 26], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `VOLTZ_PAUSER` (0x1b398e06) function"]
        pub fn voltz_pauser(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([27, 57, 142, 6], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `burn` (0x1f2f0893) function"]
        pub fn burn(
            &self,
            recipient: ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
            amount: u128,
        ) -> ethers::contract::builders::ContractCall<M, I256> {
            self.0
                .method_hash(
                    [31, 47, 8, 147],
                    (recipient, tick_lower, tick_upper, amount),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `changePauser` (0x478de324) function"]
        pub fn change_pauser(
            &self,
            account: ethers::core::types::Address,
            permission: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([71, 141, 227, 36], (account, permission))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `computeGrowthInside` (0x3c8f233e) function"]
        pub fn compute_growth_inside(
            &self,
            tick_lower: i32,
            tick_upper: i32,
        ) -> ethers::contract::builders::ContractCall<M, (I256, I256, ethers::core::types::U256)>
        {
            self.0
                .method_hash([60, 143, 35, 62], (tick_lower, tick_upper))
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
        #[doc = "Calls the contract's `feeGrowthGlobalX128` (0xad23b416) function"]
        pub fn fee_growth_global_x128(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([173, 35, 180, 22], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `feeWad` (0xf3f94990) function"]
        pub fn fee_wad(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([243, 249, 73, 144], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fixedTokenGrowthGlobalX128` (0x09d7b622) function"]
        pub fn fixed_token_growth_global_x128(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, I256> {
            self.0
                .method_hash([9, 215, 182, 34], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRateOracle` (0x77876236) function"]
        pub fn get_rate_oracle(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([119, 135, 98, 54], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0xd992d908) function"]
        pub fn initialize(
            &self,
            margin_engine: ethers::core::types::Address,
            tick_spacing: i32,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([217, 146, 217, 8], (margin_engine, tick_spacing))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initializeVAMM` (0x47f75ede) function"]
        pub fn initialize_vamm(
            &self,
            sqrt_price_x96: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([71, 247, 94, 222], sqrt_price_x96)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isAlpha` (0x88428752) function"]
        pub fn is_alpha(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([136, 66, 135, 82], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liquidity` (0x1a686502) function"]
        pub fn liquidity(&self) -> ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([26, 104, 101, 2], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `marginEngine` (0x004006e0) function"]
        pub fn margin_engine(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([0, 64, 6, 224], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `maxLiquidityPerTick` (0x70cf754a) function"]
        pub fn max_liquidity_per_tick(&self) -> ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([112, 207, 117, 74], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mint` (0xb8cca34e) function"]
        pub fn mint(
            &self,
            recipient: ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
            amount: u128,
        ) -> ethers::contract::builders::ContractCall<M, I256> {
            self.0
                .method_hash(
                    [184, 204, 163, 78],
                    (recipient, tick_lower, tick_upper, amount),
                )
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
        #[doc = "Calls the contract's `protocolFees` (0x1ad8b03b) function"]
        pub fn protocol_fees(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([26, 216, 176, 59], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `proxiableUUID` (0x52d1902d) function"]
        pub fn proxiable_uuid(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([82, 209, 144, 45], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `refreshRateOracle` (0x0bd9c1af) function"]
        pub fn refresh_rate_oracle(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([11, 217, 193, 175], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setFee` (0x69fe0e2d) function"]
        pub fn set_fee(
            &self,
            new_fee_wad: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([105, 254, 14, 45], new_fee_wad)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setFeeProtocol` (0xb613a141) function"]
        pub fn set_fee_protocol(
            &self,
            fee_protocol: u8,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([182, 19, 161, 65], fee_protocol)
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
        #[doc = "Calls the contract's `setPausability` (0x0d211954) function"]
        pub fn set_pausability(
            &self,
            state: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 33, 25, 84], state)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swap` (0x67758e6e) function"]
        pub fn swap(
            &self,
            params: SwapParams,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (I256, I256, ethers::core::types::U256, I256, I256),
        > {
            self.0
                .method_hash([103, 117, 142, 110], (params,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tickBitmap` (0x5339c296) function"]
        pub fn tick_bitmap(
            &self,
            word_position: i16,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([83, 57, 194, 150], word_position)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tickSpacing` (0xd0c93a7c) function"]
        pub fn tick_spacing(&self) -> ethers::contract::builders::ContractCall<M, i32> {
            self.0
                .method_hash([208, 201, 58, 124], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ticks` (0xf30dba93) function"]
        pub fn ticks(&self, tick: i32) -> ethers::contract::builders::ContractCall<M, Info> {
            self.0
                .method_hash([243, 13, 186, 147], tick)
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
        #[doc = "Calls the contract's `updateProtocolFees` (0x86737710) function"]
        pub fn update_protocol_fees(
            &self,
            protocol_fees_collected: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([134, 115, 119, 16], protocol_fees_collected)
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
        #[doc = "Calls the contract's `vammVars` (0x80a0f76c) function"]
        pub fn vamm_vars(&self) -> ethers::contract::builders::ContractCall<M, Vammvars> {
            self.0
                .method_hash([128, 160, 247, 108], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `variableTokenGrowthGlobalX128` (0x4e65b408) function"]
        pub fn variable_token_growth_global_x128(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, I256> {
            self.0
                .method_hash([78, 101, 180, 8], ())
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
        #[doc = "Gets the contract's `Burn` event"]
        pub fn burn_filter(&self) -> ethers::contract::builders::Event<M, BurnFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Fee` event"]
        pub fn fee_filter(&self) -> ethers::contract::builders::Event<M, FeeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `FeeProtocol` event"]
        pub fn fee_protocol_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, FeeProtocolFilter> {
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
        #[doc = "Gets the contract's `Mint` event"]
        pub fn mint_filter(&self) -> ethers::contract::builders::Event<M, MintFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Swap` event"]
        pub fn swap_filter(&self) -> ethers::contract::builders::Event<M, SwapFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Upgraded` event"]
        pub fn upgraded_filter(&self) -> ethers::contract::builders::Event<M, UpgradedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `VAMMInitialization` event"]
        pub fn vamm_initialization_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, VamminitializationFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `VAMMPriceChange` event"]
        pub fn vamm_price_change_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, VammpriceChangeFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, VAMMEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for VAMM<M> {
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
    pub enum VAMMErrors {
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
        PRBMathSD59x18__FromIntOverflow(PRBMathSD59x18__FromIntOverflow),
        PRBMathSD59x18__FromIntUnderflow(PRBMathSD59x18__FromIntUnderflow),
        PRBMathSD59x18__MulInputTooSmall(PRBMathSD59x18__MulInputTooSmall),
        PRBMathSD59x18__MulOverflow(PRBMathSD59x18__MulOverflow),
        PRBMathUD60x18__FromUintOverflow(PRBMathUD60x18__FromUintOverflow),
        PRBMath__MulDivFixedPointOverflow(PRBMath__MulDivFixedPointOverflow),
        PRBMath__MulDivOverflow(PRBMath__MulDivOverflow),
        PositionNetZero(PositionNetZero),
        PositionNotSettled(PositionNotSettled),
        RocketPoolGetEthValueReturnedZero(RocketPoolGetEthValueReturnedZero),
        WithdrawalExceedsCurrentMargin(WithdrawalExceedsCurrentMargin),
        closeToOrBeyondMaturity(closeToOrBeyondMaturity),
    }
    impl ethers::core::abi::AbiDecode for VAMMErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok (decoded) = < AavePoolGetReserveNormalizedIncomeReturnedZero as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (VAMMErrors :: AavePoolGetReserveNormalizedIncomeReturnedZero (decoded)) }
            if let Ok (decoded) = < AavePoolGetReserveNormalizedVariableDebtReturnedZero as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (VAMMErrors :: AavePoolGetReserveNormalizedVariableDebtReturnedZero (decoded)) }
            if let Ok(decoded) =
                <CTokenExchangeRateReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAMMErrors::CTokenExchangeRateReturnedZero(decoded));
            }
            if let Ok(decoded) =
                <CanOnlyTradeIfUnlocked as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMErrors::CanOnlyTradeIfUnlocked(decoded));
            }
            if let Ok(decoded) =
                <CannotLiquidate as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMErrors::CannotLiquidate(decoded));
            }
            if let Ok(decoded) =
                <CannotSettleBeforeMaturity as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMErrors::CannotSettleBeforeMaturity(decoded));
            }
            if let Ok(decoded) = <DebugError as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMErrors::DebugError(decoded));
            }
            if let Ok(decoded) =
                <ExpectedOppositeSigns as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMErrors::ExpectedOppositeSigns(decoded));
            }
            if let Ok(decoded) =
                <ExpectedSqrtPriceZeroBeforeInit as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAMMErrors::ExpectedSqrtPriceZeroBeforeInit(decoded));
            }
            if let Ok(decoded) =
                <IRSNotionalAmountSpecifiedMustBeNonZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAMMErrors::IRSNotionalAmountSpecifiedMustBeNonZero(decoded));
            }
            if let Ok(decoded) =
                <InvalidMarginDelta as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMErrors::InvalidMarginDelta(decoded));
            }
            if let Ok(decoded) =
                <LidoGetPooledEthBySharesReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAMMErrors::LidoGetPooledEthBySharesReturnedZero(decoded));
            }
            if let Ok(decoded) =
                <LiquidityDeltaMustBePositiveInBurn as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAMMErrors::LiquidityDeltaMustBePositiveInBurn(decoded));
            }
            if let Ok(decoded) =
                <LiquidityDeltaMustBePositiveInMint as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAMMErrors::LiquidityDeltaMustBePositiveInMint(decoded));
            }
            if let Ok(decoded) =
                <MarginLessThanMinimum as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMErrors::MarginLessThanMinimum(decoded));
            }
            if let Ok(decoded) =
                <MarginRequirementNotMet as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMErrors::MarginRequirementNotMet(decoded));
            }
            if let Ok(decoded) =
                <MarginRequirementNotMetFCM as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMErrors::MarginRequirementNotMetFCM(decoded));
            }
            if let Ok(decoded) =
                <NotEnoughFunds as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMErrors::NotEnoughFunds(decoded));
            }
            if let Ok(decoded) = <OOO as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(VAMMErrors::OOO(decoded));
            }
            if let Ok(decoded) = <OnlyFCM as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(VAMMErrors::OnlyFCM(decoded));
            }
            if let Ok(decoded) =
                <OnlyMarginEngine as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMErrors::OnlyMarginEngine(decoded));
            }
            if let Ok(decoded) =
                <OnlyOwnerCanUpdatePosition as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMErrors::OnlyOwnerCanUpdatePosition(decoded));
            }
            if let Ok(decoded) = <OnlyVAMM as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(VAMMErrors::OnlyVAMM(decoded));
            }
            if let Ok(decoded) =
                <PRBMathSD59x18__DivInputTooSmall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAMMErrors::PRBMathSD59x18__DivInputTooSmall(decoded));
            }
            if let Ok(decoded) =
                <PRBMathSD59x18__DivOverflow as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMErrors::PRBMathSD59x18__DivOverflow(decoded));
            }
            if let Ok(decoded) =
                <PRBMathSD59x18__FromIntOverflow as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAMMErrors::PRBMathSD59x18__FromIntOverflow(decoded));
            }
            if let Ok(decoded) =
                <PRBMathSD59x18__FromIntUnderflow as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAMMErrors::PRBMathSD59x18__FromIntUnderflow(decoded));
            }
            if let Ok(decoded) =
                <PRBMathSD59x18__MulInputTooSmall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAMMErrors::PRBMathSD59x18__MulInputTooSmall(decoded));
            }
            if let Ok(decoded) =
                <PRBMathSD59x18__MulOverflow as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMErrors::PRBMathSD59x18__MulOverflow(decoded));
            }
            if let Ok(decoded) =
                <PRBMathUD60x18__FromUintOverflow as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAMMErrors::PRBMathUD60x18__FromUintOverflow(decoded));
            }
            if let Ok(decoded) =
                <PRBMath__MulDivFixedPointOverflow as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAMMErrors::PRBMath__MulDivFixedPointOverflow(decoded));
            }
            if let Ok(decoded) =
                <PRBMath__MulDivOverflow as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMErrors::PRBMath__MulDivOverflow(decoded));
            }
            if let Ok(decoded) =
                <PositionNetZero as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMErrors::PositionNetZero(decoded));
            }
            if let Ok(decoded) =
                <PositionNotSettled as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMErrors::PositionNotSettled(decoded));
            }
            if let Ok(decoded) =
                <RocketPoolGetEthValueReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAMMErrors::RocketPoolGetEthValueReturnedZero(decoded));
            }
            if let Ok(decoded) =
                <WithdrawalExceedsCurrentMargin as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAMMErrors::WithdrawalExceedsCurrentMargin(decoded));
            }
            if let Ok(decoded) =
                <closeToOrBeyondMaturity as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMErrors::closeToOrBeyondMaturity(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for VAMMErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                VAMMErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(element) => {
                    element.encode()
                }
                VAMMErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(element) => {
                    element.encode()
                }
                VAMMErrors::CTokenExchangeRateReturnedZero(element) => element.encode(),
                VAMMErrors::CanOnlyTradeIfUnlocked(element) => element.encode(),
                VAMMErrors::CannotLiquidate(element) => element.encode(),
                VAMMErrors::CannotSettleBeforeMaturity(element) => element.encode(),
                VAMMErrors::DebugError(element) => element.encode(),
                VAMMErrors::ExpectedOppositeSigns(element) => element.encode(),
                VAMMErrors::ExpectedSqrtPriceZeroBeforeInit(element) => element.encode(),
                VAMMErrors::IRSNotionalAmountSpecifiedMustBeNonZero(element) => element.encode(),
                VAMMErrors::InvalidMarginDelta(element) => element.encode(),
                VAMMErrors::LidoGetPooledEthBySharesReturnedZero(element) => element.encode(),
                VAMMErrors::LiquidityDeltaMustBePositiveInBurn(element) => element.encode(),
                VAMMErrors::LiquidityDeltaMustBePositiveInMint(element) => element.encode(),
                VAMMErrors::MarginLessThanMinimum(element) => element.encode(),
                VAMMErrors::MarginRequirementNotMet(element) => element.encode(),
                VAMMErrors::MarginRequirementNotMetFCM(element) => element.encode(),
                VAMMErrors::NotEnoughFunds(element) => element.encode(),
                VAMMErrors::OOO(element) => element.encode(),
                VAMMErrors::OnlyFCM(element) => element.encode(),
                VAMMErrors::OnlyMarginEngine(element) => element.encode(),
                VAMMErrors::OnlyOwnerCanUpdatePosition(element) => element.encode(),
                VAMMErrors::OnlyVAMM(element) => element.encode(),
                VAMMErrors::PRBMathSD59x18__DivInputTooSmall(element) => element.encode(),
                VAMMErrors::PRBMathSD59x18__DivOverflow(element) => element.encode(),
                VAMMErrors::PRBMathSD59x18__FromIntOverflow(element) => element.encode(),
                VAMMErrors::PRBMathSD59x18__FromIntUnderflow(element) => element.encode(),
                VAMMErrors::PRBMathSD59x18__MulInputTooSmall(element) => element.encode(),
                VAMMErrors::PRBMathSD59x18__MulOverflow(element) => element.encode(),
                VAMMErrors::PRBMathUD60x18__FromUintOverflow(element) => element.encode(),
                VAMMErrors::PRBMath__MulDivFixedPointOverflow(element) => element.encode(),
                VAMMErrors::PRBMath__MulDivOverflow(element) => element.encode(),
                VAMMErrors::PositionNetZero(element) => element.encode(),
                VAMMErrors::PositionNotSettled(element) => element.encode(),
                VAMMErrors::RocketPoolGetEthValueReturnedZero(element) => element.encode(),
                VAMMErrors::WithdrawalExceedsCurrentMargin(element) => element.encode(),
                VAMMErrors::closeToOrBeyondMaturity(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for VAMMErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                VAMMErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(element) => {
                    element.fmt(f)
                }
                VAMMErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(element) => {
                    element.fmt(f)
                }
                VAMMErrors::CTokenExchangeRateReturnedZero(element) => element.fmt(f),
                VAMMErrors::CanOnlyTradeIfUnlocked(element) => element.fmt(f),
                VAMMErrors::CannotLiquidate(element) => element.fmt(f),
                VAMMErrors::CannotSettleBeforeMaturity(element) => element.fmt(f),
                VAMMErrors::DebugError(element) => element.fmt(f),
                VAMMErrors::ExpectedOppositeSigns(element) => element.fmt(f),
                VAMMErrors::ExpectedSqrtPriceZeroBeforeInit(element) => element.fmt(f),
                VAMMErrors::IRSNotionalAmountSpecifiedMustBeNonZero(element) => element.fmt(f),
                VAMMErrors::InvalidMarginDelta(element) => element.fmt(f),
                VAMMErrors::LidoGetPooledEthBySharesReturnedZero(element) => element.fmt(f),
                VAMMErrors::LiquidityDeltaMustBePositiveInBurn(element) => element.fmt(f),
                VAMMErrors::LiquidityDeltaMustBePositiveInMint(element) => element.fmt(f),
                VAMMErrors::MarginLessThanMinimum(element) => element.fmt(f),
                VAMMErrors::MarginRequirementNotMet(element) => element.fmt(f),
                VAMMErrors::MarginRequirementNotMetFCM(element) => element.fmt(f),
                VAMMErrors::NotEnoughFunds(element) => element.fmt(f),
                VAMMErrors::OOO(element) => element.fmt(f),
                VAMMErrors::OnlyFCM(element) => element.fmt(f),
                VAMMErrors::OnlyMarginEngine(element) => element.fmt(f),
                VAMMErrors::OnlyOwnerCanUpdatePosition(element) => element.fmt(f),
                VAMMErrors::OnlyVAMM(element) => element.fmt(f),
                VAMMErrors::PRBMathSD59x18__DivInputTooSmall(element) => element.fmt(f),
                VAMMErrors::PRBMathSD59x18__DivOverflow(element) => element.fmt(f),
                VAMMErrors::PRBMathSD59x18__FromIntOverflow(element) => element.fmt(f),
                VAMMErrors::PRBMathSD59x18__FromIntUnderflow(element) => element.fmt(f),
                VAMMErrors::PRBMathSD59x18__MulInputTooSmall(element) => element.fmt(f),
                VAMMErrors::PRBMathSD59x18__MulOverflow(element) => element.fmt(f),
                VAMMErrors::PRBMathUD60x18__FromUintOverflow(element) => element.fmt(f),
                VAMMErrors::PRBMath__MulDivFixedPointOverflow(element) => element.fmt(f),
                VAMMErrors::PRBMath__MulDivOverflow(element) => element.fmt(f),
                VAMMErrors::PositionNetZero(element) => element.fmt(f),
                VAMMErrors::PositionNotSettled(element) => element.fmt(f),
                VAMMErrors::RocketPoolGetEthValueReturnedZero(element) => element.fmt(f),
                VAMMErrors::WithdrawalExceedsCurrentMargin(element) => element.fmt(f),
                VAMMErrors::closeToOrBeyondMaturity(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AavePoolGetReserveNormalizedIncomeReturnedZero> for VAMMErrors {
        fn from(var: AavePoolGetReserveNormalizedIncomeReturnedZero) -> Self {
            VAMMErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(var)
        }
    }
    impl ::std::convert::From<AavePoolGetReserveNormalizedVariableDebtReturnedZero> for VAMMErrors {
        fn from(var: AavePoolGetReserveNormalizedVariableDebtReturnedZero) -> Self {
            VAMMErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(var)
        }
    }
    impl ::std::convert::From<CTokenExchangeRateReturnedZero> for VAMMErrors {
        fn from(var: CTokenExchangeRateReturnedZero) -> Self {
            VAMMErrors::CTokenExchangeRateReturnedZero(var)
        }
    }
    impl ::std::convert::From<CanOnlyTradeIfUnlocked> for VAMMErrors {
        fn from(var: CanOnlyTradeIfUnlocked) -> Self {
            VAMMErrors::CanOnlyTradeIfUnlocked(var)
        }
    }
    impl ::std::convert::From<CannotLiquidate> for VAMMErrors {
        fn from(var: CannotLiquidate) -> Self {
            VAMMErrors::CannotLiquidate(var)
        }
    }
    impl ::std::convert::From<CannotSettleBeforeMaturity> for VAMMErrors {
        fn from(var: CannotSettleBeforeMaturity) -> Self {
            VAMMErrors::CannotSettleBeforeMaturity(var)
        }
    }
    impl ::std::convert::From<DebugError> for VAMMErrors {
        fn from(var: DebugError) -> Self {
            VAMMErrors::DebugError(var)
        }
    }
    impl ::std::convert::From<ExpectedOppositeSigns> for VAMMErrors {
        fn from(var: ExpectedOppositeSigns) -> Self {
            VAMMErrors::ExpectedOppositeSigns(var)
        }
    }
    impl ::std::convert::From<ExpectedSqrtPriceZeroBeforeInit> for VAMMErrors {
        fn from(var: ExpectedSqrtPriceZeroBeforeInit) -> Self {
            VAMMErrors::ExpectedSqrtPriceZeroBeforeInit(var)
        }
    }
    impl ::std::convert::From<IRSNotionalAmountSpecifiedMustBeNonZero> for VAMMErrors {
        fn from(var: IRSNotionalAmountSpecifiedMustBeNonZero) -> Self {
            VAMMErrors::IRSNotionalAmountSpecifiedMustBeNonZero(var)
        }
    }
    impl ::std::convert::From<InvalidMarginDelta> for VAMMErrors {
        fn from(var: InvalidMarginDelta) -> Self {
            VAMMErrors::InvalidMarginDelta(var)
        }
    }
    impl ::std::convert::From<LidoGetPooledEthBySharesReturnedZero> for VAMMErrors {
        fn from(var: LidoGetPooledEthBySharesReturnedZero) -> Self {
            VAMMErrors::LidoGetPooledEthBySharesReturnedZero(var)
        }
    }
    impl ::std::convert::From<LiquidityDeltaMustBePositiveInBurn> for VAMMErrors {
        fn from(var: LiquidityDeltaMustBePositiveInBurn) -> Self {
            VAMMErrors::LiquidityDeltaMustBePositiveInBurn(var)
        }
    }
    impl ::std::convert::From<LiquidityDeltaMustBePositiveInMint> for VAMMErrors {
        fn from(var: LiquidityDeltaMustBePositiveInMint) -> Self {
            VAMMErrors::LiquidityDeltaMustBePositiveInMint(var)
        }
    }
    impl ::std::convert::From<MarginLessThanMinimum> for VAMMErrors {
        fn from(var: MarginLessThanMinimum) -> Self {
            VAMMErrors::MarginLessThanMinimum(var)
        }
    }
    impl ::std::convert::From<MarginRequirementNotMet> for VAMMErrors {
        fn from(var: MarginRequirementNotMet) -> Self {
            VAMMErrors::MarginRequirementNotMet(var)
        }
    }
    impl ::std::convert::From<MarginRequirementNotMetFCM> for VAMMErrors {
        fn from(var: MarginRequirementNotMetFCM) -> Self {
            VAMMErrors::MarginRequirementNotMetFCM(var)
        }
    }
    impl ::std::convert::From<NotEnoughFunds> for VAMMErrors {
        fn from(var: NotEnoughFunds) -> Self {
            VAMMErrors::NotEnoughFunds(var)
        }
    }
    impl ::std::convert::From<OOO> for VAMMErrors {
        fn from(var: OOO) -> Self {
            VAMMErrors::OOO(var)
        }
    }
    impl ::std::convert::From<OnlyFCM> for VAMMErrors {
        fn from(var: OnlyFCM) -> Self {
            VAMMErrors::OnlyFCM(var)
        }
    }
    impl ::std::convert::From<OnlyMarginEngine> for VAMMErrors {
        fn from(var: OnlyMarginEngine) -> Self {
            VAMMErrors::OnlyMarginEngine(var)
        }
    }
    impl ::std::convert::From<OnlyOwnerCanUpdatePosition> for VAMMErrors {
        fn from(var: OnlyOwnerCanUpdatePosition) -> Self {
            VAMMErrors::OnlyOwnerCanUpdatePosition(var)
        }
    }
    impl ::std::convert::From<OnlyVAMM> for VAMMErrors {
        fn from(var: OnlyVAMM) -> Self {
            VAMMErrors::OnlyVAMM(var)
        }
    }
    impl ::std::convert::From<PRBMathSD59x18__DivInputTooSmall> for VAMMErrors {
        fn from(var: PRBMathSD59x18__DivInputTooSmall) -> Self {
            VAMMErrors::PRBMathSD59x18__DivInputTooSmall(var)
        }
    }
    impl ::std::convert::From<PRBMathSD59x18__DivOverflow> for VAMMErrors {
        fn from(var: PRBMathSD59x18__DivOverflow) -> Self {
            VAMMErrors::PRBMathSD59x18__DivOverflow(var)
        }
    }
    impl ::std::convert::From<PRBMathSD59x18__FromIntOverflow> for VAMMErrors {
        fn from(var: PRBMathSD59x18__FromIntOverflow) -> Self {
            VAMMErrors::PRBMathSD59x18__FromIntOverflow(var)
        }
    }
    impl ::std::convert::From<PRBMathSD59x18__FromIntUnderflow> for VAMMErrors {
        fn from(var: PRBMathSD59x18__FromIntUnderflow) -> Self {
            VAMMErrors::PRBMathSD59x18__FromIntUnderflow(var)
        }
    }
    impl ::std::convert::From<PRBMathSD59x18__MulInputTooSmall> for VAMMErrors {
        fn from(var: PRBMathSD59x18__MulInputTooSmall) -> Self {
            VAMMErrors::PRBMathSD59x18__MulInputTooSmall(var)
        }
    }
    impl ::std::convert::From<PRBMathSD59x18__MulOverflow> for VAMMErrors {
        fn from(var: PRBMathSD59x18__MulOverflow) -> Self {
            VAMMErrors::PRBMathSD59x18__MulOverflow(var)
        }
    }
    impl ::std::convert::From<PRBMathUD60x18__FromUintOverflow> for VAMMErrors {
        fn from(var: PRBMathUD60x18__FromUintOverflow) -> Self {
            VAMMErrors::PRBMathUD60x18__FromUintOverflow(var)
        }
    }
    impl ::std::convert::From<PRBMath__MulDivFixedPointOverflow> for VAMMErrors {
        fn from(var: PRBMath__MulDivFixedPointOverflow) -> Self {
            VAMMErrors::PRBMath__MulDivFixedPointOverflow(var)
        }
    }
    impl ::std::convert::From<PRBMath__MulDivOverflow> for VAMMErrors {
        fn from(var: PRBMath__MulDivOverflow) -> Self {
            VAMMErrors::PRBMath__MulDivOverflow(var)
        }
    }
    impl ::std::convert::From<PositionNetZero> for VAMMErrors {
        fn from(var: PositionNetZero) -> Self {
            VAMMErrors::PositionNetZero(var)
        }
    }
    impl ::std::convert::From<PositionNotSettled> for VAMMErrors {
        fn from(var: PositionNotSettled) -> Self {
            VAMMErrors::PositionNotSettled(var)
        }
    }
    impl ::std::convert::From<RocketPoolGetEthValueReturnedZero> for VAMMErrors {
        fn from(var: RocketPoolGetEthValueReturnedZero) -> Self {
            VAMMErrors::RocketPoolGetEthValueReturnedZero(var)
        }
    }
    impl ::std::convert::From<WithdrawalExceedsCurrentMargin> for VAMMErrors {
        fn from(var: WithdrawalExceedsCurrentMargin) -> Self {
            VAMMErrors::WithdrawalExceedsCurrentMargin(var)
        }
    }
    impl ::std::convert::From<closeToOrBeyondMaturity> for VAMMErrors {
        fn from(var: closeToOrBeyondMaturity) -> Self {
            VAMMErrors::closeToOrBeyondMaturity(var)
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
    #[ethevent(name = "Burn", abi = "Burn(address,address,int24,int24,uint128)")]
    pub struct BurnFilter {
        pub sender: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub tick_lower: i32,
        #[ethevent(indexed)]
        pub tick_upper: i32,
        pub amount: u128,
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
    #[ethevent(name = "Fee", abi = "Fee(uint256)")]
    pub struct FeeFilter {
        pub fee_wad: ethers::core::types::U256,
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
    #[ethevent(name = "FeeProtocol", abi = "FeeProtocol(uint8)")]
    pub struct FeeProtocolFilter {
        pub fee_protocol: u8,
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
    #[ethevent(name = "Mint", abi = "Mint(address,address,int24,int24,uint128)")]
    pub struct MintFilter {
        pub sender: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub tick_lower: i32,
        #[ethevent(indexed)]
        pub tick_upper: i32,
        pub amount: u128,
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
        name = "Swap",
        abi = "Swap(address,address,int24,int24,int256,uint160,uint256,int256,int256,int256)"
    )]
    pub struct SwapFilter {
        pub sender: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub recipient: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub tick_lower: i32,
        #[ethevent(indexed)]
        pub tick_upper: i32,
        pub desired_notional: I256,
        pub sqrt_price_limit_x96: ethers::core::types::U256,
        pub cumulative_fee_incurred: ethers::core::types::U256,
        pub fixed_token_delta: I256,
        pub variable_token_delta: I256,
        pub fixed_token_delta_unbalanced: I256,
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
    #[ethevent(name = "VAMMInitialization", abi = "VAMMInitialization(uint160,int24)")]
    pub struct VamminitializationFilter {
        pub sqrt_price_x96: ethers::core::types::U256,
        pub tick: i32,
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
    #[ethevent(name = "VAMMPriceChange", abi = "VAMMPriceChange(int24)")]
    pub struct VammpriceChangeFilter {
        pub tick: i32,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum VAMMEvents {
        AdminChangedFilter(AdminChangedFilter),
        BeaconUpgradedFilter(BeaconUpgradedFilter),
        BurnFilter(BurnFilter),
        FeeFilter(FeeFilter),
        FeeProtocolFilter(FeeProtocolFilter),
        InitializedFilter(InitializedFilter),
        IsAlphaFilter(IsAlphaFilter),
        MintFilter(MintFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        SwapFilter(SwapFilter),
        UpgradedFilter(UpgradedFilter),
        VamminitializationFilter(VamminitializationFilter),
        VammpriceChangeFilter(VammpriceChangeFilter),
    }
    impl ethers::contract::EthLogDecode for VAMMEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AdminChangedFilter::decode_log(log) {
                return Ok(VAMMEvents::AdminChangedFilter(decoded));
            }
            if let Ok(decoded) = BeaconUpgradedFilter::decode_log(log) {
                return Ok(VAMMEvents::BeaconUpgradedFilter(decoded));
            }
            if let Ok(decoded) = BurnFilter::decode_log(log) {
                return Ok(VAMMEvents::BurnFilter(decoded));
            }
            if let Ok(decoded) = FeeFilter::decode_log(log) {
                return Ok(VAMMEvents::FeeFilter(decoded));
            }
            if let Ok(decoded) = FeeProtocolFilter::decode_log(log) {
                return Ok(VAMMEvents::FeeProtocolFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(VAMMEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = IsAlphaFilter::decode_log(log) {
                return Ok(VAMMEvents::IsAlphaFilter(decoded));
            }
            if let Ok(decoded) = MintFilter::decode_log(log) {
                return Ok(VAMMEvents::MintFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(VAMMEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = SwapFilter::decode_log(log) {
                return Ok(VAMMEvents::SwapFilter(decoded));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(VAMMEvents::UpgradedFilter(decoded));
            }
            if let Ok(decoded) = VamminitializationFilter::decode_log(log) {
                return Ok(VAMMEvents::VamminitializationFilter(decoded));
            }
            if let Ok(decoded) = VammpriceChangeFilter::decode_log(log) {
                return Ok(VAMMEvents::VammpriceChangeFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for VAMMEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                VAMMEvents::AdminChangedFilter(element) => element.fmt(f),
                VAMMEvents::BeaconUpgradedFilter(element) => element.fmt(f),
                VAMMEvents::BurnFilter(element) => element.fmt(f),
                VAMMEvents::FeeFilter(element) => element.fmt(f),
                VAMMEvents::FeeProtocolFilter(element) => element.fmt(f),
                VAMMEvents::InitializedFilter(element) => element.fmt(f),
                VAMMEvents::IsAlphaFilter(element) => element.fmt(f),
                VAMMEvents::MintFilter(element) => element.fmt(f),
                VAMMEvents::OwnershipTransferredFilter(element) => element.fmt(f),
                VAMMEvents::SwapFilter(element) => element.fmt(f),
                VAMMEvents::UpgradedFilter(element) => element.fmt(f),
                VAMMEvents::VamminitializationFilter(element) => element.fmt(f),
                VAMMEvents::VammpriceChangeFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `MAX_FEE` function with signature `MAX_FEE()` and selector `[188, 6, 62, 26]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "MAX_FEE", abi = "MAX_FEE()")]
    pub struct MaxFeeCall;
    #[doc = "Container type for all input parameters for the `VOLTZ_PAUSER` function with signature `VOLTZ_PAUSER()` and selector `[27, 57, 142, 6]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "VOLTZ_PAUSER", abi = "VOLTZ_PAUSER()")]
    pub struct VoltzPauserCall;
    #[doc = "Container type for all input parameters for the `burn` function with signature `burn(address,int24,int24,uint128)` and selector `[31, 47, 8, 147]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "burn", abi = "burn(address,int24,int24,uint128)")]
    pub struct BurnCall {
        pub recipient: ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub amount: u128,
    }
    #[doc = "Container type for all input parameters for the `changePauser` function with signature `changePauser(address,bool)` and selector `[71, 141, 227, 36]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "changePauser", abi = "changePauser(address,bool)")]
    pub struct ChangePauserCall {
        pub account: ethers::core::types::Address,
        pub permission: bool,
    }
    #[doc = "Container type for all input parameters for the `computeGrowthInside` function with signature `computeGrowthInside(int24,int24)` and selector `[60, 143, 35, 62]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "computeGrowthInside", abi = "computeGrowthInside(int24,int24)")]
    pub struct ComputeGrowthInsideCall {
        pub tick_lower: i32,
        pub tick_upper: i32,
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
    #[doc = "Container type for all input parameters for the `feeGrowthGlobalX128` function with signature `feeGrowthGlobalX128()` and selector `[173, 35, 180, 22]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "feeGrowthGlobalX128", abi = "feeGrowthGlobalX128()")]
    pub struct FeeGrowthGlobalX128Call;
    #[doc = "Container type for all input parameters for the `feeWad` function with signature `feeWad()` and selector `[243, 249, 73, 144]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "feeWad", abi = "feeWad()")]
    pub struct FeeWadCall;
    #[doc = "Container type for all input parameters for the `fixedTokenGrowthGlobalX128` function with signature `fixedTokenGrowthGlobalX128()` and selector `[9, 215, 182, 34]`"]
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
        name = "fixedTokenGrowthGlobalX128",
        abi = "fixedTokenGrowthGlobalX128()"
    )]
    pub struct FixedTokenGrowthGlobalX128Call;
    #[doc = "Container type for all input parameters for the `getRateOracle` function with signature `getRateOracle()` and selector `[119, 135, 98, 54]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getRateOracle", abi = "getRateOracle()")]
    pub struct GetRateOracleCall;
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize(address,int24)` and selector `[217, 146, 217, 8]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "initialize", abi = "initialize(address,int24)")]
    pub struct InitializeCall {
        pub margin_engine: ethers::core::types::Address,
        pub tick_spacing: i32,
    }
    #[doc = "Container type for all input parameters for the `initializeVAMM` function with signature `initializeVAMM(uint160)` and selector `[71, 247, 94, 222]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "initializeVAMM", abi = "initializeVAMM(uint160)")]
    pub struct InitializeVAMMCall {
        pub sqrt_price_x96: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `liquidity` function with signature `liquidity()` and selector `[26, 104, 101, 2]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "liquidity", abi = "liquidity()")]
    pub struct LiquidityCall;
    #[doc = "Container type for all input parameters for the `marginEngine` function with signature `marginEngine()` and selector `[0, 64, 6, 224]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "marginEngine", abi = "marginEngine()")]
    pub struct MarginEngineCall;
    #[doc = "Container type for all input parameters for the `maxLiquidityPerTick` function with signature `maxLiquidityPerTick()` and selector `[112, 207, 117, 74]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "maxLiquidityPerTick", abi = "maxLiquidityPerTick()")]
    pub struct MaxLiquidityPerTickCall;
    #[doc = "Container type for all input parameters for the `mint` function with signature `mint(address,int24,int24,uint128)` and selector `[184, 204, 163, 78]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "mint", abi = "mint(address,int24,int24,uint128)")]
    pub struct MintCall {
        pub recipient: ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub amount: u128,
    }
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
    #[doc = "Container type for all input parameters for the `protocolFees` function with signature `protocolFees()` and selector `[26, 216, 176, 59]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "protocolFees", abi = "protocolFees()")]
    pub struct ProtocolFeesCall;
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
    #[doc = "Container type for all input parameters for the `refreshRateOracle` function with signature `refreshRateOracle()` and selector `[11, 217, 193, 175]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "refreshRateOracle", abi = "refreshRateOracle()")]
    pub struct RefreshRateOracleCall;
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
    #[doc = "Container type for all input parameters for the `setFee` function with signature `setFee(uint256)` and selector `[105, 254, 14, 45]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setFee", abi = "setFee(uint256)")]
    pub struct SetFeeCall {
        pub new_fee_wad: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setFeeProtocol` function with signature `setFeeProtocol(uint8)` and selector `[182, 19, 161, 65]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setFeeProtocol", abi = "setFeeProtocol(uint8)")]
    pub struct SetFeeProtocolCall {
        pub fee_protocol: u8,
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
    #[doc = "Container type for all input parameters for the `swap` function with signature `swap((address,int256,uint160,int24,int24))` and selector `[103, 117, 142, 110]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "swap", abi = "swap((address,int256,uint160,int24,int24))")]
    pub struct SwapCall {
        pub params: SwapParams,
    }
    #[doc = "Container type for all input parameters for the `tickBitmap` function with signature `tickBitmap(int16)` and selector `[83, 57, 194, 150]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "tickBitmap", abi = "tickBitmap(int16)")]
    pub struct TickBitmapCall {
        pub word_position: i16,
    }
    #[doc = "Container type for all input parameters for the `tickSpacing` function with signature `tickSpacing()` and selector `[208, 201, 58, 124]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "tickSpacing", abi = "tickSpacing()")]
    pub struct TickSpacingCall;
    #[doc = "Container type for all input parameters for the `ticks` function with signature `ticks(int24)` and selector `[243, 13, 186, 147]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "ticks", abi = "ticks(int24)")]
    pub struct TicksCall {
        pub tick: i32,
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
    #[doc = "Container type for all input parameters for the `updateProtocolFees` function with signature `updateProtocolFees(uint256)` and selector `[134, 115, 119, 16]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "updateProtocolFees", abi = "updateProtocolFees(uint256)")]
    pub struct UpdateProtocolFeesCall {
        pub protocol_fees_collected: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `vammVars` function with signature `vammVars()` and selector `[128, 160, 247, 108]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "vammVars", abi = "vammVars()")]
    pub struct VammVarsCall;
    #[doc = "Container type for all input parameters for the `variableTokenGrowthGlobalX128` function with signature `variableTokenGrowthGlobalX128()` and selector `[78, 101, 180, 8]`"]
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
        name = "variableTokenGrowthGlobalX128",
        abi = "variableTokenGrowthGlobalX128()"
    )]
    pub struct VariableTokenGrowthGlobalX128Call;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum VAMMCalls {
        MaxFee(MaxFeeCall),
        VoltzPauser(VoltzPauserCall),
        Burn(BurnCall),
        ChangePauser(ChangePauserCall),
        ComputeGrowthInside(ComputeGrowthInsideCall),
        Factory(FactoryCall),
        FeeGrowthGlobalX128(FeeGrowthGlobalX128Call),
        FeeWad(FeeWadCall),
        FixedTokenGrowthGlobalX128(FixedTokenGrowthGlobalX128Call),
        GetRateOracle(GetRateOracleCall),
        Initialize(InitializeCall),
        InitializeVAMM(InitializeVAMMCall),
        IsAlpha(IsAlphaCall),
        Liquidity(LiquidityCall),
        MarginEngine(MarginEngineCall),
        MaxLiquidityPerTick(MaxLiquidityPerTickCall),
        Mint(MintCall),
        Owner(OwnerCall),
        Paused(PausedCall),
        ProtocolFees(ProtocolFeesCall),
        ProxiableUUID(ProxiableUUIDCall),
        RefreshRateOracle(RefreshRateOracleCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetFee(SetFeeCall),
        SetFeeProtocol(SetFeeProtocolCall),
        SetIsAlpha(SetIsAlphaCall),
        SetPausability(SetPausabilityCall),
        Swap(SwapCall),
        TickBitmap(TickBitmapCall),
        TickSpacing(TickSpacingCall),
        Ticks(TicksCall),
        TransferOwnership(TransferOwnershipCall),
        UpdateProtocolFees(UpdateProtocolFeesCall),
        UpgradeTo(UpgradeToCall),
        UpgradeToAndCall(UpgradeToAndCallCall),
        VammVars(VammVarsCall),
        VariableTokenGrowthGlobalX128(VariableTokenGrowthGlobalX128Call),
    }
    impl ethers::core::abi::AbiDecode for VAMMCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <MaxFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMCalls::MaxFee(decoded));
            }
            if let Ok(decoded) =
                <VoltzPauserCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMCalls::VoltzPauser(decoded));
            }
            if let Ok(decoded) = <BurnCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(VAMMCalls::Burn(decoded));
            }
            if let Ok(decoded) =
                <ChangePauserCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMCalls::ChangePauser(decoded));
            }
            if let Ok(decoded) =
                <ComputeGrowthInsideCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMCalls::ComputeGrowthInside(decoded));
            }
            if let Ok(decoded) =
                <FactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMCalls::Factory(decoded));
            }
            if let Ok(decoded) =
                <FeeGrowthGlobalX128Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMCalls::FeeGrowthGlobalX128(decoded));
            }
            if let Ok(decoded) = <FeeWadCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMCalls::FeeWad(decoded));
            }
            if let Ok(decoded) =
                <FixedTokenGrowthGlobalX128Call as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAMMCalls::FixedTokenGrowthGlobalX128(decoded));
            }
            if let Ok(decoded) =
                <GetRateOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMCalls::GetRateOracle(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMCalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <InitializeVAMMCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMCalls::InitializeVAMM(decoded));
            }
            if let Ok(decoded) =
                <IsAlphaCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMCalls::IsAlpha(decoded));
            }
            if let Ok(decoded) =
                <LiquidityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMCalls::Liquidity(decoded));
            }
            if let Ok(decoded) =
                <MarginEngineCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMCalls::MarginEngine(decoded));
            }
            if let Ok(decoded) =
                <MaxLiquidityPerTickCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMCalls::MaxLiquidityPerTick(decoded));
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(VAMMCalls::Mint(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMCalls::Owner(decoded));
            }
            if let Ok(decoded) = <PausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMCalls::Paused(decoded));
            }
            if let Ok(decoded) =
                <ProtocolFeesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMCalls::ProtocolFees(decoded));
            }
            if let Ok(decoded) =
                <ProxiableUUIDCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMCalls::ProxiableUUID(decoded));
            }
            if let Ok(decoded) =
                <RefreshRateOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMCalls::RefreshRateOracle(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SetFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMCalls::SetFee(decoded));
            }
            if let Ok(decoded) =
                <SetFeeProtocolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMCalls::SetFeeProtocol(decoded));
            }
            if let Ok(decoded) =
                <SetIsAlphaCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMCalls::SetIsAlpha(decoded));
            }
            if let Ok(decoded) =
                <SetPausabilityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMCalls::SetPausability(decoded));
            }
            if let Ok(decoded) = <SwapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(VAMMCalls::Swap(decoded));
            }
            if let Ok(decoded) =
                <TickBitmapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMCalls::TickBitmap(decoded));
            }
            if let Ok(decoded) =
                <TickSpacingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMCalls::TickSpacing(decoded));
            }
            if let Ok(decoded) = <TicksCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMCalls::Ticks(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <UpdateProtocolFeesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMCalls::UpdateProtocolFees(decoded));
            }
            if let Ok(decoded) =
                <UpgradeToCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMCalls::UpgradeTo(decoded));
            }
            if let Ok(decoded) =
                <UpgradeToAndCallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMCalls::UpgradeToAndCall(decoded));
            }
            if let Ok(decoded) =
                <VammVarsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VAMMCalls::VammVars(decoded));
            }
            if let Ok(decoded) =
                <VariableTokenGrowthGlobalX128Call as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAMMCalls::VariableTokenGrowthGlobalX128(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for VAMMCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                VAMMCalls::MaxFee(element) => element.encode(),
                VAMMCalls::VoltzPauser(element) => element.encode(),
                VAMMCalls::Burn(element) => element.encode(),
                VAMMCalls::ChangePauser(element) => element.encode(),
                VAMMCalls::ComputeGrowthInside(element) => element.encode(),
                VAMMCalls::Factory(element) => element.encode(),
                VAMMCalls::FeeGrowthGlobalX128(element) => element.encode(),
                VAMMCalls::FeeWad(element) => element.encode(),
                VAMMCalls::FixedTokenGrowthGlobalX128(element) => element.encode(),
                VAMMCalls::GetRateOracle(element) => element.encode(),
                VAMMCalls::Initialize(element) => element.encode(),
                VAMMCalls::InitializeVAMM(element) => element.encode(),
                VAMMCalls::IsAlpha(element) => element.encode(),
                VAMMCalls::Liquidity(element) => element.encode(),
                VAMMCalls::MarginEngine(element) => element.encode(),
                VAMMCalls::MaxLiquidityPerTick(element) => element.encode(),
                VAMMCalls::Mint(element) => element.encode(),
                VAMMCalls::Owner(element) => element.encode(),
                VAMMCalls::Paused(element) => element.encode(),
                VAMMCalls::ProtocolFees(element) => element.encode(),
                VAMMCalls::ProxiableUUID(element) => element.encode(),
                VAMMCalls::RefreshRateOracle(element) => element.encode(),
                VAMMCalls::RenounceOwnership(element) => element.encode(),
                VAMMCalls::SetFee(element) => element.encode(),
                VAMMCalls::SetFeeProtocol(element) => element.encode(),
                VAMMCalls::SetIsAlpha(element) => element.encode(),
                VAMMCalls::SetPausability(element) => element.encode(),
                VAMMCalls::Swap(element) => element.encode(),
                VAMMCalls::TickBitmap(element) => element.encode(),
                VAMMCalls::TickSpacing(element) => element.encode(),
                VAMMCalls::Ticks(element) => element.encode(),
                VAMMCalls::TransferOwnership(element) => element.encode(),
                VAMMCalls::UpdateProtocolFees(element) => element.encode(),
                VAMMCalls::UpgradeTo(element) => element.encode(),
                VAMMCalls::UpgradeToAndCall(element) => element.encode(),
                VAMMCalls::VammVars(element) => element.encode(),
                VAMMCalls::VariableTokenGrowthGlobalX128(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for VAMMCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                VAMMCalls::MaxFee(element) => element.fmt(f),
                VAMMCalls::VoltzPauser(element) => element.fmt(f),
                VAMMCalls::Burn(element) => element.fmt(f),
                VAMMCalls::ChangePauser(element) => element.fmt(f),
                VAMMCalls::ComputeGrowthInside(element) => element.fmt(f),
                VAMMCalls::Factory(element) => element.fmt(f),
                VAMMCalls::FeeGrowthGlobalX128(element) => element.fmt(f),
                VAMMCalls::FeeWad(element) => element.fmt(f),
                VAMMCalls::FixedTokenGrowthGlobalX128(element) => element.fmt(f),
                VAMMCalls::GetRateOracle(element) => element.fmt(f),
                VAMMCalls::Initialize(element) => element.fmt(f),
                VAMMCalls::InitializeVAMM(element) => element.fmt(f),
                VAMMCalls::IsAlpha(element) => element.fmt(f),
                VAMMCalls::Liquidity(element) => element.fmt(f),
                VAMMCalls::MarginEngine(element) => element.fmt(f),
                VAMMCalls::MaxLiquidityPerTick(element) => element.fmt(f),
                VAMMCalls::Mint(element) => element.fmt(f),
                VAMMCalls::Owner(element) => element.fmt(f),
                VAMMCalls::Paused(element) => element.fmt(f),
                VAMMCalls::ProtocolFees(element) => element.fmt(f),
                VAMMCalls::ProxiableUUID(element) => element.fmt(f),
                VAMMCalls::RefreshRateOracle(element) => element.fmt(f),
                VAMMCalls::RenounceOwnership(element) => element.fmt(f),
                VAMMCalls::SetFee(element) => element.fmt(f),
                VAMMCalls::SetFeeProtocol(element) => element.fmt(f),
                VAMMCalls::SetIsAlpha(element) => element.fmt(f),
                VAMMCalls::SetPausability(element) => element.fmt(f),
                VAMMCalls::Swap(element) => element.fmt(f),
                VAMMCalls::TickBitmap(element) => element.fmt(f),
                VAMMCalls::TickSpacing(element) => element.fmt(f),
                VAMMCalls::Ticks(element) => element.fmt(f),
                VAMMCalls::TransferOwnership(element) => element.fmt(f),
                VAMMCalls::UpdateProtocolFees(element) => element.fmt(f),
                VAMMCalls::UpgradeTo(element) => element.fmt(f),
                VAMMCalls::UpgradeToAndCall(element) => element.fmt(f),
                VAMMCalls::VammVars(element) => element.fmt(f),
                VAMMCalls::VariableTokenGrowthGlobalX128(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<MaxFeeCall> for VAMMCalls {
        fn from(var: MaxFeeCall) -> Self {
            VAMMCalls::MaxFee(var)
        }
    }
    impl ::std::convert::From<VoltzPauserCall> for VAMMCalls {
        fn from(var: VoltzPauserCall) -> Self {
            VAMMCalls::VoltzPauser(var)
        }
    }
    impl ::std::convert::From<BurnCall> for VAMMCalls {
        fn from(var: BurnCall) -> Self {
            VAMMCalls::Burn(var)
        }
    }
    impl ::std::convert::From<ChangePauserCall> for VAMMCalls {
        fn from(var: ChangePauserCall) -> Self {
            VAMMCalls::ChangePauser(var)
        }
    }
    impl ::std::convert::From<ComputeGrowthInsideCall> for VAMMCalls {
        fn from(var: ComputeGrowthInsideCall) -> Self {
            VAMMCalls::ComputeGrowthInside(var)
        }
    }
    impl ::std::convert::From<FactoryCall> for VAMMCalls {
        fn from(var: FactoryCall) -> Self {
            VAMMCalls::Factory(var)
        }
    }
    impl ::std::convert::From<FeeGrowthGlobalX128Call> for VAMMCalls {
        fn from(var: FeeGrowthGlobalX128Call) -> Self {
            VAMMCalls::FeeGrowthGlobalX128(var)
        }
    }
    impl ::std::convert::From<FeeWadCall> for VAMMCalls {
        fn from(var: FeeWadCall) -> Self {
            VAMMCalls::FeeWad(var)
        }
    }
    impl ::std::convert::From<FixedTokenGrowthGlobalX128Call> for VAMMCalls {
        fn from(var: FixedTokenGrowthGlobalX128Call) -> Self {
            VAMMCalls::FixedTokenGrowthGlobalX128(var)
        }
    }
    impl ::std::convert::From<GetRateOracleCall> for VAMMCalls {
        fn from(var: GetRateOracleCall) -> Self {
            VAMMCalls::GetRateOracle(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for VAMMCalls {
        fn from(var: InitializeCall) -> Self {
            VAMMCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<InitializeVAMMCall> for VAMMCalls {
        fn from(var: InitializeVAMMCall) -> Self {
            VAMMCalls::InitializeVAMM(var)
        }
    }
    impl ::std::convert::From<IsAlphaCall> for VAMMCalls {
        fn from(var: IsAlphaCall) -> Self {
            VAMMCalls::IsAlpha(var)
        }
    }
    impl ::std::convert::From<LiquidityCall> for VAMMCalls {
        fn from(var: LiquidityCall) -> Self {
            VAMMCalls::Liquidity(var)
        }
    }
    impl ::std::convert::From<MarginEngineCall> for VAMMCalls {
        fn from(var: MarginEngineCall) -> Self {
            VAMMCalls::MarginEngine(var)
        }
    }
    impl ::std::convert::From<MaxLiquidityPerTickCall> for VAMMCalls {
        fn from(var: MaxLiquidityPerTickCall) -> Self {
            VAMMCalls::MaxLiquidityPerTick(var)
        }
    }
    impl ::std::convert::From<MintCall> for VAMMCalls {
        fn from(var: MintCall) -> Self {
            VAMMCalls::Mint(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for VAMMCalls {
        fn from(var: OwnerCall) -> Self {
            VAMMCalls::Owner(var)
        }
    }
    impl ::std::convert::From<PausedCall> for VAMMCalls {
        fn from(var: PausedCall) -> Self {
            VAMMCalls::Paused(var)
        }
    }
    impl ::std::convert::From<ProtocolFeesCall> for VAMMCalls {
        fn from(var: ProtocolFeesCall) -> Self {
            VAMMCalls::ProtocolFees(var)
        }
    }
    impl ::std::convert::From<ProxiableUUIDCall> for VAMMCalls {
        fn from(var: ProxiableUUIDCall) -> Self {
            VAMMCalls::ProxiableUUID(var)
        }
    }
    impl ::std::convert::From<RefreshRateOracleCall> for VAMMCalls {
        fn from(var: RefreshRateOracleCall) -> Self {
            VAMMCalls::RefreshRateOracle(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for VAMMCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            VAMMCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<SetFeeCall> for VAMMCalls {
        fn from(var: SetFeeCall) -> Self {
            VAMMCalls::SetFee(var)
        }
    }
    impl ::std::convert::From<SetFeeProtocolCall> for VAMMCalls {
        fn from(var: SetFeeProtocolCall) -> Self {
            VAMMCalls::SetFeeProtocol(var)
        }
    }
    impl ::std::convert::From<SetIsAlphaCall> for VAMMCalls {
        fn from(var: SetIsAlphaCall) -> Self {
            VAMMCalls::SetIsAlpha(var)
        }
    }
    impl ::std::convert::From<SetPausabilityCall> for VAMMCalls {
        fn from(var: SetPausabilityCall) -> Self {
            VAMMCalls::SetPausability(var)
        }
    }
    impl ::std::convert::From<SwapCall> for VAMMCalls {
        fn from(var: SwapCall) -> Self {
            VAMMCalls::Swap(var)
        }
    }
    impl ::std::convert::From<TickBitmapCall> for VAMMCalls {
        fn from(var: TickBitmapCall) -> Self {
            VAMMCalls::TickBitmap(var)
        }
    }
    impl ::std::convert::From<TickSpacingCall> for VAMMCalls {
        fn from(var: TickSpacingCall) -> Self {
            VAMMCalls::TickSpacing(var)
        }
    }
    impl ::std::convert::From<TicksCall> for VAMMCalls {
        fn from(var: TicksCall) -> Self {
            VAMMCalls::Ticks(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for VAMMCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            VAMMCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<UpdateProtocolFeesCall> for VAMMCalls {
        fn from(var: UpdateProtocolFeesCall) -> Self {
            VAMMCalls::UpdateProtocolFees(var)
        }
    }
    impl ::std::convert::From<UpgradeToCall> for VAMMCalls {
        fn from(var: UpgradeToCall) -> Self {
            VAMMCalls::UpgradeTo(var)
        }
    }
    impl ::std::convert::From<UpgradeToAndCallCall> for VAMMCalls {
        fn from(var: UpgradeToAndCallCall) -> Self {
            VAMMCalls::UpgradeToAndCall(var)
        }
    }
    impl ::std::convert::From<VammVarsCall> for VAMMCalls {
        fn from(var: VammVarsCall) -> Self {
            VAMMCalls::VammVars(var)
        }
    }
    impl ::std::convert::From<VariableTokenGrowthGlobalX128Call> for VAMMCalls {
        fn from(var: VariableTokenGrowthGlobalX128Call) -> Self {
            VAMMCalls::VariableTokenGrowthGlobalX128(var)
        }
    }
    #[doc = "Container type for all return fields from the `MAX_FEE` function with signature `MAX_FEE()` and selector `[188, 6, 62, 26]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MaxFeeReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `VOLTZ_PAUSER` function with signature `VOLTZ_PAUSER()` and selector `[27, 57, 142, 6]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct VoltzPauserReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `burn` function with signature `burn(address,int24,int24,uint128)` and selector `[31, 47, 8, 147]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BurnReturn {
        pub position_margin_requirement: I256,
    }
    #[doc = "Container type for all return fields from the `computeGrowthInside` function with signature `computeGrowthInside(int24,int24)` and selector `[60, 143, 35, 62]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ComputeGrowthInsideReturn {
        pub fixed_token_growth_inside_x128: I256,
        pub variable_token_growth_inside_x128: I256,
        pub fee_growth_inside_x128: ethers::core::types::U256,
    }
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
    #[doc = "Container type for all return fields from the `feeGrowthGlobalX128` function with signature `feeGrowthGlobalX128()` and selector `[173, 35, 180, 22]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct FeeGrowthGlobalX128Return(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `feeWad` function with signature `feeWad()` and selector `[243, 249, 73, 144]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct FeeWadReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `fixedTokenGrowthGlobalX128` function with signature `fixedTokenGrowthGlobalX128()` and selector `[9, 215, 182, 34]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct FixedTokenGrowthGlobalX128Return(pub I256);
    #[doc = "Container type for all return fields from the `getRateOracle` function with signature `getRateOracle()` and selector `[119, 135, 98, 54]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetRateOracleReturn(pub ethers::core::types::Address);
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
    #[doc = "Container type for all return fields from the `liquidity` function with signature `liquidity()` and selector `[26, 104, 101, 2]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LiquidityReturn(pub u128);
    #[doc = "Container type for all return fields from the `marginEngine` function with signature `marginEngine()` and selector `[0, 64, 6, 224]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MarginEngineReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `maxLiquidityPerTick` function with signature `maxLiquidityPerTick()` and selector `[112, 207, 117, 74]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MaxLiquidityPerTickReturn(pub u128);
    #[doc = "Container type for all return fields from the `mint` function with signature `mint(address,int24,int24,uint128)` and selector `[184, 204, 163, 78]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MintReturn {
        pub position_margin_requirement: I256,
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
    #[doc = "Container type for all return fields from the `protocolFees` function with signature `protocolFees()` and selector `[26, 216, 176, 59]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ProtocolFeesReturn(pub ethers::core::types::U256);
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
    #[doc = "Container type for all return fields from the `swap` function with signature `swap((address,int256,uint160,int24,int24))` and selector `[103, 117, 142, 110]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SwapReturn {
        pub fixed_token_delta: I256,
        pub variable_token_delta: I256,
        pub cumulative_fee_incurred: ethers::core::types::U256,
        pub fixed_token_delta_unbalanced: I256,
        pub margin_requirement: I256,
    }
    #[doc = "Container type for all return fields from the `tickBitmap` function with signature `tickBitmap(int16)` and selector `[83, 57, 194, 150]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TickBitmapReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `tickSpacing` function with signature `tickSpacing()` and selector `[208, 201, 58, 124]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TickSpacingReturn(pub i32);
    #[doc = "Container type for all return fields from the `ticks` function with signature `ticks(int24)` and selector `[243, 13, 186, 147]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TicksReturn(pub Info);
    #[doc = "Container type for all return fields from the `vammVars` function with signature `vammVars()` and selector `[128, 160, 247, 108]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct VammVarsReturn(pub Vammvars);
    #[doc = "Container type for all return fields from the `variableTokenGrowthGlobalX128` function with signature `variableTokenGrowthGlobalX128()` and selector `[78, 101, 180, 8]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct VariableTokenGrowthGlobalX128Return(pub I256);
}
