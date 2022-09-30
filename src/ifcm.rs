pub use ifcm::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod ifcm {
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
    #[doc = "IFCM was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IFCM_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"type\":\"error\",\"name\":\"AavePoolGetReserveNormalizedIncomeReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"AavePoolGetReserveNormalizedVariableDebtReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CTokenExchangeRateReturnedZero\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"unlocked\",\"type\":\"bool\",\"components\":[]}],\"type\":\"error\",\"name\":\"CanOnlyTradeIfUnlocked\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CannotLiquidate\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CannotSettleBeforeMaturity\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"DebugError\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"amount0\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"amount1\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"ExpectedOppositeSigns\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint160\",\"name\":\"sqrtPriceX96\",\"type\":\"uint160\",\"components\":[]}],\"type\":\"error\",\"name\":\"ExpectedSqrtPriceZeroBeforeInit\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"IRSNotionalAmountSpecifiedMustBeNonZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidMarginDelta\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"LidoGetPooledEthBySharesReturnedZero\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"type\":\"error\",\"name\":\"LiquidityDeltaMustBePositiveInBurn\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"type\":\"error\",\"name\":\"LiquidityDeltaMustBePositiveInMint\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginLessThanMinimum\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tick\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"variableTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"cumulativeFeeIncurred\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenDeltaUnbalanced\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginRequirementNotMet\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginRequirementNotMetFCM\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"requested\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"available\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"NotEnoughFunds\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OOO\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyFCM\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyMarginEngine\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyOwnerCanUpdatePosition\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyVAMM\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PositionNetZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PositionNotSettled\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"RocketPoolGetEthValueReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"WithdrawalExceedsCurrentMargin\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"closeToOrBeyondMaturity\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"marginInScaledYieldBearingTokens\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"fixedTokenBalance\",\"type\":\"int256\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"variableTokenBalance\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FCMTraderUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"desiredNotional\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceLimitX96\",\"type\":\"uint160\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"cumulativeFeeIncurred\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"fixedTokenDelta\",\"type\":\"int256\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"variableTokenDelta\",\"type\":\"int256\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"fixedTokenDeltaUnbalanced\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FullyCollateralisedSwap\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"desiredNotional\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceLimitX96\",\"type\":\"uint160\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"cumulativeFeeIncurred\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"fixedTokenDelta\",\"type\":\"int256\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"variableTokenDelta\",\"type\":\"int256\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"fixedTokenDeltaUnbalanced\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FullyCollateralisedUnwind\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"int256\",\"name\":\"settlementCashflow\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"fcmPositionSettlement\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getTraderWithYieldBearingAssets\",\"outputs\":[{\"internalType\":\"struct TraderWithYieldBearingAssets.Info\",\"name\":\"traderInfo\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"marginInScaledYieldBearingTokens\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenBalance\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"variableTokenBalance\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isSettled\",\"type\":\"bool\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"contract IVAMM\",\"name\":\"__vamm\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IMarginEngine\",\"name\":\"__marginEngine\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"notional\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceLimitX96\",\"type\":\"uint160\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initiateFullyCollateralisedFixedTakerSwap\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"fixedTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"variableTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"cumulativeFeeIncurred\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenDeltaUnbalanced\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"marginEngine\",\"outputs\":[{\"internalType\":\"contract IMarginEngine\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"rateOracle\",\"outputs\":[{\"internalType\":\"contract IRateOracle\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"state\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPausability\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"settleTrader\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"marginDeltaInUnderlyingTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferMarginToMarginEngineTrader\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"notionalToUnwind\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceLimitX96\",\"type\":\"uint160\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unwindFullyCollateralisedFixedTakerSwap\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"fixedTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"variableTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"cumulativeFeeIncurred\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenDeltaUnbalanced\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"vamm\",\"outputs\":[{\"internalType\":\"contract IVAMM\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct IFCM<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IFCM<M> {
        fn clone(&self) -> Self {
            IFCM(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IFCM<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IFCM<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IFCM))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IFCM<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IFCM_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `getTraderWithYieldBearingAssets` (0x9a2f48f5) function"]
        pub fn get_trader_with_yield_bearing_assets(
            &self,
            trader: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, Info> {
            self.0
                .method_hash([154, 47, 72, 245], trader)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0x485cc955) function"]
        pub fn initialize(
            &self,
            vamm: ethers::core::types::Address,
            margin_engine: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([72, 92, 201, 85], (vamm, margin_engine))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initiateFullyCollateralisedFixedTakerSwap` (0x55468a8b) function"]
        pub fn initiate_fully_collateralised_fixed_taker_swap(
            &self,
            notional: ethers::core::types::U256,
            sqrt_price_limit_x96: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (I256, I256, ethers::core::types::U256, I256),
        > {
            self.0
                .method_hash([85, 70, 138, 139], (notional, sqrt_price_limit_x96))
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
        #[doc = "Calls the contract's `rateOracle` (0x98f4b1b2) function"]
        pub fn rate_oracle(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([152, 244, 177, 178], ())
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
        #[doc = "Calls the contract's `settleTrader` (0xebc9b02e) function"]
        pub fn settle_trader(&self) -> ethers::contract::builders::ContractCall<M, I256> {
            self.0
                .method_hash([235, 201, 176, 46], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferMarginToMarginEngineTrader` (0xc1ccfa68) function"]
        pub fn transfer_margin_to_margin_engine_trader(
            &self,
            account: ethers::core::types::Address,
            margin_delta_in_underlying_tokens: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [193, 204, 250, 104],
                    (account, margin_delta_in_underlying_tokens),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unwindFullyCollateralisedFixedTakerSwap` (0x03742274) function"]
        pub fn unwind_fully_collateralised_fixed_taker_swap(
            &self,
            notional_to_unwind: ethers::core::types::U256,
            sqrt_price_limit_x96: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (I256, I256, ethers::core::types::U256, I256),
        > {
            self.0
                .method_hash(
                    [3, 116, 34, 116],
                    (notional_to_unwind, sqrt_price_limit_x96),
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
        #[doc = "Gets the contract's `FCMTraderUpdate` event"]
        pub fn fcm_trader_update_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, FcmtraderUpdateFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `FullyCollateralisedSwap` event"]
        pub fn fully_collateralised_swap_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, FullyCollateralisedSwapFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `FullyCollateralisedUnwind` event"]
        pub fn fully_collateralised_unwind_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, FullyCollateralisedUnwindFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `fcmPositionSettlement` event"]
        pub fn fcm_position_settlement_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, FcmPositionSettlementFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, IFCMEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IFCM<M> {
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
    pub enum IFCMErrors {
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
    impl ethers::core::abi::AbiDecode for IFCMErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok (decoded) = < AavePoolGetReserveNormalizedIncomeReturnedZero as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (IFCMErrors :: AavePoolGetReserveNormalizedIncomeReturnedZero (decoded)) }
            if let Ok (decoded) = < AavePoolGetReserveNormalizedVariableDebtReturnedZero as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (IFCMErrors :: AavePoolGetReserveNormalizedVariableDebtReturnedZero (decoded)) }
            if let Ok(decoded) =
                <CTokenExchangeRateReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IFCMErrors::CTokenExchangeRateReturnedZero(decoded));
            }
            if let Ok(decoded) =
                <CanOnlyTradeIfUnlocked as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFCMErrors::CanOnlyTradeIfUnlocked(decoded));
            }
            if let Ok(decoded) =
                <CannotLiquidate as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFCMErrors::CannotLiquidate(decoded));
            }
            if let Ok(decoded) =
                <CannotSettleBeforeMaturity as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFCMErrors::CannotSettleBeforeMaturity(decoded));
            }
            if let Ok(decoded) = <DebugError as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFCMErrors::DebugError(decoded));
            }
            if let Ok(decoded) =
                <ExpectedOppositeSigns as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFCMErrors::ExpectedOppositeSigns(decoded));
            }
            if let Ok(decoded) =
                <ExpectedSqrtPriceZeroBeforeInit as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IFCMErrors::ExpectedSqrtPriceZeroBeforeInit(decoded));
            }
            if let Ok(decoded) =
                <IRSNotionalAmountSpecifiedMustBeNonZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IFCMErrors::IRSNotionalAmountSpecifiedMustBeNonZero(decoded));
            }
            if let Ok(decoded) =
                <InvalidMarginDelta as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFCMErrors::InvalidMarginDelta(decoded));
            }
            if let Ok(decoded) =
                <LidoGetPooledEthBySharesReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IFCMErrors::LidoGetPooledEthBySharesReturnedZero(decoded));
            }
            if let Ok(decoded) =
                <LiquidityDeltaMustBePositiveInBurn as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IFCMErrors::LiquidityDeltaMustBePositiveInBurn(decoded));
            }
            if let Ok(decoded) =
                <LiquidityDeltaMustBePositiveInMint as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IFCMErrors::LiquidityDeltaMustBePositiveInMint(decoded));
            }
            if let Ok(decoded) =
                <MarginLessThanMinimum as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFCMErrors::MarginLessThanMinimum(decoded));
            }
            if let Ok(decoded) =
                <MarginRequirementNotMet as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFCMErrors::MarginRequirementNotMet(decoded));
            }
            if let Ok(decoded) =
                <MarginRequirementNotMetFCM as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFCMErrors::MarginRequirementNotMetFCM(decoded));
            }
            if let Ok(decoded) =
                <NotEnoughFunds as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFCMErrors::NotEnoughFunds(decoded));
            }
            if let Ok(decoded) = <OOO as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IFCMErrors::OOO(decoded));
            }
            if let Ok(decoded) = <OnlyFCM as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IFCMErrors::OnlyFCM(decoded));
            }
            if let Ok(decoded) =
                <OnlyMarginEngine as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFCMErrors::OnlyMarginEngine(decoded));
            }
            if let Ok(decoded) =
                <OnlyOwnerCanUpdatePosition as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFCMErrors::OnlyOwnerCanUpdatePosition(decoded));
            }
            if let Ok(decoded) = <OnlyVAMM as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IFCMErrors::OnlyVAMM(decoded));
            }
            if let Ok(decoded) =
                <PositionNetZero as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFCMErrors::PositionNetZero(decoded));
            }
            if let Ok(decoded) =
                <PositionNotSettled as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFCMErrors::PositionNotSettled(decoded));
            }
            if let Ok(decoded) =
                <RocketPoolGetEthValueReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IFCMErrors::RocketPoolGetEthValueReturnedZero(decoded));
            }
            if let Ok(decoded) =
                <WithdrawalExceedsCurrentMargin as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IFCMErrors::WithdrawalExceedsCurrentMargin(decoded));
            }
            if let Ok(decoded) =
                <closeToOrBeyondMaturity as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFCMErrors::closeToOrBeyondMaturity(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IFCMErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                IFCMErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(element) => {
                    element.encode()
                }
                IFCMErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(element) => {
                    element.encode()
                }
                IFCMErrors::CTokenExchangeRateReturnedZero(element) => element.encode(),
                IFCMErrors::CanOnlyTradeIfUnlocked(element) => element.encode(),
                IFCMErrors::CannotLiquidate(element) => element.encode(),
                IFCMErrors::CannotSettleBeforeMaturity(element) => element.encode(),
                IFCMErrors::DebugError(element) => element.encode(),
                IFCMErrors::ExpectedOppositeSigns(element) => element.encode(),
                IFCMErrors::ExpectedSqrtPriceZeroBeforeInit(element) => element.encode(),
                IFCMErrors::IRSNotionalAmountSpecifiedMustBeNonZero(element) => element.encode(),
                IFCMErrors::InvalidMarginDelta(element) => element.encode(),
                IFCMErrors::LidoGetPooledEthBySharesReturnedZero(element) => element.encode(),
                IFCMErrors::LiquidityDeltaMustBePositiveInBurn(element) => element.encode(),
                IFCMErrors::LiquidityDeltaMustBePositiveInMint(element) => element.encode(),
                IFCMErrors::MarginLessThanMinimum(element) => element.encode(),
                IFCMErrors::MarginRequirementNotMet(element) => element.encode(),
                IFCMErrors::MarginRequirementNotMetFCM(element) => element.encode(),
                IFCMErrors::NotEnoughFunds(element) => element.encode(),
                IFCMErrors::OOO(element) => element.encode(),
                IFCMErrors::OnlyFCM(element) => element.encode(),
                IFCMErrors::OnlyMarginEngine(element) => element.encode(),
                IFCMErrors::OnlyOwnerCanUpdatePosition(element) => element.encode(),
                IFCMErrors::OnlyVAMM(element) => element.encode(),
                IFCMErrors::PositionNetZero(element) => element.encode(),
                IFCMErrors::PositionNotSettled(element) => element.encode(),
                IFCMErrors::RocketPoolGetEthValueReturnedZero(element) => element.encode(),
                IFCMErrors::WithdrawalExceedsCurrentMargin(element) => element.encode(),
                IFCMErrors::closeToOrBeyondMaturity(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IFCMErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IFCMErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(element) => {
                    element.fmt(f)
                }
                IFCMErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(element) => {
                    element.fmt(f)
                }
                IFCMErrors::CTokenExchangeRateReturnedZero(element) => element.fmt(f),
                IFCMErrors::CanOnlyTradeIfUnlocked(element) => element.fmt(f),
                IFCMErrors::CannotLiquidate(element) => element.fmt(f),
                IFCMErrors::CannotSettleBeforeMaturity(element) => element.fmt(f),
                IFCMErrors::DebugError(element) => element.fmt(f),
                IFCMErrors::ExpectedOppositeSigns(element) => element.fmt(f),
                IFCMErrors::ExpectedSqrtPriceZeroBeforeInit(element) => element.fmt(f),
                IFCMErrors::IRSNotionalAmountSpecifiedMustBeNonZero(element) => element.fmt(f),
                IFCMErrors::InvalidMarginDelta(element) => element.fmt(f),
                IFCMErrors::LidoGetPooledEthBySharesReturnedZero(element) => element.fmt(f),
                IFCMErrors::LiquidityDeltaMustBePositiveInBurn(element) => element.fmt(f),
                IFCMErrors::LiquidityDeltaMustBePositiveInMint(element) => element.fmt(f),
                IFCMErrors::MarginLessThanMinimum(element) => element.fmt(f),
                IFCMErrors::MarginRequirementNotMet(element) => element.fmt(f),
                IFCMErrors::MarginRequirementNotMetFCM(element) => element.fmt(f),
                IFCMErrors::NotEnoughFunds(element) => element.fmt(f),
                IFCMErrors::OOO(element) => element.fmt(f),
                IFCMErrors::OnlyFCM(element) => element.fmt(f),
                IFCMErrors::OnlyMarginEngine(element) => element.fmt(f),
                IFCMErrors::OnlyOwnerCanUpdatePosition(element) => element.fmt(f),
                IFCMErrors::OnlyVAMM(element) => element.fmt(f),
                IFCMErrors::PositionNetZero(element) => element.fmt(f),
                IFCMErrors::PositionNotSettled(element) => element.fmt(f),
                IFCMErrors::RocketPoolGetEthValueReturnedZero(element) => element.fmt(f),
                IFCMErrors::WithdrawalExceedsCurrentMargin(element) => element.fmt(f),
                IFCMErrors::closeToOrBeyondMaturity(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AavePoolGetReserveNormalizedIncomeReturnedZero> for IFCMErrors {
        fn from(var: AavePoolGetReserveNormalizedIncomeReturnedZero) -> Self {
            IFCMErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(var)
        }
    }
    impl ::std::convert::From<AavePoolGetReserveNormalizedVariableDebtReturnedZero> for IFCMErrors {
        fn from(var: AavePoolGetReserveNormalizedVariableDebtReturnedZero) -> Self {
            IFCMErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(var)
        }
    }
    impl ::std::convert::From<CTokenExchangeRateReturnedZero> for IFCMErrors {
        fn from(var: CTokenExchangeRateReturnedZero) -> Self {
            IFCMErrors::CTokenExchangeRateReturnedZero(var)
        }
    }
    impl ::std::convert::From<CanOnlyTradeIfUnlocked> for IFCMErrors {
        fn from(var: CanOnlyTradeIfUnlocked) -> Self {
            IFCMErrors::CanOnlyTradeIfUnlocked(var)
        }
    }
    impl ::std::convert::From<CannotLiquidate> for IFCMErrors {
        fn from(var: CannotLiquidate) -> Self {
            IFCMErrors::CannotLiquidate(var)
        }
    }
    impl ::std::convert::From<CannotSettleBeforeMaturity> for IFCMErrors {
        fn from(var: CannotSettleBeforeMaturity) -> Self {
            IFCMErrors::CannotSettleBeforeMaturity(var)
        }
    }
    impl ::std::convert::From<DebugError> for IFCMErrors {
        fn from(var: DebugError) -> Self {
            IFCMErrors::DebugError(var)
        }
    }
    impl ::std::convert::From<ExpectedOppositeSigns> for IFCMErrors {
        fn from(var: ExpectedOppositeSigns) -> Self {
            IFCMErrors::ExpectedOppositeSigns(var)
        }
    }
    impl ::std::convert::From<ExpectedSqrtPriceZeroBeforeInit> for IFCMErrors {
        fn from(var: ExpectedSqrtPriceZeroBeforeInit) -> Self {
            IFCMErrors::ExpectedSqrtPriceZeroBeforeInit(var)
        }
    }
    impl ::std::convert::From<IRSNotionalAmountSpecifiedMustBeNonZero> for IFCMErrors {
        fn from(var: IRSNotionalAmountSpecifiedMustBeNonZero) -> Self {
            IFCMErrors::IRSNotionalAmountSpecifiedMustBeNonZero(var)
        }
    }
    impl ::std::convert::From<InvalidMarginDelta> for IFCMErrors {
        fn from(var: InvalidMarginDelta) -> Self {
            IFCMErrors::InvalidMarginDelta(var)
        }
    }
    impl ::std::convert::From<LidoGetPooledEthBySharesReturnedZero> for IFCMErrors {
        fn from(var: LidoGetPooledEthBySharesReturnedZero) -> Self {
            IFCMErrors::LidoGetPooledEthBySharesReturnedZero(var)
        }
    }
    impl ::std::convert::From<LiquidityDeltaMustBePositiveInBurn> for IFCMErrors {
        fn from(var: LiquidityDeltaMustBePositiveInBurn) -> Self {
            IFCMErrors::LiquidityDeltaMustBePositiveInBurn(var)
        }
    }
    impl ::std::convert::From<LiquidityDeltaMustBePositiveInMint> for IFCMErrors {
        fn from(var: LiquidityDeltaMustBePositiveInMint) -> Self {
            IFCMErrors::LiquidityDeltaMustBePositiveInMint(var)
        }
    }
    impl ::std::convert::From<MarginLessThanMinimum> for IFCMErrors {
        fn from(var: MarginLessThanMinimum) -> Self {
            IFCMErrors::MarginLessThanMinimum(var)
        }
    }
    impl ::std::convert::From<MarginRequirementNotMet> for IFCMErrors {
        fn from(var: MarginRequirementNotMet) -> Self {
            IFCMErrors::MarginRequirementNotMet(var)
        }
    }
    impl ::std::convert::From<MarginRequirementNotMetFCM> for IFCMErrors {
        fn from(var: MarginRequirementNotMetFCM) -> Self {
            IFCMErrors::MarginRequirementNotMetFCM(var)
        }
    }
    impl ::std::convert::From<NotEnoughFunds> for IFCMErrors {
        fn from(var: NotEnoughFunds) -> Self {
            IFCMErrors::NotEnoughFunds(var)
        }
    }
    impl ::std::convert::From<OOO> for IFCMErrors {
        fn from(var: OOO) -> Self {
            IFCMErrors::OOO(var)
        }
    }
    impl ::std::convert::From<OnlyFCM> for IFCMErrors {
        fn from(var: OnlyFCM) -> Self {
            IFCMErrors::OnlyFCM(var)
        }
    }
    impl ::std::convert::From<OnlyMarginEngine> for IFCMErrors {
        fn from(var: OnlyMarginEngine) -> Self {
            IFCMErrors::OnlyMarginEngine(var)
        }
    }
    impl ::std::convert::From<OnlyOwnerCanUpdatePosition> for IFCMErrors {
        fn from(var: OnlyOwnerCanUpdatePosition) -> Self {
            IFCMErrors::OnlyOwnerCanUpdatePosition(var)
        }
    }
    impl ::std::convert::From<OnlyVAMM> for IFCMErrors {
        fn from(var: OnlyVAMM) -> Self {
            IFCMErrors::OnlyVAMM(var)
        }
    }
    impl ::std::convert::From<PositionNetZero> for IFCMErrors {
        fn from(var: PositionNetZero) -> Self {
            IFCMErrors::PositionNetZero(var)
        }
    }
    impl ::std::convert::From<PositionNotSettled> for IFCMErrors {
        fn from(var: PositionNotSettled) -> Self {
            IFCMErrors::PositionNotSettled(var)
        }
    }
    impl ::std::convert::From<RocketPoolGetEthValueReturnedZero> for IFCMErrors {
        fn from(var: RocketPoolGetEthValueReturnedZero) -> Self {
            IFCMErrors::RocketPoolGetEthValueReturnedZero(var)
        }
    }
    impl ::std::convert::From<WithdrawalExceedsCurrentMargin> for IFCMErrors {
        fn from(var: WithdrawalExceedsCurrentMargin) -> Self {
            IFCMErrors::WithdrawalExceedsCurrentMargin(var)
        }
    }
    impl ::std::convert::From<closeToOrBeyondMaturity> for IFCMErrors {
        fn from(var: closeToOrBeyondMaturity) -> Self {
            IFCMErrors::closeToOrBeyondMaturity(var)
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
        name = "FCMTraderUpdate",
        abi = "FCMTraderUpdate(address,uint256,int256,int256)"
    )]
    pub struct FcmtraderUpdateFilter {
        #[ethevent(indexed)]
        pub trader: ethers::core::types::Address,
        pub margin_in_scaled_yield_bearing_tokens: ethers::core::types::U256,
        pub fixed_token_balance: I256,
        pub variable_token_balance: I256,
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
        name = "FullyCollateralisedSwap",
        abi = "FullyCollateralisedSwap(address,uint256,uint160,uint256,int256,int256,int256)"
    )]
    pub struct FullyCollateralisedSwapFilter {
        #[ethevent(indexed)]
        pub trader: ethers::core::types::Address,
        pub desired_notional: ethers::core::types::U256,
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
    #[ethevent(
        name = "FullyCollateralisedUnwind",
        abi = "FullyCollateralisedUnwind(address,uint256,uint160,uint256,int256,int256,int256)"
    )]
    pub struct FullyCollateralisedUnwindFilter {
        #[ethevent(indexed)]
        pub trader: ethers::core::types::Address,
        pub desired_notional: ethers::core::types::U256,
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
    #[ethevent(
        name = "fcmPositionSettlement",
        abi = "fcmPositionSettlement(address,int256)"
    )]
    pub struct FcmPositionSettlementFilter {
        #[ethevent(indexed)]
        pub trader: ethers::core::types::Address,
        pub settlement_cashflow: I256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IFCMEvents {
        FcmtraderUpdateFilter(FcmtraderUpdateFilter),
        FullyCollateralisedSwapFilter(FullyCollateralisedSwapFilter),
        FullyCollateralisedUnwindFilter(FullyCollateralisedUnwindFilter),
        FcmPositionSettlementFilter(FcmPositionSettlementFilter),
    }
    impl ethers::contract::EthLogDecode for IFCMEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = FcmtraderUpdateFilter::decode_log(log) {
                return Ok(IFCMEvents::FcmtraderUpdateFilter(decoded));
            }
            if let Ok(decoded) = FullyCollateralisedSwapFilter::decode_log(log) {
                return Ok(IFCMEvents::FullyCollateralisedSwapFilter(decoded));
            }
            if let Ok(decoded) = FullyCollateralisedUnwindFilter::decode_log(log) {
                return Ok(IFCMEvents::FullyCollateralisedUnwindFilter(decoded));
            }
            if let Ok(decoded) = FcmPositionSettlementFilter::decode_log(log) {
                return Ok(IFCMEvents::FcmPositionSettlementFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IFCMEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IFCMEvents::FcmtraderUpdateFilter(element) => element.fmt(f),
                IFCMEvents::FullyCollateralisedSwapFilter(element) => element.fmt(f),
                IFCMEvents::FullyCollateralisedUnwindFilter(element) => element.fmt(f),
                IFCMEvents::FcmPositionSettlementFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `getTraderWithYieldBearingAssets` function with signature `getTraderWithYieldBearingAssets(address)` and selector `[154, 47, 72, 245]`"]
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
        name = "getTraderWithYieldBearingAssets",
        abi = "getTraderWithYieldBearingAssets(address)"
    )]
    pub struct GetTraderWithYieldBearingAssetsCall {
        pub trader: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize(address,address)` and selector `[72, 92, 201, 85]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "initialize", abi = "initialize(address,address)")]
    pub struct InitializeCall {
        pub vamm: ethers::core::types::Address,
        pub margin_engine: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `initiateFullyCollateralisedFixedTakerSwap` function with signature `initiateFullyCollateralisedFixedTakerSwap(uint256,uint160)` and selector `[85, 70, 138, 139]`"]
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
        abi = "initiateFullyCollateralisedFixedTakerSwap(uint256,uint160)"
    )]
    pub struct InitiateFullyCollateralisedFixedTakerSwapCall {
        pub notional: ethers::core::types::U256,
        pub sqrt_price_limit_x96: ethers::core::types::U256,
    }
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
    #[doc = "Container type for all input parameters for the `settleTrader` function with signature `settleTrader()` and selector `[235, 201, 176, 46]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "settleTrader", abi = "settleTrader()")]
    pub struct SettleTraderCall;
    #[doc = "Container type for all input parameters for the `transferMarginToMarginEngineTrader` function with signature `transferMarginToMarginEngineTrader(address,uint256)` and selector `[193, 204, 250, 104]`"]
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
        name = "transferMarginToMarginEngineTrader",
        abi = "transferMarginToMarginEngineTrader(address,uint256)"
    )]
    pub struct TransferMarginToMarginEngineTraderCall {
        pub account: ethers::core::types::Address,
        pub margin_delta_in_underlying_tokens: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `unwindFullyCollateralisedFixedTakerSwap` function with signature `unwindFullyCollateralisedFixedTakerSwap(uint256,uint160)` and selector `[3, 116, 34, 116]`"]
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
        abi = "unwindFullyCollateralisedFixedTakerSwap(uint256,uint160)"
    )]
    pub struct UnwindFullyCollateralisedFixedTakerSwapCall {
        pub notional_to_unwind: ethers::core::types::U256,
        pub sqrt_price_limit_x96: ethers::core::types::U256,
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
    pub enum IFCMCalls {
        GetTraderWithYieldBearingAssets(GetTraderWithYieldBearingAssetsCall),
        Initialize(InitializeCall),
        InitiateFullyCollateralisedFixedTakerSwap(InitiateFullyCollateralisedFixedTakerSwapCall),
        MarginEngine(MarginEngineCall),
        RateOracle(RateOracleCall),
        SetPausability(SetPausabilityCall),
        SettleTrader(SettleTraderCall),
        TransferMarginToMarginEngineTrader(TransferMarginToMarginEngineTraderCall),
        UnwindFullyCollateralisedFixedTakerSwap(UnwindFullyCollateralisedFixedTakerSwapCall),
        Vamm(VammCall),
    }
    impl ethers::core::abi::AbiDecode for IFCMCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetTraderWithYieldBearingAssetsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IFCMCalls::GetTraderWithYieldBearingAssets(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFCMCalls::Initialize(decoded));
            }
            if let Ok (decoded) = < InitiateFullyCollateralisedFixedTakerSwapCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (IFCMCalls :: InitiateFullyCollateralisedFixedTakerSwap (decoded)) }
            if let Ok(decoded) =
                <MarginEngineCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFCMCalls::MarginEngine(decoded));
            }
            if let Ok(decoded) =
                <RateOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFCMCalls::RateOracle(decoded));
            }
            if let Ok(decoded) =
                <SetPausabilityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFCMCalls::SetPausability(decoded));
            }
            if let Ok(decoded) =
                <SettleTraderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFCMCalls::SettleTrader(decoded));
            }
            if let Ok(decoded) =
                <TransferMarginToMarginEngineTraderCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IFCMCalls::TransferMarginToMarginEngineTrader(decoded));
            }
            if let Ok (decoded) = < UnwindFullyCollateralisedFixedTakerSwapCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (IFCMCalls :: UnwindFullyCollateralisedFixedTakerSwap (decoded)) }
            if let Ok(decoded) = <VammCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IFCMCalls::Vamm(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IFCMCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IFCMCalls::GetTraderWithYieldBearingAssets(element) => element.encode(),
                IFCMCalls::Initialize(element) => element.encode(),
                IFCMCalls::InitiateFullyCollateralisedFixedTakerSwap(element) => element.encode(),
                IFCMCalls::MarginEngine(element) => element.encode(),
                IFCMCalls::RateOracle(element) => element.encode(),
                IFCMCalls::SetPausability(element) => element.encode(),
                IFCMCalls::SettleTrader(element) => element.encode(),
                IFCMCalls::TransferMarginToMarginEngineTrader(element) => element.encode(),
                IFCMCalls::UnwindFullyCollateralisedFixedTakerSwap(element) => element.encode(),
                IFCMCalls::Vamm(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IFCMCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IFCMCalls::GetTraderWithYieldBearingAssets(element) => element.fmt(f),
                IFCMCalls::Initialize(element) => element.fmt(f),
                IFCMCalls::InitiateFullyCollateralisedFixedTakerSwap(element) => element.fmt(f),
                IFCMCalls::MarginEngine(element) => element.fmt(f),
                IFCMCalls::RateOracle(element) => element.fmt(f),
                IFCMCalls::SetPausability(element) => element.fmt(f),
                IFCMCalls::SettleTrader(element) => element.fmt(f),
                IFCMCalls::TransferMarginToMarginEngineTrader(element) => element.fmt(f),
                IFCMCalls::UnwindFullyCollateralisedFixedTakerSwap(element) => element.fmt(f),
                IFCMCalls::Vamm(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetTraderWithYieldBearingAssetsCall> for IFCMCalls {
        fn from(var: GetTraderWithYieldBearingAssetsCall) -> Self {
            IFCMCalls::GetTraderWithYieldBearingAssets(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for IFCMCalls {
        fn from(var: InitializeCall) -> Self {
            IFCMCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<InitiateFullyCollateralisedFixedTakerSwapCall> for IFCMCalls {
        fn from(var: InitiateFullyCollateralisedFixedTakerSwapCall) -> Self {
            IFCMCalls::InitiateFullyCollateralisedFixedTakerSwap(var)
        }
    }
    impl ::std::convert::From<MarginEngineCall> for IFCMCalls {
        fn from(var: MarginEngineCall) -> Self {
            IFCMCalls::MarginEngine(var)
        }
    }
    impl ::std::convert::From<RateOracleCall> for IFCMCalls {
        fn from(var: RateOracleCall) -> Self {
            IFCMCalls::RateOracle(var)
        }
    }
    impl ::std::convert::From<SetPausabilityCall> for IFCMCalls {
        fn from(var: SetPausabilityCall) -> Self {
            IFCMCalls::SetPausability(var)
        }
    }
    impl ::std::convert::From<SettleTraderCall> for IFCMCalls {
        fn from(var: SettleTraderCall) -> Self {
            IFCMCalls::SettleTrader(var)
        }
    }
    impl ::std::convert::From<TransferMarginToMarginEngineTraderCall> for IFCMCalls {
        fn from(var: TransferMarginToMarginEngineTraderCall) -> Self {
            IFCMCalls::TransferMarginToMarginEngineTrader(var)
        }
    }
    impl ::std::convert::From<UnwindFullyCollateralisedFixedTakerSwapCall> for IFCMCalls {
        fn from(var: UnwindFullyCollateralisedFixedTakerSwapCall) -> Self {
            IFCMCalls::UnwindFullyCollateralisedFixedTakerSwap(var)
        }
    }
    impl ::std::convert::From<VammCall> for IFCMCalls {
        fn from(var: VammCall) -> Self {
            IFCMCalls::Vamm(var)
        }
    }
    #[doc = "Container type for all return fields from the `getTraderWithYieldBearingAssets` function with signature `getTraderWithYieldBearingAssets(address)` and selector `[154, 47, 72, 245]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetTraderWithYieldBearingAssetsReturn {
        pub trader_info: Info,
    }
    #[doc = "Container type for all return fields from the `initiateFullyCollateralisedFixedTakerSwap` function with signature `initiateFullyCollateralisedFixedTakerSwap(uint256,uint160)` and selector `[85, 70, 138, 139]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct InitiateFullyCollateralisedFixedTakerSwapReturn {
        pub fixed_token_delta: I256,
        pub variable_token_delta: I256,
        pub cumulative_fee_incurred: ethers::core::types::U256,
        pub fixed_token_delta_unbalanced: I256,
    }
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
    #[doc = "Container type for all return fields from the `settleTrader` function with signature `settleTrader()` and selector `[235, 201, 176, 46]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SettleTraderReturn(pub I256);
    #[doc = "Container type for all return fields from the `unwindFullyCollateralisedFixedTakerSwap` function with signature `unwindFullyCollateralisedFixedTakerSwap(uint256,uint160)` and selector `[3, 116, 34, 116]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct UnwindFullyCollateralisedFixedTakerSwapReturn {
        pub fixed_token_delta: I256,
        pub variable_token_delta: I256,
        pub cumulative_fee_incurred: ethers::core::types::U256,
        pub fixed_token_delta_unbalanced: I256,
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
