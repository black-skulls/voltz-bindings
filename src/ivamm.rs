pub use ivamm::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod ivamm {
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
    #[doc = "IVAMM was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IVAMM_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"type\":\"error\",\"name\":\"AavePoolGetReserveNormalizedIncomeReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"AavePoolGetReserveNormalizedVariableDebtReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CTokenExchangeRateReturnedZero\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"unlocked\",\"type\":\"bool\",\"components\":[]}],\"type\":\"error\",\"name\":\"CanOnlyTradeIfUnlocked\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CannotLiquidate\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CannotSettleBeforeMaturity\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"DebugError\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"amount0\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"amount1\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"ExpectedOppositeSigns\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint160\",\"name\":\"sqrtPriceX96\",\"type\":\"uint160\",\"components\":[]}],\"type\":\"error\",\"name\":\"ExpectedSqrtPriceZeroBeforeInit\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"IRSNotionalAmountSpecifiedMustBeNonZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidMarginDelta\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"LidoGetPooledEthBySharesReturnedZero\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"type\":\"error\",\"name\":\"LiquidityDeltaMustBePositiveInBurn\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"type\":\"error\",\"name\":\"LiquidityDeltaMustBePositiveInMint\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginLessThanMinimum\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tick\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"variableTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"cumulativeFeeIncurred\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenDeltaUnbalanced\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginRequirementNotMet\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginRequirementNotMetFCM\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"requested\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"available\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"NotEnoughFunds\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OOO\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyFCM\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyMarginEngine\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyOwnerCanUpdatePosition\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyVAMM\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PositionNetZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PositionNotSettled\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"RocketPoolGetEthValueReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"WithdrawalExceedsCurrentMargin\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"closeToOrBeyondMaturity\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[],\"indexed\":true},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Burn\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"feeWad\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Fee\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"feeProtocol\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FeeProtocol\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"__isAlpha\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"IsAlpha\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[],\"indexed\":true},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Mint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[],\"indexed\":true},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[],\"indexed\":true},{\"internalType\":\"int256\",\"name\":\"desiredNotional\",\"type\":\"int256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceLimitX96\",\"type\":\"uint160\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"cumulativeFeeIncurred\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"fixedTokenDelta\",\"type\":\"int256\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"variableTokenDelta\",\"type\":\"int256\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"fixedTokenDeltaUnbalanced\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Swap\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint160\",\"name\":\"sqrtPriceX96\",\"type\":\"uint160\",\"components\":[],\"indexed\":false},{\"internalType\":\"int24\",\"name\":\"tick\",\"type\":\"int24\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"VAMMInitialization\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"int24\",\"name\":\"tick\",\"type\":\"int24\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"VAMMPriceChange\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burn\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"positionMarginRequirement\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"computeGrowthInside\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"fixedTokenGrowthInsideX128\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"variableTokenGrowthInsideX128\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"feeGrowthInsideX128\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"factory\",\"outputs\":[{\"internalType\":\"contract IFactory\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"feeGrowthGlobalX128\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"feeWad\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"fixedTokenGrowthGlobalX128\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRateOracle\",\"outputs\":[{\"internalType\":\"contract IRateOracle\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract IMarginEngine\",\"name\":\"__marginEngine\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"__tickSpacing\",\"type\":\"int24\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint160\",\"name\":\"sqrtPriceX96\",\"type\":\"uint160\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initializeVAMM\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isAlpha\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"liquidity\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"\",\"type\":\"uint128\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"marginEngine\",\"outputs\":[{\"internalType\":\"contract IMarginEngine\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"maxLiquidityPerTick\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"\",\"type\":\"uint128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mint\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"positionMarginRequirement\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"protocolFees\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"refreshRateOracle\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_fee\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setFee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"feeProtocol\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setFeeProtocol\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"__isAlpha\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setIsAlpha\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"state\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPausability\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IVAMM.SwapParams\",\"name\":\"params\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"amountSpecified\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceLimitX96\",\"type\":\"uint160\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"swap\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"fixedTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"variableTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"cumulativeFeeIncurred\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenDeltaUnbalanced\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"int16\",\"name\":\"wordPosition\",\"type\":\"int16\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"tickBitmap\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"tickSpacing\",\"outputs\":[{\"internalType\":\"int24\",\"name\":\"\",\"type\":\"int24\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"int24\",\"name\":\"tick\",\"type\":\"int24\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ticks\",\"outputs\":[{\"internalType\":\"struct Tick.Info\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint128\",\"name\":\"liquidityGross\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"liquidityNet\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenGrowthOutsideX128\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"variableTokenGrowthOutsideX128\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"feeGrowthOutsideX128\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"initialized\",\"type\":\"bool\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"protocolFeesCollected\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateProtocolFees\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"vammVars\",\"outputs\":[{\"internalType\":\"struct IVAMM.VAMMVars\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint160\",\"name\":\"sqrtPriceX96\",\"type\":\"uint160\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tick\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"feeProtocol\",\"type\":\"uint8\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"variableTokenGrowthGlobalX128\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct IVAMM<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IVAMM<M> {
        fn clone(&self) -> Self {
            IVAMM(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IVAMM<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IVAMM<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IVAMM))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IVAMM<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IVAMM_ABI.clone(), client).into()
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
        #[doc = "Calls the contract's `protocolFees` (0x1ad8b03b) function"]
        pub fn protocol_fees(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([26, 216, 176, 59], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `refreshRateOracle` (0x0bd9c1af) function"]
        pub fn refresh_rate_oracle(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([11, 217, 193, 175], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setFee` (0x69fe0e2d) function"]
        pub fn set_fee(
            &self,
            fee: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([105, 254, 14, 45], fee)
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
        #[doc = "Calls the contract's `updateProtocolFees` (0x86737710) function"]
        pub fn update_protocol_fees(
            &self,
            protocol_fees_collected: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([134, 115, 119, 16], protocol_fees_collected)
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
        #[doc = "Gets the contract's `IsAlpha` event"]
        pub fn is_alpha_filter(&self) -> ethers::contract::builders::Event<M, IsAlphaFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Mint` event"]
        pub fn mint_filter(&self) -> ethers::contract::builders::Event<M, MintFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Swap` event"]
        pub fn swap_filter(&self) -> ethers::contract::builders::Event<M, SwapFilter> {
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, IVAMMEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IVAMM<M> {
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
    pub enum IVAMMErrors {
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
    impl ethers::core::abi::AbiDecode for IVAMMErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok (decoded) = < AavePoolGetReserveNormalizedIncomeReturnedZero as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (IVAMMErrors :: AavePoolGetReserveNormalizedIncomeReturnedZero (decoded)) }
            if let Ok (decoded) = < AavePoolGetReserveNormalizedVariableDebtReturnedZero as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (IVAMMErrors :: AavePoolGetReserveNormalizedVariableDebtReturnedZero (decoded)) }
            if let Ok(decoded) =
                <CTokenExchangeRateReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IVAMMErrors::CTokenExchangeRateReturnedZero(decoded));
            }
            if let Ok(decoded) =
                <CanOnlyTradeIfUnlocked as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVAMMErrors::CanOnlyTradeIfUnlocked(decoded));
            }
            if let Ok(decoded) =
                <CannotLiquidate as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVAMMErrors::CannotLiquidate(decoded));
            }
            if let Ok(decoded) =
                <CannotSettleBeforeMaturity as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVAMMErrors::CannotSettleBeforeMaturity(decoded));
            }
            if let Ok(decoded) = <DebugError as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVAMMErrors::DebugError(decoded));
            }
            if let Ok(decoded) =
                <ExpectedOppositeSigns as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVAMMErrors::ExpectedOppositeSigns(decoded));
            }
            if let Ok(decoded) =
                <ExpectedSqrtPriceZeroBeforeInit as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IVAMMErrors::ExpectedSqrtPriceZeroBeforeInit(decoded));
            }
            if let Ok(decoded) =
                <IRSNotionalAmountSpecifiedMustBeNonZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IVAMMErrors::IRSNotionalAmountSpecifiedMustBeNonZero(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <InvalidMarginDelta as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVAMMErrors::InvalidMarginDelta(decoded));
            }
            if let Ok(decoded) =
                <LidoGetPooledEthBySharesReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IVAMMErrors::LidoGetPooledEthBySharesReturnedZero(decoded));
            }
            if let Ok(decoded) =
                <LiquidityDeltaMustBePositiveInBurn as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IVAMMErrors::LiquidityDeltaMustBePositiveInBurn(decoded));
            }
            if let Ok(decoded) =
                <LiquidityDeltaMustBePositiveInMint as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IVAMMErrors::LiquidityDeltaMustBePositiveInMint(decoded));
            }
            if let Ok(decoded) =
                <MarginLessThanMinimum as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVAMMErrors::MarginLessThanMinimum(decoded));
            }
            if let Ok(decoded) =
                <MarginRequirementNotMet as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVAMMErrors::MarginRequirementNotMet(decoded));
            }
            if let Ok(decoded) =
                <MarginRequirementNotMetFCM as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVAMMErrors::MarginRequirementNotMetFCM(decoded));
            }
            if let Ok(decoded) =
                <NotEnoughFunds as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVAMMErrors::NotEnoughFunds(decoded));
            }
            if let Ok(decoded) = <OOO as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IVAMMErrors::OOO(decoded));
            }
            if let Ok(decoded) = <OnlyFCM as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IVAMMErrors::OnlyFCM(decoded));
            }
            if let Ok(decoded) =
                <OnlyMarginEngine as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVAMMErrors::OnlyMarginEngine(decoded));
            }
            if let Ok(decoded) =
                <OnlyOwnerCanUpdatePosition as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVAMMErrors::OnlyOwnerCanUpdatePosition(decoded));
            }
            if let Ok(decoded) = <OnlyVAMM as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IVAMMErrors::OnlyVAMM(decoded));
            }
            if let Ok(decoded) =
                <PositionNetZero as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVAMMErrors::PositionNetZero(decoded));
            }
            if let Ok(decoded) =
                <PositionNotSettled as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVAMMErrors::PositionNotSettled(decoded));
            }
            if let Ok(decoded) =
                <RocketPoolGetEthValueReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IVAMMErrors::RocketPoolGetEthValueReturnedZero(decoded));
            }
            if let Ok(decoded) =
                <WithdrawalExceedsCurrentMargin as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IVAMMErrors::WithdrawalExceedsCurrentMargin(decoded));
            }
            if let Ok(decoded) =
                <closeToOrBeyondMaturity as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVAMMErrors::closeToOrBeyondMaturity(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IVAMMErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                IVAMMErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(element) => {
                    element.encode()
                }
                IVAMMErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(element) => {
                    element.encode()
                }
                IVAMMErrors::CTokenExchangeRateReturnedZero(element) => element.encode(),
                IVAMMErrors::CanOnlyTradeIfUnlocked(element) => element.encode(),
                IVAMMErrors::CannotLiquidate(element) => element.encode(),
                IVAMMErrors::CannotSettleBeforeMaturity(element) => element.encode(),
                IVAMMErrors::DebugError(element) => element.encode(),
                IVAMMErrors::ExpectedOppositeSigns(element) => element.encode(),
                IVAMMErrors::ExpectedSqrtPriceZeroBeforeInit(element) => element.encode(),
                IVAMMErrors::IRSNotionalAmountSpecifiedMustBeNonZero(element) => element.encode(),
                IVAMMErrors::InvalidMarginDelta(element) => element.encode(),
                IVAMMErrors::LidoGetPooledEthBySharesReturnedZero(element) => element.encode(),
                IVAMMErrors::LiquidityDeltaMustBePositiveInBurn(element) => element.encode(),
                IVAMMErrors::LiquidityDeltaMustBePositiveInMint(element) => element.encode(),
                IVAMMErrors::MarginLessThanMinimum(element) => element.encode(),
                IVAMMErrors::MarginRequirementNotMet(element) => element.encode(),
                IVAMMErrors::MarginRequirementNotMetFCM(element) => element.encode(),
                IVAMMErrors::NotEnoughFunds(element) => element.encode(),
                IVAMMErrors::OOO(element) => element.encode(),
                IVAMMErrors::OnlyFCM(element) => element.encode(),
                IVAMMErrors::OnlyMarginEngine(element) => element.encode(),
                IVAMMErrors::OnlyOwnerCanUpdatePosition(element) => element.encode(),
                IVAMMErrors::OnlyVAMM(element) => element.encode(),
                IVAMMErrors::PositionNetZero(element) => element.encode(),
                IVAMMErrors::PositionNotSettled(element) => element.encode(),
                IVAMMErrors::RocketPoolGetEthValueReturnedZero(element) => element.encode(),
                IVAMMErrors::WithdrawalExceedsCurrentMargin(element) => element.encode(),
                IVAMMErrors::closeToOrBeyondMaturity(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IVAMMErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IVAMMErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(element) => {
                    element.fmt(f)
                }
                IVAMMErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(element) => {
                    element.fmt(f)
                }
                IVAMMErrors::CTokenExchangeRateReturnedZero(element) => element.fmt(f),
                IVAMMErrors::CanOnlyTradeIfUnlocked(element) => element.fmt(f),
                IVAMMErrors::CannotLiquidate(element) => element.fmt(f),
                IVAMMErrors::CannotSettleBeforeMaturity(element) => element.fmt(f),
                IVAMMErrors::DebugError(element) => element.fmt(f),
                IVAMMErrors::ExpectedOppositeSigns(element) => element.fmt(f),
                IVAMMErrors::ExpectedSqrtPriceZeroBeforeInit(element) => element.fmt(f),
                IVAMMErrors::IRSNotionalAmountSpecifiedMustBeNonZero(element) => element.fmt(f),
                IVAMMErrors::InvalidMarginDelta(element) => element.fmt(f),
                IVAMMErrors::LidoGetPooledEthBySharesReturnedZero(element) => element.fmt(f),
                IVAMMErrors::LiquidityDeltaMustBePositiveInBurn(element) => element.fmt(f),
                IVAMMErrors::LiquidityDeltaMustBePositiveInMint(element) => element.fmt(f),
                IVAMMErrors::MarginLessThanMinimum(element) => element.fmt(f),
                IVAMMErrors::MarginRequirementNotMet(element) => element.fmt(f),
                IVAMMErrors::MarginRequirementNotMetFCM(element) => element.fmt(f),
                IVAMMErrors::NotEnoughFunds(element) => element.fmt(f),
                IVAMMErrors::OOO(element) => element.fmt(f),
                IVAMMErrors::OnlyFCM(element) => element.fmt(f),
                IVAMMErrors::OnlyMarginEngine(element) => element.fmt(f),
                IVAMMErrors::OnlyOwnerCanUpdatePosition(element) => element.fmt(f),
                IVAMMErrors::OnlyVAMM(element) => element.fmt(f),
                IVAMMErrors::PositionNetZero(element) => element.fmt(f),
                IVAMMErrors::PositionNotSettled(element) => element.fmt(f),
                IVAMMErrors::RocketPoolGetEthValueReturnedZero(element) => element.fmt(f),
                IVAMMErrors::WithdrawalExceedsCurrentMargin(element) => element.fmt(f),
                IVAMMErrors::closeToOrBeyondMaturity(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AavePoolGetReserveNormalizedIncomeReturnedZero> for IVAMMErrors {
        fn from(var: AavePoolGetReserveNormalizedIncomeReturnedZero) -> Self {
            IVAMMErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(var)
        }
    }
    impl ::std::convert::From<AavePoolGetReserveNormalizedVariableDebtReturnedZero> for IVAMMErrors {
        fn from(var: AavePoolGetReserveNormalizedVariableDebtReturnedZero) -> Self {
            IVAMMErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(var)
        }
    }
    impl ::std::convert::From<CTokenExchangeRateReturnedZero> for IVAMMErrors {
        fn from(var: CTokenExchangeRateReturnedZero) -> Self {
            IVAMMErrors::CTokenExchangeRateReturnedZero(var)
        }
    }
    impl ::std::convert::From<CanOnlyTradeIfUnlocked> for IVAMMErrors {
        fn from(var: CanOnlyTradeIfUnlocked) -> Self {
            IVAMMErrors::CanOnlyTradeIfUnlocked(var)
        }
    }
    impl ::std::convert::From<CannotLiquidate> for IVAMMErrors {
        fn from(var: CannotLiquidate) -> Self {
            IVAMMErrors::CannotLiquidate(var)
        }
    }
    impl ::std::convert::From<CannotSettleBeforeMaturity> for IVAMMErrors {
        fn from(var: CannotSettleBeforeMaturity) -> Self {
            IVAMMErrors::CannotSettleBeforeMaturity(var)
        }
    }
    impl ::std::convert::From<DebugError> for IVAMMErrors {
        fn from(var: DebugError) -> Self {
            IVAMMErrors::DebugError(var)
        }
    }
    impl ::std::convert::From<ExpectedOppositeSigns> for IVAMMErrors {
        fn from(var: ExpectedOppositeSigns) -> Self {
            IVAMMErrors::ExpectedOppositeSigns(var)
        }
    }
    impl ::std::convert::From<ExpectedSqrtPriceZeroBeforeInit> for IVAMMErrors {
        fn from(var: ExpectedSqrtPriceZeroBeforeInit) -> Self {
            IVAMMErrors::ExpectedSqrtPriceZeroBeforeInit(var)
        }
    }
    impl ::std::convert::From<IRSNotionalAmountSpecifiedMustBeNonZero> for IVAMMErrors {
        fn from(var: IRSNotionalAmountSpecifiedMustBeNonZero) -> Self {
            IVAMMErrors::IRSNotionalAmountSpecifiedMustBeNonZero(var)
        }
    }
    impl ::std::convert::From<InvalidMarginDelta> for IVAMMErrors {
        fn from(var: InvalidMarginDelta) -> Self {
            IVAMMErrors::InvalidMarginDelta(var)
        }
    }
    impl ::std::convert::From<LidoGetPooledEthBySharesReturnedZero> for IVAMMErrors {
        fn from(var: LidoGetPooledEthBySharesReturnedZero) -> Self {
            IVAMMErrors::LidoGetPooledEthBySharesReturnedZero(var)
        }
    }
    impl ::std::convert::From<LiquidityDeltaMustBePositiveInBurn> for IVAMMErrors {
        fn from(var: LiquidityDeltaMustBePositiveInBurn) -> Self {
            IVAMMErrors::LiquidityDeltaMustBePositiveInBurn(var)
        }
    }
    impl ::std::convert::From<LiquidityDeltaMustBePositiveInMint> for IVAMMErrors {
        fn from(var: LiquidityDeltaMustBePositiveInMint) -> Self {
            IVAMMErrors::LiquidityDeltaMustBePositiveInMint(var)
        }
    }
    impl ::std::convert::From<MarginLessThanMinimum> for IVAMMErrors {
        fn from(var: MarginLessThanMinimum) -> Self {
            IVAMMErrors::MarginLessThanMinimum(var)
        }
    }
    impl ::std::convert::From<MarginRequirementNotMet> for IVAMMErrors {
        fn from(var: MarginRequirementNotMet) -> Self {
            IVAMMErrors::MarginRequirementNotMet(var)
        }
    }
    impl ::std::convert::From<MarginRequirementNotMetFCM> for IVAMMErrors {
        fn from(var: MarginRequirementNotMetFCM) -> Self {
            IVAMMErrors::MarginRequirementNotMetFCM(var)
        }
    }
    impl ::std::convert::From<NotEnoughFunds> for IVAMMErrors {
        fn from(var: NotEnoughFunds) -> Self {
            IVAMMErrors::NotEnoughFunds(var)
        }
    }
    impl ::std::convert::From<OOO> for IVAMMErrors {
        fn from(var: OOO) -> Self {
            IVAMMErrors::OOO(var)
        }
    }
    impl ::std::convert::From<OnlyFCM> for IVAMMErrors {
        fn from(var: OnlyFCM) -> Self {
            IVAMMErrors::OnlyFCM(var)
        }
    }
    impl ::std::convert::From<OnlyMarginEngine> for IVAMMErrors {
        fn from(var: OnlyMarginEngine) -> Self {
            IVAMMErrors::OnlyMarginEngine(var)
        }
    }
    impl ::std::convert::From<OnlyOwnerCanUpdatePosition> for IVAMMErrors {
        fn from(var: OnlyOwnerCanUpdatePosition) -> Self {
            IVAMMErrors::OnlyOwnerCanUpdatePosition(var)
        }
    }
    impl ::std::convert::From<OnlyVAMM> for IVAMMErrors {
        fn from(var: OnlyVAMM) -> Self {
            IVAMMErrors::OnlyVAMM(var)
        }
    }
    impl ::std::convert::From<PositionNetZero> for IVAMMErrors {
        fn from(var: PositionNetZero) -> Self {
            IVAMMErrors::PositionNetZero(var)
        }
    }
    impl ::std::convert::From<PositionNotSettled> for IVAMMErrors {
        fn from(var: PositionNotSettled) -> Self {
            IVAMMErrors::PositionNotSettled(var)
        }
    }
    impl ::std::convert::From<RocketPoolGetEthValueReturnedZero> for IVAMMErrors {
        fn from(var: RocketPoolGetEthValueReturnedZero) -> Self {
            IVAMMErrors::RocketPoolGetEthValueReturnedZero(var)
        }
    }
    impl ::std::convert::From<WithdrawalExceedsCurrentMargin> for IVAMMErrors {
        fn from(var: WithdrawalExceedsCurrentMargin) -> Self {
            IVAMMErrors::WithdrawalExceedsCurrentMargin(var)
        }
    }
    impl ::std::convert::From<closeToOrBeyondMaturity> for IVAMMErrors {
        fn from(var: closeToOrBeyondMaturity) -> Self {
            IVAMMErrors::closeToOrBeyondMaturity(var)
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
    pub enum IVAMMEvents {
        BurnFilter(BurnFilter),
        FeeFilter(FeeFilter),
        FeeProtocolFilter(FeeProtocolFilter),
        IsAlphaFilter(IsAlphaFilter),
        MintFilter(MintFilter),
        SwapFilter(SwapFilter),
        VamminitializationFilter(VamminitializationFilter),
        VammpriceChangeFilter(VammpriceChangeFilter),
    }
    impl ethers::contract::EthLogDecode for IVAMMEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = BurnFilter::decode_log(log) {
                return Ok(IVAMMEvents::BurnFilter(decoded));
            }
            if let Ok(decoded) = FeeFilter::decode_log(log) {
                return Ok(IVAMMEvents::FeeFilter(decoded));
            }
            if let Ok(decoded) = FeeProtocolFilter::decode_log(log) {
                return Ok(IVAMMEvents::FeeProtocolFilter(decoded));
            }
            if let Ok(decoded) = IsAlphaFilter::decode_log(log) {
                return Ok(IVAMMEvents::IsAlphaFilter(decoded));
            }
            if let Ok(decoded) = MintFilter::decode_log(log) {
                return Ok(IVAMMEvents::MintFilter(decoded));
            }
            if let Ok(decoded) = SwapFilter::decode_log(log) {
                return Ok(IVAMMEvents::SwapFilter(decoded));
            }
            if let Ok(decoded) = VamminitializationFilter::decode_log(log) {
                return Ok(IVAMMEvents::VamminitializationFilter(decoded));
            }
            if let Ok(decoded) = VammpriceChangeFilter::decode_log(log) {
                return Ok(IVAMMEvents::VammpriceChangeFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IVAMMEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IVAMMEvents::BurnFilter(element) => element.fmt(f),
                IVAMMEvents::FeeFilter(element) => element.fmt(f),
                IVAMMEvents::FeeProtocolFilter(element) => element.fmt(f),
                IVAMMEvents::IsAlphaFilter(element) => element.fmt(f),
                IVAMMEvents::MintFilter(element) => element.fmt(f),
                IVAMMEvents::SwapFilter(element) => element.fmt(f),
                IVAMMEvents::VamminitializationFilter(element) => element.fmt(f),
                IVAMMEvents::VammpriceChangeFilter(element) => element.fmt(f),
            }
        }
    }
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
        pub fee: ethers::core::types::U256,
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
    pub enum IVAMMCalls {
        Burn(BurnCall),
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
        ProtocolFees(ProtocolFeesCall),
        RefreshRateOracle(RefreshRateOracleCall),
        SetFee(SetFeeCall),
        SetFeeProtocol(SetFeeProtocolCall),
        SetIsAlpha(SetIsAlphaCall),
        SetPausability(SetPausabilityCall),
        Swap(SwapCall),
        TickBitmap(TickBitmapCall),
        TickSpacing(TickSpacingCall),
        Ticks(TicksCall),
        UpdateProtocolFees(UpdateProtocolFeesCall),
        VammVars(VammVarsCall),
        VariableTokenGrowthGlobalX128(VariableTokenGrowthGlobalX128Call),
    }
    impl ethers::core::abi::AbiDecode for IVAMMCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <BurnCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IVAMMCalls::Burn(decoded));
            }
            if let Ok(decoded) =
                <ComputeGrowthInsideCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVAMMCalls::ComputeGrowthInside(decoded));
            }
            if let Ok(decoded) =
                <FactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVAMMCalls::Factory(decoded));
            }
            if let Ok(decoded) =
                <FeeGrowthGlobalX128Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVAMMCalls::FeeGrowthGlobalX128(decoded));
            }
            if let Ok(decoded) = <FeeWadCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVAMMCalls::FeeWad(decoded));
            }
            if let Ok(decoded) =
                <FixedTokenGrowthGlobalX128Call as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IVAMMCalls::FixedTokenGrowthGlobalX128(decoded));
            }
            if let Ok(decoded) =
                <GetRateOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVAMMCalls::GetRateOracle(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVAMMCalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <InitializeVAMMCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVAMMCalls::InitializeVAMM(decoded));
            }
            if let Ok(decoded) =
                <IsAlphaCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVAMMCalls::IsAlpha(decoded));
            }
            if let Ok(decoded) =
                <LiquidityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVAMMCalls::Liquidity(decoded));
            }
            if let Ok(decoded) =
                <MarginEngineCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVAMMCalls::MarginEngine(decoded));
            }
            if let Ok(decoded) =
                <MaxLiquidityPerTickCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVAMMCalls::MaxLiquidityPerTick(decoded));
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IVAMMCalls::Mint(decoded));
            }
            if let Ok(decoded) =
                <ProtocolFeesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVAMMCalls::ProtocolFees(decoded));
            }
            if let Ok(decoded) =
                <RefreshRateOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVAMMCalls::RefreshRateOracle(decoded));
            }
            if let Ok(decoded) = <SetFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVAMMCalls::SetFee(decoded));
            }
            if let Ok(decoded) =
                <SetFeeProtocolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVAMMCalls::SetFeeProtocol(decoded));
            }
            if let Ok(decoded) =
                <SetIsAlphaCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVAMMCalls::SetIsAlpha(decoded));
            }
            if let Ok(decoded) =
                <SetPausabilityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVAMMCalls::SetPausability(decoded));
            }
            if let Ok(decoded) = <SwapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IVAMMCalls::Swap(decoded));
            }
            if let Ok(decoded) =
                <TickBitmapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVAMMCalls::TickBitmap(decoded));
            }
            if let Ok(decoded) =
                <TickSpacingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVAMMCalls::TickSpacing(decoded));
            }
            if let Ok(decoded) = <TicksCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVAMMCalls::Ticks(decoded));
            }
            if let Ok(decoded) =
                <UpdateProtocolFeesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVAMMCalls::UpdateProtocolFees(decoded));
            }
            if let Ok(decoded) =
                <VammVarsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVAMMCalls::VammVars(decoded));
            }
            if let Ok(decoded) =
                <VariableTokenGrowthGlobalX128Call as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IVAMMCalls::VariableTokenGrowthGlobalX128(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IVAMMCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IVAMMCalls::Burn(element) => element.encode(),
                IVAMMCalls::ComputeGrowthInside(element) => element.encode(),
                IVAMMCalls::Factory(element) => element.encode(),
                IVAMMCalls::FeeGrowthGlobalX128(element) => element.encode(),
                IVAMMCalls::FeeWad(element) => element.encode(),
                IVAMMCalls::FixedTokenGrowthGlobalX128(element) => element.encode(),
                IVAMMCalls::GetRateOracle(element) => element.encode(),
                IVAMMCalls::Initialize(element) => element.encode(),
                IVAMMCalls::InitializeVAMM(element) => element.encode(),
                IVAMMCalls::IsAlpha(element) => element.encode(),
                IVAMMCalls::Liquidity(element) => element.encode(),
                IVAMMCalls::MarginEngine(element) => element.encode(),
                IVAMMCalls::MaxLiquidityPerTick(element) => element.encode(),
                IVAMMCalls::Mint(element) => element.encode(),
                IVAMMCalls::ProtocolFees(element) => element.encode(),
                IVAMMCalls::RefreshRateOracle(element) => element.encode(),
                IVAMMCalls::SetFee(element) => element.encode(),
                IVAMMCalls::SetFeeProtocol(element) => element.encode(),
                IVAMMCalls::SetIsAlpha(element) => element.encode(),
                IVAMMCalls::SetPausability(element) => element.encode(),
                IVAMMCalls::Swap(element) => element.encode(),
                IVAMMCalls::TickBitmap(element) => element.encode(),
                IVAMMCalls::TickSpacing(element) => element.encode(),
                IVAMMCalls::Ticks(element) => element.encode(),
                IVAMMCalls::UpdateProtocolFees(element) => element.encode(),
                IVAMMCalls::VammVars(element) => element.encode(),
                IVAMMCalls::VariableTokenGrowthGlobalX128(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IVAMMCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IVAMMCalls::Burn(element) => element.fmt(f),
                IVAMMCalls::ComputeGrowthInside(element) => element.fmt(f),
                IVAMMCalls::Factory(element) => element.fmt(f),
                IVAMMCalls::FeeGrowthGlobalX128(element) => element.fmt(f),
                IVAMMCalls::FeeWad(element) => element.fmt(f),
                IVAMMCalls::FixedTokenGrowthGlobalX128(element) => element.fmt(f),
                IVAMMCalls::GetRateOracle(element) => element.fmt(f),
                IVAMMCalls::Initialize(element) => element.fmt(f),
                IVAMMCalls::InitializeVAMM(element) => element.fmt(f),
                IVAMMCalls::IsAlpha(element) => element.fmt(f),
                IVAMMCalls::Liquidity(element) => element.fmt(f),
                IVAMMCalls::MarginEngine(element) => element.fmt(f),
                IVAMMCalls::MaxLiquidityPerTick(element) => element.fmt(f),
                IVAMMCalls::Mint(element) => element.fmt(f),
                IVAMMCalls::ProtocolFees(element) => element.fmt(f),
                IVAMMCalls::RefreshRateOracle(element) => element.fmt(f),
                IVAMMCalls::SetFee(element) => element.fmt(f),
                IVAMMCalls::SetFeeProtocol(element) => element.fmt(f),
                IVAMMCalls::SetIsAlpha(element) => element.fmt(f),
                IVAMMCalls::SetPausability(element) => element.fmt(f),
                IVAMMCalls::Swap(element) => element.fmt(f),
                IVAMMCalls::TickBitmap(element) => element.fmt(f),
                IVAMMCalls::TickSpacing(element) => element.fmt(f),
                IVAMMCalls::Ticks(element) => element.fmt(f),
                IVAMMCalls::UpdateProtocolFees(element) => element.fmt(f),
                IVAMMCalls::VammVars(element) => element.fmt(f),
                IVAMMCalls::VariableTokenGrowthGlobalX128(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BurnCall> for IVAMMCalls {
        fn from(var: BurnCall) -> Self {
            IVAMMCalls::Burn(var)
        }
    }
    impl ::std::convert::From<ComputeGrowthInsideCall> for IVAMMCalls {
        fn from(var: ComputeGrowthInsideCall) -> Self {
            IVAMMCalls::ComputeGrowthInside(var)
        }
    }
    impl ::std::convert::From<FactoryCall> for IVAMMCalls {
        fn from(var: FactoryCall) -> Self {
            IVAMMCalls::Factory(var)
        }
    }
    impl ::std::convert::From<FeeGrowthGlobalX128Call> for IVAMMCalls {
        fn from(var: FeeGrowthGlobalX128Call) -> Self {
            IVAMMCalls::FeeGrowthGlobalX128(var)
        }
    }
    impl ::std::convert::From<FeeWadCall> for IVAMMCalls {
        fn from(var: FeeWadCall) -> Self {
            IVAMMCalls::FeeWad(var)
        }
    }
    impl ::std::convert::From<FixedTokenGrowthGlobalX128Call> for IVAMMCalls {
        fn from(var: FixedTokenGrowthGlobalX128Call) -> Self {
            IVAMMCalls::FixedTokenGrowthGlobalX128(var)
        }
    }
    impl ::std::convert::From<GetRateOracleCall> for IVAMMCalls {
        fn from(var: GetRateOracleCall) -> Self {
            IVAMMCalls::GetRateOracle(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for IVAMMCalls {
        fn from(var: InitializeCall) -> Self {
            IVAMMCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<InitializeVAMMCall> for IVAMMCalls {
        fn from(var: InitializeVAMMCall) -> Self {
            IVAMMCalls::InitializeVAMM(var)
        }
    }
    impl ::std::convert::From<IsAlphaCall> for IVAMMCalls {
        fn from(var: IsAlphaCall) -> Self {
            IVAMMCalls::IsAlpha(var)
        }
    }
    impl ::std::convert::From<LiquidityCall> for IVAMMCalls {
        fn from(var: LiquidityCall) -> Self {
            IVAMMCalls::Liquidity(var)
        }
    }
    impl ::std::convert::From<MarginEngineCall> for IVAMMCalls {
        fn from(var: MarginEngineCall) -> Self {
            IVAMMCalls::MarginEngine(var)
        }
    }
    impl ::std::convert::From<MaxLiquidityPerTickCall> for IVAMMCalls {
        fn from(var: MaxLiquidityPerTickCall) -> Self {
            IVAMMCalls::MaxLiquidityPerTick(var)
        }
    }
    impl ::std::convert::From<MintCall> for IVAMMCalls {
        fn from(var: MintCall) -> Self {
            IVAMMCalls::Mint(var)
        }
    }
    impl ::std::convert::From<ProtocolFeesCall> for IVAMMCalls {
        fn from(var: ProtocolFeesCall) -> Self {
            IVAMMCalls::ProtocolFees(var)
        }
    }
    impl ::std::convert::From<RefreshRateOracleCall> for IVAMMCalls {
        fn from(var: RefreshRateOracleCall) -> Self {
            IVAMMCalls::RefreshRateOracle(var)
        }
    }
    impl ::std::convert::From<SetFeeCall> for IVAMMCalls {
        fn from(var: SetFeeCall) -> Self {
            IVAMMCalls::SetFee(var)
        }
    }
    impl ::std::convert::From<SetFeeProtocolCall> for IVAMMCalls {
        fn from(var: SetFeeProtocolCall) -> Self {
            IVAMMCalls::SetFeeProtocol(var)
        }
    }
    impl ::std::convert::From<SetIsAlphaCall> for IVAMMCalls {
        fn from(var: SetIsAlphaCall) -> Self {
            IVAMMCalls::SetIsAlpha(var)
        }
    }
    impl ::std::convert::From<SetPausabilityCall> for IVAMMCalls {
        fn from(var: SetPausabilityCall) -> Self {
            IVAMMCalls::SetPausability(var)
        }
    }
    impl ::std::convert::From<SwapCall> for IVAMMCalls {
        fn from(var: SwapCall) -> Self {
            IVAMMCalls::Swap(var)
        }
    }
    impl ::std::convert::From<TickBitmapCall> for IVAMMCalls {
        fn from(var: TickBitmapCall) -> Self {
            IVAMMCalls::TickBitmap(var)
        }
    }
    impl ::std::convert::From<TickSpacingCall> for IVAMMCalls {
        fn from(var: TickSpacingCall) -> Self {
            IVAMMCalls::TickSpacing(var)
        }
    }
    impl ::std::convert::From<TicksCall> for IVAMMCalls {
        fn from(var: TicksCall) -> Self {
            IVAMMCalls::Ticks(var)
        }
    }
    impl ::std::convert::From<UpdateProtocolFeesCall> for IVAMMCalls {
        fn from(var: UpdateProtocolFeesCall) -> Self {
            IVAMMCalls::UpdateProtocolFees(var)
        }
    }
    impl ::std::convert::From<VammVarsCall> for IVAMMCalls {
        fn from(var: VammVarsCall) -> Self {
            IVAMMCalls::VammVars(var)
        }
    }
    impl ::std::convert::From<VariableTokenGrowthGlobalX128Call> for IVAMMCalls {
        fn from(var: VariableTokenGrowthGlobalX128Call) -> Self {
            IVAMMCalls::VariableTokenGrowthGlobalX128(var)
        }
    }
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
