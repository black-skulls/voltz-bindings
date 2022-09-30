pub use mock_aave_lending_pool::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod mock_aave_lending_pool {
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
    #[doc = "MockAaveLendingPool was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static MOCKAAVELENDINGPOOL_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMathUD60x18__Exp2InputTooBig\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMathUD60x18__LogInputTooSmall\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"prod1\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMath__MulDivFixedPointOverflow\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract IERC20Minimal\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getReserveData\",\"outputs\":[{\"internalType\":\"struct IAaveV2LendingPool.ReserveData\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IAaveV2LendingPool.ReserveConfigurationMap\",\"name\":\"configuration\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"data\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"uint128\",\"name\":\"liquidityIndex\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"variableBorrowIndex\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"currentLiquidityRate\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"currentVariableBorrowRate\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"currentStableBorrowRate\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint40\",\"name\":\"lastUpdateTimestamp\",\"type\":\"uint40\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"aTokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"stableDebtTokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"variableDebtTokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"interestRateStrategyAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"id\",\"type\":\"uint8\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"contract IERC20Minimal\",\"name\":\"_underlyingAsset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getReserveNormalizedIncome\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract IERC20Minimal\",\"name\":\"_underlyingAsset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getReserveNormalizedVariableDebt\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract IERC20Minimal\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"aTokenAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initReserve\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract IERC20Minimal\",\"name\":\"_underlyingAsset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_factorPerSecondInRay\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setFactorPerSecondInRay\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract IERC20Minimal\",\"name\":\"_underlyingAsset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_reserveNormalizedIncome\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setReserveNormalizedIncome\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract IERC20Minimal\",\"name\":\"_underlyingAsset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_reserveNormalizedVariableDebt\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setReserveNormalizedVariableDebt\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract IERC20Minimal\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdraw\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MOCKAAVELENDINGPOOL_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b506112e0806100206000396000f3fe608060405234801561001057600080fd5b50600436106100885760003560e01c806369328dec1161005b57806369328dec146101475780636c3fbb801461015a578063d15e00531461016d578063fdb387d51461018057600080fd5b806335ea6a751461008d578063374f1b08146100b6578063386497fd146100f0578063455ee00c14610111575b600080fd5b6100a061009b36600461107a565b6101aa565b6040516100ad9190611097565b60405180910390f35b6100ee6100c43660046111a5565b6001600160a01b039091166000908152600160209081526040808320939093556002905220429055565b005b6101036100fe36600461107a565b610288565b6040519081526020016100ad565b6100ee61011f3660046111a5565b6001600160a01b03909116600090815260208181526040808320939093556002905220429055565b6101036101553660046111d1565b61031e565b6100ee610168366004611213565b610456565b61010361017b36600461107a565b61056e565b6100ee61018e3660046111a5565b6001600160a01b03909116600090815260036020526040902055565b6101b2610ff7565b506001600160a01b0390811660009081526004602081815260409283902083516101a08101855281546101808201908152815260018201546001600160801b0380821694830194909452600160801b908190048416958201959095526002820154808416606083015285900483166080820152600382015492831660a08201529390910464ffffffffff1660c084015290810154831660e0830152600581015483166101008301526006810154831661012083015260070154918216610140820152600160a01b90910460ff1661016082015290565b6001600160a01b0381166000908152600360205260408120548015610301576001600160a01b0383166000908152600260205260408120546102ca9042611262565b6001600160a01b0385166000908152600160205260409020549091506102f9906102f484846105f7565b61063f565b949350505050565b50506001600160a01b031660009081526001602052604090205490565b6001600160a01b0383811660009081526004602081905260408083208083015491516370a0823160e01b8152339381019390935292931690839082906370a082319060240160206040518083038186803b15801561037b57600080fd5b505afa15801561038f573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906103b39190611279565b9050856000198114156103c35750805b826001600160a01b031663d7020d0a3388846103de8d61056e565b6040516001600160e01b031960e087901b1681526001600160a01b03948516600482015293909216602484015260448301526064820152608401600060405180830381600087803b15801561043257600080fd5b505af1158015610446573d6000803e3d6000fd5b50929a9950505050505050505050565b61045e610ff7565b6001600160a01b0391821660e08201908152928216600090815260046020818152604092839020845151815590840151928401516001600160801b03938416600160801b918516820217600183015560608501516080860151908516908516820217600283015560a085015160038301805460c0880151929096166001600160a81b03199687161764ffffffffff909216909202179055945190850180546001600160a01b03199081169286169290921790556101008301516005860180548316918616919091179055610120830151600686018054909216908516179055610140820151600790940180546101609093015194909316911617600160a01b60ff90931692909202919091179055565b6001600160a01b03811660009081526003602052604081205480156105da576001600160a01b0383166000908152600260205260408120546105b09042611262565b6001600160a01b0385166000908152602081905260409020549091506102f9906102f484846105f7565b50506001600160a01b031660009081526020819052604090205490565b60008261061c57811561060b576000610615565b670de0b6b3a76400005b9050610639565b61063661063161062b8561064b565b8461063f565b610700565b90505b92915050565b6000610636838361074d565b6000670de0b6b3a764000082101561067e57604051633621413760e21b8152600481018390526024015b60405180910390fd5b6000610693670de0b6b3a7640000840461080f565b670de0b6b3a7640000808202935090915083821c908114156106b6575050919050565b6706f05b59d3b200005b80156106f857670de0b6b3a7640000828002049150671bc16d674ec8000082106106f0579283019260019190911c905b60011c6106c0565b505050919050565b6000680a688906bd8b000000821061072e57604051634a4f26f160e01b815260048101839052602401610675565b670de0b6b3a7640000604083901b04610746816108fb565b9392505050565b60008080600019848609848602925082811083820303915050670de0b6b3a764000081106107915760405163698d9a0160e11b815260048101829052602401610675565b600080670de0b6b3a76400008688099150506706f05b59d3b1ffff8111826107cb5780670de0b6b3a7640000850401945050505050610639565b620400008285030493909111909103600160ee1b02919091177faccb18165bd6fe31ae1cf318dc5b51eee0e1ba569b88cd74c1773b91fac106690201905092915050565b6000600160801b821061082f57608091821c9161082c9082611292565b90505b68010000000000000000821061085257604091821c9161084f9082611292565b90505b640100000000821061087157602091821c9161086e9082611292565b90505b62010000821061088e57601091821c9161088b9082611292565b90505b61010082106108aa57600891821c916108a79082611292565b90505b601082106108c557600491821c916108c29082611292565b90505b600482106108e057600291821c916108dd9082611292565b90505b600282106108f6576108f3600182611292565b90505b919050565b600160bf1b67800000000000000082161561091f5768016a09e667f3bcc9090260401c5b67400000000000000082161561093e576801306fe0a31b7152df0260401c5b67200000000000000082161561095d576801172b83c7d517adce0260401c5b67100000000000000082161561097c5768010b5586cf9890f62a0260401c5b67080000000000000082161561099b576801059b0d31585743ae0260401c5b6704000000000000008216156109ba57680102c9a3e778060ee70260401c5b6702000000000000008216156109d95768010163da9fb33356d80260401c5b6701000000000000008216156109f857680100b1afa5abcbed610260401c5b6680000000000000821615610a165768010058c86da1c09ea20260401c5b6640000000000000821615610a34576801002c605e2e8cec500260401c5b6620000000000000821615610a5257680100162f3904051fa10260401c5b6610000000000000821615610a70576801000b175effdc76ba0260401c5b6608000000000000821615610a8e57680100058ba01fb9f96d0260401c5b6604000000000000821615610aac5768010002c5cc37da94920260401c5b6602000000000000821615610aca576801000162e525ee05470260401c5b6601000000000000821615610ae85768010000b17255775c040260401c5b65800000000000821615610b05576801000058b91b5bc9ae0260401c5b65400000000000821615610b2257680100002c5c89d5ec6d0260401c5b65200000000000821615610b3f5768010000162e43f4f8310260401c5b65100000000000821615610b5c57680100000b1721bcfc9a0260401c5b65080000000000821615610b795768010000058b90cf1e6e0260401c5b65040000000000821615610b96576801000002c5c863b73f0260401c5b65020000000000821615610bb357680100000162e430e5a20260401c5b65010000000000821615610bd0576801000000b1721835510260401c5b648000000000821615610bec57680100000058b90c0b490260401c5b644000000000821615610c085768010000002c5c8601cc0260401c5b642000000000821615610c24576801000000162e42fff00260401c5b641000000000821615610c405768010000000b17217fbb0260401c5b640800000000821615610c5c576801000000058b90bfce0260401c5b640400000000821615610c7857680100000002c5c85fe30260401c5b640200000000821615610c945768010000000162e42ff10260401c5b640100000000821615610cb057680100000000b17217f80260401c5b6380000000821615610ccb5768010000000058b90bfc0260401c5b6340000000821615610ce6576801000000002c5c85fe0260401c5b6320000000821615610d0157680100000000162e42ff0260401c5b6310000000821615610d1c576801000000000b17217f0260401c5b6308000000821615610d3757680100000000058b90c00260401c5b6304000000821615610d525768010000000002c5c8600260401c5b6302000000821615610d6d576801000000000162e4300260401c5b6301000000821615610d885768010000000000b172180260401c5b62800000821615610da2576801000000000058b90c0260401c5b62400000821615610dbc57680100000000002c5c860260401c5b62200000821615610dd65768010000000000162e430260401c5b62100000821615610df057680100000000000b17210260401c5b62080000821615610e0a5768010000000000058b910260401c5b62040000821615610e24576801000000000002c5c80260401c5b62020000821615610e3e57680100000000000162e40260401c5b62010000821615610e58576801000000000000b1720260401c5b618000821615610e7157680100000000000058b90260401c5b614000821615610e8a5768010000000000002c5d0260401c5b612000821615610ea3576801000000000000162e0260401c5b611000821615610ebc5768010000000000000b170260401c5b610800821615610ed5576801000000000000058c0260401c5b610400821615610eee57680100000000000002c60260401c5b610200821615610f0757680100000000000001630260401c5b610100821615610f2057680100000000000000b10260401c5b6080821615610f3857680100000000000000590260401c5b6040821615610f50576801000000000000002c0260401c5b6020821615610f6857680100000000000000160260401c5b6010821615610f80576801000000000000000b0260401c5b6008821615610f9857680100000000000000060260401c5b6004821615610fb057680100000000000000030260401c5b6002821615610fc857680100000000000000010260401c5b6001821615610fe057680100000000000000010260401c5b670de0b6b3a76400000260409190911c60bf031c90565b604080516101a08101825260006101808201818152825260208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e0810182905261010081018290526101208101829052610140810182905261016081019190915290565b6001600160a01b038116811461107757600080fd5b50565b60006020828403121561108c57600080fd5b813561074681611062565b8151518152610180810160208301516110bb60208401826001600160801b03169052565b5060408301516110d660408401826001600160801b03169052565b5060608301516110f160608401826001600160801b03169052565b50608083015161110c60808401826001600160801b03169052565b5060a083015161112760a08401826001600160801b03169052565b5060c083015161114060c084018264ffffffffff169052565b5060e083015161115b60e08401826001600160a01b03169052565b50610100838101516001600160a01b03908116918401919091526101208085015182169084015261014080850151909116908301526101609283015160ff16929091019190915290565b600080604083850312156111b857600080fd5b82356111c381611062565b946020939093013593505050565b6000806000606084860312156111e657600080fd5b83356111f181611062565b925060208401359150604084013561120881611062565b809150509250925092565b6000806040838503121561122657600080fd5b823561123181611062565b9150602083013561124181611062565b809150509250929050565b634e487b7160e01b600052601160045260246000fd5b6000828210156112745761127461124c565b500390565b60006020828403121561128b57600080fd5b5051919050565b600082198211156112a5576112a561124c565b50019056fea2646970667358221220955a2cc522ca16a57d03144275dcd01f265cce80f5b54adb7a92436292c2dd9664736f6c63430008090033" . parse () . expect ("invalid bytecode")
        });
    pub struct MockAaveLendingPool<M>(ethers::contract::Contract<M>);
    impl<M> Clone for MockAaveLendingPool<M> {
        fn clone(&self) -> Self {
            MockAaveLendingPool(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for MockAaveLendingPool<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for MockAaveLendingPool<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MockAaveLendingPool))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> MockAaveLendingPool<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), MOCKAAVELENDINGPOOL_ABI.clone(), client)
                .into()
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
                MOCKAAVELENDINGPOOL_ABI.clone(),
                MOCKAAVELENDINGPOOL_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
        #[doc = "Calls the contract's `initReserve` (0x6c3fbb80) function"]
        pub fn init_reserve(
            &self,
            asset: ethers::core::types::Address,
            a_token_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([108, 63, 187, 128], (asset, a_token_address))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setFactorPerSecondInRay` (0xfdb387d5) function"]
        pub fn set_factor_per_second_in_ray(
            &self,
            underlying_asset: ethers::core::types::Address,
            factor_per_second_in_ray: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [253, 179, 135, 213],
                    (underlying_asset, factor_per_second_in_ray),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setReserveNormalizedIncome` (0x455ee00c) function"]
        pub fn set_reserve_normalized_income(
            &self,
            underlying_asset: ethers::core::types::Address,
            reserve_normalized_income: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [69, 94, 224, 12],
                    (underlying_asset, reserve_normalized_income),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setReserveNormalizedVariableDebt` (0x374f1b08) function"]
        pub fn set_reserve_normalized_variable_debt(
            &self,
            underlying_asset: ethers::core::types::Address,
            reserve_normalized_variable_debt: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [55, 79, 27, 8],
                    (underlying_asset, reserve_normalized_variable_debt),
                )
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
        for MockAaveLendingPool<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Custom Error type `PRBMathUD60x18__Exp2InputTooBig` with signature `PRBMathUD60x18__Exp2InputTooBig(uint256)` and selector `[74, 79, 38, 241]`"]
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
        name = "PRBMathUD60x18__Exp2InputTooBig",
        abi = "PRBMathUD60x18__Exp2InputTooBig(uint256)"
    )]
    pub struct PRBMathUD60x18__Exp2InputTooBig {
        pub x: ethers::core::types::U256,
    }
    #[doc = "Custom Error type `PRBMathUD60x18__LogInputTooSmall` with signature `PRBMathUD60x18__LogInputTooSmall(uint256)` and selector `[216, 133, 4, 220]`"]
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
        name = "PRBMathUD60x18__LogInputTooSmall",
        abi = "PRBMathUD60x18__LogInputTooSmall(uint256)"
    )]
    pub struct PRBMathUD60x18__LogInputTooSmall {
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MockAaveLendingPoolErrors {
        PRBMathUD60x18__Exp2InputTooBig(PRBMathUD60x18__Exp2InputTooBig),
        PRBMathUD60x18__LogInputTooSmall(PRBMathUD60x18__LogInputTooSmall),
        PRBMath__MulDivFixedPointOverflow(PRBMath__MulDivFixedPointOverflow),
    }
    impl ethers::core::abi::AbiDecode for MockAaveLendingPoolErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <PRBMathUD60x18__Exp2InputTooBig as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MockAaveLendingPoolErrors::PRBMathUD60x18__Exp2InputTooBig(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <PRBMathUD60x18__LogInputTooSmall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MockAaveLendingPoolErrors::PRBMathUD60x18__LogInputTooSmall(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <PRBMath__MulDivFixedPointOverflow as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MockAaveLendingPoolErrors::PRBMath__MulDivFixedPointOverflow(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MockAaveLendingPoolErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                MockAaveLendingPoolErrors::PRBMathUD60x18__Exp2InputTooBig(element) => {
                    element.encode()
                }
                MockAaveLendingPoolErrors::PRBMathUD60x18__LogInputTooSmall(element) => {
                    element.encode()
                }
                MockAaveLendingPoolErrors::PRBMath__MulDivFixedPointOverflow(element) => {
                    element.encode()
                }
            }
        }
    }
    impl ::std::fmt::Display for MockAaveLendingPoolErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MockAaveLendingPoolErrors::PRBMathUD60x18__Exp2InputTooBig(element) => {
                    element.fmt(f)
                }
                MockAaveLendingPoolErrors::PRBMathUD60x18__LogInputTooSmall(element) => {
                    element.fmt(f)
                }
                MockAaveLendingPoolErrors::PRBMath__MulDivFixedPointOverflow(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    impl ::std::convert::From<PRBMathUD60x18__Exp2InputTooBig> for MockAaveLendingPoolErrors {
        fn from(var: PRBMathUD60x18__Exp2InputTooBig) -> Self {
            MockAaveLendingPoolErrors::PRBMathUD60x18__Exp2InputTooBig(var)
        }
    }
    impl ::std::convert::From<PRBMathUD60x18__LogInputTooSmall> for MockAaveLendingPoolErrors {
        fn from(var: PRBMathUD60x18__LogInputTooSmall) -> Self {
            MockAaveLendingPoolErrors::PRBMathUD60x18__LogInputTooSmall(var)
        }
    }
    impl ::std::convert::From<PRBMath__MulDivFixedPointOverflow> for MockAaveLendingPoolErrors {
        fn from(var: PRBMath__MulDivFixedPointOverflow) -> Self {
            MockAaveLendingPoolErrors::PRBMath__MulDivFixedPointOverflow(var)
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
    #[doc = "Container type for all input parameters for the `initReserve` function with signature `initReserve(address,address)` and selector `[108, 63, 187, 128]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "initReserve", abi = "initReserve(address,address)")]
    pub struct InitReserveCall {
        pub asset: ethers::core::types::Address,
        pub a_token_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setFactorPerSecondInRay` function with signature `setFactorPerSecondInRay(address,uint256)` and selector `[253, 179, 135, 213]`"]
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
        name = "setFactorPerSecondInRay",
        abi = "setFactorPerSecondInRay(address,uint256)"
    )]
    pub struct SetFactorPerSecondInRayCall {
        pub underlying_asset: ethers::core::types::Address,
        pub factor_per_second_in_ray: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setReserveNormalizedIncome` function with signature `setReserveNormalizedIncome(address,uint256)` and selector `[69, 94, 224, 12]`"]
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
        name = "setReserveNormalizedIncome",
        abi = "setReserveNormalizedIncome(address,uint256)"
    )]
    pub struct SetReserveNormalizedIncomeCall {
        pub underlying_asset: ethers::core::types::Address,
        pub reserve_normalized_income: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setReserveNormalizedVariableDebt` function with signature `setReserveNormalizedVariableDebt(address,uint256)` and selector `[55, 79, 27, 8]`"]
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
        name = "setReserveNormalizedVariableDebt",
        abi = "setReserveNormalizedVariableDebt(address,uint256)"
    )]
    pub struct SetReserveNormalizedVariableDebtCall {
        pub underlying_asset: ethers::core::types::Address,
        pub reserve_normalized_variable_debt: ethers::core::types::U256,
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
    pub enum MockAaveLendingPoolCalls {
        GetReserveData(GetReserveDataCall),
        GetReserveNormalizedIncome(GetReserveNormalizedIncomeCall),
        GetReserveNormalizedVariableDebt(GetReserveNormalizedVariableDebtCall),
        InitReserve(InitReserveCall),
        SetFactorPerSecondInRay(SetFactorPerSecondInRayCall),
        SetReserveNormalizedIncome(SetReserveNormalizedIncomeCall),
        SetReserveNormalizedVariableDebt(SetReserveNormalizedVariableDebtCall),
        Withdraw(WithdrawCall),
    }
    impl ethers::core::abi::AbiDecode for MockAaveLendingPoolCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetReserveDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAaveLendingPoolCalls::GetReserveData(decoded));
            }
            if let Ok(decoded) =
                <GetReserveNormalizedIncomeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MockAaveLendingPoolCalls::GetReserveNormalizedIncome(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetReserveNormalizedVariableDebtCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MockAaveLendingPoolCalls::GetReserveNormalizedVariableDebt(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <InitReserveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAaveLendingPoolCalls::InitReserve(decoded));
            }
            if let Ok(decoded) =
                <SetFactorPerSecondInRayCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAaveLendingPoolCalls::SetFactorPerSecondInRay(decoded));
            }
            if let Ok(decoded) =
                <SetReserveNormalizedIncomeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MockAaveLendingPoolCalls::SetReserveNormalizedIncome(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetReserveNormalizedVariableDebtCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MockAaveLendingPoolCalls::SetReserveNormalizedVariableDebt(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAaveLendingPoolCalls::Withdraw(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MockAaveLendingPoolCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MockAaveLendingPoolCalls::GetReserveData(element) => element.encode(),
                MockAaveLendingPoolCalls::GetReserveNormalizedIncome(element) => element.encode(),
                MockAaveLendingPoolCalls::GetReserveNormalizedVariableDebt(element) => {
                    element.encode()
                }
                MockAaveLendingPoolCalls::InitReserve(element) => element.encode(),
                MockAaveLendingPoolCalls::SetFactorPerSecondInRay(element) => element.encode(),
                MockAaveLendingPoolCalls::SetReserveNormalizedIncome(element) => element.encode(),
                MockAaveLendingPoolCalls::SetReserveNormalizedVariableDebt(element) => {
                    element.encode()
                }
                MockAaveLendingPoolCalls::Withdraw(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MockAaveLendingPoolCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MockAaveLendingPoolCalls::GetReserveData(element) => element.fmt(f),
                MockAaveLendingPoolCalls::GetReserveNormalizedIncome(element) => element.fmt(f),
                MockAaveLendingPoolCalls::GetReserveNormalizedVariableDebt(element) => {
                    element.fmt(f)
                }
                MockAaveLendingPoolCalls::InitReserve(element) => element.fmt(f),
                MockAaveLendingPoolCalls::SetFactorPerSecondInRay(element) => element.fmt(f),
                MockAaveLendingPoolCalls::SetReserveNormalizedIncome(element) => element.fmt(f),
                MockAaveLendingPoolCalls::SetReserveNormalizedVariableDebt(element) => {
                    element.fmt(f)
                }
                MockAaveLendingPoolCalls::Withdraw(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetReserveDataCall> for MockAaveLendingPoolCalls {
        fn from(var: GetReserveDataCall) -> Self {
            MockAaveLendingPoolCalls::GetReserveData(var)
        }
    }
    impl ::std::convert::From<GetReserveNormalizedIncomeCall> for MockAaveLendingPoolCalls {
        fn from(var: GetReserveNormalizedIncomeCall) -> Self {
            MockAaveLendingPoolCalls::GetReserveNormalizedIncome(var)
        }
    }
    impl ::std::convert::From<GetReserveNormalizedVariableDebtCall> for MockAaveLendingPoolCalls {
        fn from(var: GetReserveNormalizedVariableDebtCall) -> Self {
            MockAaveLendingPoolCalls::GetReserveNormalizedVariableDebt(var)
        }
    }
    impl ::std::convert::From<InitReserveCall> for MockAaveLendingPoolCalls {
        fn from(var: InitReserveCall) -> Self {
            MockAaveLendingPoolCalls::InitReserve(var)
        }
    }
    impl ::std::convert::From<SetFactorPerSecondInRayCall> for MockAaveLendingPoolCalls {
        fn from(var: SetFactorPerSecondInRayCall) -> Self {
            MockAaveLendingPoolCalls::SetFactorPerSecondInRay(var)
        }
    }
    impl ::std::convert::From<SetReserveNormalizedIncomeCall> for MockAaveLendingPoolCalls {
        fn from(var: SetReserveNormalizedIncomeCall) -> Self {
            MockAaveLendingPoolCalls::SetReserveNormalizedIncome(var)
        }
    }
    impl ::std::convert::From<SetReserveNormalizedVariableDebtCall> for MockAaveLendingPoolCalls {
        fn from(var: SetReserveNormalizedVariableDebtCall) -> Self {
            MockAaveLendingPoolCalls::SetReserveNormalizedVariableDebt(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for MockAaveLendingPoolCalls {
        fn from(var: WithdrawCall) -> Self {
            MockAaveLendingPoolCalls::Withdraw(var)
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
