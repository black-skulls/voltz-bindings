pub use i_factory::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_factory {
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
    #[doc = "IFactory was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IFACTORY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"type\":\"error\",\"name\":\"AavePoolGetReserveNormalizedIncomeReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"AavePoolGetReserveNormalizedVariableDebtReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CTokenExchangeRateReturnedZero\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"unlocked\",\"type\":\"bool\",\"components\":[]}],\"type\":\"error\",\"name\":\"CanOnlyTradeIfUnlocked\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CannotLiquidate\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CannotSettleBeforeMaturity\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"DebugError\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"amount0\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"amount1\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"ExpectedOppositeSigns\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint160\",\"name\":\"sqrtPriceX96\",\"type\":\"uint160\",\"components\":[]}],\"type\":\"error\",\"name\":\"ExpectedSqrtPriceZeroBeforeInit\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"IRSNotionalAmountSpecifiedMustBeNonZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidMarginDelta\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"LidoGetPooledEthBySharesReturnedZero\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"type\":\"error\",\"name\":\"LiquidityDeltaMustBePositiveInBurn\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"type\":\"error\",\"name\":\"LiquidityDeltaMustBePositiveInMint\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginLessThanMinimum\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tick\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"variableTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"cumulativeFeeIncurred\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenDeltaUnbalanced\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginRequirementNotMet\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginRequirementNotMetFCM\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"requested\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"available\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"NotEnoughFunds\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OOO\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyFCM\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyMarginEngine\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyOwnerCanUpdatePosition\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyVAMM\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PositionNetZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PositionNotSettled\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"RocketPoolGetEthValueReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"WithdrawalExceedsCurrentMargin\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"closeToOrBeyondMaturity\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"intAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"isApproved\",\"type\":\"bool\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract IERC20Minimal\",\"name\":\"underlyingToken\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"contract IRateOracle\",\"name\":\"rateOracle\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"termStartTimestampWad\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"termEndTimestampWad\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"int24\",\"name\":\"tickSpacing\",\"type\":\"int24\",\"components\":[],\"indexed\":false},{\"internalType\":\"contract IMarginEngine\",\"name\":\"marginEngine\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"contract IVAMM\",\"name\":\"vamm\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"contract IFCM\",\"name\":\"fcm\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint8\",\"name\":\"yieldBearingProtocolID\",\"type\":\"uint8\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint8\",\"name\":\"underlyingTokenDecimals\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"IrsInstance\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract IFCM\",\"name\":\"masterFCMAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint8\",\"name\":\"yieldBearingProtocolID\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MasterFCM\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract IPeriphery\",\"name\":\"periphery\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PeripheryUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract IERC20Minimal\",\"name\":\"_underlyingToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IRateOracle\",\"name\":\"_rateOracle\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_termStartTimestampWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_termEndTimestampWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"_tickSpacing\",\"type\":\"int24\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deployIrsInstance\",\"outputs\":[{\"internalType\":\"contract IMarginEngine\",\"name\":\"marginEngineProxy\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IVAMM\",\"name\":\"vammProxy\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IFCM\",\"name\":\"fcmProxy\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"intAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isApproved\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"yieldBearingProtocolID\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"masterFCMs\",\"outputs\":[{\"internalType\":\"contract IFCM\",\"name\":\"masterFCM\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"masterMarginEngine\",\"outputs\":[{\"internalType\":\"contract IMarginEngine\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"masterVAMM\",\"outputs\":[{\"internalType\":\"contract IVAMM\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"periphery\",\"outputs\":[{\"internalType\":\"contract IPeriphery\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"intAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"allowIntegration\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setApproval\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract IFCM\",\"name\":\"masterFCM\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"yieldBearingProtocolID\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setMasterFCM\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract IMarginEngine\",\"name\":\"_masterMarginEngine\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setMasterMarginEngine\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract IVAMM\",\"name\":\"_masterVAMM\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setMasterVAMM\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract IPeriphery\",\"name\":\"_periphery\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPeriphery\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    pub struct IFactory<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IFactory<M> {
        fn clone(&self) -> Self {
            IFactory(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IFactory<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IFactory<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IFactory))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IFactory<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IFACTORY_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `deployIrsInstance` (0x0e8a0648) function"]
        pub fn deploy_irs_instance(
            &self,
            underlying_token: ethers::core::types::Address,
            rate_oracle: ethers::core::types::Address,
            term_start_timestamp_wad: ethers::core::types::U256,
            term_end_timestamp_wad: ethers::core::types::U256,
            tick_spacing: i32,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::Address,
                ethers::core::types::Address,
                ethers::core::types::Address,
            ),
        > {
            self.0
                .method_hash(
                    [14, 138, 6, 72],
                    (
                        underlying_token,
                        rate_oracle,
                        term_start_timestamp_wad,
                        term_end_timestamp_wad,
                        tick_spacing,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isApproved` (0xa389783e) function"]
        pub fn is_approved(
            &self,
            owner: ethers::core::types::Address,
            int_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([163, 137, 120, 62], (owner, int_address))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `masterFCMs` (0xca5183b7) function"]
        pub fn master_fc_ms(
            &self,
            yield_bearing_protocol_id: u8,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([202, 81, 131, 183], yield_bearing_protocol_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `masterMarginEngine` (0x9fe1b354) function"]
        pub fn master_margin_engine(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([159, 225, 179, 84], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `masterVAMM` (0xfebfe75e) function"]
        pub fn master_vamm(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([254, 191, 231, 94], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `periphery` (0x77aace1a) function"]
        pub fn periphery(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([119, 170, 206, 26], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setApproval` (0xdb9b7170) function"]
        pub fn set_approval(
            &self,
            int_address: ethers::core::types::Address,
            allow_integration: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([219, 155, 113, 112], (int_address, allow_integration))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setMasterFCM` (0x95858f98) function"]
        pub fn set_master_fcm(
            &self,
            master_fcm: ethers::core::types::Address,
            yield_bearing_protocol_id: u8,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [149, 133, 143, 152],
                    (master_fcm, yield_bearing_protocol_id),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setMasterMarginEngine` (0x973cd931) function"]
        pub fn set_master_margin_engine(
            &self,
            master_margin_engine: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([151, 60, 217, 49], master_margin_engine)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setMasterVAMM` (0x94a9b1f7) function"]
        pub fn set_master_vamm(
            &self,
            master_vamm: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([148, 169, 177, 247], master_vamm)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPeriphery` (0xaeb22934) function"]
        pub fn set_periphery(
            &self,
            periphery: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([174, 178, 41, 52], periphery)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `IrsInstance` event"]
        pub fn irs_instance_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, IrsInstanceFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `MasterFCM` event"]
        pub fn master_fcm_filter(&self) -> ethers::contract::builders::Event<M, MasterFCMFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PeripheryUpdate` event"]
        pub fn periphery_update_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PeripheryUpdateFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, IFactoryEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IFactory<M> {
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
    pub enum IFactoryErrors {
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
    impl ethers::core::abi::AbiDecode for IFactoryErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok (decoded) = < AavePoolGetReserveNormalizedIncomeReturnedZero as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (IFactoryErrors :: AavePoolGetReserveNormalizedIncomeReturnedZero (decoded)) }
            if let Ok (decoded) = < AavePoolGetReserveNormalizedVariableDebtReturnedZero as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (IFactoryErrors :: AavePoolGetReserveNormalizedVariableDebtReturnedZero (decoded)) }
            if let Ok(decoded) =
                <CTokenExchangeRateReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IFactoryErrors::CTokenExchangeRateReturnedZero(decoded));
            }
            if let Ok(decoded) =
                <CanOnlyTradeIfUnlocked as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFactoryErrors::CanOnlyTradeIfUnlocked(decoded));
            }
            if let Ok(decoded) =
                <CannotLiquidate as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFactoryErrors::CannotLiquidate(decoded));
            }
            if let Ok(decoded) =
                <CannotSettleBeforeMaturity as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFactoryErrors::CannotSettleBeforeMaturity(decoded));
            }
            if let Ok(decoded) = <DebugError as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFactoryErrors::DebugError(decoded));
            }
            if let Ok(decoded) =
                <ExpectedOppositeSigns as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFactoryErrors::ExpectedOppositeSigns(decoded));
            }
            if let Ok(decoded) =
                <ExpectedSqrtPriceZeroBeforeInit as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IFactoryErrors::ExpectedSqrtPriceZeroBeforeInit(decoded));
            }
            if let Ok(decoded) =
                <IRSNotionalAmountSpecifiedMustBeNonZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IFactoryErrors::IRSNotionalAmountSpecifiedMustBeNonZero(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <InvalidMarginDelta as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFactoryErrors::InvalidMarginDelta(decoded));
            }
            if let Ok(decoded) =
                <LidoGetPooledEthBySharesReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IFactoryErrors::LidoGetPooledEthBySharesReturnedZero(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <LiquidityDeltaMustBePositiveInBurn as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IFactoryErrors::LiquidityDeltaMustBePositiveInBurn(decoded));
            }
            if let Ok(decoded) =
                <LiquidityDeltaMustBePositiveInMint as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IFactoryErrors::LiquidityDeltaMustBePositiveInMint(decoded));
            }
            if let Ok(decoded) =
                <MarginLessThanMinimum as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFactoryErrors::MarginLessThanMinimum(decoded));
            }
            if let Ok(decoded) =
                <MarginRequirementNotMet as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFactoryErrors::MarginRequirementNotMet(decoded));
            }
            if let Ok(decoded) =
                <MarginRequirementNotMetFCM as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFactoryErrors::MarginRequirementNotMetFCM(decoded));
            }
            if let Ok(decoded) =
                <NotEnoughFunds as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFactoryErrors::NotEnoughFunds(decoded));
            }
            if let Ok(decoded) = <OOO as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IFactoryErrors::OOO(decoded));
            }
            if let Ok(decoded) = <OnlyFCM as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IFactoryErrors::OnlyFCM(decoded));
            }
            if let Ok(decoded) =
                <OnlyMarginEngine as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFactoryErrors::OnlyMarginEngine(decoded));
            }
            if let Ok(decoded) =
                <OnlyOwnerCanUpdatePosition as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFactoryErrors::OnlyOwnerCanUpdatePosition(decoded));
            }
            if let Ok(decoded) = <OnlyVAMM as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IFactoryErrors::OnlyVAMM(decoded));
            }
            if let Ok(decoded) =
                <PositionNetZero as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFactoryErrors::PositionNetZero(decoded));
            }
            if let Ok(decoded) =
                <PositionNotSettled as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFactoryErrors::PositionNotSettled(decoded));
            }
            if let Ok(decoded) =
                <RocketPoolGetEthValueReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IFactoryErrors::RocketPoolGetEthValueReturnedZero(decoded));
            }
            if let Ok(decoded) =
                <WithdrawalExceedsCurrentMargin as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IFactoryErrors::WithdrawalExceedsCurrentMargin(decoded));
            }
            if let Ok(decoded) =
                <closeToOrBeyondMaturity as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFactoryErrors::closeToOrBeyondMaturity(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IFactoryErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                IFactoryErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(element) => {
                    element.encode()
                }
                IFactoryErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(element) => {
                    element.encode()
                }
                IFactoryErrors::CTokenExchangeRateReturnedZero(element) => element.encode(),
                IFactoryErrors::CanOnlyTradeIfUnlocked(element) => element.encode(),
                IFactoryErrors::CannotLiquidate(element) => element.encode(),
                IFactoryErrors::CannotSettleBeforeMaturity(element) => element.encode(),
                IFactoryErrors::DebugError(element) => element.encode(),
                IFactoryErrors::ExpectedOppositeSigns(element) => element.encode(),
                IFactoryErrors::ExpectedSqrtPriceZeroBeforeInit(element) => element.encode(),
                IFactoryErrors::IRSNotionalAmountSpecifiedMustBeNonZero(element) => {
                    element.encode()
                }
                IFactoryErrors::InvalidMarginDelta(element) => element.encode(),
                IFactoryErrors::LidoGetPooledEthBySharesReturnedZero(element) => element.encode(),
                IFactoryErrors::LiquidityDeltaMustBePositiveInBurn(element) => element.encode(),
                IFactoryErrors::LiquidityDeltaMustBePositiveInMint(element) => element.encode(),
                IFactoryErrors::MarginLessThanMinimum(element) => element.encode(),
                IFactoryErrors::MarginRequirementNotMet(element) => element.encode(),
                IFactoryErrors::MarginRequirementNotMetFCM(element) => element.encode(),
                IFactoryErrors::NotEnoughFunds(element) => element.encode(),
                IFactoryErrors::OOO(element) => element.encode(),
                IFactoryErrors::OnlyFCM(element) => element.encode(),
                IFactoryErrors::OnlyMarginEngine(element) => element.encode(),
                IFactoryErrors::OnlyOwnerCanUpdatePosition(element) => element.encode(),
                IFactoryErrors::OnlyVAMM(element) => element.encode(),
                IFactoryErrors::PositionNetZero(element) => element.encode(),
                IFactoryErrors::PositionNotSettled(element) => element.encode(),
                IFactoryErrors::RocketPoolGetEthValueReturnedZero(element) => element.encode(),
                IFactoryErrors::WithdrawalExceedsCurrentMargin(element) => element.encode(),
                IFactoryErrors::closeToOrBeyondMaturity(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IFactoryErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IFactoryErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(element) => {
                    element.fmt(f)
                }
                IFactoryErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(element) => {
                    element.fmt(f)
                }
                IFactoryErrors::CTokenExchangeRateReturnedZero(element) => element.fmt(f),
                IFactoryErrors::CanOnlyTradeIfUnlocked(element) => element.fmt(f),
                IFactoryErrors::CannotLiquidate(element) => element.fmt(f),
                IFactoryErrors::CannotSettleBeforeMaturity(element) => element.fmt(f),
                IFactoryErrors::DebugError(element) => element.fmt(f),
                IFactoryErrors::ExpectedOppositeSigns(element) => element.fmt(f),
                IFactoryErrors::ExpectedSqrtPriceZeroBeforeInit(element) => element.fmt(f),
                IFactoryErrors::IRSNotionalAmountSpecifiedMustBeNonZero(element) => element.fmt(f),
                IFactoryErrors::InvalidMarginDelta(element) => element.fmt(f),
                IFactoryErrors::LidoGetPooledEthBySharesReturnedZero(element) => element.fmt(f),
                IFactoryErrors::LiquidityDeltaMustBePositiveInBurn(element) => element.fmt(f),
                IFactoryErrors::LiquidityDeltaMustBePositiveInMint(element) => element.fmt(f),
                IFactoryErrors::MarginLessThanMinimum(element) => element.fmt(f),
                IFactoryErrors::MarginRequirementNotMet(element) => element.fmt(f),
                IFactoryErrors::MarginRequirementNotMetFCM(element) => element.fmt(f),
                IFactoryErrors::NotEnoughFunds(element) => element.fmt(f),
                IFactoryErrors::OOO(element) => element.fmt(f),
                IFactoryErrors::OnlyFCM(element) => element.fmt(f),
                IFactoryErrors::OnlyMarginEngine(element) => element.fmt(f),
                IFactoryErrors::OnlyOwnerCanUpdatePosition(element) => element.fmt(f),
                IFactoryErrors::OnlyVAMM(element) => element.fmt(f),
                IFactoryErrors::PositionNetZero(element) => element.fmt(f),
                IFactoryErrors::PositionNotSettled(element) => element.fmt(f),
                IFactoryErrors::RocketPoolGetEthValueReturnedZero(element) => element.fmt(f),
                IFactoryErrors::WithdrawalExceedsCurrentMargin(element) => element.fmt(f),
                IFactoryErrors::closeToOrBeyondMaturity(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AavePoolGetReserveNormalizedIncomeReturnedZero> for IFactoryErrors {
        fn from(var: AavePoolGetReserveNormalizedIncomeReturnedZero) -> Self {
            IFactoryErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(var)
        }
    }
    impl ::std::convert::From<AavePoolGetReserveNormalizedVariableDebtReturnedZero> for IFactoryErrors {
        fn from(var: AavePoolGetReserveNormalizedVariableDebtReturnedZero) -> Self {
            IFactoryErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(var)
        }
    }
    impl ::std::convert::From<CTokenExchangeRateReturnedZero> for IFactoryErrors {
        fn from(var: CTokenExchangeRateReturnedZero) -> Self {
            IFactoryErrors::CTokenExchangeRateReturnedZero(var)
        }
    }
    impl ::std::convert::From<CanOnlyTradeIfUnlocked> for IFactoryErrors {
        fn from(var: CanOnlyTradeIfUnlocked) -> Self {
            IFactoryErrors::CanOnlyTradeIfUnlocked(var)
        }
    }
    impl ::std::convert::From<CannotLiquidate> for IFactoryErrors {
        fn from(var: CannotLiquidate) -> Self {
            IFactoryErrors::CannotLiquidate(var)
        }
    }
    impl ::std::convert::From<CannotSettleBeforeMaturity> for IFactoryErrors {
        fn from(var: CannotSettleBeforeMaturity) -> Self {
            IFactoryErrors::CannotSettleBeforeMaturity(var)
        }
    }
    impl ::std::convert::From<DebugError> for IFactoryErrors {
        fn from(var: DebugError) -> Self {
            IFactoryErrors::DebugError(var)
        }
    }
    impl ::std::convert::From<ExpectedOppositeSigns> for IFactoryErrors {
        fn from(var: ExpectedOppositeSigns) -> Self {
            IFactoryErrors::ExpectedOppositeSigns(var)
        }
    }
    impl ::std::convert::From<ExpectedSqrtPriceZeroBeforeInit> for IFactoryErrors {
        fn from(var: ExpectedSqrtPriceZeroBeforeInit) -> Self {
            IFactoryErrors::ExpectedSqrtPriceZeroBeforeInit(var)
        }
    }
    impl ::std::convert::From<IRSNotionalAmountSpecifiedMustBeNonZero> for IFactoryErrors {
        fn from(var: IRSNotionalAmountSpecifiedMustBeNonZero) -> Self {
            IFactoryErrors::IRSNotionalAmountSpecifiedMustBeNonZero(var)
        }
    }
    impl ::std::convert::From<InvalidMarginDelta> for IFactoryErrors {
        fn from(var: InvalidMarginDelta) -> Self {
            IFactoryErrors::InvalidMarginDelta(var)
        }
    }
    impl ::std::convert::From<LidoGetPooledEthBySharesReturnedZero> for IFactoryErrors {
        fn from(var: LidoGetPooledEthBySharesReturnedZero) -> Self {
            IFactoryErrors::LidoGetPooledEthBySharesReturnedZero(var)
        }
    }
    impl ::std::convert::From<LiquidityDeltaMustBePositiveInBurn> for IFactoryErrors {
        fn from(var: LiquidityDeltaMustBePositiveInBurn) -> Self {
            IFactoryErrors::LiquidityDeltaMustBePositiveInBurn(var)
        }
    }
    impl ::std::convert::From<LiquidityDeltaMustBePositiveInMint> for IFactoryErrors {
        fn from(var: LiquidityDeltaMustBePositiveInMint) -> Self {
            IFactoryErrors::LiquidityDeltaMustBePositiveInMint(var)
        }
    }
    impl ::std::convert::From<MarginLessThanMinimum> for IFactoryErrors {
        fn from(var: MarginLessThanMinimum) -> Self {
            IFactoryErrors::MarginLessThanMinimum(var)
        }
    }
    impl ::std::convert::From<MarginRequirementNotMet> for IFactoryErrors {
        fn from(var: MarginRequirementNotMet) -> Self {
            IFactoryErrors::MarginRequirementNotMet(var)
        }
    }
    impl ::std::convert::From<MarginRequirementNotMetFCM> for IFactoryErrors {
        fn from(var: MarginRequirementNotMetFCM) -> Self {
            IFactoryErrors::MarginRequirementNotMetFCM(var)
        }
    }
    impl ::std::convert::From<NotEnoughFunds> for IFactoryErrors {
        fn from(var: NotEnoughFunds) -> Self {
            IFactoryErrors::NotEnoughFunds(var)
        }
    }
    impl ::std::convert::From<OOO> for IFactoryErrors {
        fn from(var: OOO) -> Self {
            IFactoryErrors::OOO(var)
        }
    }
    impl ::std::convert::From<OnlyFCM> for IFactoryErrors {
        fn from(var: OnlyFCM) -> Self {
            IFactoryErrors::OnlyFCM(var)
        }
    }
    impl ::std::convert::From<OnlyMarginEngine> for IFactoryErrors {
        fn from(var: OnlyMarginEngine) -> Self {
            IFactoryErrors::OnlyMarginEngine(var)
        }
    }
    impl ::std::convert::From<OnlyOwnerCanUpdatePosition> for IFactoryErrors {
        fn from(var: OnlyOwnerCanUpdatePosition) -> Self {
            IFactoryErrors::OnlyOwnerCanUpdatePosition(var)
        }
    }
    impl ::std::convert::From<OnlyVAMM> for IFactoryErrors {
        fn from(var: OnlyVAMM) -> Self {
            IFactoryErrors::OnlyVAMM(var)
        }
    }
    impl ::std::convert::From<PositionNetZero> for IFactoryErrors {
        fn from(var: PositionNetZero) -> Self {
            IFactoryErrors::PositionNetZero(var)
        }
    }
    impl ::std::convert::From<PositionNotSettled> for IFactoryErrors {
        fn from(var: PositionNotSettled) -> Self {
            IFactoryErrors::PositionNotSettled(var)
        }
    }
    impl ::std::convert::From<RocketPoolGetEthValueReturnedZero> for IFactoryErrors {
        fn from(var: RocketPoolGetEthValueReturnedZero) -> Self {
            IFactoryErrors::RocketPoolGetEthValueReturnedZero(var)
        }
    }
    impl ::std::convert::From<WithdrawalExceedsCurrentMargin> for IFactoryErrors {
        fn from(var: WithdrawalExceedsCurrentMargin) -> Self {
            IFactoryErrors::WithdrawalExceedsCurrentMargin(var)
        }
    }
    impl ::std::convert::From<closeToOrBeyondMaturity> for IFactoryErrors {
        fn from(var: closeToOrBeyondMaturity) -> Self {
            IFactoryErrors::closeToOrBeyondMaturity(var)
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,bool)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub int_address: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub is_approved: bool,
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
        name = "IrsInstance",
        abi = "IrsInstance(address,address,uint256,uint256,int24,address,address,address,uint8,uint8)"
    )]
    pub struct IrsInstanceFilter {
        #[ethevent(indexed)]
        pub underlying_token: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub rate_oracle: ethers::core::types::Address,
        pub term_start_timestamp_wad: ethers::core::types::U256,
        pub term_end_timestamp_wad: ethers::core::types::U256,
        pub tick_spacing: i32,
        pub margin_engine: ethers::core::types::Address,
        pub vamm: ethers::core::types::Address,
        pub fcm: ethers::core::types::Address,
        pub yield_bearing_protocol_id: u8,
        pub underlying_token_decimals: u8,
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
    #[ethevent(name = "MasterFCM", abi = "MasterFCM(address,uint8)")]
    pub struct MasterFCMFilter {
        pub master_fcm_address: ethers::core::types::Address,
        pub yield_bearing_protocol_id: u8,
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
    #[ethevent(name = "PeripheryUpdate", abi = "PeripheryUpdate(address)")]
    pub struct PeripheryUpdateFilter {
        pub periphery: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IFactoryEvents {
        ApprovalFilter(ApprovalFilter),
        IrsInstanceFilter(IrsInstanceFilter),
        MasterFCMFilter(MasterFCMFilter),
        PeripheryUpdateFilter(PeripheryUpdateFilter),
    }
    impl ethers::contract::EthLogDecode for IFactoryEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(IFactoryEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = IrsInstanceFilter::decode_log(log) {
                return Ok(IFactoryEvents::IrsInstanceFilter(decoded));
            }
            if let Ok(decoded) = MasterFCMFilter::decode_log(log) {
                return Ok(IFactoryEvents::MasterFCMFilter(decoded));
            }
            if let Ok(decoded) = PeripheryUpdateFilter::decode_log(log) {
                return Ok(IFactoryEvents::PeripheryUpdateFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IFactoryEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IFactoryEvents::ApprovalFilter(element) => element.fmt(f),
                IFactoryEvents::IrsInstanceFilter(element) => element.fmt(f),
                IFactoryEvents::MasterFCMFilter(element) => element.fmt(f),
                IFactoryEvents::PeripheryUpdateFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `deployIrsInstance` function with signature `deployIrsInstance(address,address,uint256,uint256,int24)` and selector `[14, 138, 6, 72]`"]
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
        name = "deployIrsInstance",
        abi = "deployIrsInstance(address,address,uint256,uint256,int24)"
    )]
    pub struct DeployIrsInstanceCall {
        pub underlying_token: ethers::core::types::Address,
        pub rate_oracle: ethers::core::types::Address,
        pub term_start_timestamp_wad: ethers::core::types::U256,
        pub term_end_timestamp_wad: ethers::core::types::U256,
        pub tick_spacing: i32,
    }
    #[doc = "Container type for all input parameters for the `isApproved` function with signature `isApproved(address,address)` and selector `[163, 137, 120, 62]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "isApproved", abi = "isApproved(address,address)")]
    pub struct IsApprovedCall {
        pub owner: ethers::core::types::Address,
        pub int_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `masterFCMs` function with signature `masterFCMs(uint8)` and selector `[202, 81, 131, 183]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "masterFCMs", abi = "masterFCMs(uint8)")]
    pub struct MasterFCMsCall {
        pub yield_bearing_protocol_id: u8,
    }
    #[doc = "Container type for all input parameters for the `masterMarginEngine` function with signature `masterMarginEngine()` and selector `[159, 225, 179, 84]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "masterMarginEngine", abi = "masterMarginEngine()")]
    pub struct MasterMarginEngineCall;
    #[doc = "Container type for all input parameters for the `masterVAMM` function with signature `masterVAMM()` and selector `[254, 191, 231, 94]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "masterVAMM", abi = "masterVAMM()")]
    pub struct MasterVAMMCall;
    #[doc = "Container type for all input parameters for the `periphery` function with signature `periphery()` and selector `[119, 170, 206, 26]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "periphery", abi = "periphery()")]
    pub struct PeripheryCall;
    #[doc = "Container type for all input parameters for the `setApproval` function with signature `setApproval(address,bool)` and selector `[219, 155, 113, 112]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setApproval", abi = "setApproval(address,bool)")]
    pub struct SetApprovalCall {
        pub int_address: ethers::core::types::Address,
        pub allow_integration: bool,
    }
    #[doc = "Container type for all input parameters for the `setMasterFCM` function with signature `setMasterFCM(address,uint8)` and selector `[149, 133, 143, 152]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setMasterFCM", abi = "setMasterFCM(address,uint8)")]
    pub struct SetMasterFCMCall {
        pub master_fcm: ethers::core::types::Address,
        pub yield_bearing_protocol_id: u8,
    }
    #[doc = "Container type for all input parameters for the `setMasterMarginEngine` function with signature `setMasterMarginEngine(address)` and selector `[151, 60, 217, 49]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setMasterMarginEngine", abi = "setMasterMarginEngine(address)")]
    pub struct SetMasterMarginEngineCall {
        pub master_margin_engine: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setMasterVAMM` function with signature `setMasterVAMM(address)` and selector `[148, 169, 177, 247]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setMasterVAMM", abi = "setMasterVAMM(address)")]
    pub struct SetMasterVAMMCall {
        pub master_vamm: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setPeriphery` function with signature `setPeriphery(address)` and selector `[174, 178, 41, 52]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setPeriphery", abi = "setPeriphery(address)")]
    pub struct SetPeripheryCall {
        pub periphery: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IFactoryCalls {
        DeployIrsInstance(DeployIrsInstanceCall),
        IsApproved(IsApprovedCall),
        MasterFCMs(MasterFCMsCall),
        MasterMarginEngine(MasterMarginEngineCall),
        MasterVAMM(MasterVAMMCall),
        Periphery(PeripheryCall),
        SetApproval(SetApprovalCall),
        SetMasterFCM(SetMasterFCMCall),
        SetMasterMarginEngine(SetMasterMarginEngineCall),
        SetMasterVAMM(SetMasterVAMMCall),
        SetPeriphery(SetPeripheryCall),
    }
    impl ethers::core::abi::AbiDecode for IFactoryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DeployIrsInstanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFactoryCalls::DeployIrsInstance(decoded));
            }
            if let Ok(decoded) =
                <IsApprovedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFactoryCalls::IsApproved(decoded));
            }
            if let Ok(decoded) =
                <MasterFCMsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFactoryCalls::MasterFCMs(decoded));
            }
            if let Ok(decoded) =
                <MasterMarginEngineCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFactoryCalls::MasterMarginEngine(decoded));
            }
            if let Ok(decoded) =
                <MasterVAMMCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFactoryCalls::MasterVAMM(decoded));
            }
            if let Ok(decoded) =
                <PeripheryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFactoryCalls::Periphery(decoded));
            }
            if let Ok(decoded) =
                <SetApprovalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFactoryCalls::SetApproval(decoded));
            }
            if let Ok(decoded) =
                <SetMasterFCMCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFactoryCalls::SetMasterFCM(decoded));
            }
            if let Ok(decoded) =
                <SetMasterMarginEngineCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFactoryCalls::SetMasterMarginEngine(decoded));
            }
            if let Ok(decoded) =
                <SetMasterVAMMCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFactoryCalls::SetMasterVAMM(decoded));
            }
            if let Ok(decoded) =
                <SetPeripheryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IFactoryCalls::SetPeriphery(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IFactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IFactoryCalls::DeployIrsInstance(element) => element.encode(),
                IFactoryCalls::IsApproved(element) => element.encode(),
                IFactoryCalls::MasterFCMs(element) => element.encode(),
                IFactoryCalls::MasterMarginEngine(element) => element.encode(),
                IFactoryCalls::MasterVAMM(element) => element.encode(),
                IFactoryCalls::Periphery(element) => element.encode(),
                IFactoryCalls::SetApproval(element) => element.encode(),
                IFactoryCalls::SetMasterFCM(element) => element.encode(),
                IFactoryCalls::SetMasterMarginEngine(element) => element.encode(),
                IFactoryCalls::SetMasterVAMM(element) => element.encode(),
                IFactoryCalls::SetPeriphery(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IFactoryCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IFactoryCalls::DeployIrsInstance(element) => element.fmt(f),
                IFactoryCalls::IsApproved(element) => element.fmt(f),
                IFactoryCalls::MasterFCMs(element) => element.fmt(f),
                IFactoryCalls::MasterMarginEngine(element) => element.fmt(f),
                IFactoryCalls::MasterVAMM(element) => element.fmt(f),
                IFactoryCalls::Periphery(element) => element.fmt(f),
                IFactoryCalls::SetApproval(element) => element.fmt(f),
                IFactoryCalls::SetMasterFCM(element) => element.fmt(f),
                IFactoryCalls::SetMasterMarginEngine(element) => element.fmt(f),
                IFactoryCalls::SetMasterVAMM(element) => element.fmt(f),
                IFactoryCalls::SetPeriphery(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DeployIrsInstanceCall> for IFactoryCalls {
        fn from(var: DeployIrsInstanceCall) -> Self {
            IFactoryCalls::DeployIrsInstance(var)
        }
    }
    impl ::std::convert::From<IsApprovedCall> for IFactoryCalls {
        fn from(var: IsApprovedCall) -> Self {
            IFactoryCalls::IsApproved(var)
        }
    }
    impl ::std::convert::From<MasterFCMsCall> for IFactoryCalls {
        fn from(var: MasterFCMsCall) -> Self {
            IFactoryCalls::MasterFCMs(var)
        }
    }
    impl ::std::convert::From<MasterMarginEngineCall> for IFactoryCalls {
        fn from(var: MasterMarginEngineCall) -> Self {
            IFactoryCalls::MasterMarginEngine(var)
        }
    }
    impl ::std::convert::From<MasterVAMMCall> for IFactoryCalls {
        fn from(var: MasterVAMMCall) -> Self {
            IFactoryCalls::MasterVAMM(var)
        }
    }
    impl ::std::convert::From<PeripheryCall> for IFactoryCalls {
        fn from(var: PeripheryCall) -> Self {
            IFactoryCalls::Periphery(var)
        }
    }
    impl ::std::convert::From<SetApprovalCall> for IFactoryCalls {
        fn from(var: SetApprovalCall) -> Self {
            IFactoryCalls::SetApproval(var)
        }
    }
    impl ::std::convert::From<SetMasterFCMCall> for IFactoryCalls {
        fn from(var: SetMasterFCMCall) -> Self {
            IFactoryCalls::SetMasterFCM(var)
        }
    }
    impl ::std::convert::From<SetMasterMarginEngineCall> for IFactoryCalls {
        fn from(var: SetMasterMarginEngineCall) -> Self {
            IFactoryCalls::SetMasterMarginEngine(var)
        }
    }
    impl ::std::convert::From<SetMasterVAMMCall> for IFactoryCalls {
        fn from(var: SetMasterVAMMCall) -> Self {
            IFactoryCalls::SetMasterVAMM(var)
        }
    }
    impl ::std::convert::From<SetPeripheryCall> for IFactoryCalls {
        fn from(var: SetPeripheryCall) -> Self {
            IFactoryCalls::SetPeriphery(var)
        }
    }
    #[doc = "Container type for all return fields from the `deployIrsInstance` function with signature `deployIrsInstance(address,address,uint256,uint256,int24)` and selector `[14, 138, 6, 72]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DeployIrsInstanceReturn {
        pub margin_engine_proxy: ethers::core::types::Address,
        pub vamm_proxy: ethers::core::types::Address,
        pub fcm_proxy: ethers::core::types::Address,
    }
    #[doc = "Container type for all return fields from the `isApproved` function with signature `isApproved(address,address)` and selector `[163, 137, 120, 62]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsApprovedReturn(pub bool);
    #[doc = "Container type for all return fields from the `masterFCMs` function with signature `masterFCMs(uint8)` and selector `[202, 81, 131, 183]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MasterFCMsReturn {
        pub master_fcm: ethers::core::types::Address,
    }
    #[doc = "Container type for all return fields from the `masterMarginEngine` function with signature `masterMarginEngine()` and selector `[159, 225, 179, 84]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MasterMarginEngineReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `masterVAMM` function with signature `masterVAMM()` and selector `[254, 191, 231, 94]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MasterVAMMReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `periphery` function with signature `periphery()` and selector `[119, 170, 206, 26]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PeripheryReturn(pub ethers::core::types::Address);
}
