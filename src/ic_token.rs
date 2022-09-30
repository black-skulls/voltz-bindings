pub use ic_token::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod ic_token {
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
    #[doc = "ICToken was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ICTOKEN_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"accrualBlockNumber\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"borrowBalanceCurrent\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrowIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrowRatePerBlock\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"exchangeRateCurrent\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"exchangeRateStored\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"redeemAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"redeemUnderlying\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"supplyRatePerBlock\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"underlying\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct ICToken<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ICToken<M> {
        fn clone(&self) -> Self {
            ICToken(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ICToken<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ICToken<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ICToken))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ICToken<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ICTOKEN_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `accrualBlockNumber` (0x6c540baf) function"]
        pub fn accrual_block_number(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([108, 84, 11, 175], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `borrowBalanceCurrent` (0x17bfdfbc) function"]
        pub fn borrow_balance_current(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([23, 191, 223, 188], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `borrowIndex` (0xaa5af0fd) function"]
        pub fn borrow_index(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([170, 90, 240, 253], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `borrowRatePerBlock` (0xf8f9da28) function"]
        pub fn borrow_rate_per_block(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([248, 249, 218, 40], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `exchangeRateCurrent` (0xbd6d894d) function"]
        pub fn exchange_rate_current(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([189, 109, 137, 77], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `exchangeRateStored` (0x182df0f5) function"]
        pub fn exchange_rate_stored(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([24, 45, 240, 245], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `redeemUnderlying` (0x852a12e3) function"]
        pub fn redeem_underlying(
            &self,
            redeem_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([133, 42, 18, 227], redeem_amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `supplyRatePerBlock` (0xae9d70b0) function"]
        pub fn supply_rate_per_block(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([174, 157, 112, 176], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `underlying` (0x6f307dc3) function"]
        pub fn underlying(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([111, 48, 125, 195], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ICToken<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `accrualBlockNumber` function with signature `accrualBlockNumber()` and selector `[108, 84, 11, 175]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "accrualBlockNumber", abi = "accrualBlockNumber()")]
    pub struct AccrualBlockNumberCall;
    #[doc = "Container type for all input parameters for the `borrowBalanceCurrent` function with signature `borrowBalanceCurrent(address)` and selector `[23, 191, 223, 188]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "borrowBalanceCurrent", abi = "borrowBalanceCurrent(address)")]
    pub struct BorrowBalanceCurrentCall {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `borrowIndex` function with signature `borrowIndex()` and selector `[170, 90, 240, 253]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "borrowIndex", abi = "borrowIndex()")]
    pub struct BorrowIndexCall;
    #[doc = "Container type for all input parameters for the `borrowRatePerBlock` function with signature `borrowRatePerBlock()` and selector `[248, 249, 218, 40]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "borrowRatePerBlock", abi = "borrowRatePerBlock()")]
    pub struct BorrowRatePerBlockCall;
    #[doc = "Container type for all input parameters for the `exchangeRateCurrent` function with signature `exchangeRateCurrent()` and selector `[189, 109, 137, 77]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "exchangeRateCurrent", abi = "exchangeRateCurrent()")]
    pub struct ExchangeRateCurrentCall;
    #[doc = "Container type for all input parameters for the `exchangeRateStored` function with signature `exchangeRateStored()` and selector `[24, 45, 240, 245]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "exchangeRateStored", abi = "exchangeRateStored()")]
    pub struct ExchangeRateStoredCall;
    #[doc = "Container type for all input parameters for the `redeemUnderlying` function with signature `redeemUnderlying(uint256)` and selector `[133, 42, 18, 227]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "redeemUnderlying", abi = "redeemUnderlying(uint256)")]
    pub struct RedeemUnderlyingCall {
        pub redeem_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `supplyRatePerBlock` function with signature `supplyRatePerBlock()` and selector `[174, 157, 112, 176]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "supplyRatePerBlock", abi = "supplyRatePerBlock()")]
    pub struct SupplyRatePerBlockCall;
    #[doc = "Container type for all input parameters for the `underlying` function with signature `underlying()` and selector `[111, 48, 125, 195]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "underlying", abi = "underlying()")]
    pub struct UnderlyingCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ICTokenCalls {
        AccrualBlockNumber(AccrualBlockNumberCall),
        BorrowBalanceCurrent(BorrowBalanceCurrentCall),
        BorrowIndex(BorrowIndexCall),
        BorrowRatePerBlock(BorrowRatePerBlockCall),
        ExchangeRateCurrent(ExchangeRateCurrentCall),
        ExchangeRateStored(ExchangeRateStoredCall),
        RedeemUnderlying(RedeemUnderlyingCall),
        SupplyRatePerBlock(SupplyRatePerBlockCall),
        Underlying(UnderlyingCall),
    }
    impl ethers::core::abi::AbiDecode for ICTokenCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AccrualBlockNumberCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICTokenCalls::AccrualBlockNumber(decoded));
            }
            if let Ok(decoded) =
                <BorrowBalanceCurrentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICTokenCalls::BorrowBalanceCurrent(decoded));
            }
            if let Ok(decoded) =
                <BorrowIndexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICTokenCalls::BorrowIndex(decoded));
            }
            if let Ok(decoded) =
                <BorrowRatePerBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICTokenCalls::BorrowRatePerBlock(decoded));
            }
            if let Ok(decoded) =
                <ExchangeRateCurrentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICTokenCalls::ExchangeRateCurrent(decoded));
            }
            if let Ok(decoded) =
                <ExchangeRateStoredCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICTokenCalls::ExchangeRateStored(decoded));
            }
            if let Ok(decoded) =
                <RedeemUnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICTokenCalls::RedeemUnderlying(decoded));
            }
            if let Ok(decoded) =
                <SupplyRatePerBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICTokenCalls::SupplyRatePerBlock(decoded));
            }
            if let Ok(decoded) =
                <UnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICTokenCalls::Underlying(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ICTokenCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ICTokenCalls::AccrualBlockNumber(element) => element.encode(),
                ICTokenCalls::BorrowBalanceCurrent(element) => element.encode(),
                ICTokenCalls::BorrowIndex(element) => element.encode(),
                ICTokenCalls::BorrowRatePerBlock(element) => element.encode(),
                ICTokenCalls::ExchangeRateCurrent(element) => element.encode(),
                ICTokenCalls::ExchangeRateStored(element) => element.encode(),
                ICTokenCalls::RedeemUnderlying(element) => element.encode(),
                ICTokenCalls::SupplyRatePerBlock(element) => element.encode(),
                ICTokenCalls::Underlying(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ICTokenCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ICTokenCalls::AccrualBlockNumber(element) => element.fmt(f),
                ICTokenCalls::BorrowBalanceCurrent(element) => element.fmt(f),
                ICTokenCalls::BorrowIndex(element) => element.fmt(f),
                ICTokenCalls::BorrowRatePerBlock(element) => element.fmt(f),
                ICTokenCalls::ExchangeRateCurrent(element) => element.fmt(f),
                ICTokenCalls::ExchangeRateStored(element) => element.fmt(f),
                ICTokenCalls::RedeemUnderlying(element) => element.fmt(f),
                ICTokenCalls::SupplyRatePerBlock(element) => element.fmt(f),
                ICTokenCalls::Underlying(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AccrualBlockNumberCall> for ICTokenCalls {
        fn from(var: AccrualBlockNumberCall) -> Self {
            ICTokenCalls::AccrualBlockNumber(var)
        }
    }
    impl ::std::convert::From<BorrowBalanceCurrentCall> for ICTokenCalls {
        fn from(var: BorrowBalanceCurrentCall) -> Self {
            ICTokenCalls::BorrowBalanceCurrent(var)
        }
    }
    impl ::std::convert::From<BorrowIndexCall> for ICTokenCalls {
        fn from(var: BorrowIndexCall) -> Self {
            ICTokenCalls::BorrowIndex(var)
        }
    }
    impl ::std::convert::From<BorrowRatePerBlockCall> for ICTokenCalls {
        fn from(var: BorrowRatePerBlockCall) -> Self {
            ICTokenCalls::BorrowRatePerBlock(var)
        }
    }
    impl ::std::convert::From<ExchangeRateCurrentCall> for ICTokenCalls {
        fn from(var: ExchangeRateCurrentCall) -> Self {
            ICTokenCalls::ExchangeRateCurrent(var)
        }
    }
    impl ::std::convert::From<ExchangeRateStoredCall> for ICTokenCalls {
        fn from(var: ExchangeRateStoredCall) -> Self {
            ICTokenCalls::ExchangeRateStored(var)
        }
    }
    impl ::std::convert::From<RedeemUnderlyingCall> for ICTokenCalls {
        fn from(var: RedeemUnderlyingCall) -> Self {
            ICTokenCalls::RedeemUnderlying(var)
        }
    }
    impl ::std::convert::From<SupplyRatePerBlockCall> for ICTokenCalls {
        fn from(var: SupplyRatePerBlockCall) -> Self {
            ICTokenCalls::SupplyRatePerBlock(var)
        }
    }
    impl ::std::convert::From<UnderlyingCall> for ICTokenCalls {
        fn from(var: UnderlyingCall) -> Self {
            ICTokenCalls::Underlying(var)
        }
    }
    #[doc = "Container type for all return fields from the `accrualBlockNumber` function with signature `accrualBlockNumber()` and selector `[108, 84, 11, 175]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct AccrualBlockNumberReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `borrowBalanceCurrent` function with signature `borrowBalanceCurrent(address)` and selector `[23, 191, 223, 188]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BorrowBalanceCurrentReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `borrowIndex` function with signature `borrowIndex()` and selector `[170, 90, 240, 253]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BorrowIndexReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `borrowRatePerBlock` function with signature `borrowRatePerBlock()` and selector `[248, 249, 218, 40]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BorrowRatePerBlockReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `exchangeRateCurrent` function with signature `exchangeRateCurrent()` and selector `[189, 109, 137, 77]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ExchangeRateCurrentReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `exchangeRateStored` function with signature `exchangeRateStored()` and selector `[24, 45, 240, 245]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ExchangeRateStoredReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `redeemUnderlying` function with signature `redeemUnderlying(uint256)` and selector `[133, 42, 18, 227]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct RedeemUnderlyingReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `supplyRatePerBlock` function with signature `supplyRatePerBlock()` and selector `[174, 157, 112, 176]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SupplyRatePerBlockReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `underlying` function with signature `underlying()` and selector `[111, 48, 125, 195]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct UnderlyingReturn(pub ethers::core::types::Address);
}
