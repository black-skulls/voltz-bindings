pub use periphery::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod periphery {
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
    #[doc = "Periphery was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static PERIPHERY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"AavePoolGetReserveNormalizedIncomeReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"AavePoolGetReserveNormalizedVariableDebtReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CTokenExchangeRateReturnedZero\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"unlocked\",\"type\":\"bool\",\"components\":[]}],\"type\":\"error\",\"name\":\"CanOnlyTradeIfUnlocked\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CannotLiquidate\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CannotSettleBeforeMaturity\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"DebugError\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"amount0\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"amount1\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"ExpectedOppositeSigns\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint160\",\"name\":\"sqrtPriceX96\",\"type\":\"uint160\",\"components\":[]}],\"type\":\"error\",\"name\":\"ExpectedSqrtPriceZeroBeforeInit\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"IRSNotionalAmountSpecifiedMustBeNonZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidMarginDelta\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"LidoGetPooledEthBySharesReturnedZero\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"type\":\"error\",\"name\":\"LiquidityDeltaMustBePositiveInBurn\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"type\":\"error\",\"name\":\"LiquidityDeltaMustBePositiveInMint\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginLessThanMinimum\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tick\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"variableTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"cumulativeFeeIncurred\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenDeltaUnbalanced\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginRequirementNotMet\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginRequirementNotMetFCM\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"requested\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"available\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"NotEnoughFunds\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OOO\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyFCM\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyMarginEngine\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyOwnerCanUpdatePosition\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyVAMM\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"x\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMathSD59x18__FromIntOverflow\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"x\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMathSD59x18__FromIntUnderflow\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PRBMathSD59x18__MulInputTooSmall\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"rAbs\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMathSD59x18__MulOverflow\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMathUD60x18__FromUintOverflow\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"prod1\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMath__MulDivFixedPointOverflow\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"prod1\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"denominator\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMath__MulDivOverflow\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PositionNetZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PositionNotSettled\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"RocketPoolGetEthValueReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"WithdrawalExceedsCurrentMargin\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"closeToOrBeyondMaturity\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AdminChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"beacon\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"BeaconUpgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"version\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Initialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract IVAMM\",\"name\":\"vamm\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"lpMarginCapNew\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MarginCap\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"implementation\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Upgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"struct IPeriphery.SwapPeripheryParams\",\"name\":\"params\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"contract IMarginEngine\",\"name\":\"marginEngine\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isFT\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"notional\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceLimitX96\",\"type\":\"uint160\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"marginDelta\",\"type\":\"int256\",\"components\":[]}]},{\"internalType\":\"uint256\",\"name\":\"variableFactorFromStartToNowWad\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"fullyCollateralisedVTSwap\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"_fixedTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_variableTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_cumulativeFeeIncurred\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_fixedTokenDeltaUnbalanced\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_marginRequirement\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"_tickAfter\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"marginDelta\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract IMarginEngine\",\"name\":\"marginEngine\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCurrentTick\",\"outputs\":[{\"internalType\":\"int24\",\"name\":\"currentTick\",\"type\":\"int24\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint160\",\"name\":\"sqrtRatioAX96\",\"type\":\"uint160\",\"components\":[]},{\"internalType\":\"uint160\",\"name\":\"sqrtRatioBX96\",\"type\":\"uint160\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"notionalAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getLiquidityForNotional\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"liquidity\",\"type\":\"uint128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract IWETH\",\"name\":\"weth_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract IVAMM\",\"name\":\"vamm\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lpMarginCaps\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract IVAMM\",\"name\":\"vamm\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lpMarginCumulatives\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct IPeriphery.MintOrBurnParams\",\"name\":\"params\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"contract IMarginEngine\",\"name\":\"marginEngine\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"notional\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isMint\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"marginDelta\",\"type\":\"int256\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"mintOrBurn\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"positionMarginRequirement\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"proxiableUUID\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract IMarginEngine\",\"name\":\"marginEngine\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"struct IPeriphery.MintOrBurnParams\",\"name\":\"paramsNewPosition\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"contract IMarginEngine\",\"name\":\"marginEngine\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"notional\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isMint\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"marginDelta\",\"type\":\"int256\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"rolloverWithMint\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"newPositionMarginRequirement\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract IMarginEngine\",\"name\":\"marginEngine\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"struct IPeriphery.SwapPeripheryParams\",\"name\":\"paramsNewPosition\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"contract IMarginEngine\",\"name\":\"marginEngine\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isFT\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"notional\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceLimitX96\",\"type\":\"uint160\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"marginDelta\",\"type\":\"int256\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"rolloverWithSwap\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"_fixedTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_variableTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_cumulativeFeeIncurred\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_fixedTokenDeltaUnbalanced\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_marginRequirement\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"_tickAfter\",\"type\":\"int24\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract IVAMM\",\"name\":\"vamm\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"lpMarginCapNew\",\"type\":\"int256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLPMarginCap\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract IVAMM\",\"name\":\"vamm\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"lpMarginCumulative\",\"type\":\"int256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLPMarginCumulative\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract IMarginEngine\",\"name\":\"marginEngine\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"settlePositionAndWithdrawMargin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IPeriphery.SwapPeripheryParams\",\"name\":\"params\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"contract IMarginEngine\",\"name\":\"marginEngine\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isFT\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"notional\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceLimitX96\",\"type\":\"uint160\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"marginDelta\",\"type\":\"int256\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"swap\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"_fixedTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_variableTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_cumulativeFeeIncurred\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_fixedTokenDeltaUnbalanced\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_marginRequirement\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"_tickAfter\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"marginDelta\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract IMarginEngine\",\"name\":\"marginEngine\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"marginDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"fullyWithdraw\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"updatePositionMargin\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newImplementation\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"upgradeTo\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newImplementation\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"upgradeToAndCall\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"weth\",\"outputs\":[{\"internalType\":\"contract IWETH\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static PERIPHERY_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60a0604052306080523480156200001557600080fd5b50603654610100900460ff1615808015620000375750603654600160ff909116105b8062000067575062000054306200014160201b620020cf1760201c565b15801562000067575060365460ff166001145b620000cf5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b606482015260840160405180910390fd5b6036805460ff191660011790558015620000f3576036805461ff0019166101001790555b80156200013a576036805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b5062000150565b6001600160a01b03163b151590565b6080516140836200018860003960008181610ca901528181610ce901528181610d8901528181610dc90152610e5c01526140836000f3fe60806040526004361061012a5760003560e01c8063715018a6116100ab578063932e0eff1161006f578063932e0eff1461039c578063c19be595146103af578063c4d66de8146103cf578063efa7c3d6146103ef578063f2fde38b1461040f578063f93964071461042f57600080fd5b8063715018a6146102dd57806378164868146102f2578063782085b51461033557806378f70b871461036b5780638da5cb5b1461037e57600080fd5b80633fc8cef3116100f25780633fc8cef3146102025780634f1ef2861461023457806352d1902d1461024757806357b69a681461025c57806361b02452146102a757600080fd5b8063040a5dc11461012f5780631b44093d146101675780632676938b1461018957806332e00daf146101c15780633659cfe6146101e2575b600080fd5b34801561013b57600080fd5b5061014f61014a366004613471565b610442565b60405160029190910b81526020015b60405180910390f35b34801561017357600080fd5b5061018761018236600461348e565b610534565b005b34801561019557600080fd5b506101a96101a43660046134ba565b61069a565b6040516001600160801b03909116815260200161015e565b6101d46101cf366004613635565b6106ed565b60405190815260200161015e565b3480156101ee57600080fd5b506101876101fd366004613471565b610c9e565b34801561020e57600080fd5b506000546001600160a01b03165b6040516001600160a01b03909116815260200161015e565b610187610242366004613651565b610d7e565b34801561025357600080fd5b506101d4610e4f565b61026f61026a3660046137a6565b610f02565b604080519788526020880196909652948601939093526060850191909152608084015260020b60a083015260c082015260e00161015e565b3480156102b357600080fd5b506101d46102c2366004613471565b6001600160a01b031660009081526002602052604090205490565b3480156102e957600080fd5b506101876110d8565b6103056103003660046137d2565b6110ec565b604080519687526020870195909552938501929092526060840152608083015260020b60a082015260c00161015e565b34801561034157600080fd5b506101d4610350366004613471565b6001600160a01b031660009081526001602052604090205490565b6101d4610379366004613841565b611130565b34801561038a57600080fd5b506069546001600160a01b031661021c565b61026f6103aa3660046138a4565b61118e565b3480156103bb57600080fd5b506101876103ca3660046138c0565b6116b2565b3480156103db57600080fd5b506101876103ea366004613471565b61172a565b3480156103fb57600080fd5b5061018761040a36600461348e565b6118a5565b34801561041b57600080fd5b5061018761042a366004613471565b6119ca565b6101d461043d36600461391c565b611a40565b600080826001600160a01b031663e098372c6040518163ffffffff1660e01b815260040160206040518083038186803b15801561047e57600080fd5b505afa158015610492573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906104b69190613984565b9050806001600160a01b03166380a0f76c6040518163ffffffff1660e01b815260040160606040518083038186803b1580156104f157600080fd5b505afa158015610505573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061052991906139a1565b602001519392505050565b816001600160a01b0381166105815760405162461bcd60e51b815260206004820152600e60248201526d76616d6d2061646472207a65726f60901b60448201526064015b60405180910390fd5b6000816001600160a01b0316638da5cb5b6040518163ffffffff1660e01b815260040160206040518083038186803b1580156105bc57600080fd5b505afa1580156105d0573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105f49190613984565b9050336001600160a01b038216146106405760405162461bcd60e51b815260206004820152600f60248201526e37b7363c903b30b6b69037bbb732b960891b6044820152606401610578565b6001600160a01b038416600081815260016020908152604091829020869055815192835282018590527f359cd6003fbab20721d687a623408d04791a7b8de287826b2590c13db66f02fb910160405180910390a150505050565b6000826001600160a01b0316846001600160a01b031611156106ba579192915b6106e36106de83600160601b6106d08888613a2f565b6001600160a01b03166120de565b6121f1565b90505b9392505050565b60006107018260200151836040015161225e565b600082600001516001600160a01b031663e098372c6040518163ffffffff1660e01b815260040160206040518083038186803b15801561074057600080fd5b505afa158015610754573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107789190613984565b835160208501516040808701519051634904f4dd60e11b81529394506000936001600160a01b0390931692639209e9ba926107b7923392600401613a57565b61014060405180830381600087803b1580156107d257600080fd5b505af11580156107e6573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061080a9190613a9e565b9050600084600001516001600160a01b031663884287526040518163ffffffff1660e01b815260040160206040518083038186803b15801561084b57600080fd5b505afa15801561085f573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906108839190613b33565b9050600033848760000151886020015189604001516040516020016108ac959493929190613b50565b60405160208183030381529060405280519060200120905060008084602001516001600160801b03161190508280156108e25750805b1561091057600082815260036020526040902054610910576040808501516000848152600360205291909120555b6000856001600160a01b03166380a0f76c6040518163ffffffff1660e01b815260040160606040518083038186803b15801561094b57600080fd5b505afa15801561095f573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061098391906139a1565b805160208a01519192506001600160a01b03161515906000906109a59061231f565b905060006109b68b6040015161231f565b905082610a4e57600060028c604001518d602001516109d59190613b95565b6109df9190613bf2565b905060006109ec8261231f565b6040516323fbaf6f60e11b81526001600160a01b038083166004830152919250908c16906347f75ede90602401600060405180830381600087803b158015610a3357600080fd5b505af1158015610a47573d6000803e3d6000fd5b5050505050505b60a08b0151151580610a605750600034115b15610a8557610a838b600001518c602001518d604001518e60a001516000611a40565b505b6000610a9683838e6060015161069a565b905060009a508b6080015115610b395760208c01516040808e01519051635c6651a760e11b81526001600160a01b038d169263b8cca34e92610ae092339291908790600401613c2c565b602060405180830381600087803b158015610afa57600080fd5b505af1158015610b0e573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610b329190613c5f565b9a50610bc8565b60208c01516040808e01519051631f2f089360e01b81526001600160a01b038d1692631f2f089392610b7392339291908790600401613c2c565b602060405180830381600087803b158015610b8d57600080fd5b505af1158015610ba1573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610bc59190613c5f565b9a505b8b5160208d01516040808f01519051634904f4dd60e11b81526001600160a01b0390931692639209e9ba92610c01923392600401613a57565b61014060405180830381600087803b158015610c1c57600080fd5b505af1158015610c30573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c549190613a9e565b60208101519099506001600160801b03161515888015610c7857508680610c785750805b15610c8e57610c8e8b898c604001518a856126bf565b5050505050505050505050919050565b306001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161415610ce75760405162461bcd60e51b815260040161057890613c78565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316610d30600080516020614007833981519152546001600160a01b031690565b6001600160a01b031614610d565760405162461bcd60e51b815260040161057890613cc4565b610d5f816127f5565b60408051600080825260208201909252610d7b918391906127fd565b50565b306001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161415610dc75760405162461bcd60e51b815260040161057890613c78565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316610e10600080516020614007833981519152546001600160a01b031690565b6001600160a01b031614610e365760405162461bcd60e51b815260040161057890613cc4565b610e3f826127f5565b610e4b828260016127fd565b5050565b6000306001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614610eef5760405162461bcd60e51b815260206004820152603860248201527f555550535570677261646561626c653a206d757374206e6f742062652063616c60448201527f6c6564207468726f7567682064656c656761746563616c6c00000000000000006064820152608401610578565b5060008051602061400783398151915290565b6000806000806000806000886020015115156000151514610f585760405162461bcd60e51b815260206004820152601060248201526f6f6e6c7920666320565420737761707360801b6044820152606401610578565b610f618961118e565b8f516040805163652c30b760e01b81529051989f50969d50949b50929950909750955093509161106f918a918a916001600160a01b0386169163652c30b791600480820192602092909190829003018186803b158015610fc057600080fd5b505afa158015610fd4573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610ff89190613c5f565b846001600160a01b03166393edb4546040518163ffffffff1660e01b815260040160206040518083038186803b15801561103157600080fd5b505afa158015611045573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906110699190613c5f565b8d61297c565b61107890613d10565b611081876129e5565b61108b9190613d2d565b8212156110cb5760405162461bcd60e51b815260206004820152600e60248201526d56542073776170206e6f7420666360901b6044820152606401610578565b5092959891949750929550565b6110e0612a4f565b6110ea6000612aa9565b565b6000806000806000806111018b8b8b8b6116b2565b61110a8761118e565b50809650819750829850839950849a50859b505050505050509550955095509550955095565b6000816080015161116f5760405162461bcd60e51b81526020600482015260096024820152681bdb9b1e481b5a5b9d60ba1b6044820152606401610578565b61117b868686866116b2565b611184826106ed565b9695505050505050565b6000806000806000806000808860c001511280156111ac5750600034115b156111f95760405162461bcd60e51b815260206004820152601e60248201527f4f6e6c7920616464206f72206f6e6c792072656d6f7665206d617267696e00006044820152606401610578565b61120b88608001518960a0015161225e565b600088600001516001600160a01b031663e098372c6040518163ffffffff1660e01b815260040160206040518083038186803b15801561124a57600080fd5b505afa15801561125e573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906112829190613984565b9050886080015160020b60001480156112a0575060a089015160020b155b1561140d576000816001600160a01b031663d0c93a7c6040518163ffffffff1660e01b815260040160206040518083038186803b1580156112e057600080fd5b505afa1580156112f4573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906113189190613d6e565b90506000826001600160a01b03166380a0f76c6040518163ffffffff1660e01b815260040160606040518083038186803b15801561135557600080fd5b505afa158015611369573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061138d91906139a1565b905060008282602001516113a19190613d8b565b905060008383602001516113b59190613b95565b905062010deb19600283900b12156113cf5762010deb1991505b6113dc62010deb19613dd3565b60020b8160020b13156113f9576113f662010deb19613dd3565b90505b600291820b60808e0152900b60a08c015250505b60008960c0015113806114205750600034115b156114465761144389600001518a608001518b60a001518c60c001516000611a40565b91505b60008960200151156114665761145f8a604001516129e5565b905061147f565b6114738a604001516129e5565b61147c90613d10565b90505b60006040518060a00160405280336001600160a01b031681526020018381526020018c606001516001600160a01b03166000146114c0578c606001516114fe565b8c60200151156114e7576114e260016c1fa71f3f5f68a90479ee3f8fec613a2f565b6114fe565b6114fe6b0816769404766de590afe04e6001613df6565b6001600160a01b031681526020018c6080015160020b81526020018c60a0015160020b8152509050826001600160a01b03166367758e6e826040518263ffffffff1660e01b81526004016115999190600060a08201905060018060a01b038084511683526020840151602084015280604085015116604084015250606083015160020b6060830152608083015160020b608083015292915050565b60a060405180830381600087803b1580156115b357600080fd5b505af11580156115c7573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906115eb9190613e21565b809a50819b50829c50839d50849e505050505050826001600160a01b03166380a0f76c6040518163ffffffff1660e01b815260040160606040518083038186803b15801561163857600080fd5b505afa15801561164c573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061167091906139a1565b60200151945060008b60c0015112156116a4576116a18b600001518c608001518d60a001518e60c001516000611a40565b93505b505050919395979092949650565b60405163a725b96560e01b81526001600160a01b0385169063a725b965906116e290869086908690600401613a57565b600060405180830381600087803b1580156116fc57600080fd5b505af1158015611710573d6000803e3d6000fd5b5050505061172384838360006001611a40565b5050505050565b603654610100900460ff161580801561174a5750603654600160ff909116105b806117645750303b158015611764575060365460ff166001145b6117c75760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608401610578565b6036805460ff1916600117905580156117ea576036805461ff0019166101001790555b6001600160a01b0382166118315760405162461bcd60e51b815260206004820152600e60248201526d776574682061646472207a65726f60901b6044820152606401610578565b600080546001600160a01b0319166001600160a01b038416179055611854612afb565b61185c612b2a565b8015610e4b576036805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15050565b816001600160a01b0381166118ed5760405162461bcd60e51b815260206004820152600e60248201526d76616d6d2061646472207a65726f60901b6044820152606401610578565b6000816001600160a01b0316638da5cb5b6040518163ffffffff1660e01b815260040160206040518083038186803b15801561192857600080fd5b505afa15801561193c573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906119609190613984565b9050336001600160a01b038216146119ac5760405162461bcd60e51b815260206004820152600f60248201526e37b7363c903b30b6b69037bbb732b960891b6044820152606401610578565b50506001600160a01b03909116600090815260026020526040902055565b6119d2612a4f565b6001600160a01b038116611a375760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610578565b610d7b81612aa9565b600080866001600160a01b0316639209e9ba3388886040518463ffffffff1660e01b8152600401611a7393929190613a57565b61014060405180830381600087803b158015611a8e57600080fd5b505af1158015611aa2573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611ac69190613a9e565b90506000876001600160a01b031663884287526040518163ffffffff1660e01b815260040160206040518083038186803b158015611b0357600080fd5b505afa158015611b17573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611b3b9190613b33565b90506000886001600160a01b031663e098372c6040518163ffffffff1660e01b815260040160206040518083038186803b158015611b7857600080fd5b505afa158015611b8c573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611bb09190613984565b9050600033828b8b8b604051602001611bcd959493929190613b50565b604051602081830303815290604052805190602001209050828015611bff5750600084602001516001600160801b0316115b15611c2d57600081815260036020526040902054611c2d576040808501516000838152600360205291909120555b60008a6001600160a01b0316632495a5996040518163ffffffff1660e01b815260040160206040518083038186803b158015611c6857600080fd5b505afa158015611c7c573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611ca09190613984565b90508615611cb9578460400151611cb690613d10565b97505b6000546001600160a01b0382811691161415611efa576000881215611d4157604051637717797f60e01b81526001600160a01b038c1690637717797f90611d0a9033908e908e908e90600401613e61565b600060405180830381600087803b158015611d2457600080fd5b505af1158015611d38573d6000803e3d6000fd5b5050505061200a565b6000881315611d6a57611d6a3330611d588b612b51565b6001600160a01b038516929190612ba3565b3415611ded5760008054906101000a90046001600160a01b03166001600160a01b031663d0e30db0346040518263ffffffff1660e01b81526004016000604051808303818588803b158015611dbe57600080fd5b505af1158015611dd2573d6000803e3d6000fd5b5050505050611de0346129e5565b611dea9089613d2d565b97505b604051636eb1769f60e11b81523060048201526001600160a01b038c811660248301526000919083169063dd62ed3e9060440160206040518083038186803b158015611e3857600080fd5b505afa158015611e4c573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611e709190613c5f565b9050611e908c611e7f8b612b51565b6001600160a01b0385169190612c14565b604051637717797f60e01b81526001600160a01b038d1690637717797f90611ec29033908f908f908f90600401613e61565b600060405180830381600087803b158015611edc57600080fd5b505af1158015611ef0573d6000803e3d6000fd5b505050505061200a565b6000881315611fa557611f113330611d588b612b51565b604051636eb1769f60e11b81523060048201526001600160a01b038c811660248301526000919083169063dd62ed3e9060440160206040518083038186803b158015611f5c57600080fd5b505afa158015611f70573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611f949190613c5f565b9050611fa38c611e7f8b612b51565b505b604051637717797f60e01b81526001600160a01b038c1690637717797f90611fd79033908e908e908e90600401613e61565b600060405180830381600087803b158015611ff157600080fd5b505af1158015612005573d6000803e3d6000fd5b505050505b604051634904f4dd60e11b81526001600160a01b038c1690639209e9ba9061203a9033908e908e90600401613a57565b61014060405180830381600087803b15801561205557600080fd5b505af1158015612069573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061208d9190613a9e565b94508380156120a95750600085602001516001600160801b0316115b156120c0576120c0838387604001516001806126bf565b50959998505050505050505050565b6001600160a01b03163b151590565b60008080600019858709858702925082811083820303915050806000141561214e57600084116121435760405162461bcd60e51b815260206004820152601060248201526f4469766973696f6e206279207a65726f60801b6044820152606401610578565b5082900490506106e6565b8084116121885760405162461bcd60e51b81526020600482015260086024820152676f766572666c6f7760c01b6044820152606401610578565b60008486880960026001871981018816978890046003810283188082028403028082028403028082028403028082028403028082028403029081029092039091026000889003889004909101858311909403939093029303949094049190911702949350505050565b60006001600160801b0382111561225a5760405162461bcd60e51b815260206004820152602760248201527f53616665436173743a2076616c756520646f65736e27742066697420696e20316044820152663238206269747360c81b6064820152608401610578565b5090565b8060020b8260020b126122995760405162461bcd60e51b8152602060048201526003602482015262544c5560e81b6044820152606401610578565b62010deb19600283900b12156122d75760405162461bcd60e51b8152602060048201526003602482015262544c4d60e81b6044820152606401610578565b6122e462010deb19613dd3565b60020b8160020b1315610e4b5760405162461bcd60e51b815260206004820152600360248201526254554d60e81b6044820152606401610578565b60008060008360020b12612336578260020b612343565b8260020b61234390613d10565b905061235262010deb19613dd3565b60020b8111156123885760405162461bcd60e51b81526020600482015260016024820152601560fa1b6044820152606401610578565b60006001821661239c57600160801b6123ae565b6ffffcb933bd6fad37aa2d162d1a5940015b70ffffffffffffffffffffffffffffffffff16905060028216156123ed5760806123e8826ffff97272373d413259a46990580e213a613e8b565b901c90505b6004821615612417576080612412826ffff2e50f5f656932ef12357cf3c7fdcc613e8b565b901c90505b600882161561244157608061243c826fffe5caca7e10e4e61c3624eaa0941cd0613e8b565b901c90505b601082161561246b576080612466826fffcb9843d60f6159c9db58835c926644613e8b565b901c90505b6020821615612495576080612490826fff973b41fa98c081472e6896dfb254c0613e8b565b901c90505b60408216156124bf5760806124ba826fff2ea16466c96a3843ec78b326b52861613e8b565b901c90505b60808216156124e95760806124e4826ffe5dee046a99a2a811c461f1969c3053613e8b565b901c90505b61010082161561251457608061250f826ffcbe86c7900a88aedcffc83b479aa3a4613e8b565b901c90505b61020082161561253f57608061253a826ff987a7253ac413176f2b074cf7815e54613e8b565b901c90505b61040082161561256a576080612565826ff3392b0822b70005940c7a398e4b70f3613e8b565b901c90505b610800821615612595576080612590826fe7159475a2c29b7443b29c7fa6e889d9613e8b565b901c90505b6110008216156125c05760806125bb826fd097f3bdfd2022b8845ad8f792aa5825613e8b565b901c90505b6120008216156125eb5760806125e6826fa9f746462d870fdf8a65dc1f90e061e5613e8b565b901c90505b614000821615612616576080612611826f70d869a156d2a1b890bb3df62baf32f7613e8b565b901c90505b61800082161561264157608061263c826f31be135f97d08fd981231505542fcfa6613e8b565b901c90505b6201000082161561266d576080612668826f09aa508b5b7a84e1c677de54f3e99bc9613e8b565b901c90505b60008460020b13156126885761268581600019613eaa565b90505b61269764010000000082613ebe565b156126a35760016126a6565b60005b6126b79060ff16602083901c613ed2565b949350505050565b8015612742576000848152600360209081526040808320546001600160a01b0389168452600290925282208054919290916126fb908490613eea565b909155505060008481526003602090815260408083208690556001600160a01b0388168352600290915281208054859290612737908490613d2d565b909155506127939050565b8115612793576000848152600360209081526040808320546001600160a01b03891684526002909252822080549192909161277e908490613eea565b90915550506000848152600360205260408120555b6001600160a01b03851660009081526001602090815260408083205460029092529091205413156117235760405162461bcd60e51b815260206004820152600c60248201526b1b1c0818d85c081b1a5b5a5d60a21b6044820152606401610578565b610d7b612a4f565b7f4910fdfa16fed3260ed0e7147f7cc6da11a60208b5b9406d12a635614ffd91435460ff16156128355761283083612cd1565b505050565b826001600160a01b03166352d1902d6040518163ffffffff1660e01b815260040160206040518083038186803b15801561286e57600080fd5b505afa92505050801561289e575060408051601f3d908101601f1916820190925261289b91810190613c5f565b60015b6129015760405162461bcd60e51b815260206004820152602e60248201527f45524331393637557067726164653a206e657720696d706c656d656e7461746960448201526d6f6e206973206e6f74205555505360901b6064820152608401610578565b60008051602061400783398151915281146129705760405162461bcd60e51b815260206004820152602960248201527f45524331393637557067726164653a20756e737570706f727465642070726f786044820152681a58589b195555525160ba1b6064820152608401610578565b50612830838383612d6d565b60008061298887612d92565b9050600061299587612d92565b905060006129af6129a860018989612e1d565b8490612ee8565b905060006129bd8387612ee8565b905060006129cb8284613d2d565b670de0b6b3a764000090059b9a5050505050505050505050565b60006001600160ff1b0382111561225a5760405162461bcd60e51b815260206004820152602860248201527f53616665436173743a2076616c756520646f65736e27742066697420696e2061604482015267371034b73a191a9b60c11b6064820152608401610578565b6069546001600160a01b031633146110ea5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610578565b606980546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b603654610100900460ff16612b225760405162461bcd60e51b815260040161057890613f29565b6110ea612fad565b603654610100900460ff166110ea5760405162461bcd60e51b815260040161057890613f29565b60008082121561225a5760405162461bcd60e51b815260206004820181905260248201527f53616665436173743a2076616c7565206d75737420626520706f7369746976656044820152606401610578565b6040516001600160a01b0380851660248301528316604482015260648101829052612c0e9085906323b872dd60e01b906084015b60408051601f198184030181529190526020810180516001600160e01b03166001600160e01b031990931692909217909152612fdd565b50505050565b604051636eb1769f60e11b81523060048201526001600160a01b0383811660248301526000919085169063dd62ed3e9060440160206040518083038186803b158015612c5f57600080fd5b505afa158015612c73573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612c979190613c5f565b905081811015612c0e576040516001600160a01b038416602482015260448101839052612c0e90859063095ea7b360e01b90606401612bd7565b6001600160a01b0381163b612d3e5760405162461bcd60e51b815260206004820152602d60248201527f455243313936373a206e657720696d706c656d656e746174696f6e206973206e60448201526c1bdd08184818dbdb9d1c9858dd609a1b6064820152608401610578565b60008051602061400783398151915280546001600160a01b0319166001600160a01b0392909216919091179055565b612d768361305e565b600082511180612d835750805b1561283057612c0e838361309e565b60007809392ee8e921d5d073aff322e62439fcf32d7f344649470f8f19821215612dd25760405163e608e18b60e01b815260048101839052602401610578565b7809392ee8e921d5d073aff322e62439fcf32d7f344649470f90821315612e0f576040516371f72a3160e01b815260048101839052602401610578565b50670de0b6b3a76400000290565b6000828211612e575760405162461bcd60e51b815260040161057890602080825260049082015263453c3d5360e01b604082015260600190565b6000612e61613194565b905083811015612e9b5760405162461bcd60e51b8152602060048201526005602482015264422e543c5360d81b6044820152606401610578565b60008580612ea95750838210155b15612ebf57612eb88585613f74565b9050612ecc565b612ec98583613f74565b90505b61118468056bc75e2d63100000612ee2836131a4565b906131b7565b6000600160ff1b831480612eff5750600160ff1b82145b15612f1d57604051630d01a11b60e21b815260040160405180910390fd5b60008060008512612f2e5784612f33565b846000035b915060008412612f435783612f48565b836000035b90506000612f5683836131cc565b90506001600160ff1b03811115612f835760405163bf79e8d960e01b815260048101829052602401610578565b600019808713908613808218600114612f9c5782612fa1565b826000035b98975050505050505050565b603654610100900460ff16612fd45760405162461bcd60e51b815260040161057890613f29565b6110ea33612aa9565b600061300983836040518060400160405280600781526020016629aa261032b93960c91b81525061328e565b80519091501561283057808060200190518101906130279190613b33565b6128305760405162461bcd60e51b815260206004820152600860248201526714d5130819985a5b60c21b6044820152606401610578565b61306781612cd1565b6040516001600160a01b038216907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b90600090a250565b60606001600160a01b0383163b6131065760405162461bcd60e51b815260206004820152602660248201527f416464726573733a2064656c65676174652063616c6c20746f206e6f6e2d636f6044820152651b9d1c9858dd60d21b6064820152608401610578565b600080846001600160a01b0316846040516131219190613fb7565b600060405180830381855af49150503d806000811461315c576040519150601f19603f3d011682016040523d82523d6000602084013e613161565b606091505b509150915061318982826040518060600160405280602781526020016140276027913961333c565b925050505b92915050565b600061319f42613355565b905090565b600061318e826a1a1601fc4ea7109e0000005b60006106e683670de0b6b3a764000084613394565b60008080600019848609848602925082811083820303915050670de0b6b3a764000081106132105760405163698d9a0160e11b815260048101829052602401610578565b600080670de0b6b3a76400008688099150506706f05b59d3b1ffff81118261324a5780670de0b6b3a764000085040194505050505061318e565b620400008285030493909111909103600160ee1b02919091177faccb18165bd6fe31ae1cf318dc5b51eee0e1ba569b88cd74c1773b91fac106690201905092915050565b6060833b6132cd5760405162461bcd60e51b815260206004820152600c60248201526b1b9bdb8b58dbdb9d1c9858dd60a21b6044820152606401610578565b600080856001600160a01b03166000866040516132ea9190613fb7565b60006040518083038185875af1925050503d8060008114613327576040519150601f19603f3d011682016040523d82523d6000602084013e61332c565b606091505b50915091506111848282866133f9565b6060831561334b5750816106e6565b6106e68383613432565b60007812725dd1d243aba0e75fe645cc4873f9e65afe688c928e1f21821115612e0f57604051633492ffd960e01b815260048101839052602401610578565b6000808060001985870985870292508281108382030391505080600014156133cf578382816133c5576133c5613bdc565b04925050506106e6565b83811061218857604051631dcf306360e21b81526004810182905260248101859052604401610578565b606083156134085750816106e6565b8251156134185782518084602001fd5b8160405162461bcd60e51b81526004016105789190613fd3565b8151156134425781518083602001fd5b8060405162461bcd60e51b81526004016105789190613fd3565b6001600160a01b0381168114610d7b57600080fd5b60006020828403121561348357600080fd5b81356106e68161345c565b600080604083850312156134a157600080fd5b82356134ac8161345c565b946020939093013593505050565b6000806000606084860312156134cf57600080fd5b83356134da8161345c565b925060208401356134ea8161345c565b929592945050506040919091013590565b634e487b7160e01b600052604160045260246000fd5b604051610140810167ffffffffffffffff81118282101715613535576135356134fb565b60405290565b604051601f8201601f1916810167ffffffffffffffff81118282101715613564576135646134fb565b604052919050565b8060020b8114610d7b57600080fd5b80356135868161356c565b919050565b8015158114610d7b57600080fd5b600060c082840312156135ab57600080fd5b60405160c0810181811067ffffffffffffffff821117156135ce576135ce6134fb565b60405290508082356135df8161345c565b815260208301356135ef8161356c565b602082015260408301356136028161356c565b604082015260608381013590820152608083013561361f8161358b565b608082015260a092830135920191909152919050565b600060c0828403121561364757600080fd5b6106e68383613599565b6000806040838503121561366457600080fd5b823561366f8161345c565b915060208381013567ffffffffffffffff8082111561368d57600080fd5b818601915086601f8301126136a157600080fd5b8135818111156136b3576136b36134fb565b6136c5601f8201601f1916850161353b565b915080825287848285010111156136db57600080fd5b80848401858401376000848284010152508093505050509250929050565b600060e0828403121561370b57600080fd5b60405160e0810181811067ffffffffffffffff8211171561372e5761372e6134fb565b604052905080823561373f8161345c565b8152602083013561374f8161358b565b602082015260408381013590820152606083013561376c8161345c565b6060820152608083013561377f8161356c565b608082015261379060a0840161357b565b60a082015260c083013560c08201525092915050565b60008061010083850312156137ba57600080fd5b6137c484846136f9565b9460e0939093013593505050565b600080600080600061016086880312156137eb57600080fd5b85356137f68161345c565b945060208601356138068161345c565b935060408601356138168161356c565b925060608601356138268161356c565b915061383587608088016136f9565b90509295509295909350565b6000806000806000610140868803121561385a57600080fd5b85356138658161345c565b945060208601356138758161345c565b935060408601356138858161356c565b925060608601356138958161356c565b91506138358760808801613599565b600060e082840312156138b657600080fd5b6106e683836136f9565b600080600080608085870312156138d657600080fd5b84356138e18161345c565b935060208501356138f18161345c565b925060408501356139018161356c565b915060608501356139118161356c565b939692955090935050565b600080600080600060a0868803121561393457600080fd5b853561393f8161345c565b9450602086013561394f8161356c565b9350604086013561395f8161356c565b92506060860135915060808601356139768161358b565b809150509295509295909350565b60006020828403121561399657600080fd5b81516106e68161345c565b6000606082840312156139b357600080fd5b6040516060810181811067ffffffffffffffff821117156139d6576139d66134fb565b60405282516139e48161345c565b815260208301516139f48161356c565b6020820152604083015160ff81168114613a0d57600080fd5b60408201529392505050565b634e487b7160e01b600052601160045260246000fd5b60006001600160a01b0383811690831681811015613a4f57613a4f613a19565b039392505050565b6001600160a01b03939093168352600291820b6020840152900b604082015260600190565b80516135868161358b565b80516001600160801b038116811461358657600080fd5b60006101408284031215613ab157600080fd5b613ab9613511565b613ac283613a7c565b8152613ad060208401613a87565b602082015260408301516040820152606083015160608201526080830151608082015260a083015160a082015260c083015160c082015260e083015160e08201526101008084015181830152506101208084015181830152508091505092915050565b600060208284031215613b4557600080fd5b81516106e68161358b565b6bffffffffffffffffffffffff19606096871b8116825294861b851660148201529290941b909216602882015260e891821b603c82015291901b603f82015260420190565b60008160020b8360020b6000821282627fffff03821381151615613bbb57613bbb613a19565b82627fffff19038212811615613bd357613bd3613a19565b50019392505050565b634e487b7160e01b600052601260045260246000fd5b60008160020b8360020b80613c0957613c09613bdc565b627fffff19821460001982141615613c2357613c23613a19565b90059392505050565b6001600160a01b03949094168452600292830b6020850152910b60408301526001600160801b0316606082015260800190565b600060208284031215613c7157600080fd5b5051919050565b6020808252602c908201527f46756e6374696f6e206d7573742062652063616c6c6564207468726f7567682060408201526b19195b1959d85d1958d85b1b60a21b606082015260800190565b6020808252602c908201527f46756e6374696f6e206d7573742062652063616c6c6564207468726f7567682060408201526b6163746976652070726f787960a01b606082015260800190565b6000600160ff1b821415613d2657613d26613a19565b5060000390565b600080821280156001600160ff1b0384900385131615613d4f57613d4f613a19565b600160ff1b8390038412811615613d6857613d68613a19565b50500190565b600060208284031215613d8057600080fd5b81516106e68161356c565b60008160020b8360020b6000811281627fffff1901831281151615613db257613db2613a19565b81627fffff018313811615613dc957613dc9613a19565b5090039392505050565b60008160020b627fffff19811415613ded57613ded613a19565b60000392915050565b60006001600160a01b03828116848216808303821115613e1857613e18613a19565b01949350505050565b600080600080600060a08688031215613e3957600080fd5b5050835160208501516040860151606087015160809097015192989197509594509092509050565b6001600160a01b03949094168452600292830b6020850152910b6040830152606082015260800190565b6000816000190483118215151615613ea557613ea5613a19565b500290565b600082613eb957613eb9613bdc565b500490565b600082613ecd57613ecd613bdc565b500690565b60008219821115613ee557613ee5613a19565b500190565b60008083128015600160ff1b850184121615613f0857613f08613a19565b6001600160ff1b0384018313811615613f2357613f23613a19565b50500390565b6020808252602b908201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960408201526a6e697469616c697a696e6760a81b606082015260800190565b600082821015613f8657613f86613a19565b500390565b60005b83811015613fa6578181015183820152602001613f8e565b83811115612c0e5750506000910152565b60008251613fc9818460208701613f8b565b9190910192915050565b6020815260008251806020840152613ff2816040850160208701613f8b565b601f01601f1916919091016040019291505056fe360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc416464726573733a206c6f772d6c6576656c2064656c65676174652063616c6c206661696c6564a2646970667358221220928747f0a3c341d39a487a9db79a0f79be7d126775a787efb0bfc1d793a1506f64736f6c63430008090033" . parse () . expect ("invalid bytecode")
        });
    pub struct Periphery<M>(ethers::contract::Contract<M>);
    impl<M> Clone for Periphery<M> {
        fn clone(&self) -> Self {
            Periphery(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Periphery<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for Periphery<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Periphery))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> Periphery<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), PERIPHERY_ABI.clone(), client).into()
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
                PERIPHERY_ABI.clone(),
                PERIPHERY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `fullyCollateralisedVTSwap` (0x57b69a68) function"]
        pub fn fully_collateralised_vt_swap(
            &self,
            params: SwapPeripheryParams,
            variable_factor_from_start_to_now_wad: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (I256, I256, ethers::core::types::U256, I256, I256, i32, I256),
        > {
            self.0
                .method_hash(
                    [87, 182, 154, 104],
                    (params, variable_factor_from_start_to_now_wad),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCurrentTick` (0x040a5dc1) function"]
        pub fn get_current_tick(
            &self,
            margin_engine: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, i32> {
            self.0
                .method_hash([4, 10, 93, 193], margin_engine)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLiquidityForNotional` (0x2676938b) function"]
        pub fn get_liquidity_for_notional(
            &self,
            sqrt_ratio_ax96: ethers::core::types::U256,
            sqrt_ratio_bx96: ethers::core::types::U256,
            notional_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash(
                    [38, 118, 147, 139],
                    (sqrt_ratio_ax96, sqrt_ratio_bx96, notional_amount),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0xc4d66de8) function"]
        pub fn initialize(
            &self,
            weth: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 214, 109, 232], weth)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lpMarginCaps` (0x782085b5) function"]
        pub fn lp_margin_caps(
            &self,
            vamm: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, I256> {
            self.0
                .method_hash([120, 32, 133, 181], vamm)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lpMarginCumulatives` (0x61b02452) function"]
        pub fn lp_margin_cumulatives(
            &self,
            vamm: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, I256> {
            self.0
                .method_hash([97, 176, 36, 82], vamm)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mintOrBurn` (0x32e00daf) function"]
        pub fn mint_or_burn(
            &self,
            params: MintOrBurnParams,
        ) -> ethers::contract::builders::ContractCall<M, I256> {
            self.0
                .method_hash([50, 224, 13, 175], (params,))
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
        #[doc = "Calls the contract's `proxiableUUID` (0x52d1902d) function"]
        pub fn proxiable_uuid(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([82, 209, 144, 45], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rolloverWithMint` (0x78f70b87) function"]
        pub fn rollover_with_mint(
            &self,
            margin_engine: ethers::core::types::Address,
            owner: ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
            params_new_position: MintOrBurnParams,
        ) -> ethers::contract::builders::ContractCall<M, I256> {
            self.0
                .method_hash(
                    [120, 247, 11, 135],
                    (
                        margin_engine,
                        owner,
                        tick_lower,
                        tick_upper,
                        params_new_position,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rolloverWithSwap` (0x78164868) function"]
        pub fn rollover_with_swap(
            &self,
            margin_engine: ethers::core::types::Address,
            owner: ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
            params_new_position: SwapPeripheryParams,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (I256, I256, ethers::core::types::U256, I256, I256, i32),
        > {
            self.0
                .method_hash(
                    [120, 22, 72, 104],
                    (
                        margin_engine,
                        owner,
                        tick_lower,
                        tick_upper,
                        params_new_position,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLPMarginCap` (0x1b44093d) function"]
        pub fn set_lp_margin_cap(
            &self,
            vamm: ethers::core::types::Address,
            lp_margin_cap_new: I256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([27, 68, 9, 61], (vamm, lp_margin_cap_new))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLPMarginCumulative` (0xefa7c3d6) function"]
        pub fn set_lp_margin_cumulative(
            &self,
            vamm: ethers::core::types::Address,
            lp_margin_cumulative: I256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([239, 167, 195, 214], (vamm, lp_margin_cumulative))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `settlePositionAndWithdrawMargin` (0xc19be595) function"]
        pub fn settle_position_and_withdraw_margin(
            &self,
            margin_engine: ethers::core::types::Address,
            owner: ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [193, 155, 229, 149],
                    (margin_engine, owner, tick_lower, tick_upper),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swap` (0x932e0eff) function"]
        pub fn swap(
            &self,
            params: SwapPeripheryParams,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (I256, I256, ethers::core::types::U256, I256, I256, i32, I256),
        > {
            self.0
                .method_hash([147, 46, 14, 255], (params,))
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
        #[doc = "Calls the contract's `updatePositionMargin` (0xf9396407) function"]
        pub fn update_position_margin(
            &self,
            margin_engine: ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
            margin_delta: I256,
            fully_withdraw: bool,
        ) -> ethers::contract::builders::ContractCall<M, I256> {
            self.0
                .method_hash(
                    [249, 57, 100, 7],
                    (
                        margin_engine,
                        tick_lower,
                        tick_upper,
                        margin_delta,
                        fully_withdraw,
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
        #[doc = "Calls the contract's `weth` (0x3fc8cef3) function"]
        pub fn weth(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([63, 200, 206, 243], ())
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
        #[doc = "Gets the contract's `Initialized` event"]
        pub fn initialized_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, InitializedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `MarginCap` event"]
        pub fn margin_cap_filter(&self) -> ethers::contract::builders::Event<M, MarginCapFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Upgraded` event"]
        pub fn upgraded_filter(&self) -> ethers::contract::builders::Event<M, UpgradedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, PeripheryEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Periphery<M> {
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
    pub enum PeripheryErrors {
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
    impl ethers::core::abi::AbiDecode for PeripheryErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok (decoded) = < AavePoolGetReserveNormalizedIncomeReturnedZero as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (PeripheryErrors :: AavePoolGetReserveNormalizedIncomeReturnedZero (decoded)) }
            if let Ok (decoded) = < AavePoolGetReserveNormalizedVariableDebtReturnedZero as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (PeripheryErrors :: AavePoolGetReserveNormalizedVariableDebtReturnedZero (decoded)) }
            if let Ok(decoded) =
                <CTokenExchangeRateReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PeripheryErrors::CTokenExchangeRateReturnedZero(decoded));
            }
            if let Ok(decoded) =
                <CanOnlyTradeIfUnlocked as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeripheryErrors::CanOnlyTradeIfUnlocked(decoded));
            }
            if let Ok(decoded) =
                <CannotLiquidate as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeripheryErrors::CannotLiquidate(decoded));
            }
            if let Ok(decoded) =
                <CannotSettleBeforeMaturity as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeripheryErrors::CannotSettleBeforeMaturity(decoded));
            }
            if let Ok(decoded) = <DebugError as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeripheryErrors::DebugError(decoded));
            }
            if let Ok(decoded) =
                <ExpectedOppositeSigns as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeripheryErrors::ExpectedOppositeSigns(decoded));
            }
            if let Ok(decoded) =
                <ExpectedSqrtPriceZeroBeforeInit as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PeripheryErrors::ExpectedSqrtPriceZeroBeforeInit(decoded));
            }
            if let Ok(decoded) =
                <IRSNotionalAmountSpecifiedMustBeNonZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PeripheryErrors::IRSNotionalAmountSpecifiedMustBeNonZero(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <InvalidMarginDelta as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeripheryErrors::InvalidMarginDelta(decoded));
            }
            if let Ok(decoded) =
                <LidoGetPooledEthBySharesReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PeripheryErrors::LidoGetPooledEthBySharesReturnedZero(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <LiquidityDeltaMustBePositiveInBurn as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PeripheryErrors::LiquidityDeltaMustBePositiveInBurn(decoded));
            }
            if let Ok(decoded) =
                <LiquidityDeltaMustBePositiveInMint as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PeripheryErrors::LiquidityDeltaMustBePositiveInMint(decoded));
            }
            if let Ok(decoded) =
                <MarginLessThanMinimum as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeripheryErrors::MarginLessThanMinimum(decoded));
            }
            if let Ok(decoded) =
                <MarginRequirementNotMet as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeripheryErrors::MarginRequirementNotMet(decoded));
            }
            if let Ok(decoded) =
                <MarginRequirementNotMetFCM as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeripheryErrors::MarginRequirementNotMetFCM(decoded));
            }
            if let Ok(decoded) =
                <NotEnoughFunds as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeripheryErrors::NotEnoughFunds(decoded));
            }
            if let Ok(decoded) = <OOO as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(PeripheryErrors::OOO(decoded));
            }
            if let Ok(decoded) = <OnlyFCM as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(PeripheryErrors::OnlyFCM(decoded));
            }
            if let Ok(decoded) =
                <OnlyMarginEngine as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeripheryErrors::OnlyMarginEngine(decoded));
            }
            if let Ok(decoded) =
                <OnlyOwnerCanUpdatePosition as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeripheryErrors::OnlyOwnerCanUpdatePosition(decoded));
            }
            if let Ok(decoded) = <OnlyVAMM as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(PeripheryErrors::OnlyVAMM(decoded));
            }
            if let Ok(decoded) =
                <PRBMathSD59x18__FromIntOverflow as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PeripheryErrors::PRBMathSD59x18__FromIntOverflow(decoded));
            }
            if let Ok(decoded) =
                <PRBMathSD59x18__FromIntUnderflow as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PeripheryErrors::PRBMathSD59x18__FromIntUnderflow(decoded));
            }
            if let Ok(decoded) =
                <PRBMathSD59x18__MulInputTooSmall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PeripheryErrors::PRBMathSD59x18__MulInputTooSmall(decoded));
            }
            if let Ok(decoded) =
                <PRBMathSD59x18__MulOverflow as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeripheryErrors::PRBMathSD59x18__MulOverflow(decoded));
            }
            if let Ok(decoded) =
                <PRBMathUD60x18__FromUintOverflow as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PeripheryErrors::PRBMathUD60x18__FromUintOverflow(decoded));
            }
            if let Ok(decoded) =
                <PRBMath__MulDivFixedPointOverflow as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PeripheryErrors::PRBMath__MulDivFixedPointOverflow(decoded));
            }
            if let Ok(decoded) =
                <PRBMath__MulDivOverflow as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeripheryErrors::PRBMath__MulDivOverflow(decoded));
            }
            if let Ok(decoded) =
                <PositionNetZero as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeripheryErrors::PositionNetZero(decoded));
            }
            if let Ok(decoded) =
                <PositionNotSettled as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeripheryErrors::PositionNotSettled(decoded));
            }
            if let Ok(decoded) =
                <RocketPoolGetEthValueReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PeripheryErrors::RocketPoolGetEthValueReturnedZero(decoded));
            }
            if let Ok(decoded) =
                <WithdrawalExceedsCurrentMargin as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PeripheryErrors::WithdrawalExceedsCurrentMargin(decoded));
            }
            if let Ok(decoded) =
                <closeToOrBeyondMaturity as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeripheryErrors::closeToOrBeyondMaturity(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for PeripheryErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                PeripheryErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(element) => {
                    element.encode()
                }
                PeripheryErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(element) => {
                    element.encode()
                }
                PeripheryErrors::CTokenExchangeRateReturnedZero(element) => element.encode(),
                PeripheryErrors::CanOnlyTradeIfUnlocked(element) => element.encode(),
                PeripheryErrors::CannotLiquidate(element) => element.encode(),
                PeripheryErrors::CannotSettleBeforeMaturity(element) => element.encode(),
                PeripheryErrors::DebugError(element) => element.encode(),
                PeripheryErrors::ExpectedOppositeSigns(element) => element.encode(),
                PeripheryErrors::ExpectedSqrtPriceZeroBeforeInit(element) => element.encode(),
                PeripheryErrors::IRSNotionalAmountSpecifiedMustBeNonZero(element) => {
                    element.encode()
                }
                PeripheryErrors::InvalidMarginDelta(element) => element.encode(),
                PeripheryErrors::LidoGetPooledEthBySharesReturnedZero(element) => element.encode(),
                PeripheryErrors::LiquidityDeltaMustBePositiveInBurn(element) => element.encode(),
                PeripheryErrors::LiquidityDeltaMustBePositiveInMint(element) => element.encode(),
                PeripheryErrors::MarginLessThanMinimum(element) => element.encode(),
                PeripheryErrors::MarginRequirementNotMet(element) => element.encode(),
                PeripheryErrors::MarginRequirementNotMetFCM(element) => element.encode(),
                PeripheryErrors::NotEnoughFunds(element) => element.encode(),
                PeripheryErrors::OOO(element) => element.encode(),
                PeripheryErrors::OnlyFCM(element) => element.encode(),
                PeripheryErrors::OnlyMarginEngine(element) => element.encode(),
                PeripheryErrors::OnlyOwnerCanUpdatePosition(element) => element.encode(),
                PeripheryErrors::OnlyVAMM(element) => element.encode(),
                PeripheryErrors::PRBMathSD59x18__FromIntOverflow(element) => element.encode(),
                PeripheryErrors::PRBMathSD59x18__FromIntUnderflow(element) => element.encode(),
                PeripheryErrors::PRBMathSD59x18__MulInputTooSmall(element) => element.encode(),
                PeripheryErrors::PRBMathSD59x18__MulOverflow(element) => element.encode(),
                PeripheryErrors::PRBMathUD60x18__FromUintOverflow(element) => element.encode(),
                PeripheryErrors::PRBMath__MulDivFixedPointOverflow(element) => element.encode(),
                PeripheryErrors::PRBMath__MulDivOverflow(element) => element.encode(),
                PeripheryErrors::PositionNetZero(element) => element.encode(),
                PeripheryErrors::PositionNotSettled(element) => element.encode(),
                PeripheryErrors::RocketPoolGetEthValueReturnedZero(element) => element.encode(),
                PeripheryErrors::WithdrawalExceedsCurrentMargin(element) => element.encode(),
                PeripheryErrors::closeToOrBeyondMaturity(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for PeripheryErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PeripheryErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(element) => {
                    element.fmt(f)
                }
                PeripheryErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(element) => {
                    element.fmt(f)
                }
                PeripheryErrors::CTokenExchangeRateReturnedZero(element) => element.fmt(f),
                PeripheryErrors::CanOnlyTradeIfUnlocked(element) => element.fmt(f),
                PeripheryErrors::CannotLiquidate(element) => element.fmt(f),
                PeripheryErrors::CannotSettleBeforeMaturity(element) => element.fmt(f),
                PeripheryErrors::DebugError(element) => element.fmt(f),
                PeripheryErrors::ExpectedOppositeSigns(element) => element.fmt(f),
                PeripheryErrors::ExpectedSqrtPriceZeroBeforeInit(element) => element.fmt(f),
                PeripheryErrors::IRSNotionalAmountSpecifiedMustBeNonZero(element) => element.fmt(f),
                PeripheryErrors::InvalidMarginDelta(element) => element.fmt(f),
                PeripheryErrors::LidoGetPooledEthBySharesReturnedZero(element) => element.fmt(f),
                PeripheryErrors::LiquidityDeltaMustBePositiveInBurn(element) => element.fmt(f),
                PeripheryErrors::LiquidityDeltaMustBePositiveInMint(element) => element.fmt(f),
                PeripheryErrors::MarginLessThanMinimum(element) => element.fmt(f),
                PeripheryErrors::MarginRequirementNotMet(element) => element.fmt(f),
                PeripheryErrors::MarginRequirementNotMetFCM(element) => element.fmt(f),
                PeripheryErrors::NotEnoughFunds(element) => element.fmt(f),
                PeripheryErrors::OOO(element) => element.fmt(f),
                PeripheryErrors::OnlyFCM(element) => element.fmt(f),
                PeripheryErrors::OnlyMarginEngine(element) => element.fmt(f),
                PeripheryErrors::OnlyOwnerCanUpdatePosition(element) => element.fmt(f),
                PeripheryErrors::OnlyVAMM(element) => element.fmt(f),
                PeripheryErrors::PRBMathSD59x18__FromIntOverflow(element) => element.fmt(f),
                PeripheryErrors::PRBMathSD59x18__FromIntUnderflow(element) => element.fmt(f),
                PeripheryErrors::PRBMathSD59x18__MulInputTooSmall(element) => element.fmt(f),
                PeripheryErrors::PRBMathSD59x18__MulOverflow(element) => element.fmt(f),
                PeripheryErrors::PRBMathUD60x18__FromUintOverflow(element) => element.fmt(f),
                PeripheryErrors::PRBMath__MulDivFixedPointOverflow(element) => element.fmt(f),
                PeripheryErrors::PRBMath__MulDivOverflow(element) => element.fmt(f),
                PeripheryErrors::PositionNetZero(element) => element.fmt(f),
                PeripheryErrors::PositionNotSettled(element) => element.fmt(f),
                PeripheryErrors::RocketPoolGetEthValueReturnedZero(element) => element.fmt(f),
                PeripheryErrors::WithdrawalExceedsCurrentMargin(element) => element.fmt(f),
                PeripheryErrors::closeToOrBeyondMaturity(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AavePoolGetReserveNormalizedIncomeReturnedZero> for PeripheryErrors {
        fn from(var: AavePoolGetReserveNormalizedIncomeReturnedZero) -> Self {
            PeripheryErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(var)
        }
    }
    impl ::std::convert::From<AavePoolGetReserveNormalizedVariableDebtReturnedZero>
        for PeripheryErrors
    {
        fn from(var: AavePoolGetReserveNormalizedVariableDebtReturnedZero) -> Self {
            PeripheryErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(var)
        }
    }
    impl ::std::convert::From<CTokenExchangeRateReturnedZero> for PeripheryErrors {
        fn from(var: CTokenExchangeRateReturnedZero) -> Self {
            PeripheryErrors::CTokenExchangeRateReturnedZero(var)
        }
    }
    impl ::std::convert::From<CanOnlyTradeIfUnlocked> for PeripheryErrors {
        fn from(var: CanOnlyTradeIfUnlocked) -> Self {
            PeripheryErrors::CanOnlyTradeIfUnlocked(var)
        }
    }
    impl ::std::convert::From<CannotLiquidate> for PeripheryErrors {
        fn from(var: CannotLiquidate) -> Self {
            PeripheryErrors::CannotLiquidate(var)
        }
    }
    impl ::std::convert::From<CannotSettleBeforeMaturity> for PeripheryErrors {
        fn from(var: CannotSettleBeforeMaturity) -> Self {
            PeripheryErrors::CannotSettleBeforeMaturity(var)
        }
    }
    impl ::std::convert::From<DebugError> for PeripheryErrors {
        fn from(var: DebugError) -> Self {
            PeripheryErrors::DebugError(var)
        }
    }
    impl ::std::convert::From<ExpectedOppositeSigns> for PeripheryErrors {
        fn from(var: ExpectedOppositeSigns) -> Self {
            PeripheryErrors::ExpectedOppositeSigns(var)
        }
    }
    impl ::std::convert::From<ExpectedSqrtPriceZeroBeforeInit> for PeripheryErrors {
        fn from(var: ExpectedSqrtPriceZeroBeforeInit) -> Self {
            PeripheryErrors::ExpectedSqrtPriceZeroBeforeInit(var)
        }
    }
    impl ::std::convert::From<IRSNotionalAmountSpecifiedMustBeNonZero> for PeripheryErrors {
        fn from(var: IRSNotionalAmountSpecifiedMustBeNonZero) -> Self {
            PeripheryErrors::IRSNotionalAmountSpecifiedMustBeNonZero(var)
        }
    }
    impl ::std::convert::From<InvalidMarginDelta> for PeripheryErrors {
        fn from(var: InvalidMarginDelta) -> Self {
            PeripheryErrors::InvalidMarginDelta(var)
        }
    }
    impl ::std::convert::From<LidoGetPooledEthBySharesReturnedZero> for PeripheryErrors {
        fn from(var: LidoGetPooledEthBySharesReturnedZero) -> Self {
            PeripheryErrors::LidoGetPooledEthBySharesReturnedZero(var)
        }
    }
    impl ::std::convert::From<LiquidityDeltaMustBePositiveInBurn> for PeripheryErrors {
        fn from(var: LiquidityDeltaMustBePositiveInBurn) -> Self {
            PeripheryErrors::LiquidityDeltaMustBePositiveInBurn(var)
        }
    }
    impl ::std::convert::From<LiquidityDeltaMustBePositiveInMint> for PeripheryErrors {
        fn from(var: LiquidityDeltaMustBePositiveInMint) -> Self {
            PeripheryErrors::LiquidityDeltaMustBePositiveInMint(var)
        }
    }
    impl ::std::convert::From<MarginLessThanMinimum> for PeripheryErrors {
        fn from(var: MarginLessThanMinimum) -> Self {
            PeripheryErrors::MarginLessThanMinimum(var)
        }
    }
    impl ::std::convert::From<MarginRequirementNotMet> for PeripheryErrors {
        fn from(var: MarginRequirementNotMet) -> Self {
            PeripheryErrors::MarginRequirementNotMet(var)
        }
    }
    impl ::std::convert::From<MarginRequirementNotMetFCM> for PeripheryErrors {
        fn from(var: MarginRequirementNotMetFCM) -> Self {
            PeripheryErrors::MarginRequirementNotMetFCM(var)
        }
    }
    impl ::std::convert::From<NotEnoughFunds> for PeripheryErrors {
        fn from(var: NotEnoughFunds) -> Self {
            PeripheryErrors::NotEnoughFunds(var)
        }
    }
    impl ::std::convert::From<OOO> for PeripheryErrors {
        fn from(var: OOO) -> Self {
            PeripheryErrors::OOO(var)
        }
    }
    impl ::std::convert::From<OnlyFCM> for PeripheryErrors {
        fn from(var: OnlyFCM) -> Self {
            PeripheryErrors::OnlyFCM(var)
        }
    }
    impl ::std::convert::From<OnlyMarginEngine> for PeripheryErrors {
        fn from(var: OnlyMarginEngine) -> Self {
            PeripheryErrors::OnlyMarginEngine(var)
        }
    }
    impl ::std::convert::From<OnlyOwnerCanUpdatePosition> for PeripheryErrors {
        fn from(var: OnlyOwnerCanUpdatePosition) -> Self {
            PeripheryErrors::OnlyOwnerCanUpdatePosition(var)
        }
    }
    impl ::std::convert::From<OnlyVAMM> for PeripheryErrors {
        fn from(var: OnlyVAMM) -> Self {
            PeripheryErrors::OnlyVAMM(var)
        }
    }
    impl ::std::convert::From<PRBMathSD59x18__FromIntOverflow> for PeripheryErrors {
        fn from(var: PRBMathSD59x18__FromIntOverflow) -> Self {
            PeripheryErrors::PRBMathSD59x18__FromIntOverflow(var)
        }
    }
    impl ::std::convert::From<PRBMathSD59x18__FromIntUnderflow> for PeripheryErrors {
        fn from(var: PRBMathSD59x18__FromIntUnderflow) -> Self {
            PeripheryErrors::PRBMathSD59x18__FromIntUnderflow(var)
        }
    }
    impl ::std::convert::From<PRBMathSD59x18__MulInputTooSmall> for PeripheryErrors {
        fn from(var: PRBMathSD59x18__MulInputTooSmall) -> Self {
            PeripheryErrors::PRBMathSD59x18__MulInputTooSmall(var)
        }
    }
    impl ::std::convert::From<PRBMathSD59x18__MulOverflow> for PeripheryErrors {
        fn from(var: PRBMathSD59x18__MulOverflow) -> Self {
            PeripheryErrors::PRBMathSD59x18__MulOverflow(var)
        }
    }
    impl ::std::convert::From<PRBMathUD60x18__FromUintOverflow> for PeripheryErrors {
        fn from(var: PRBMathUD60x18__FromUintOverflow) -> Self {
            PeripheryErrors::PRBMathUD60x18__FromUintOverflow(var)
        }
    }
    impl ::std::convert::From<PRBMath__MulDivFixedPointOverflow> for PeripheryErrors {
        fn from(var: PRBMath__MulDivFixedPointOverflow) -> Self {
            PeripheryErrors::PRBMath__MulDivFixedPointOverflow(var)
        }
    }
    impl ::std::convert::From<PRBMath__MulDivOverflow> for PeripheryErrors {
        fn from(var: PRBMath__MulDivOverflow) -> Self {
            PeripheryErrors::PRBMath__MulDivOverflow(var)
        }
    }
    impl ::std::convert::From<PositionNetZero> for PeripheryErrors {
        fn from(var: PositionNetZero) -> Self {
            PeripheryErrors::PositionNetZero(var)
        }
    }
    impl ::std::convert::From<PositionNotSettled> for PeripheryErrors {
        fn from(var: PositionNotSettled) -> Self {
            PeripheryErrors::PositionNotSettled(var)
        }
    }
    impl ::std::convert::From<RocketPoolGetEthValueReturnedZero> for PeripheryErrors {
        fn from(var: RocketPoolGetEthValueReturnedZero) -> Self {
            PeripheryErrors::RocketPoolGetEthValueReturnedZero(var)
        }
    }
    impl ::std::convert::From<WithdrawalExceedsCurrentMargin> for PeripheryErrors {
        fn from(var: WithdrawalExceedsCurrentMargin) -> Self {
            PeripheryErrors::WithdrawalExceedsCurrentMargin(var)
        }
    }
    impl ::std::convert::From<closeToOrBeyondMaturity> for PeripheryErrors {
        fn from(var: closeToOrBeyondMaturity) -> Self {
            PeripheryErrors::closeToOrBeyondMaturity(var)
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
    #[ethevent(name = "MarginCap", abi = "MarginCap(address,int256)")]
    pub struct MarginCapFilter {
        pub vamm: ethers::core::types::Address,
        pub lp_margin_cap_new: I256,
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
    #[ethevent(name = "Upgraded", abi = "Upgraded(address)")]
    pub struct UpgradedFilter {
        #[ethevent(indexed)]
        pub implementation: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PeripheryEvents {
        AdminChangedFilter(AdminChangedFilter),
        BeaconUpgradedFilter(BeaconUpgradedFilter),
        InitializedFilter(InitializedFilter),
        MarginCapFilter(MarginCapFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        UpgradedFilter(UpgradedFilter),
    }
    impl ethers::contract::EthLogDecode for PeripheryEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AdminChangedFilter::decode_log(log) {
                return Ok(PeripheryEvents::AdminChangedFilter(decoded));
            }
            if let Ok(decoded) = BeaconUpgradedFilter::decode_log(log) {
                return Ok(PeripheryEvents::BeaconUpgradedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(PeripheryEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = MarginCapFilter::decode_log(log) {
                return Ok(PeripheryEvents::MarginCapFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(PeripheryEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(PeripheryEvents::UpgradedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for PeripheryEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PeripheryEvents::AdminChangedFilter(element) => element.fmt(f),
                PeripheryEvents::BeaconUpgradedFilter(element) => element.fmt(f),
                PeripheryEvents::InitializedFilter(element) => element.fmt(f),
                PeripheryEvents::MarginCapFilter(element) => element.fmt(f),
                PeripheryEvents::OwnershipTransferredFilter(element) => element.fmt(f),
                PeripheryEvents::UpgradedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `fullyCollateralisedVTSwap` function with signature `fullyCollateralisedVTSwap((address,bool,uint256,uint160,int24,int24,int256),uint256)` and selector `[87, 182, 154, 104]`"]
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
        name = "fullyCollateralisedVTSwap",
        abi = "fullyCollateralisedVTSwap((address,bool,uint256,uint160,int24,int24,int256),uint256)"
    )]
    pub struct FullyCollateralisedVTSwapCall {
        pub params: SwapPeripheryParams,
        pub variable_factor_from_start_to_now_wad: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getCurrentTick` function with signature `getCurrentTick(address)` and selector `[4, 10, 93, 193]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getCurrentTick", abi = "getCurrentTick(address)")]
    pub struct GetCurrentTickCall {
        pub margin_engine: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getLiquidityForNotional` function with signature `getLiquidityForNotional(uint160,uint160,uint256)` and selector `[38, 118, 147, 139]`"]
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
        name = "getLiquidityForNotional",
        abi = "getLiquidityForNotional(uint160,uint160,uint256)"
    )]
    pub struct GetLiquidityForNotionalCall {
        pub sqrt_ratio_ax96: ethers::core::types::U256,
        pub sqrt_ratio_bx96: ethers::core::types::U256,
        pub notional_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize(address)` and selector `[196, 214, 109, 232]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "initialize", abi = "initialize(address)")]
    pub struct InitializeCall {
        pub weth: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `lpMarginCaps` function with signature `lpMarginCaps(address)` and selector `[120, 32, 133, 181]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "lpMarginCaps", abi = "lpMarginCaps(address)")]
    pub struct LpMarginCapsCall {
        pub vamm: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `lpMarginCumulatives` function with signature `lpMarginCumulatives(address)` and selector `[97, 176, 36, 82]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "lpMarginCumulatives", abi = "lpMarginCumulatives(address)")]
    pub struct LpMarginCumulativesCall {
        pub vamm: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `mintOrBurn` function with signature `mintOrBurn((address,int24,int24,uint256,bool,int256))` and selector `[50, 224, 13, 175]`"]
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
        name = "mintOrBurn",
        abi = "mintOrBurn((address,int24,int24,uint256,bool,int256))"
    )]
    pub struct MintOrBurnCall {
        pub params: MintOrBurnParams,
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
    #[doc = "Container type for all input parameters for the `rolloverWithMint` function with signature `rolloverWithMint(address,address,int24,int24,(address,int24,int24,uint256,bool,int256))` and selector `[120, 247, 11, 135]`"]
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
        name = "rolloverWithMint",
        abi = "rolloverWithMint(address,address,int24,int24,(address,int24,int24,uint256,bool,int256))"
    )]
    pub struct RolloverWithMintCall {
        pub margin_engine: ethers::core::types::Address,
        pub owner: ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub params_new_position: MintOrBurnParams,
    }
    #[doc = "Container type for all input parameters for the `rolloverWithSwap` function with signature `rolloverWithSwap(address,address,int24,int24,(address,bool,uint256,uint160,int24,int24,int256))` and selector `[120, 22, 72, 104]`"]
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
        name = "rolloverWithSwap",
        abi = "rolloverWithSwap(address,address,int24,int24,(address,bool,uint256,uint160,int24,int24,int256))"
    )]
    pub struct RolloverWithSwapCall {
        pub margin_engine: ethers::core::types::Address,
        pub owner: ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub params_new_position: SwapPeripheryParams,
    }
    #[doc = "Container type for all input parameters for the `setLPMarginCap` function with signature `setLPMarginCap(address,int256)` and selector `[27, 68, 9, 61]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setLPMarginCap", abi = "setLPMarginCap(address,int256)")]
    pub struct SetLPMarginCapCall {
        pub vamm: ethers::core::types::Address,
        pub lp_margin_cap_new: I256,
    }
    #[doc = "Container type for all input parameters for the `setLPMarginCumulative` function with signature `setLPMarginCumulative(address,int256)` and selector `[239, 167, 195, 214]`"]
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
        name = "setLPMarginCumulative",
        abi = "setLPMarginCumulative(address,int256)"
    )]
    pub struct SetLPMarginCumulativeCall {
        pub vamm: ethers::core::types::Address,
        pub lp_margin_cumulative: I256,
    }
    #[doc = "Container type for all input parameters for the `settlePositionAndWithdrawMargin` function with signature `settlePositionAndWithdrawMargin(address,address,int24,int24)` and selector `[193, 155, 229, 149]`"]
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
        name = "settlePositionAndWithdrawMargin",
        abi = "settlePositionAndWithdrawMargin(address,address,int24,int24)"
    )]
    pub struct SettlePositionAndWithdrawMarginCall {
        pub margin_engine: ethers::core::types::Address,
        pub owner: ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
    }
    #[doc = "Container type for all input parameters for the `swap` function with signature `swap((address,bool,uint256,uint160,int24,int24,int256))` and selector `[147, 46, 14, 255]`"]
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
        name = "swap",
        abi = "swap((address,bool,uint256,uint160,int24,int24,int256))"
    )]
    pub struct SwapCall {
        pub params: SwapPeripheryParams,
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
    #[doc = "Container type for all input parameters for the `updatePositionMargin` function with signature `updatePositionMargin(address,int24,int24,int256,bool)` and selector `[249, 57, 100, 7]`"]
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
        abi = "updatePositionMargin(address,int24,int24,int256,bool)"
    )]
    pub struct UpdatePositionMarginCall {
        pub margin_engine: ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub margin_delta: I256,
        pub fully_withdraw: bool,
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
    #[doc = "Container type for all input parameters for the `weth` function with signature `weth()` and selector `[63, 200, 206, 243]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "weth", abi = "weth()")]
    pub struct WethCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PeripheryCalls {
        FullyCollateralisedVTSwap(FullyCollateralisedVTSwapCall),
        GetCurrentTick(GetCurrentTickCall),
        GetLiquidityForNotional(GetLiquidityForNotionalCall),
        Initialize(InitializeCall),
        LpMarginCaps(LpMarginCapsCall),
        LpMarginCumulatives(LpMarginCumulativesCall),
        MintOrBurn(MintOrBurnCall),
        Owner(OwnerCall),
        ProxiableUUID(ProxiableUUIDCall),
        RenounceOwnership(RenounceOwnershipCall),
        RolloverWithMint(RolloverWithMintCall),
        RolloverWithSwap(RolloverWithSwapCall),
        SetLPMarginCap(SetLPMarginCapCall),
        SetLPMarginCumulative(SetLPMarginCumulativeCall),
        SettlePositionAndWithdrawMargin(SettlePositionAndWithdrawMarginCall),
        Swap(SwapCall),
        TransferOwnership(TransferOwnershipCall),
        UpdatePositionMargin(UpdatePositionMarginCall),
        UpgradeTo(UpgradeToCall),
        UpgradeToAndCall(UpgradeToAndCallCall),
        Weth(WethCall),
    }
    impl ethers::core::abi::AbiDecode for PeripheryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <FullyCollateralisedVTSwapCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PeripheryCalls::FullyCollateralisedVTSwap(decoded));
            }
            if let Ok(decoded) =
                <GetCurrentTickCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeripheryCalls::GetCurrentTick(decoded));
            }
            if let Ok(decoded) =
                <GetLiquidityForNotionalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeripheryCalls::GetLiquidityForNotional(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeripheryCalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <LpMarginCapsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeripheryCalls::LpMarginCaps(decoded));
            }
            if let Ok(decoded) =
                <LpMarginCumulativesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeripheryCalls::LpMarginCumulatives(decoded));
            }
            if let Ok(decoded) =
                <MintOrBurnCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeripheryCalls::MintOrBurn(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeripheryCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <ProxiableUUIDCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeripheryCalls::ProxiableUUID(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeripheryCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <RolloverWithMintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeripheryCalls::RolloverWithMint(decoded));
            }
            if let Ok(decoded) =
                <RolloverWithSwapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeripheryCalls::RolloverWithSwap(decoded));
            }
            if let Ok(decoded) =
                <SetLPMarginCapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeripheryCalls::SetLPMarginCap(decoded));
            }
            if let Ok(decoded) =
                <SetLPMarginCumulativeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeripheryCalls::SetLPMarginCumulative(decoded));
            }
            if let Ok(decoded) =
                <SettlePositionAndWithdrawMarginCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PeripheryCalls::SettlePositionAndWithdrawMargin(decoded));
            }
            if let Ok(decoded) = <SwapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(PeripheryCalls::Swap(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeripheryCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <UpdatePositionMarginCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeripheryCalls::UpdatePositionMargin(decoded));
            }
            if let Ok(decoded) =
                <UpgradeToCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeripheryCalls::UpgradeTo(decoded));
            }
            if let Ok(decoded) =
                <UpgradeToAndCallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeripheryCalls::UpgradeToAndCall(decoded));
            }
            if let Ok(decoded) = <WethCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(PeripheryCalls::Weth(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for PeripheryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                PeripheryCalls::FullyCollateralisedVTSwap(element) => element.encode(),
                PeripheryCalls::GetCurrentTick(element) => element.encode(),
                PeripheryCalls::GetLiquidityForNotional(element) => element.encode(),
                PeripheryCalls::Initialize(element) => element.encode(),
                PeripheryCalls::LpMarginCaps(element) => element.encode(),
                PeripheryCalls::LpMarginCumulatives(element) => element.encode(),
                PeripheryCalls::MintOrBurn(element) => element.encode(),
                PeripheryCalls::Owner(element) => element.encode(),
                PeripheryCalls::ProxiableUUID(element) => element.encode(),
                PeripheryCalls::RenounceOwnership(element) => element.encode(),
                PeripheryCalls::RolloverWithMint(element) => element.encode(),
                PeripheryCalls::RolloverWithSwap(element) => element.encode(),
                PeripheryCalls::SetLPMarginCap(element) => element.encode(),
                PeripheryCalls::SetLPMarginCumulative(element) => element.encode(),
                PeripheryCalls::SettlePositionAndWithdrawMargin(element) => element.encode(),
                PeripheryCalls::Swap(element) => element.encode(),
                PeripheryCalls::TransferOwnership(element) => element.encode(),
                PeripheryCalls::UpdatePositionMargin(element) => element.encode(),
                PeripheryCalls::UpgradeTo(element) => element.encode(),
                PeripheryCalls::UpgradeToAndCall(element) => element.encode(),
                PeripheryCalls::Weth(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for PeripheryCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PeripheryCalls::FullyCollateralisedVTSwap(element) => element.fmt(f),
                PeripheryCalls::GetCurrentTick(element) => element.fmt(f),
                PeripheryCalls::GetLiquidityForNotional(element) => element.fmt(f),
                PeripheryCalls::Initialize(element) => element.fmt(f),
                PeripheryCalls::LpMarginCaps(element) => element.fmt(f),
                PeripheryCalls::LpMarginCumulatives(element) => element.fmt(f),
                PeripheryCalls::MintOrBurn(element) => element.fmt(f),
                PeripheryCalls::Owner(element) => element.fmt(f),
                PeripheryCalls::ProxiableUUID(element) => element.fmt(f),
                PeripheryCalls::RenounceOwnership(element) => element.fmt(f),
                PeripheryCalls::RolloverWithMint(element) => element.fmt(f),
                PeripheryCalls::RolloverWithSwap(element) => element.fmt(f),
                PeripheryCalls::SetLPMarginCap(element) => element.fmt(f),
                PeripheryCalls::SetLPMarginCumulative(element) => element.fmt(f),
                PeripheryCalls::SettlePositionAndWithdrawMargin(element) => element.fmt(f),
                PeripheryCalls::Swap(element) => element.fmt(f),
                PeripheryCalls::TransferOwnership(element) => element.fmt(f),
                PeripheryCalls::UpdatePositionMargin(element) => element.fmt(f),
                PeripheryCalls::UpgradeTo(element) => element.fmt(f),
                PeripheryCalls::UpgradeToAndCall(element) => element.fmt(f),
                PeripheryCalls::Weth(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<FullyCollateralisedVTSwapCall> for PeripheryCalls {
        fn from(var: FullyCollateralisedVTSwapCall) -> Self {
            PeripheryCalls::FullyCollateralisedVTSwap(var)
        }
    }
    impl ::std::convert::From<GetCurrentTickCall> for PeripheryCalls {
        fn from(var: GetCurrentTickCall) -> Self {
            PeripheryCalls::GetCurrentTick(var)
        }
    }
    impl ::std::convert::From<GetLiquidityForNotionalCall> for PeripheryCalls {
        fn from(var: GetLiquidityForNotionalCall) -> Self {
            PeripheryCalls::GetLiquidityForNotional(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for PeripheryCalls {
        fn from(var: InitializeCall) -> Self {
            PeripheryCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<LpMarginCapsCall> for PeripheryCalls {
        fn from(var: LpMarginCapsCall) -> Self {
            PeripheryCalls::LpMarginCaps(var)
        }
    }
    impl ::std::convert::From<LpMarginCumulativesCall> for PeripheryCalls {
        fn from(var: LpMarginCumulativesCall) -> Self {
            PeripheryCalls::LpMarginCumulatives(var)
        }
    }
    impl ::std::convert::From<MintOrBurnCall> for PeripheryCalls {
        fn from(var: MintOrBurnCall) -> Self {
            PeripheryCalls::MintOrBurn(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for PeripheryCalls {
        fn from(var: OwnerCall) -> Self {
            PeripheryCalls::Owner(var)
        }
    }
    impl ::std::convert::From<ProxiableUUIDCall> for PeripheryCalls {
        fn from(var: ProxiableUUIDCall) -> Self {
            PeripheryCalls::ProxiableUUID(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for PeripheryCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            PeripheryCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<RolloverWithMintCall> for PeripheryCalls {
        fn from(var: RolloverWithMintCall) -> Self {
            PeripheryCalls::RolloverWithMint(var)
        }
    }
    impl ::std::convert::From<RolloverWithSwapCall> for PeripheryCalls {
        fn from(var: RolloverWithSwapCall) -> Self {
            PeripheryCalls::RolloverWithSwap(var)
        }
    }
    impl ::std::convert::From<SetLPMarginCapCall> for PeripheryCalls {
        fn from(var: SetLPMarginCapCall) -> Self {
            PeripheryCalls::SetLPMarginCap(var)
        }
    }
    impl ::std::convert::From<SetLPMarginCumulativeCall> for PeripheryCalls {
        fn from(var: SetLPMarginCumulativeCall) -> Self {
            PeripheryCalls::SetLPMarginCumulative(var)
        }
    }
    impl ::std::convert::From<SettlePositionAndWithdrawMarginCall> for PeripheryCalls {
        fn from(var: SettlePositionAndWithdrawMarginCall) -> Self {
            PeripheryCalls::SettlePositionAndWithdrawMargin(var)
        }
    }
    impl ::std::convert::From<SwapCall> for PeripheryCalls {
        fn from(var: SwapCall) -> Self {
            PeripheryCalls::Swap(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for PeripheryCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            PeripheryCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<UpdatePositionMarginCall> for PeripheryCalls {
        fn from(var: UpdatePositionMarginCall) -> Self {
            PeripheryCalls::UpdatePositionMargin(var)
        }
    }
    impl ::std::convert::From<UpgradeToCall> for PeripheryCalls {
        fn from(var: UpgradeToCall) -> Self {
            PeripheryCalls::UpgradeTo(var)
        }
    }
    impl ::std::convert::From<UpgradeToAndCallCall> for PeripheryCalls {
        fn from(var: UpgradeToAndCallCall) -> Self {
            PeripheryCalls::UpgradeToAndCall(var)
        }
    }
    impl ::std::convert::From<WethCall> for PeripheryCalls {
        fn from(var: WethCall) -> Self {
            PeripheryCalls::Weth(var)
        }
    }
    #[doc = "Container type for all return fields from the `fullyCollateralisedVTSwap` function with signature `fullyCollateralisedVTSwap((address,bool,uint256,uint160,int24,int24,int256),uint256)` and selector `[87, 182, 154, 104]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct FullyCollateralisedVTSwapReturn {
        pub fixed_token_delta: I256,
        pub variable_token_delta: I256,
        pub cumulative_fee_incurred: ethers::core::types::U256,
        pub fixed_token_delta_unbalanced: I256,
        pub margin_requirement: I256,
        pub tick_after: i32,
        pub margin_delta: I256,
    }
    #[doc = "Container type for all return fields from the `getCurrentTick` function with signature `getCurrentTick(address)` and selector `[4, 10, 93, 193]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetCurrentTickReturn {
        pub current_tick: i32,
    }
    #[doc = "Container type for all return fields from the `getLiquidityForNotional` function with signature `getLiquidityForNotional(uint160,uint160,uint256)` and selector `[38, 118, 147, 139]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetLiquidityForNotionalReturn {
        pub liquidity: u128,
    }
    #[doc = "Container type for all return fields from the `lpMarginCaps` function with signature `lpMarginCaps(address)` and selector `[120, 32, 133, 181]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LpMarginCapsReturn(pub I256);
    #[doc = "Container type for all return fields from the `lpMarginCumulatives` function with signature `lpMarginCumulatives(address)` and selector `[97, 176, 36, 82]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LpMarginCumulativesReturn(pub I256);
    #[doc = "Container type for all return fields from the `mintOrBurn` function with signature `mintOrBurn((address,int24,int24,uint256,bool,int256))` and selector `[50, 224, 13, 175]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MintOrBurnReturn {
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
    #[doc = "Container type for all return fields from the `rolloverWithMint` function with signature `rolloverWithMint(address,address,int24,int24,(address,int24,int24,uint256,bool,int256))` and selector `[120, 247, 11, 135]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct RolloverWithMintReturn {
        pub new_position_margin_requirement: I256,
    }
    #[doc = "Container type for all return fields from the `rolloverWithSwap` function with signature `rolloverWithSwap(address,address,int24,int24,(address,bool,uint256,uint160,int24,int24,int256))` and selector `[120, 22, 72, 104]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct RolloverWithSwapReturn {
        pub fixed_token_delta: I256,
        pub variable_token_delta: I256,
        pub cumulative_fee_incurred: ethers::core::types::U256,
        pub fixed_token_delta_unbalanced: I256,
        pub margin_requirement: I256,
        pub tick_after: i32,
    }
    #[doc = "Container type for all return fields from the `swap` function with signature `swap((address,bool,uint256,uint160,int24,int24,int256))` and selector `[147, 46, 14, 255]`"]
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
        pub tick_after: i32,
        pub margin_delta: I256,
    }
    #[doc = "Container type for all return fields from the `updatePositionMargin` function with signature `updatePositionMargin(address,int24,int24,int256,bool)` and selector `[249, 57, 100, 7]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct UpdatePositionMarginReturn(pub I256);
    #[doc = "Container type for all return fields from the `weth` function with signature `weth()` and selector `[63, 200, 206, 243]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct WethReturn(pub ethers::core::types::Address);
}
