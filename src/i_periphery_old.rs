pub use i_periphery_old::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_periphery_old {
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
    #[doc = "IPeripheryOld was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IPERIPHERYOLD_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"type\":\"error\",\"name\":\"AavePoolGetReserveNormalizedIncomeReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"AavePoolGetReserveNormalizedVariableDebtReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CTokenExchangeRateReturnedZero\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"unlocked\",\"type\":\"bool\",\"components\":[]}],\"type\":\"error\",\"name\":\"CanOnlyTradeIfUnlocked\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CannotLiquidate\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CannotSettleBeforeMaturity\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"DebugError\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"amount0\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"amount1\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"ExpectedOppositeSigns\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint160\",\"name\":\"sqrtPriceX96\",\"type\":\"uint160\",\"components\":[]}],\"type\":\"error\",\"name\":\"ExpectedSqrtPriceZeroBeforeInit\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"IRSNotionalAmountSpecifiedMustBeNonZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidMarginDelta\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"LidoGetPooledEthBySharesReturnedZero\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"type\":\"error\",\"name\":\"LiquidityDeltaMustBePositiveInBurn\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"type\":\"error\",\"name\":\"LiquidityDeltaMustBePositiveInMint\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginLessThanMinimum\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tick\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"variableTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"cumulativeFeeIncurred\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenDeltaUnbalanced\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginRequirementNotMet\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginRequirementNotMetFCM\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"requested\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"available\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"NotEnoughFunds\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OOO\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyFCM\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyMarginEngine\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyOwnerCanUpdatePosition\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyVAMM\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PositionNetZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PositionNotSettled\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"RocketPoolGetEthValueReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"WithdrawalExceedsCurrentMargin\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"closeToOrBeyondMaturity\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract IVAMM\",\"name\":\"vamm\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"lpMarginCapNew\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MarginCap\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract IMarginEngine\",\"name\":\"marginEngine\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCurrentTick\",\"outputs\":[{\"internalType\":\"int24\",\"name\":\"currentTick\",\"type\":\"int24\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract IVAMM\",\"name\":\"vamm\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lpMarginCaps\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract IVAMM\",\"name\":\"vamm\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lpMarginCumulatives\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct IPeripheryOld.MintOrBurnParams\",\"name\":\"params\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"contract IMarginEngine\",\"name\":\"marginEngine\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"notional\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isMint\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"marginDelta\",\"type\":\"int256\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"mintOrBurn\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"positionMarginRequirement\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract IMarginEngine\",\"name\":\"marginEngine\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"struct IPeripheryOld.MintOrBurnParams\",\"name\":\"paramsNewPosition\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"contract IMarginEngine\",\"name\":\"marginEngine\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"notional\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isMint\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"marginDelta\",\"type\":\"int256\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"rolloverWithMint\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"newPositionMarginRequirement\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract IMarginEngine\",\"name\":\"marginEngine\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"struct IPeripheryOld.SwapPeripheryParams\",\"name\":\"paramsNewPosition\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"contract IMarginEngine\",\"name\":\"marginEngine\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isFT\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"notional\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceLimitX96\",\"type\":\"uint160\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"marginDelta\",\"type\":\"uint256\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"rolloverWithSwap\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"_fixedTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_variableTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_cumulativeFeeIncurred\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_fixedTokenDeltaUnbalanced\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_marginRequirement\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"_tickAfter\",\"type\":\"int24\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract IVAMM\",\"name\":\"vamm\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"lpMarginCapNew\",\"type\":\"int256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLPMarginCap\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract IVAMM\",\"name\":\"vamm\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"lpMarginCumulative\",\"type\":\"int256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLPMarginCumulative\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract IMarginEngine\",\"name\":\"marginEngine\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"settlePositionAndWithdrawMargin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IPeripheryOld.SwapPeripheryParams\",\"name\":\"params\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"contract IMarginEngine\",\"name\":\"marginEngine\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isFT\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"notional\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceLimitX96\",\"type\":\"uint160\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"marginDelta\",\"type\":\"uint256\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"swap\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"_fixedTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_variableTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_cumulativeFeeIncurred\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_fixedTokenDeltaUnbalanced\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_marginRequirement\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"_tickAfter\",\"type\":\"int24\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract IMarginEngine\",\"name\":\"marginEngine\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"marginDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"fullyWithdraw\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"updatePositionMargin\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    pub struct IPeripheryOld<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IPeripheryOld<M> {
        fn clone(&self) -> Self {
            IPeripheryOld(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IPeripheryOld<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IPeripheryOld<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IPeripheryOld))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IPeripheryOld<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IPERIPHERYOLD_ABI.clone(), client)
                .into()
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
        #[doc = "Calls the contract's `rolloverWithSwap` (0x8d8e0983) function"]
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
                    [141, 142, 9, 131],
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
        #[doc = "Calls the contract's `swap` (0x34907e34) function"]
        pub fn swap(
            &self,
            params: SwapPeripheryParams,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (I256, I256, ethers::core::types::U256, I256, I256, i32),
        > {
            self.0
                .method_hash([52, 144, 126, 52], (params,))
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
        ) -> ethers::contract::builders::ContractCall<M, ()> {
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
        #[doc = "Gets the contract's `MarginCap` event"]
        pub fn margin_cap_filter(&self) -> ethers::contract::builders::Event<M, MarginCapFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, MarginCapFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IPeripheryOld<M> {
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
    pub enum IPeripheryOldErrors {
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
    impl ethers::core::abi::AbiDecode for IPeripheryOldErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok (decoded) = < AavePoolGetReserveNormalizedIncomeReturnedZero as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (IPeripheryOldErrors :: AavePoolGetReserveNormalizedIncomeReturnedZero (decoded)) }
            if let Ok (decoded) = < AavePoolGetReserveNormalizedVariableDebtReturnedZero as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (IPeripheryOldErrors :: AavePoolGetReserveNormalizedVariableDebtReturnedZero (decoded)) }
            if let Ok(decoded) =
                <CTokenExchangeRateReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IPeripheryOldErrors::CTokenExchangeRateReturnedZero(decoded));
            }
            if let Ok(decoded) =
                <CanOnlyTradeIfUnlocked as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPeripheryOldErrors::CanOnlyTradeIfUnlocked(decoded));
            }
            if let Ok(decoded) =
                <CannotLiquidate as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPeripheryOldErrors::CannotLiquidate(decoded));
            }
            if let Ok(decoded) =
                <CannotSettleBeforeMaturity as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPeripheryOldErrors::CannotSettleBeforeMaturity(decoded));
            }
            if let Ok(decoded) = <DebugError as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPeripheryOldErrors::DebugError(decoded));
            }
            if let Ok(decoded) =
                <ExpectedOppositeSigns as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPeripheryOldErrors::ExpectedOppositeSigns(decoded));
            }
            if let Ok(decoded) =
                <ExpectedSqrtPriceZeroBeforeInit as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IPeripheryOldErrors::ExpectedSqrtPriceZeroBeforeInit(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <IRSNotionalAmountSpecifiedMustBeNonZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IPeripheryOldErrors::IRSNotionalAmountSpecifiedMustBeNonZero(decoded));
            }
            if let Ok(decoded) =
                <InvalidMarginDelta as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPeripheryOldErrors::InvalidMarginDelta(decoded));
            }
            if let Ok(decoded) =
                <LidoGetPooledEthBySharesReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IPeripheryOldErrors::LidoGetPooledEthBySharesReturnedZero(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <LiquidityDeltaMustBePositiveInBurn as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IPeripheryOldErrors::LiquidityDeltaMustBePositiveInBurn(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <LiquidityDeltaMustBePositiveInMint as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IPeripheryOldErrors::LiquidityDeltaMustBePositiveInMint(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <MarginLessThanMinimum as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPeripheryOldErrors::MarginLessThanMinimum(decoded));
            }
            if let Ok(decoded) =
                <MarginRequirementNotMet as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPeripheryOldErrors::MarginRequirementNotMet(decoded));
            }
            if let Ok(decoded) =
                <MarginRequirementNotMetFCM as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPeripheryOldErrors::MarginRequirementNotMetFCM(decoded));
            }
            if let Ok(decoded) =
                <NotEnoughFunds as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPeripheryOldErrors::NotEnoughFunds(decoded));
            }
            if let Ok(decoded) = <OOO as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IPeripheryOldErrors::OOO(decoded));
            }
            if let Ok(decoded) = <OnlyFCM as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IPeripheryOldErrors::OnlyFCM(decoded));
            }
            if let Ok(decoded) =
                <OnlyMarginEngine as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPeripheryOldErrors::OnlyMarginEngine(decoded));
            }
            if let Ok(decoded) =
                <OnlyOwnerCanUpdatePosition as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPeripheryOldErrors::OnlyOwnerCanUpdatePosition(decoded));
            }
            if let Ok(decoded) = <OnlyVAMM as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IPeripheryOldErrors::OnlyVAMM(decoded));
            }
            if let Ok(decoded) =
                <PositionNetZero as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPeripheryOldErrors::PositionNetZero(decoded));
            }
            if let Ok(decoded) =
                <PositionNotSettled as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPeripheryOldErrors::PositionNotSettled(decoded));
            }
            if let Ok(decoded) =
                <RocketPoolGetEthValueReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IPeripheryOldErrors::RocketPoolGetEthValueReturnedZero(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <WithdrawalExceedsCurrentMargin as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IPeripheryOldErrors::WithdrawalExceedsCurrentMargin(decoded));
            }
            if let Ok(decoded) =
                <closeToOrBeyondMaturity as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPeripheryOldErrors::closeToOrBeyondMaturity(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IPeripheryOldErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                IPeripheryOldErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(element) => {
                    element.encode()
                }
                IPeripheryOldErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(
                    element,
                ) => element.encode(),
                IPeripheryOldErrors::CTokenExchangeRateReturnedZero(element) => element.encode(),
                IPeripheryOldErrors::CanOnlyTradeIfUnlocked(element) => element.encode(),
                IPeripheryOldErrors::CannotLiquidate(element) => element.encode(),
                IPeripheryOldErrors::CannotSettleBeforeMaturity(element) => element.encode(),
                IPeripheryOldErrors::DebugError(element) => element.encode(),
                IPeripheryOldErrors::ExpectedOppositeSigns(element) => element.encode(),
                IPeripheryOldErrors::ExpectedSqrtPriceZeroBeforeInit(element) => element.encode(),
                IPeripheryOldErrors::IRSNotionalAmountSpecifiedMustBeNonZero(element) => {
                    element.encode()
                }
                IPeripheryOldErrors::InvalidMarginDelta(element) => element.encode(),
                IPeripheryOldErrors::LidoGetPooledEthBySharesReturnedZero(element) => {
                    element.encode()
                }
                IPeripheryOldErrors::LiquidityDeltaMustBePositiveInBurn(element) => {
                    element.encode()
                }
                IPeripheryOldErrors::LiquidityDeltaMustBePositiveInMint(element) => {
                    element.encode()
                }
                IPeripheryOldErrors::MarginLessThanMinimum(element) => element.encode(),
                IPeripheryOldErrors::MarginRequirementNotMet(element) => element.encode(),
                IPeripheryOldErrors::MarginRequirementNotMetFCM(element) => element.encode(),
                IPeripheryOldErrors::NotEnoughFunds(element) => element.encode(),
                IPeripheryOldErrors::OOO(element) => element.encode(),
                IPeripheryOldErrors::OnlyFCM(element) => element.encode(),
                IPeripheryOldErrors::OnlyMarginEngine(element) => element.encode(),
                IPeripheryOldErrors::OnlyOwnerCanUpdatePosition(element) => element.encode(),
                IPeripheryOldErrors::OnlyVAMM(element) => element.encode(),
                IPeripheryOldErrors::PositionNetZero(element) => element.encode(),
                IPeripheryOldErrors::PositionNotSettled(element) => element.encode(),
                IPeripheryOldErrors::RocketPoolGetEthValueReturnedZero(element) => element.encode(),
                IPeripheryOldErrors::WithdrawalExceedsCurrentMargin(element) => element.encode(),
                IPeripheryOldErrors::closeToOrBeyondMaturity(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IPeripheryOldErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IPeripheryOldErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(element) => {
                    element.fmt(f)
                }
                IPeripheryOldErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(
                    element,
                ) => element.fmt(f),
                IPeripheryOldErrors::CTokenExchangeRateReturnedZero(element) => element.fmt(f),
                IPeripheryOldErrors::CanOnlyTradeIfUnlocked(element) => element.fmt(f),
                IPeripheryOldErrors::CannotLiquidate(element) => element.fmt(f),
                IPeripheryOldErrors::CannotSettleBeforeMaturity(element) => element.fmt(f),
                IPeripheryOldErrors::DebugError(element) => element.fmt(f),
                IPeripheryOldErrors::ExpectedOppositeSigns(element) => element.fmt(f),
                IPeripheryOldErrors::ExpectedSqrtPriceZeroBeforeInit(element) => element.fmt(f),
                IPeripheryOldErrors::IRSNotionalAmountSpecifiedMustBeNonZero(element) => {
                    element.fmt(f)
                }
                IPeripheryOldErrors::InvalidMarginDelta(element) => element.fmt(f),
                IPeripheryOldErrors::LidoGetPooledEthBySharesReturnedZero(element) => {
                    element.fmt(f)
                }
                IPeripheryOldErrors::LiquidityDeltaMustBePositiveInBurn(element) => element.fmt(f),
                IPeripheryOldErrors::LiquidityDeltaMustBePositiveInMint(element) => element.fmt(f),
                IPeripheryOldErrors::MarginLessThanMinimum(element) => element.fmt(f),
                IPeripheryOldErrors::MarginRequirementNotMet(element) => element.fmt(f),
                IPeripheryOldErrors::MarginRequirementNotMetFCM(element) => element.fmt(f),
                IPeripheryOldErrors::NotEnoughFunds(element) => element.fmt(f),
                IPeripheryOldErrors::OOO(element) => element.fmt(f),
                IPeripheryOldErrors::OnlyFCM(element) => element.fmt(f),
                IPeripheryOldErrors::OnlyMarginEngine(element) => element.fmt(f),
                IPeripheryOldErrors::OnlyOwnerCanUpdatePosition(element) => element.fmt(f),
                IPeripheryOldErrors::OnlyVAMM(element) => element.fmt(f),
                IPeripheryOldErrors::PositionNetZero(element) => element.fmt(f),
                IPeripheryOldErrors::PositionNotSettled(element) => element.fmt(f),
                IPeripheryOldErrors::RocketPoolGetEthValueReturnedZero(element) => element.fmt(f),
                IPeripheryOldErrors::WithdrawalExceedsCurrentMargin(element) => element.fmt(f),
                IPeripheryOldErrors::closeToOrBeyondMaturity(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AavePoolGetReserveNormalizedIncomeReturnedZero> for IPeripheryOldErrors {
        fn from(var: AavePoolGetReserveNormalizedIncomeReturnedZero) -> Self {
            IPeripheryOldErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(var)
        }
    }
    impl ::std::convert::From<AavePoolGetReserveNormalizedVariableDebtReturnedZero>
        for IPeripheryOldErrors
    {
        fn from(var: AavePoolGetReserveNormalizedVariableDebtReturnedZero) -> Self {
            IPeripheryOldErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(var)
        }
    }
    impl ::std::convert::From<CTokenExchangeRateReturnedZero> for IPeripheryOldErrors {
        fn from(var: CTokenExchangeRateReturnedZero) -> Self {
            IPeripheryOldErrors::CTokenExchangeRateReturnedZero(var)
        }
    }
    impl ::std::convert::From<CanOnlyTradeIfUnlocked> for IPeripheryOldErrors {
        fn from(var: CanOnlyTradeIfUnlocked) -> Self {
            IPeripheryOldErrors::CanOnlyTradeIfUnlocked(var)
        }
    }
    impl ::std::convert::From<CannotLiquidate> for IPeripheryOldErrors {
        fn from(var: CannotLiquidate) -> Self {
            IPeripheryOldErrors::CannotLiquidate(var)
        }
    }
    impl ::std::convert::From<CannotSettleBeforeMaturity> for IPeripheryOldErrors {
        fn from(var: CannotSettleBeforeMaturity) -> Self {
            IPeripheryOldErrors::CannotSettleBeforeMaturity(var)
        }
    }
    impl ::std::convert::From<DebugError> for IPeripheryOldErrors {
        fn from(var: DebugError) -> Self {
            IPeripheryOldErrors::DebugError(var)
        }
    }
    impl ::std::convert::From<ExpectedOppositeSigns> for IPeripheryOldErrors {
        fn from(var: ExpectedOppositeSigns) -> Self {
            IPeripheryOldErrors::ExpectedOppositeSigns(var)
        }
    }
    impl ::std::convert::From<ExpectedSqrtPriceZeroBeforeInit> for IPeripheryOldErrors {
        fn from(var: ExpectedSqrtPriceZeroBeforeInit) -> Self {
            IPeripheryOldErrors::ExpectedSqrtPriceZeroBeforeInit(var)
        }
    }
    impl ::std::convert::From<IRSNotionalAmountSpecifiedMustBeNonZero> for IPeripheryOldErrors {
        fn from(var: IRSNotionalAmountSpecifiedMustBeNonZero) -> Self {
            IPeripheryOldErrors::IRSNotionalAmountSpecifiedMustBeNonZero(var)
        }
    }
    impl ::std::convert::From<InvalidMarginDelta> for IPeripheryOldErrors {
        fn from(var: InvalidMarginDelta) -> Self {
            IPeripheryOldErrors::InvalidMarginDelta(var)
        }
    }
    impl ::std::convert::From<LidoGetPooledEthBySharesReturnedZero> for IPeripheryOldErrors {
        fn from(var: LidoGetPooledEthBySharesReturnedZero) -> Self {
            IPeripheryOldErrors::LidoGetPooledEthBySharesReturnedZero(var)
        }
    }
    impl ::std::convert::From<LiquidityDeltaMustBePositiveInBurn> for IPeripheryOldErrors {
        fn from(var: LiquidityDeltaMustBePositiveInBurn) -> Self {
            IPeripheryOldErrors::LiquidityDeltaMustBePositiveInBurn(var)
        }
    }
    impl ::std::convert::From<LiquidityDeltaMustBePositiveInMint> for IPeripheryOldErrors {
        fn from(var: LiquidityDeltaMustBePositiveInMint) -> Self {
            IPeripheryOldErrors::LiquidityDeltaMustBePositiveInMint(var)
        }
    }
    impl ::std::convert::From<MarginLessThanMinimum> for IPeripheryOldErrors {
        fn from(var: MarginLessThanMinimum) -> Self {
            IPeripheryOldErrors::MarginLessThanMinimum(var)
        }
    }
    impl ::std::convert::From<MarginRequirementNotMet> for IPeripheryOldErrors {
        fn from(var: MarginRequirementNotMet) -> Self {
            IPeripheryOldErrors::MarginRequirementNotMet(var)
        }
    }
    impl ::std::convert::From<MarginRequirementNotMetFCM> for IPeripheryOldErrors {
        fn from(var: MarginRequirementNotMetFCM) -> Self {
            IPeripheryOldErrors::MarginRequirementNotMetFCM(var)
        }
    }
    impl ::std::convert::From<NotEnoughFunds> for IPeripheryOldErrors {
        fn from(var: NotEnoughFunds) -> Self {
            IPeripheryOldErrors::NotEnoughFunds(var)
        }
    }
    impl ::std::convert::From<OOO> for IPeripheryOldErrors {
        fn from(var: OOO) -> Self {
            IPeripheryOldErrors::OOO(var)
        }
    }
    impl ::std::convert::From<OnlyFCM> for IPeripheryOldErrors {
        fn from(var: OnlyFCM) -> Self {
            IPeripheryOldErrors::OnlyFCM(var)
        }
    }
    impl ::std::convert::From<OnlyMarginEngine> for IPeripheryOldErrors {
        fn from(var: OnlyMarginEngine) -> Self {
            IPeripheryOldErrors::OnlyMarginEngine(var)
        }
    }
    impl ::std::convert::From<OnlyOwnerCanUpdatePosition> for IPeripheryOldErrors {
        fn from(var: OnlyOwnerCanUpdatePosition) -> Self {
            IPeripheryOldErrors::OnlyOwnerCanUpdatePosition(var)
        }
    }
    impl ::std::convert::From<OnlyVAMM> for IPeripheryOldErrors {
        fn from(var: OnlyVAMM) -> Self {
            IPeripheryOldErrors::OnlyVAMM(var)
        }
    }
    impl ::std::convert::From<PositionNetZero> for IPeripheryOldErrors {
        fn from(var: PositionNetZero) -> Self {
            IPeripheryOldErrors::PositionNetZero(var)
        }
    }
    impl ::std::convert::From<PositionNotSettled> for IPeripheryOldErrors {
        fn from(var: PositionNotSettled) -> Self {
            IPeripheryOldErrors::PositionNotSettled(var)
        }
    }
    impl ::std::convert::From<RocketPoolGetEthValueReturnedZero> for IPeripheryOldErrors {
        fn from(var: RocketPoolGetEthValueReturnedZero) -> Self {
            IPeripheryOldErrors::RocketPoolGetEthValueReturnedZero(var)
        }
    }
    impl ::std::convert::From<WithdrawalExceedsCurrentMargin> for IPeripheryOldErrors {
        fn from(var: WithdrawalExceedsCurrentMargin) -> Self {
            IPeripheryOldErrors::WithdrawalExceedsCurrentMargin(var)
        }
    }
    impl ::std::convert::From<closeToOrBeyondMaturity> for IPeripheryOldErrors {
        fn from(var: closeToOrBeyondMaturity) -> Self {
            IPeripheryOldErrors::closeToOrBeyondMaturity(var)
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
    #[ethevent(name = "MarginCap", abi = "MarginCap(address,int256)")]
    pub struct MarginCapFilter {
        pub vamm: ethers::core::types::Address,
        pub lp_margin_cap_new: I256,
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
    #[doc = "Container type for all input parameters for the `rolloverWithSwap` function with signature `rolloverWithSwap(address,address,int24,int24,(address,bool,uint256,uint160,int24,int24,uint256))` and selector `[141, 142, 9, 131]`"]
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
        abi = "rolloverWithSwap(address,address,int24,int24,(address,bool,uint256,uint160,int24,int24,uint256))"
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
    #[doc = "Container type for all input parameters for the `swap` function with signature `swap((address,bool,uint256,uint160,int24,int24,uint256))` and selector `[52, 144, 126, 52]`"]
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
        abi = "swap((address,bool,uint256,uint160,int24,int24,uint256))"
    )]
    pub struct SwapCall {
        pub params: SwapPeripheryParams,
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IPeripheryOldCalls {
        GetCurrentTick(GetCurrentTickCall),
        LpMarginCaps(LpMarginCapsCall),
        LpMarginCumulatives(LpMarginCumulativesCall),
        MintOrBurn(MintOrBurnCall),
        RolloverWithMint(RolloverWithMintCall),
        RolloverWithSwap(RolloverWithSwapCall),
        SetLPMarginCap(SetLPMarginCapCall),
        SetLPMarginCumulative(SetLPMarginCumulativeCall),
        SettlePositionAndWithdrawMargin(SettlePositionAndWithdrawMarginCall),
        Swap(SwapCall),
        UpdatePositionMargin(UpdatePositionMarginCall),
    }
    impl ethers::core::abi::AbiDecode for IPeripheryOldCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetCurrentTickCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPeripheryOldCalls::GetCurrentTick(decoded));
            }
            if let Ok(decoded) =
                <LpMarginCapsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPeripheryOldCalls::LpMarginCaps(decoded));
            }
            if let Ok(decoded) =
                <LpMarginCumulativesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPeripheryOldCalls::LpMarginCumulatives(decoded));
            }
            if let Ok(decoded) =
                <MintOrBurnCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPeripheryOldCalls::MintOrBurn(decoded));
            }
            if let Ok(decoded) =
                <RolloverWithMintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPeripheryOldCalls::RolloverWithMint(decoded));
            }
            if let Ok(decoded) =
                <RolloverWithSwapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPeripheryOldCalls::RolloverWithSwap(decoded));
            }
            if let Ok(decoded) =
                <SetLPMarginCapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPeripheryOldCalls::SetLPMarginCap(decoded));
            }
            if let Ok(decoded) =
                <SetLPMarginCumulativeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPeripheryOldCalls::SetLPMarginCumulative(decoded));
            }
            if let Ok(decoded) =
                <SettlePositionAndWithdrawMarginCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IPeripheryOldCalls::SettlePositionAndWithdrawMargin(decoded));
            }
            if let Ok(decoded) = <SwapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IPeripheryOldCalls::Swap(decoded));
            }
            if let Ok(decoded) =
                <UpdatePositionMarginCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPeripheryOldCalls::UpdatePositionMargin(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IPeripheryOldCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IPeripheryOldCalls::GetCurrentTick(element) => element.encode(),
                IPeripheryOldCalls::LpMarginCaps(element) => element.encode(),
                IPeripheryOldCalls::LpMarginCumulatives(element) => element.encode(),
                IPeripheryOldCalls::MintOrBurn(element) => element.encode(),
                IPeripheryOldCalls::RolloverWithMint(element) => element.encode(),
                IPeripheryOldCalls::RolloverWithSwap(element) => element.encode(),
                IPeripheryOldCalls::SetLPMarginCap(element) => element.encode(),
                IPeripheryOldCalls::SetLPMarginCumulative(element) => element.encode(),
                IPeripheryOldCalls::SettlePositionAndWithdrawMargin(element) => element.encode(),
                IPeripheryOldCalls::Swap(element) => element.encode(),
                IPeripheryOldCalls::UpdatePositionMargin(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IPeripheryOldCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IPeripheryOldCalls::GetCurrentTick(element) => element.fmt(f),
                IPeripheryOldCalls::LpMarginCaps(element) => element.fmt(f),
                IPeripheryOldCalls::LpMarginCumulatives(element) => element.fmt(f),
                IPeripheryOldCalls::MintOrBurn(element) => element.fmt(f),
                IPeripheryOldCalls::RolloverWithMint(element) => element.fmt(f),
                IPeripheryOldCalls::RolloverWithSwap(element) => element.fmt(f),
                IPeripheryOldCalls::SetLPMarginCap(element) => element.fmt(f),
                IPeripheryOldCalls::SetLPMarginCumulative(element) => element.fmt(f),
                IPeripheryOldCalls::SettlePositionAndWithdrawMargin(element) => element.fmt(f),
                IPeripheryOldCalls::Swap(element) => element.fmt(f),
                IPeripheryOldCalls::UpdatePositionMargin(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetCurrentTickCall> for IPeripheryOldCalls {
        fn from(var: GetCurrentTickCall) -> Self {
            IPeripheryOldCalls::GetCurrentTick(var)
        }
    }
    impl ::std::convert::From<LpMarginCapsCall> for IPeripheryOldCalls {
        fn from(var: LpMarginCapsCall) -> Self {
            IPeripheryOldCalls::LpMarginCaps(var)
        }
    }
    impl ::std::convert::From<LpMarginCumulativesCall> for IPeripheryOldCalls {
        fn from(var: LpMarginCumulativesCall) -> Self {
            IPeripheryOldCalls::LpMarginCumulatives(var)
        }
    }
    impl ::std::convert::From<MintOrBurnCall> for IPeripheryOldCalls {
        fn from(var: MintOrBurnCall) -> Self {
            IPeripheryOldCalls::MintOrBurn(var)
        }
    }
    impl ::std::convert::From<RolloverWithMintCall> for IPeripheryOldCalls {
        fn from(var: RolloverWithMintCall) -> Self {
            IPeripheryOldCalls::RolloverWithMint(var)
        }
    }
    impl ::std::convert::From<RolloverWithSwapCall> for IPeripheryOldCalls {
        fn from(var: RolloverWithSwapCall) -> Self {
            IPeripheryOldCalls::RolloverWithSwap(var)
        }
    }
    impl ::std::convert::From<SetLPMarginCapCall> for IPeripheryOldCalls {
        fn from(var: SetLPMarginCapCall) -> Self {
            IPeripheryOldCalls::SetLPMarginCap(var)
        }
    }
    impl ::std::convert::From<SetLPMarginCumulativeCall> for IPeripheryOldCalls {
        fn from(var: SetLPMarginCumulativeCall) -> Self {
            IPeripheryOldCalls::SetLPMarginCumulative(var)
        }
    }
    impl ::std::convert::From<SettlePositionAndWithdrawMarginCall> for IPeripheryOldCalls {
        fn from(var: SettlePositionAndWithdrawMarginCall) -> Self {
            IPeripheryOldCalls::SettlePositionAndWithdrawMargin(var)
        }
    }
    impl ::std::convert::From<SwapCall> for IPeripheryOldCalls {
        fn from(var: SwapCall) -> Self {
            IPeripheryOldCalls::Swap(var)
        }
    }
    impl ::std::convert::From<UpdatePositionMarginCall> for IPeripheryOldCalls {
        fn from(var: UpdatePositionMarginCall) -> Self {
            IPeripheryOldCalls::UpdatePositionMargin(var)
        }
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
    #[doc = "Container type for all return fields from the `rolloverWithSwap` function with signature `rolloverWithSwap(address,address,int24,int24,(address,bool,uint256,uint160,int24,int24,uint256))` and selector `[141, 142, 9, 131]`"]
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
    #[doc = "Container type for all return fields from the `swap` function with signature `swap((address,bool,uint256,uint160,int24,int24,uint256))` and selector `[52, 144, 126, 52]`"]
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
    }
}
