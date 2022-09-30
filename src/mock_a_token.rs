pub use mock_a_token::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod mock_a_token {
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
    #[doc = "MockAToken was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static MOCKATOKEN_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"contract IAaveV2LendingPool\",\"name\":\"pool\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IERC20Minimal\",\"name\":\"underlyingAsset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"name\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"symbol\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approveInternal\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"receiverOfUnderlying\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burn\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"subtractedValue\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"decreaseAllowance\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"addedValue\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"increaseAllowance\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mint\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MOCKATOKEN_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60806040523480156200001157600080fd5b50604051620013b8380380620013b883398101604081905262000034916200022b565b8151829082906200004d9060039060208501906200009f565b508051620000639060049060208401906200009f565b5050600580546001600160a01b039687166001600160a01b031991821617909155600680549590961694169390931790935550620002fd915050565b828054620000ad90620002c0565b90600052602060002090601f016020900481019282620000d157600085556200011c565b82601f10620000ec57805160ff19168380011785556200011c565b828001600101855582156200011c579182015b828111156200011c578251825591602001919060010190620000ff565b506200012a9291506200012e565b5090565b5b808211156200012a57600081556001016200012f565b6001600160a01b03811681146200015b57600080fd5b50565b634e487b7160e01b600052604160045260246000fd5b600082601f8301126200018657600080fd5b81516001600160401b0380821115620001a357620001a36200015e565b604051601f8301601f19908116603f01168101908282118183101715620001ce57620001ce6200015e565b81604052838152602092508683858801011115620001eb57600080fd5b600091505b838210156200020f5785820183015181830184015290820190620001f0565b83821115620002215760008385830101525b9695505050505050565b600080600080608085870312156200024257600080fd5b84516200024f8162000145565b6020860151909450620002628162000145565b60408601519093506001600160401b03808211156200028057600080fd5b6200028e8883890162000174565b93506060870151915080821115620002a557600080fd5b50620002b48782880162000174565b91505092959194509250565b600181811c90821680620002d557607f821691505b60208210811415620002f757634e487b7160e01b600052602260045260246000fd5b50919050565b6110ab806200030d6000396000f3fe608060405234801561001057600080fd5b50600436106100ea5760003560e01c806356189cb41161008c578063a457c2d711610066578063a457c2d7146101ba578063a9059cbb146101cd578063d7020d0a146101e0578063dd62ed3e146101f357600080fd5b806356189cb41461018a57806370a082311461019f57806395d89b41146101b257600080fd5b806318160ddd116100c857806318160ddd1461014357806323b872dd14610155578063313ce56714610168578063395093511461017757600080fd5b806306fdde03146100ef578063095ea7b31461010d578063156e29f614610130575b600080fd5b6100f7610206565b6040516101049190610dc0565b60405180910390f35b61012061011b366004610e31565b610298565b6040519015158152602001610104565b61012061013e366004610e5b565b6102b0565b6002545b604051908152602001610104565b610120610163366004610e8e565b610367565b60405160128152602001610104565b610120610185366004610e31565b61038b565b61019d610198366004610e8e565b6103ad565b005b6101476101ad366004610eca565b6103bd565b6100f7610467565b6101206101c8366004610e31565b610476565b6101206101db366004610e31565b6104f1565b61019d6101ee366004610ee5565b6104ff565b610147610201366004610f27565b610674565b60606003805461021590610f5a565b80601f016020809104026020016040519081016040528092919081815260200182805461024190610f5a565b801561028e5780601f106102635761010080835404028352916020019161028e565b820191906000526020600020905b81548152906001019060200180831161027157829003601f168201915b5050505050905090565b6000336102a681858561069f565b5060019392505050565b6001600160a01b03831660009081526020819052604081205460006102d585856107c3565b9050806103225760405162461bcd60e51b815260206004820152601660248201527510d517d253959053125117d352539517d05353d5539560521b60448201526064015b60405180910390fd5b61032c86826107d1565b6040518581526001600160a01b038716906000906000805160206110568339815191529060200160405180910390a3501590505b9392505050565b60003361037585828561087e565b6103808585856108f8565b506001949350505050565b6000336102a681858561039e8383610674565b6103a89190610fab565b61069f565b6103b883838361069f565b505050565b60055460065460405163d15e005360e01b81526001600160a01b03918216600482015260009261046192169063d15e00539060240160206040518083038186803b15801561040a57600080fd5b505afa15801561041e573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906104429190610fc3565b6001600160a01b0384166000908152602081905260409020549061099e565b92915050565b60606004805461021590610f5a565b600033816104848286610674565b9050838110156104e45760405162461bcd60e51b815260206004820152602560248201527f45524332303a2064656372656173656420616c6c6f77616e63652062656c6f77604482015264207a65726f60d81b6064820152608401610319565b610380828686840361069f565b6000336102a68185856108f8565b6005546001600160a01b031633146105595760405162461bcd60e51b815260206004820152601e60248201527f43545f43414c4c45525f4d5553545f42455f4c454e44494e475f504f4f4c00006044820152606401610319565b600061056583836107c3565b9050806105ad5760405162461bcd60e51b815260206004820152601660248201527510d517d253959053125117d095549397d05353d5539560521b6044820152606401610319565b6105b785826109ac565b60065460405163a9059cbb60e01b81526001600160a01b038681166004830152602482018690529091169063a9059cbb90604401602060405180830381600087803b15801561060557600080fd5b505af1158015610619573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061063d9190610fdc565b506040518381526000906001600160a01b038716906000805160206110568339815191529060200160405180910390a35050505050565b6001600160a01b03918216600090815260016020908152604080832093909416825291909152205490565b6001600160a01b0383166107015760405162461bcd60e51b8152602060048201526024808201527f45524332303a20617070726f76652066726f6d20746865207a65726f206164646044820152637265737360e01b6064820152608401610319565b6001600160a01b0382166107625760405162461bcd60e51b815260206004820152602260248201527f45524332303a20617070726f766520746f20746865207a65726f206164647265604482015261737360f01b6064820152608401610319565b6001600160a01b0383811660008181526001602090815260408083209487168084529482529182902085905590518481527f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925910160405180910390a3505050565b600061036083836001610acc565b6001600160a01b0382166108275760405162461bcd60e51b815260206004820152601f60248201527f45524332303a206d696e7420746f20746865207a65726f2061646472657373006044820152606401610319565b80600260008282546108399190610fab565b90915550506001600160a01b03821660008181526020818152604080832080548601905551848152600080516020611056833981519152910160405180910390a35050565b600061088a8484610674565b905060001981146108f257818110156108e55760405162461bcd60e51b815260206004820152601d60248201527f45524332303a20696e73756666696369656e7420616c6c6f77616e63650000006044820152606401610319565b6108f2848484840361069f565b50505050565b60065460055460405163d15e005360e01b81526001600160a01b03928316600482018190529290911690600090829063d15e00539060240160206040518083038186803b15801561094857600080fd5b505afa15801561095c573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906109809190610fc3565b9050610996868661099187856107c3565b610b71565b505050505050565b600061036083836001610d03565b6001600160a01b038216610a0c5760405162461bcd60e51b815260206004820152602160248201527f45524332303a206275726e2066726f6d20746865207a65726f206164647265736044820152607360f81b6064820152608401610319565b6001600160a01b03821660009081526020819052604090205481811015610a805760405162461bcd60e51b815260206004820152602260248201527f45524332303a206275726e20616d6f756e7420657863656564732062616c616e604482015261636560f01b6064820152608401610319565b6001600160a01b038316600081815260208181526040808320868603905560028054879003905551858152919291600080516020611056833981519152910160405180910390a3505050565b600082610b045760405162461bcd60e51b8152600401610319906020808252600490820152630444956360e41b604082015260600190565b6000610b11600285610ffe565b905083816001856001811115610b2957610b29611020565b14610b3c57670de0b6b3a7640000610b4a565b6b033b2e3c9fd0803ce80000005b610b549088611036565b610b5e9190610fab565b610b689190610ffe565b95945050505050565b6001600160a01b038316610bd55760405162461bcd60e51b815260206004820152602560248201527f45524332303a207472616e736665722066726f6d20746865207a65726f206164604482015264647265737360d81b6064820152608401610319565b6001600160a01b038216610c375760405162461bcd60e51b815260206004820152602360248201527f45524332303a207472616e7366657220746f20746865207a65726f206164647260448201526265737360e81b6064820152608401610319565b6001600160a01b03831660009081526020819052604090205481811015610caf5760405162461bcd60e51b815260206004820152602660248201527f45524332303a207472616e7366657220616d6f756e7420657863656564732062604482015265616c616e636560d01b6064820152608401610319565b6001600160a01b0384811660008181526020818152604080832087870390559387168083529184902080548701905592518581529092600080516020611056833981519152910160405180910390a36108f2565b6000831580610d10575082155b15610d1d57506000610360565b6001826001811115610d3157610d31611020565b14610d4457670de0b6b3a7640000610d52565b6b033b2e3c9fd0803ce80000005b6001836001811115610d6657610d66611020565b14610d8357610d7e6002670de0b6b3a7640000610ffe565b610d9a565b610d9a60026b033b2e3c9fd0803ce8000000610ffe565b610da48587611036565b610dae9190610fab565b610db89190610ffe565b949350505050565b600060208083528351808285015260005b81811015610ded57858101830151858201604001528201610dd1565b81811115610dff576000604083870101525b50601f01601f1916929092016040019392505050565b80356001600160a01b0381168114610e2c57600080fd5b919050565b60008060408385031215610e4457600080fd5b610e4d83610e15565b946020939093013593505050565b600080600060608486031215610e7057600080fd5b610e7984610e15565b95602085013595506040909401359392505050565b600080600060608486031215610ea357600080fd5b610eac84610e15565b9250610eba60208501610e15565b9150604084013590509250925092565b600060208284031215610edc57600080fd5b61036082610e15565b60008060008060808587031215610efb57600080fd5b610f0485610e15565b9350610f1260208601610e15565b93969395505050506040820135916060013590565b60008060408385031215610f3a57600080fd5b610f4383610e15565b9150610f5160208401610e15565b90509250929050565b600181811c90821680610f6e57607f821691505b60208210811415610f8f57634e487b7160e01b600052602260045260246000fd5b50919050565b634e487b7160e01b600052601160045260246000fd5b60008219821115610fbe57610fbe610f95565b500190565b600060208284031215610fd557600080fd5b5051919050565b600060208284031215610fee57600080fd5b8151801515811461036057600080fd5b60008261101b57634e487b7160e01b600052601260045260246000fd5b500490565b634e487b7160e01b600052602160045260246000fd5b600081600019048311821515161561105057611050610f95565b50029056feddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa26469706673582212206307df3d15da9a04c6996e7b58afea279b2735abbdeb568c134cedd6d31f08e964736f6c63430008090033" . parse () . expect ("invalid bytecode")
        });
    pub struct MockAToken<M>(ethers::contract::Contract<M>);
    impl<M> Clone for MockAToken<M> {
        fn clone(&self) -> Self {
            MockAToken(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for MockAToken<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for MockAToken<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MockAToken))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> MockAToken<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), MOCKATOKEN_ABI.clone(), client).into()
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
                MOCKATOKEN_ABI.clone(),
                MOCKATOKEN_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `allowance` (0xdd62ed3e) function"]
        pub fn allowance(
            &self,
            owner: ethers::core::types::Address,
            spender: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approve` (0x095ea7b3) function"]
        pub fn approve(
            &self,
            spender: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approveInternal` (0x56189cb4) function"]
        pub fn approve_internal(
            &self,
            owner: ethers::core::types::Address,
            spender: ethers::core::types::Address,
            value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([86, 24, 156, 180], (owner, spender, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOf` (0x70a08231) function"]
        pub fn balance_of(
            &self,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `burn` (0xd7020d0a) function"]
        pub fn burn(
            &self,
            user: ethers::core::types::Address,
            receiver_of_underlying: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [215, 2, 13, 10],
                    (user, receiver_of_underlying, amount, index),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decreaseAllowance` (0xa457c2d7) function"]
        pub fn decrease_allowance(
            &self,
            spender: ethers::core::types::Address,
            subtracted_value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([164, 87, 194, 215], (spender, subtracted_value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `increaseAllowance` (0x39509351) function"]
        pub fn increase_allowance(
            &self,
            spender: ethers::core::types::Address,
            added_value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([57, 80, 147, 81], (spender, added_value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mint` (0x156e29f6) function"]
        pub fn mint(
            &self,
            user: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([21, 110, 41, 246], (user, amount, index))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `symbol` (0x95d89b41) function"]
        pub fn symbol(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalSupply` (0x18160ddd) function"]
        pub fn total_supply(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transfer` (0xa9059cbb) function"]
        pub fn transfer(
            &self,
            to: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (to, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferFrom` (0x23b872dd) function"]
        pub fn transfer_from(
            &self,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, MockATokenEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for MockAToken<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MockATokenEvents {
        ApprovalFilter(ApprovalFilter),
        TransferFilter(TransferFilter),
    }
    impl ethers::contract::EthLogDecode for MockATokenEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(MockATokenEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(MockATokenEvents::TransferFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for MockATokenEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MockATokenEvents::ApprovalFilter(element) => element.fmt(f),
                MockATokenEvents::TransferFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `[221, 98, 237, 62]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall {
        pub owner: ethers::core::types::Address,
        pub spender: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `[9, 94, 167, 179]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `approveInternal` function with signature `approveInternal(address,address,uint256)` and selector `[86, 24, 156, 180]`"]
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
        name = "approveInternal",
        abi = "approveInternal(address,address,uint256)"
    )]
    pub struct ApproveInternalCall {
        pub owner: ethers::core::types::Address,
        pub spender: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `burn` function with signature `burn(address,address,uint256,uint256)` and selector `[215, 2, 13, 10]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "burn", abi = "burn(address,address,uint256,uint256)")]
    pub struct BurnCall {
        pub user: ethers::core::types::Address,
        pub receiver_of_underlying: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `[49, 60, 229, 103]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    #[doc = "Container type for all input parameters for the `decreaseAllowance` function with signature `decreaseAllowance(address,uint256)` and selector `[164, 87, 194, 215]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "decreaseAllowance", abi = "decreaseAllowance(address,uint256)")]
    pub struct DecreaseAllowanceCall {
        pub spender: ethers::core::types::Address,
        pub subtracted_value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `increaseAllowance` function with signature `increaseAllowance(address,uint256)` and selector `[57, 80, 147, 81]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "increaseAllowance", abi = "increaseAllowance(address,uint256)")]
    pub struct IncreaseAllowanceCall {
        pub spender: ethers::core::types::Address,
        pub added_value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `mint` function with signature `mint(address,uint256,uint256)` and selector `[21, 110, 41, 246]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "mint", abi = "mint(address,uint256,uint256)")]
    pub struct MintCall {
        pub user: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `name` function with signature `name()` and selector `[6, 253, 222, 3]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    #[doc = "Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    #[doc = "Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    #[doc = "Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `[169, 5, 156, 187]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub to: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `[35, 184, 114, 221]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MockATokenCalls {
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        ApproveInternal(ApproveInternalCall),
        BalanceOf(BalanceOfCall),
        Burn(BurnCall),
        Decimals(DecimalsCall),
        DecreaseAllowance(DecreaseAllowanceCall),
        IncreaseAllowance(IncreaseAllowanceCall),
        Mint(MintCall),
        Name(NameCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
    }
    impl ethers::core::abi::AbiDecode for MockATokenCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockATokenCalls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockATokenCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <ApproveInternalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockATokenCalls::ApproveInternal(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockATokenCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) = <BurnCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(MockATokenCalls::Burn(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockATokenCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <DecreaseAllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockATokenCalls::DecreaseAllowance(decoded));
            }
            if let Ok(decoded) =
                <IncreaseAllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockATokenCalls::IncreaseAllowance(decoded));
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(MockATokenCalls::Mint(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(MockATokenCalls::Name(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockATokenCalls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockATokenCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockATokenCalls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockATokenCalls::TransferFrom(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MockATokenCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MockATokenCalls::Allowance(element) => element.encode(),
                MockATokenCalls::Approve(element) => element.encode(),
                MockATokenCalls::ApproveInternal(element) => element.encode(),
                MockATokenCalls::BalanceOf(element) => element.encode(),
                MockATokenCalls::Burn(element) => element.encode(),
                MockATokenCalls::Decimals(element) => element.encode(),
                MockATokenCalls::DecreaseAllowance(element) => element.encode(),
                MockATokenCalls::IncreaseAllowance(element) => element.encode(),
                MockATokenCalls::Mint(element) => element.encode(),
                MockATokenCalls::Name(element) => element.encode(),
                MockATokenCalls::Symbol(element) => element.encode(),
                MockATokenCalls::TotalSupply(element) => element.encode(),
                MockATokenCalls::Transfer(element) => element.encode(),
                MockATokenCalls::TransferFrom(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MockATokenCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MockATokenCalls::Allowance(element) => element.fmt(f),
                MockATokenCalls::Approve(element) => element.fmt(f),
                MockATokenCalls::ApproveInternal(element) => element.fmt(f),
                MockATokenCalls::BalanceOf(element) => element.fmt(f),
                MockATokenCalls::Burn(element) => element.fmt(f),
                MockATokenCalls::Decimals(element) => element.fmt(f),
                MockATokenCalls::DecreaseAllowance(element) => element.fmt(f),
                MockATokenCalls::IncreaseAllowance(element) => element.fmt(f),
                MockATokenCalls::Mint(element) => element.fmt(f),
                MockATokenCalls::Name(element) => element.fmt(f),
                MockATokenCalls::Symbol(element) => element.fmt(f),
                MockATokenCalls::TotalSupply(element) => element.fmt(f),
                MockATokenCalls::Transfer(element) => element.fmt(f),
                MockATokenCalls::TransferFrom(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AllowanceCall> for MockATokenCalls {
        fn from(var: AllowanceCall) -> Self {
            MockATokenCalls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for MockATokenCalls {
        fn from(var: ApproveCall) -> Self {
            MockATokenCalls::Approve(var)
        }
    }
    impl ::std::convert::From<ApproveInternalCall> for MockATokenCalls {
        fn from(var: ApproveInternalCall) -> Self {
            MockATokenCalls::ApproveInternal(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for MockATokenCalls {
        fn from(var: BalanceOfCall) -> Self {
            MockATokenCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BurnCall> for MockATokenCalls {
        fn from(var: BurnCall) -> Self {
            MockATokenCalls::Burn(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for MockATokenCalls {
        fn from(var: DecimalsCall) -> Self {
            MockATokenCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<DecreaseAllowanceCall> for MockATokenCalls {
        fn from(var: DecreaseAllowanceCall) -> Self {
            MockATokenCalls::DecreaseAllowance(var)
        }
    }
    impl ::std::convert::From<IncreaseAllowanceCall> for MockATokenCalls {
        fn from(var: IncreaseAllowanceCall) -> Self {
            MockATokenCalls::IncreaseAllowance(var)
        }
    }
    impl ::std::convert::From<MintCall> for MockATokenCalls {
        fn from(var: MintCall) -> Self {
            MockATokenCalls::Mint(var)
        }
    }
    impl ::std::convert::From<NameCall> for MockATokenCalls {
        fn from(var: NameCall) -> Self {
            MockATokenCalls::Name(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for MockATokenCalls {
        fn from(var: SymbolCall) -> Self {
            MockATokenCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for MockATokenCalls {
        fn from(var: TotalSupplyCall) -> Self {
            MockATokenCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for MockATokenCalls {
        fn from(var: TransferCall) -> Self {
            MockATokenCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for MockATokenCalls {
        fn from(var: TransferFromCall) -> Self {
            MockATokenCalls::TransferFrom(var)
        }
    }
    #[doc = "Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `[221, 98, 237, 62]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct AllowanceReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `approve` function with signature `approve(address,uint256)` and selector `[9, 94, 167, 179]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ApproveReturn(pub bool);
    #[doc = "Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BalanceOfReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `decimals` function with signature `decimals()` and selector `[49, 60, 229, 103]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DecimalsReturn(pub u8);
    #[doc = "Container type for all return fields from the `decreaseAllowance` function with signature `decreaseAllowance(address,uint256)` and selector `[164, 87, 194, 215]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DecreaseAllowanceReturn(pub bool);
    #[doc = "Container type for all return fields from the `increaseAllowance` function with signature `increaseAllowance(address,uint256)` and selector `[57, 80, 147, 81]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IncreaseAllowanceReturn(pub bool);
    #[doc = "Container type for all return fields from the `mint` function with signature `mint(address,uint256,uint256)` and selector `[21, 110, 41, 246]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MintReturn(pub bool);
    #[doc = "Container type for all return fields from the `name` function with signature `name()` and selector `[6, 253, 222, 3]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct NameReturn(pub String);
    #[doc = "Container type for all return fields from the `symbol` function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SymbolReturn(pub String);
    #[doc = "Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TotalSupplyReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `transfer` function with signature `transfer(address,uint256)` and selector `[169, 5, 156, 187]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TransferReturn(pub bool);
    #[doc = "Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `[35, 184, 114, 221]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TransferFromReturn(pub bool);
}
