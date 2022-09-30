pub use i_aave_v2_lending_pool::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_aave_v2_lending_pool {
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
    #[doc = "IAaveV2LendingPool was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IAAVEV2LENDINGPOOL_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"contract IERC20Minimal\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getReserveData\",\"outputs\":[{\"internalType\":\"struct IAaveV2LendingPool.ReserveData\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IAaveV2LendingPool.ReserveConfigurationMap\",\"name\":\"configuration\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"data\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"uint128\",\"name\":\"liquidityIndex\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"variableBorrowIndex\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"currentLiquidityRate\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"currentVariableBorrowRate\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"currentStableBorrowRate\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint40\",\"name\":\"lastUpdateTimestamp\",\"type\":\"uint40\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"aTokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"stableDebtTokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"variableDebtTokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"interestRateStrategyAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"id\",\"type\":\"uint8\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"contract IERC20Minimal\",\"name\":\"underlyingAsset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getReserveNormalizedIncome\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract IERC20Minimal\",\"name\":\"underlyingAsset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getReserveNormalizedVariableDebt\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract IERC20Minimal\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdraw\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct IAaveV2LendingPool<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IAaveV2LendingPool<M> {
        fn clone(&self) -> Self {
            IAaveV2LendingPool(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IAaveV2LendingPool<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IAaveV2LendingPool<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IAaveV2LendingPool))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IAaveV2LendingPool<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IAAVEV2LENDINGPOOL_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `getReserveData` (0x35ea6a75) function"]
        pub fn get_reserve_data(
            &self,
            asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ReserveData> {
            self.0
                .method_hash([53, 234, 106, 117], asset)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getReserveNormalizedIncome` (0xd15e0053) function"]
        pub fn get_reserve_normalized_income(
            &self,
            underlying_asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([209, 94, 0, 83], underlying_asset)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getReserveNormalizedVariableDebt` (0x386497fd) function"]
        pub fn get_reserve_normalized_variable_debt(
            &self,
            underlying_asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([56, 100, 151, 253], underlying_asset)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0x69328dec) function"]
        pub fn withdraw(
            &self,
            asset: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            to: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([105, 50, 141, 236], (asset, amount, to))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for IAaveV2LendingPool<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `getReserveData` function with signature `getReserveData(address)` and selector `[53, 234, 106, 117]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getReserveData", abi = "getReserveData(address)")]
    pub struct GetReserveDataCall {
        pub asset: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getReserveNormalizedIncome` function with signature `getReserveNormalizedIncome(address)` and selector `[209, 94, 0, 83]`"]
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
        name = "getReserveNormalizedIncome",
        abi = "getReserveNormalizedIncome(address)"
    )]
    pub struct GetReserveNormalizedIncomeCall {
        pub underlying_asset: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getReserveNormalizedVariableDebt` function with signature `getReserveNormalizedVariableDebt(address)` and selector `[56, 100, 151, 253]`"]
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
        name = "getReserveNormalizedVariableDebt",
        abi = "getReserveNormalizedVariableDebt(address)"
    )]
    pub struct GetReserveNormalizedVariableDebtCall {
        pub underlying_asset: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `withdraw` function with signature `withdraw(address,uint256,address)` and selector `[105, 50, 141, 236]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "withdraw", abi = "withdraw(address,uint256,address)")]
    pub struct WithdrawCall {
        pub asset: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub to: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IAaveV2LendingPoolCalls {
        GetReserveData(GetReserveDataCall),
        GetReserveNormalizedIncome(GetReserveNormalizedIncomeCall),
        GetReserveNormalizedVariableDebt(GetReserveNormalizedVariableDebtCall),
        Withdraw(WithdrawCall),
    }
    impl ethers::core::abi::AbiDecode for IAaveV2LendingPoolCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetReserveDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveV2LendingPoolCalls::GetReserveData(decoded));
            }
            if let Ok(decoded) =
                <GetReserveNormalizedIncomeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IAaveV2LendingPoolCalls::GetReserveNormalizedIncome(decoded));
            }
            if let Ok(decoded) =
                <GetReserveNormalizedVariableDebtCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IAaveV2LendingPoolCalls::GetReserveNormalizedVariableDebt(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveV2LendingPoolCalls::Withdraw(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IAaveV2LendingPoolCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IAaveV2LendingPoolCalls::GetReserveData(element) => element.encode(),
                IAaveV2LendingPoolCalls::GetReserveNormalizedIncome(element) => element.encode(),
                IAaveV2LendingPoolCalls::GetReserveNormalizedVariableDebt(element) => {
                    element.encode()
                }
                IAaveV2LendingPoolCalls::Withdraw(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IAaveV2LendingPoolCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IAaveV2LendingPoolCalls::GetReserveData(element) => element.fmt(f),
                IAaveV2LendingPoolCalls::GetReserveNormalizedIncome(element) => element.fmt(f),
                IAaveV2LendingPoolCalls::GetReserveNormalizedVariableDebt(element) => {
                    element.fmt(f)
                }
                IAaveV2LendingPoolCalls::Withdraw(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetReserveDataCall> for IAaveV2LendingPoolCalls {
        fn from(var: GetReserveDataCall) -> Self {
            IAaveV2LendingPoolCalls::GetReserveData(var)
        }
    }
    impl ::std::convert::From<GetReserveNormalizedIncomeCall> for IAaveV2LendingPoolCalls {
        fn from(var: GetReserveNormalizedIncomeCall) -> Self {
            IAaveV2LendingPoolCalls::GetReserveNormalizedIncome(var)
        }
    }
    impl ::std::convert::From<GetReserveNormalizedVariableDebtCall> for IAaveV2LendingPoolCalls {
        fn from(var: GetReserveNormalizedVariableDebtCall) -> Self {
            IAaveV2LendingPoolCalls::GetReserveNormalizedVariableDebt(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for IAaveV2LendingPoolCalls {
        fn from(var: WithdrawCall) -> Self {
            IAaveV2LendingPoolCalls::Withdraw(var)
        }
    }
    #[doc = "Container type for all return fields from the `getReserveData` function with signature `getReserveData(address)` and selector `[53, 234, 106, 117]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetReserveDataReturn(pub ReserveData);
    #[doc = "Container type for all return fields from the `getReserveNormalizedIncome` function with signature `getReserveNormalizedIncome(address)` and selector `[209, 94, 0, 83]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetReserveNormalizedIncomeReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getReserveNormalizedVariableDebt` function with signature `getReserveNormalizedVariableDebt(address)` and selector `[56, 100, 151, 253]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetReserveNormalizedVariableDebtReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `withdraw` function with signature `withdraw(address,uint256,address)` and selector `[105, 50, 141, 236]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct WithdrawReturn(pub ethers::core::types::U256);
}
