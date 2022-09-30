pub use erc721::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod erc721 {
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
    #[doc = "ERC721 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ERC721_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"string\",\"name\":\"name_\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"symbol_\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"approved\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ApprovalForAll\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getApproved\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isApprovedForAll\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ownerOf\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"safeTransferFrom\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"safeTransferFrom\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setApprovalForAll\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"interfaceId\",\"type\":\"bytes4\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"supportsInterface\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"tokenURI\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ERC721_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60806040523480156200001157600080fd5b50604051620013a6380380620013a68339810160408190526200003491620001db565b81516200004990600090602085019062000068565b5080516200005f90600190602084019062000068565b50505062000282565b828054620000769062000245565b90600052602060002090601f0160209004810192826200009a5760008555620000e5565b82601f10620000b557805160ff1916838001178555620000e5565b82800160010185558215620000e5579182015b82811115620000e5578251825591602001919060010190620000c8565b50620000f3929150620000f7565b5090565b5b80821115620000f35760008155600101620000f8565b634e487b7160e01b600052604160045260246000fd5b600082601f8301126200013657600080fd5b81516001600160401b03808211156200015357620001536200010e565b604051601f8301601f19908116603f011681019082821181831017156200017e576200017e6200010e565b816040528381526020925086838588010111156200019b57600080fd5b600091505b83821015620001bf5785820183015181830184015290820190620001a0565b83821115620001d15760008385830101525b9695505050505050565b60008060408385031215620001ef57600080fd5b82516001600160401b03808211156200020757600080fd5b620002158683870162000124565b935060208501519150808211156200022c57600080fd5b506200023b8582860162000124565b9150509250929050565b600181811c908216806200025a57607f821691505b602082108114156200027c57634e487b7160e01b600052602260045260246000fd5b50919050565b61111480620002926000396000f3fe608060405234801561001057600080fd5b50600436106100cf5760003560e01c80636352211e1161008c578063a22cb46511610066578063a22cb465146101b3578063b88d4fde146101c6578063c87b56dd146101d9578063e985e9c5146101ec57600080fd5b80636352211e1461017757806370a082311461018a57806395d89b41146101ab57600080fd5b806301ffc9a7146100d457806306fdde03146100fc578063081812fc14610111578063095ea7b31461013c57806323b872dd1461015157806342842e0e14610164575b600080fd5b6100e76100e2366004610c97565b610228565b60405190151581526020015b60405180910390f35b61010461027a565b6040516100f39190610d0c565b61012461011f366004610d1f565b61030c565b6040516001600160a01b0390911681526020016100f3565b61014f61014a366004610d54565b610333565b005b61014f61015f366004610d7e565b61044e565b61014f610172366004610d7e565b61047f565b610124610185366004610d1f565b61049a565b61019d610198366004610dba565b6104fa565b6040519081526020016100f3565b610104610580565b61014f6101c1366004610dd5565b61058f565b61014f6101d4366004610e27565b61059e565b6101046101e7366004610d1f565b6105d6565b6100e76101fa366004610f03565b6001600160a01b03918216600090815260056020908152604080832093909416825291909152205460ff1690565b60006001600160e01b031982166380ac58cd60e01b148061025957506001600160e01b03198216635b5e139f60e01b145b8061027457506301ffc9a760e01b6001600160e01b03198316145b92915050565b60606000805461028990610f36565b80601f01602080910402602001604051908101604052809291908181526020018280546102b590610f36565b80156103025780601f106102d757610100808354040283529160200191610302565b820191906000526020600020905b8154815290600101906020018083116102e557829003601f168201915b5050505050905090565b60006103178261064a565b506000908152600460205260409020546001600160a01b031690565b600061033e8261049a565b9050806001600160a01b0316836001600160a01b031614156103b15760405162461bcd60e51b815260206004820152602160248201527f4552433732313a20617070726f76616c20746f2063757272656e74206f776e656044820152603960f91b60648201526084015b60405180910390fd5b336001600160a01b03821614806103cd57506103cd81336101fa565b61043f5760405162461bcd60e51b815260206004820152603d60248201527f4552433732313a20617070726f76652063616c6c6572206973206e6f7420746f60448201527f6b656e206f776e6572206f7220617070726f76656420666f7220616c6c00000060648201526084016103a8565b61044983836106ac565b505050565b610458338261071a565b6104745760405162461bcd60e51b81526004016103a890610f71565b610449838383610799565b6104498383836040518060200160405280600081525061059e565b6000818152600260205260408120546001600160a01b0316806102745760405162461bcd60e51b8152602060048201526018602482015277115490cdcc8c4e881a5b9d985b1a59081d1bdad95b88125160421b60448201526064016103a8565b60006001600160a01b0382166105645760405162461bcd60e51b815260206004820152602960248201527f4552433732313a2061646472657373207a65726f206973206e6f7420612076616044820152683634b21037bbb732b960b91b60648201526084016103a8565b506001600160a01b031660009081526003602052604090205490565b60606001805461028990610f36565b61059a3383836108fd565b5050565b6105a8338361071a565b6105c45760405162461bcd60e51b81526004016103a890610f71565b6105d0848484846109cc565b50505050565b60606105e18261064a565b60006105f860408051602081019091526000815290565b905060008151116106185760405180602001604052806000815250610643565b80610622846109ff565b604051602001610633929190610fbe565b6040516020818303038152906040525b9392505050565b6000818152600260205260409020546001600160a01b03166106a95760405162461bcd60e51b8152602060048201526018602482015277115490cdcc8c4e881a5b9d985b1a59081d1bdad95b88125160421b60448201526064016103a8565b50565b600081815260046020526040902080546001600160a01b0319166001600160a01b03841690811790915581906106e18261049a565b6001600160a01b03167f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92560405160405180910390a45050565b6000806107268361049a565b9050806001600160a01b0316846001600160a01b0316148061076d57506001600160a01b0380821660009081526005602090815260408083209388168352929052205460ff165b806107915750836001600160a01b03166107868461030c565b6001600160a01b0316145b949350505050565b826001600160a01b03166107ac8261049a565b6001600160a01b0316146107d25760405162461bcd60e51b81526004016103a890610fed565b6001600160a01b0382166108345760405162461bcd60e51b8152602060048201526024808201527f4552433732313a207472616e7366657220746f20746865207a65726f206164646044820152637265737360e01b60648201526084016103a8565b826001600160a01b03166108478261049a565b6001600160a01b03161461086d5760405162461bcd60e51b81526004016103a890610fed565b600081815260046020908152604080832080546001600160a01b03199081169091556001600160a01b0387811680865260038552838620805460001901905590871680865283862080546001019055868652600290945282852080549092168417909155905184937fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef91a4505050565b816001600160a01b0316836001600160a01b0316141561095f5760405162461bcd60e51b815260206004820152601960248201527f4552433732313a20617070726f766520746f2063616c6c65720000000000000060448201526064016103a8565b6001600160a01b03838116600081815260056020908152604080832094871680845294825291829020805460ff191686151590811790915591519182527f17307eab39ab6107e8899845ad3d59bd9653f200f220920489ca2b5937696c31910160405180910390a3505050565b6109d7848484610799565b6109e384848484610a9c565b6105d05760405162461bcd60e51b81526004016103a890611032565b60606000610a0c83610ba9565b600101905060008167ffffffffffffffff811115610a2c57610a2c610e11565b6040519080825280601f01601f191660200182016040528015610a56576020820181803683370190505b5090508181016020015b600019016f181899199a1a9b1b9c1cb0b131b232b360811b600a86061a8153600a8504945084610a8f57610a94565b610a60565b509392505050565b60006001600160a01b0384163b15610b9e57604051630a85bd0160e11b81526001600160a01b0385169063150b7a0290610ae0903390899088908890600401611084565b602060405180830381600087803b158015610afa57600080fd5b505af1925050508015610b2a575060408051601f3d908101601f19168201909252610b27918101906110c1565b60015b610b84573d808015610b58576040519150601f19603f3d011682016040523d82523d6000602084013e610b5d565b606091505b508051610b7c5760405162461bcd60e51b81526004016103a890611032565b805181602001fd5b6001600160e01b031916630a85bd0160e11b149050610791565b506001949350505050565b60008072184f03e93ff9f4daa797ed6e38ed64bf6a1f0160401b8310610be85772184f03e93ff9f4daa797ed6e38ed64bf6a1f0160401b830492506040015b6d04ee2d6d415b85acef81000000008310610c14576d04ee2d6d415b85acef8100000000830492506020015b662386f26fc100008310610c3257662386f26fc10000830492506010015b6305f5e1008310610c4a576305f5e100830492506008015b6127108310610c5e57612710830492506004015b60648310610c70576064830492506002015b600a83106102745760010192915050565b6001600160e01b0319811681146106a957600080fd5b600060208284031215610ca957600080fd5b813561064381610c81565b60005b83811015610ccf578181015183820152602001610cb7565b838111156105d05750506000910152565b60008151808452610cf8816020860160208601610cb4565b601f01601f19169290920160200192915050565b6020815260006106436020830184610ce0565b600060208284031215610d3157600080fd5b5035919050565b80356001600160a01b0381168114610d4f57600080fd5b919050565b60008060408385031215610d6757600080fd5b610d7083610d38565b946020939093013593505050565b600080600060608486031215610d9357600080fd5b610d9c84610d38565b9250610daa60208501610d38565b9150604084013590509250925092565b600060208284031215610dcc57600080fd5b61064382610d38565b60008060408385031215610de857600080fd5b610df183610d38565b915060208301358015158114610e0657600080fd5b809150509250929050565b634e487b7160e01b600052604160045260246000fd5b60008060008060808587031215610e3d57600080fd5b610e4685610d38565b9350610e5460208601610d38565b925060408501359150606085013567ffffffffffffffff80821115610e7857600080fd5b818701915087601f830112610e8c57600080fd5b813581811115610e9e57610e9e610e11565b604051601f8201601f19908116603f01168101908382118183101715610ec657610ec6610e11565b816040528281528a6020848701011115610edf57600080fd5b82602086016020830137600060208483010152809550505050505092959194509250565b60008060408385031215610f1657600080fd5b610f1f83610d38565b9150610f2d60208401610d38565b90509250929050565b600181811c90821680610f4a57607f821691505b60208210811415610f6b57634e487b7160e01b600052602260045260246000fd5b50919050565b6020808252602d908201527f4552433732313a2063616c6c6572206973206e6f7420746f6b656e206f776e6560408201526c1c881bdc88185c1c1c9bdd9959609a1b606082015260800190565b60008351610fd0818460208801610cb4565b835190830190610fe4818360208801610cb4565b01949350505050565b60208082526025908201527f4552433732313a207472616e736665722066726f6d20696e636f72726563742060408201526437bbb732b960d91b606082015260800190565b60208082526032908201527f4552433732313a207472616e7366657220746f206e6f6e20455243373231526560408201527131b2b4bb32b91034b6b83632b6b2b73a32b960711b606082015260800190565b6001600160a01b03858116825284166020820152604081018390526080606082018190526000906110b790830184610ce0565b9695505050505050565b6000602082840312156110d357600080fd5b815161064381610c8156fea2646970667358221220c6f40c43b89b0d15d2b90fbebbf78aa34b877fb4d2212d62c04a42532f9b0b3964736f6c63430008090033" . parse () . expect ("invalid bytecode")
        });
    pub struct ERC721<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ERC721<M> {
        fn clone(&self) -> Self {
            ERC721(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ERC721<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ERC721<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ERC721))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ERC721<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ERC721_ABI.clone(), client).into()
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
                ERC721_ABI.clone(),
                ERC721_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `approve` (0x095ea7b3) function"]
        pub fn approve(
            &self,
            to: ethers::core::types::Address,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 94, 167, 179], (to, token_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOf` (0x70a08231) function"]
        pub fn balance_of(
            &self,
            owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getApproved` (0x081812fc) function"]
        pub fn get_approved(
            &self,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([8, 24, 18, 252], token_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isApprovedForAll` (0xe985e9c5) function"]
        pub fn is_approved_for_all(
            &self,
            owner: ethers::core::types::Address,
            operator: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([233, 133, 233, 197], (owner, operator))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ownerOf` (0x6352211e) function"]
        pub fn owner_of(
            &self,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([99, 82, 33, 30], token_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `safeTransferFrom` (0x42842e0e) function"]
        pub fn safe_transfer_from(
            &self,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 132, 46, 14], (from, to, token_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `safeTransferFrom` (0xb88d4fde) function"]
        pub fn safe_transfer_from_with_data(
            &self,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            token_id: ethers::core::types::U256,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([184, 141, 79, 222], (from, to, token_id, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setApprovalForAll` (0xa22cb465) function"]
        pub fn set_approval_for_all(
            &self,
            operator: ethers::core::types::Address,
            approved: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 44, 180, 101], (operator, approved))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `supportsInterface` (0x01ffc9a7) function"]
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `symbol` (0x95d89b41) function"]
        pub fn symbol(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tokenURI` (0xc87b56dd) function"]
        pub fn token_uri(
            &self,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([200, 123, 86, 221], token_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferFrom` (0x23b872dd) function"]
        pub fn transfer_from(
            &self,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, token_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ApprovalForAll` event"]
        pub fn approval_for_all_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ApprovalForAllFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, ERC721Events> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ERC721<M> {
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
        pub approved: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_id: ethers::core::types::U256,
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
    #[ethevent(name = "ApprovalForAll", abi = "ApprovalForAll(address,address,bool)")]
    pub struct ApprovalForAllFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ethers::core::types::Address,
        pub approved: bool,
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
        #[ethevent(indexed)]
        pub token_id: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ERC721Events {
        ApprovalFilter(ApprovalFilter),
        ApprovalForAllFilter(ApprovalForAllFilter),
        TransferFilter(TransferFilter),
    }
    impl ethers::contract::EthLogDecode for ERC721Events {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(ERC721Events::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(ERC721Events::ApprovalForAllFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(ERC721Events::TransferFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ERC721Events {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ERC721Events::ApprovalFilter(element) => element.fmt(f),
                ERC721Events::ApprovalForAllFilter(element) => element.fmt(f),
                ERC721Events::TransferFilter(element) => element.fmt(f),
            }
        }
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
        pub to: ethers::core::types::Address,
        pub token_id: ethers::core::types::U256,
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
        pub owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getApproved` function with signature `getApproved(uint256)` and selector `[8, 24, 18, 252]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getApproved", abi = "getApproved(uint256)")]
    pub struct GetApprovedCall {
        pub token_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `[233, 133, 233, 197]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "isApprovedForAll", abi = "isApprovedForAll(address,address)")]
    pub struct IsApprovedForAllCall {
        pub owner: ethers::core::types::Address,
        pub operator: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `ownerOf` function with signature `ownerOf(uint256)` and selector `[99, 82, 33, 30]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "ownerOf", abi = "ownerOf(uint256)")]
    pub struct OwnerOfCall {
        pub token_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256)` and selector `[66, 132, 46, 14]`"]
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
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256)"
    )]
    pub struct SafeTransferFromCall {
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub token_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256,bytes)` and selector `[184, 141, 79, 222]`"]
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
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256,bytes)"
    )]
    pub struct SafeTransferFromWithDataCall {
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub token_id: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `setApprovalForAll` function with signature `setApprovalForAll(address,bool)` and selector `[162, 44, 180, 101]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setApprovalForAll", abi = "setApprovalForAll(address,bool)")]
    pub struct SetApprovalForAllCall {
        pub operator: ethers::core::types::Address,
        pub approved: bool,
    }
    #[doc = "Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `[1, 255, 201, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
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
    #[doc = "Container type for all input parameters for the `tokenURI` function with signature `tokenURI(uint256)` and selector `[200, 123, 86, 221]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "tokenURI", abi = "tokenURI(uint256)")]
    pub struct TokenURICall {
        pub token_id: ethers::core::types::U256,
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
        pub token_id: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ERC721Calls {
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        GetApproved(GetApprovedCall),
        IsApprovedForAll(IsApprovedForAllCall),
        Name(NameCall),
        OwnerOf(OwnerOfCall),
        SafeTransferFrom(SafeTransferFromCall),
        SafeTransferFromWithData(SafeTransferFromWithDataCall),
        SetApprovalForAll(SetApprovalForAllCall),
        SupportsInterface(SupportsInterfaceCall),
        Symbol(SymbolCall),
        TokenURI(TokenURICall),
        TransferFrom(TransferFromCall),
    }
    impl ethers::core::abi::AbiDecode for ERC721Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <GetApprovedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::GetApproved(decoded));
            }
            if let Ok(decoded) =
                <IsApprovedForAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::IsApprovedForAll(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ERC721Calls::Name(decoded));
            }
            if let Ok(decoded) =
                <OwnerOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::OwnerOf(decoded));
            }
            if let Ok(decoded) =
                <SafeTransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::SafeTransferFrom(decoded));
            }
            if let Ok(decoded) =
                <SafeTransferFromWithDataCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ERC721Calls::SafeTransferFromWithData(decoded));
            }
            if let Ok(decoded) =
                <SetApprovalForAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::SetApprovalForAll(decoded));
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TokenURICall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::TokenURI(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721Calls::TransferFrom(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ERC721Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                ERC721Calls::Approve(element) => element.encode(),
                ERC721Calls::BalanceOf(element) => element.encode(),
                ERC721Calls::GetApproved(element) => element.encode(),
                ERC721Calls::IsApprovedForAll(element) => element.encode(),
                ERC721Calls::Name(element) => element.encode(),
                ERC721Calls::OwnerOf(element) => element.encode(),
                ERC721Calls::SafeTransferFrom(element) => element.encode(),
                ERC721Calls::SafeTransferFromWithData(element) => element.encode(),
                ERC721Calls::SetApprovalForAll(element) => element.encode(),
                ERC721Calls::SupportsInterface(element) => element.encode(),
                ERC721Calls::Symbol(element) => element.encode(),
                ERC721Calls::TokenURI(element) => element.encode(),
                ERC721Calls::TransferFrom(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ERC721Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ERC721Calls::Approve(element) => element.fmt(f),
                ERC721Calls::BalanceOf(element) => element.fmt(f),
                ERC721Calls::GetApproved(element) => element.fmt(f),
                ERC721Calls::IsApprovedForAll(element) => element.fmt(f),
                ERC721Calls::Name(element) => element.fmt(f),
                ERC721Calls::OwnerOf(element) => element.fmt(f),
                ERC721Calls::SafeTransferFrom(element) => element.fmt(f),
                ERC721Calls::SafeTransferFromWithData(element) => element.fmt(f),
                ERC721Calls::SetApprovalForAll(element) => element.fmt(f),
                ERC721Calls::SupportsInterface(element) => element.fmt(f),
                ERC721Calls::Symbol(element) => element.fmt(f),
                ERC721Calls::TokenURI(element) => element.fmt(f),
                ERC721Calls::TransferFrom(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ApproveCall> for ERC721Calls {
        fn from(var: ApproveCall) -> Self {
            ERC721Calls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for ERC721Calls {
        fn from(var: BalanceOfCall) -> Self {
            ERC721Calls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<GetApprovedCall> for ERC721Calls {
        fn from(var: GetApprovedCall) -> Self {
            ERC721Calls::GetApproved(var)
        }
    }
    impl ::std::convert::From<IsApprovedForAllCall> for ERC721Calls {
        fn from(var: IsApprovedForAllCall) -> Self {
            ERC721Calls::IsApprovedForAll(var)
        }
    }
    impl ::std::convert::From<NameCall> for ERC721Calls {
        fn from(var: NameCall) -> Self {
            ERC721Calls::Name(var)
        }
    }
    impl ::std::convert::From<OwnerOfCall> for ERC721Calls {
        fn from(var: OwnerOfCall) -> Self {
            ERC721Calls::OwnerOf(var)
        }
    }
    impl ::std::convert::From<SafeTransferFromCall> for ERC721Calls {
        fn from(var: SafeTransferFromCall) -> Self {
            ERC721Calls::SafeTransferFrom(var)
        }
    }
    impl ::std::convert::From<SafeTransferFromWithDataCall> for ERC721Calls {
        fn from(var: SafeTransferFromWithDataCall) -> Self {
            ERC721Calls::SafeTransferFromWithData(var)
        }
    }
    impl ::std::convert::From<SetApprovalForAllCall> for ERC721Calls {
        fn from(var: SetApprovalForAllCall) -> Self {
            ERC721Calls::SetApprovalForAll(var)
        }
    }
    impl ::std::convert::From<SupportsInterfaceCall> for ERC721Calls {
        fn from(var: SupportsInterfaceCall) -> Self {
            ERC721Calls::SupportsInterface(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for ERC721Calls {
        fn from(var: SymbolCall) -> Self {
            ERC721Calls::Symbol(var)
        }
    }
    impl ::std::convert::From<TokenURICall> for ERC721Calls {
        fn from(var: TokenURICall) -> Self {
            ERC721Calls::TokenURI(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for ERC721Calls {
        fn from(var: TransferFromCall) -> Self {
            ERC721Calls::TransferFrom(var)
        }
    }
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
    #[doc = "Container type for all return fields from the `getApproved` function with signature `getApproved(uint256)` and selector `[8, 24, 18, 252]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetApprovedReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `[233, 133, 233, 197]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsApprovedForAllReturn(pub bool);
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
    #[doc = "Container type for all return fields from the `ownerOf` function with signature `ownerOf(uint256)` and selector `[99, 82, 33, 30]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct OwnerOfReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `[1, 255, 201, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SupportsInterfaceReturn(pub bool);
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
    #[doc = "Container type for all return fields from the `tokenURI` function with signature `tokenURI(uint256)` and selector `[200, 123, 86, 221]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TokenURIReturn(pub String);
}
