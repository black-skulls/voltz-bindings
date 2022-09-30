pub use mock_c_token::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod mock_c_token {
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
    #[doc = "MockCToken was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static MOCKCTOKEN_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"underlyingAsset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"name\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"symbol\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"accrualBlockNumber\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approveInternal\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"balanceOfUnderlying\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"borrowBalanceCurrent\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrowIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrowRatePerBlock\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"subtractedValue\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"decreaseAllowance\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"exchangeRateCurrent\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"exchangeRateStored\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"addedValue\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"increaseAllowance\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mint\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"redeemAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"redeemUnderlying\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"accrualBlockNumber\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setAccrualBlockNumber\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"borrowIndex\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setBorrowIndex\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"borrowRatePerBlock\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setBorrowRatePerBlock\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"rate\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setExchangeRate\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"supplyRatePerBlock\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setSupplyRatePerBlock\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"supplyRatePerBlock\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"underlying\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MOCKCTOKEN_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60806040523480156200001157600080fd5b50604051620013bf380380620013bf833981016040819052620000349162000203565b8151829082906200004d90600390602085019062000090565b5080516200006390600490602084019062000090565b5050600580546001600160a01b0319166001600160a01b03959095169490941790935550620002ca915050565b8280546200009e906200028d565b90600052602060002090601f016020900481019282620000c257600085556200010d565b82601f10620000dd57805160ff19168380011785556200010d565b828001600101855582156200010d579182015b828111156200010d578251825591602001919060010190620000f0565b506200011b9291506200011f565b5090565b5b808211156200011b576000815560010162000120565b634e487b7160e01b600052604160045260246000fd5b600082601f8301126200015e57600080fd5b81516001600160401b03808211156200017b576200017b62000136565b604051601f8301601f19908116603f01168101908282118183101715620001a657620001a662000136565b81604052838152602092508683858801011115620001c357600080fd5b600091505b83821015620001e75785820183015181830184015290820190620001c8565b83821115620001f95760008385830101525b9695505050505050565b6000806000606084860312156200021957600080fd5b83516001600160a01b03811681146200023157600080fd5b60208501519093506001600160401b03808211156200024f57600080fd5b6200025d878388016200014c565b935060408601519150808211156200027457600080fd5b5062000283868287016200014c565b9150509250925092565b600181811c90821680620002a257607f821691505b60208210811415620002c457634e487b7160e01b600052602260045260246000fd5b50919050565b6110e580620002da6000396000f3fe608060405234801561001057600080fd5b50600436106101c45760003560e01c806370a08231116100f9578063bd6d894d11610097578063cbf4d87711610071578063cbf4d87714610382578063db068e0e14610395578063dd62ed3e146103a8578063f8f9da28146103bb57600080fd5b8063bd6d894d14610234578063c5a6c9431461035c578063c7437e211461036f57600080fd5b8063a457c2d7116100d3578063a457c2d714610326578063a9059cbb14610339578063aa5af0fd1461034c578063ae9d70b01461035457600080fd5b806370a08231146102e2578063852a12e31461030b57806395d89b411461031e57600080fd5b806337d9bd921161016657806340c10f191161014057806340c10f191461029957806356189cb4146102ac5780636c540baf146102bf5780636f307dc3146102c757600080fd5b806337d9bd921461025e57806339509351146102735780633af9e6691461028657600080fd5b806318160ddd116101a257806318160ddd1461022c578063182df0f51461023457806323b872dd1461023c578063313ce5671461024f57600080fd5b806306fdde03146101c9578063095ea7b3146101e757806317bfdfbc1461020a575b600080fd5b6101d16103c3565b6040516101de9190610e75565b60405180910390f35b6101fa6101f5366004610ec4565b610455565b60405190151581526020016101de565b61021e610218366004610eee565b50600090565b6040519081526020016101de565b60025461021e565b60065461021e565b6101fa61024a366004610f09565b61046d565b604051601281526020016101de565b61027161026c366004610f45565b600955565b005b6101fa610281366004610ec4565b610493565b61021e610294366004610eee565b6104b5565b6101fa6102a7366004610ec4565b6104fc565b6102716102ba366004610f09565b6105a1565b600a5461021e565b6005546040516001600160a01b0390911681526020016101de565b61021e6102f0366004610eee565b6001600160a01b031660009081526020819052604090205490565b61021e610319366004610f45565b6105b1565b6101d1610607565b6101fa610334366004610ec4565b610616565b6101fa610347366004610ec4565b61069c565b60095461021e565b60075461021e565b61027161036a366004610f45565b600a55565b61027161037d366004610f45565b600755565b610271610390366004610f45565b600855565b6102716103a3366004610f45565b600655565b61021e6103b6366004610f5e565b6106aa565b60085461021e565b6060600380546103d290610f91565b80601f01602080910402602001604051908101604052809291908181526020018280546103fe90610f91565b801561044b5780601f106104205761010080835404028352916020019161044b565b820191906000526020600020905b81548152906001019060200180831161042e57829003601f168201915b5050505050905090565b6000336104638185856106d5565b5060019392505050565b60003361047b8582856107f9565b610486858585610873565b60019150505b9392505050565b6000336104638185856104a683836106aa565b6104b09190610fe2565b6106d5565b6000670de0b6b3a76400006104c960065490565b6001600160a01b0384166000908152602081905260409020546104ec9190610ffa565b6104f69190611019565b92915050565b6001600160a01b038216600090815260208190526040812054826105605760405162461bcd60e51b815260206004820152601660248201527510d517d253959053125117d352539517d05353d5539560521b60448201526064015b60405180910390fd5b61056a8484610a05565b6040518381526001600160a01b038516906000906000805160206110908339815191529060200160405180910390a3159392505050565b6105ac8383836106d5565b505050565b6000806105c960065484610ab290919063ffffffff16565b336000818152602081905260409020549192506105e69082610ac0565b6005546105fd906001600160a01b03163386610be0565b5060009392505050565b6060600480546103d290610f91565b6000338161062482866106aa565b9050838110156106845760405162461bcd60e51b815260206004820152602560248201527f45524332303a2064656372656173656420616c6c6f77616e63652062656c6f77604482015264207a65726f60d81b6064820152608401610557565b61069182868684036106d5565b506001949350505050565b600033610463818585610873565b6001600160a01b03918216600090815260016020908152604080832093909416825291909152205490565b6001600160a01b0383166107375760405162461bcd60e51b8152602060048201526024808201527f45524332303a20617070726f76652066726f6d20746865207a65726f206164646044820152637265737360e01b6064820152608401610557565b6001600160a01b0382166107985760405162461bcd60e51b815260206004820152602260248201527f45524332303a20617070726f766520746f20746865207a65726f206164647265604482015261737360f01b6064820152608401610557565b6001600160a01b0383811660008181526001602090815260408083209487168084529482529182902085905590518481527f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925910160405180910390a3505050565b600061080584846106aa565b9050600019811461086d57818110156108605760405162461bcd60e51b815260206004820152601d60248201527f45524332303a20696e73756666696369656e7420616c6c6f77616e63650000006044820152606401610557565b61086d84848484036106d5565b50505050565b6001600160a01b0383166108d75760405162461bcd60e51b815260206004820152602560248201527f45524332303a207472616e736665722066726f6d20746865207a65726f206164604482015264647265737360d81b6064820152608401610557565b6001600160a01b0382166109395760405162461bcd60e51b815260206004820152602360248201527f45524332303a207472616e7366657220746f20746865207a65726f206164647260448201526265737360e81b6064820152608401610557565b6001600160a01b038316600090815260208190526040902054818110156109b15760405162461bcd60e51b815260206004820152602660248201527f45524332303a207472616e7366657220616d6f756e7420657863656564732062604482015265616c616e636560d01b6064820152608401610557565b6001600160a01b0384811660008181526020818152604080832087870390559387168083529184902080548701905592518581529092600080516020611090833981519152910160405180910390a361086d565b6001600160a01b038216610a5b5760405162461bcd60e51b815260206004820152601f60248201527f45524332303a206d696e7420746f20746865207a65726f2061646472657373006044820152606401610557565b8060026000828254610a6d9190610fe2565b90915550506001600160a01b03821660008181526020818152604080832080548601905551848152600080516020611090833981519152910160405180910390a35050565b600061048c83836000610c32565b6001600160a01b038216610b205760405162461bcd60e51b815260206004820152602160248201527f45524332303a206275726e2066726f6d20746865207a65726f206164647265736044820152607360f81b6064820152608401610557565b6001600160a01b03821660009081526020819052604090205481811015610b945760405162461bcd60e51b815260206004820152602260248201527f45524332303a206275726e20616d6f756e7420657863656564732062616c616e604482015261636560f01b6064820152608401610557565b6001600160a01b038316600081815260208181526040808320868603905560028054879003905551858152919291600080516020611090833981519152910160405180910390a3505050565b604080516001600160a01b038416602482015260448082018490528251808303909101815260649091019091526020810180516001600160e01b031663a9059cbb60e01b1790526105ac908490610cd7565b600082610c6a5760405162461bcd60e51b8152600401610557906020808252600490820152630444956360e41b604082015260600190565b6000610c77600285611019565b905083816001856001811115610c8f57610c8f61103b565b14610ca257670de0b6b3a7640000610cb0565b6b033b2e3c9fd0803ce80000005b610cba9088610ffa565b610cc49190610fe2565b610cce9190611019565b95945050505050565b6000610d0383836040518060400160405280600781526020016629aa261032b93960c91b815250610d58565b8051909150156105ac5780806020019051810190610d219190611051565b6105ac5760405162461bcd60e51b815260206004820152600860248201526714d5130819985a5b60c21b6044820152606401610557565b6060833b610d975760405162461bcd60e51b815260206004820152600c60248201526b1b9bdb8b58dbdb9d1c9858dd60a21b6044820152606401610557565b600080856001600160a01b0316600086604051610db49190611073565b60006040518083038185875af1925050503d8060008114610df1576040519150601f19603f3d011682016040523d82523d6000602084013e610df6565b606091505b5091509150610e06828286610e10565b9695505050505050565b60608315610e1f57508161048c565b825115610e2f5782518084602001fd5b8160405162461bcd60e51b81526004016105579190610e75565b60005b83811015610e64578181015183820152602001610e4c565b8381111561086d5750506000910152565b6020815260008251806020840152610e94816040850160208701610e49565b601f01601f19169190910160400192915050565b80356001600160a01b0381168114610ebf57600080fd5b919050565b60008060408385031215610ed757600080fd5b610ee083610ea8565b946020939093013593505050565b600060208284031215610f0057600080fd5b61048c82610ea8565b600080600060608486031215610f1e57600080fd5b610f2784610ea8565b9250610f3560208501610ea8565b9150604084013590509250925092565b600060208284031215610f5757600080fd5b5035919050565b60008060408385031215610f7157600080fd5b610f7a83610ea8565b9150610f8860208401610ea8565b90509250929050565b600181811c90821680610fa557607f821691505b60208210811415610fc657634e487b7160e01b600052602260045260246000fd5b50919050565b634e487b7160e01b600052601160045260246000fd5b60008219821115610ff557610ff5610fcc565b500190565b600081600019048311821515161561101457611014610fcc565b500290565b60008261103657634e487b7160e01b600052601260045260246000fd5b500490565b634e487b7160e01b600052602160045260246000fd5b60006020828403121561106357600080fd5b8151801515811461048c57600080fd5b60008251611085818460208701610e49565b919091019291505056feddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa2646970667358221220871241cc5dce0d9bf61d00a32d7ef275a8435bad2d724f5e1505646934c6856b64736f6c63430008090033" . parse () . expect ("invalid bytecode")
        });
    pub struct MockCToken<M>(ethers::contract::Contract<M>);
    impl<M> Clone for MockCToken<M> {
        fn clone(&self) -> Self {
            MockCToken(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for MockCToken<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for MockCToken<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MockCToken))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> MockCToken<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), MOCKCTOKEN_ABI.clone(), client).into()
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
                MOCKCTOKEN_ABI.clone(),
                MOCKCTOKEN_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `accrualBlockNumber` (0x6c540baf) function"]
        pub fn accrual_block_number(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([108, 84, 11, 175], ())
                .expect("method not found (this should never happen)")
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
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOfUnderlying` (0x3af9e669) function"]
        pub fn balance_of_underlying(
            &self,
            owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([58, 249, 230, 105], owner)
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
        #[doc = "Calls the contract's `mint` (0x40c10f19) function"]
        pub fn mint(
            &self,
            user: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([64, 193, 15, 25], (user, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
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
        #[doc = "Calls the contract's `setAccrualBlockNumber` (0xc5a6c943) function"]
        pub fn set_accrual_block_number(
            &self,
            accrual_block_number: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([197, 166, 201, 67], accrual_block_number)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setBorrowIndex` (0x37d9bd92) function"]
        pub fn set_borrow_index(
            &self,
            borrow_index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([55, 217, 189, 146], borrow_index)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setBorrowRatePerBlock` (0xcbf4d877) function"]
        pub fn set_borrow_rate_per_block(
            &self,
            borrow_rate_per_block: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([203, 244, 216, 119], borrow_rate_per_block)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setExchangeRate` (0xdb068e0e) function"]
        pub fn set_exchange_rate(
            &self,
            rate: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([219, 6, 142, 14], rate)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setSupplyRatePerBlock` (0xc7437e21) function"]
        pub fn set_supply_rate_per_block(
            &self,
            supply_rate_per_block: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([199, 67, 126, 33], supply_rate_per_block)
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
        #[doc = "Calls the contract's `underlying` (0x6f307dc3) function"]
        pub fn underlying(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([111, 48, 125, 195], ())
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, MockCTokenEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for MockCToken<M> {
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
    pub enum MockCTokenEvents {
        ApprovalFilter(ApprovalFilter),
        TransferFilter(TransferFilter),
    }
    impl ethers::contract::EthLogDecode for MockCTokenEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(MockCTokenEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(MockCTokenEvents::TransferFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for MockCTokenEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MockCTokenEvents::ApprovalFilter(element) => element.fmt(f),
                MockCTokenEvents::TransferFilter(element) => element.fmt(f),
            }
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
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `balanceOfUnderlying` function with signature `balanceOfUnderlying(address)` and selector `[58, 249, 230, 105]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "balanceOfUnderlying", abi = "balanceOfUnderlying(address)")]
    pub struct BalanceOfUnderlyingCall {
        pub owner: ethers::core::types::Address,
    }
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
    #[doc = "Container type for all input parameters for the `mint` function with signature `mint(address,uint256)` and selector `[64, 193, 15, 25]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "mint", abi = "mint(address,uint256)")]
    pub struct MintCall {
        pub user: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `setAccrualBlockNumber` function with signature `setAccrualBlockNumber(uint256)` and selector `[197, 166, 201, 67]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setAccrualBlockNumber", abi = "setAccrualBlockNumber(uint256)")]
    pub struct SetAccrualBlockNumberCall {
        pub accrual_block_number: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setBorrowIndex` function with signature `setBorrowIndex(uint256)` and selector `[55, 217, 189, 146]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setBorrowIndex", abi = "setBorrowIndex(uint256)")]
    pub struct SetBorrowIndexCall {
        pub borrow_index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setBorrowRatePerBlock` function with signature `setBorrowRatePerBlock(uint256)` and selector `[203, 244, 216, 119]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setBorrowRatePerBlock", abi = "setBorrowRatePerBlock(uint256)")]
    pub struct SetBorrowRatePerBlockCall {
        pub borrow_rate_per_block: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setExchangeRate` function with signature `setExchangeRate(uint256)` and selector `[219, 6, 142, 14]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setExchangeRate", abi = "setExchangeRate(uint256)")]
    pub struct SetExchangeRateCall {
        pub rate: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setSupplyRatePerBlock` function with signature `setSupplyRatePerBlock(uint256)` and selector `[199, 67, 126, 33]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setSupplyRatePerBlock", abi = "setSupplyRatePerBlock(uint256)")]
    pub struct SetSupplyRatePerBlockCall {
        pub supply_rate_per_block: ethers::core::types::U256,
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
    pub enum MockCTokenCalls {
        AccrualBlockNumber(AccrualBlockNumberCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        ApproveInternal(ApproveInternalCall),
        BalanceOf(BalanceOfCall),
        BalanceOfUnderlying(BalanceOfUnderlyingCall),
        BorrowBalanceCurrent(BorrowBalanceCurrentCall),
        BorrowIndex(BorrowIndexCall),
        BorrowRatePerBlock(BorrowRatePerBlockCall),
        Decimals(DecimalsCall),
        DecreaseAllowance(DecreaseAllowanceCall),
        ExchangeRateCurrent(ExchangeRateCurrentCall),
        ExchangeRateStored(ExchangeRateStoredCall),
        IncreaseAllowance(IncreaseAllowanceCall),
        Mint(MintCall),
        Name(NameCall),
        RedeemUnderlying(RedeemUnderlyingCall),
        SetAccrualBlockNumber(SetAccrualBlockNumberCall),
        SetBorrowIndex(SetBorrowIndexCall),
        SetBorrowRatePerBlock(SetBorrowRatePerBlockCall),
        SetExchangeRate(SetExchangeRateCall),
        SetSupplyRatePerBlock(SetSupplyRatePerBlockCall),
        SupplyRatePerBlock(SupplyRatePerBlockCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        Underlying(UnderlyingCall),
    }
    impl ethers::core::abi::AbiDecode for MockCTokenCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AccrualBlockNumberCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockCTokenCalls::AccrualBlockNumber(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockCTokenCalls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockCTokenCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <ApproveInternalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockCTokenCalls::ApproveInternal(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockCTokenCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfUnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockCTokenCalls::BalanceOfUnderlying(decoded));
            }
            if let Ok(decoded) =
                <BorrowBalanceCurrentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockCTokenCalls::BorrowBalanceCurrent(decoded));
            }
            if let Ok(decoded) =
                <BorrowIndexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockCTokenCalls::BorrowIndex(decoded));
            }
            if let Ok(decoded) =
                <BorrowRatePerBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockCTokenCalls::BorrowRatePerBlock(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockCTokenCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <DecreaseAllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockCTokenCalls::DecreaseAllowance(decoded));
            }
            if let Ok(decoded) =
                <ExchangeRateCurrentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockCTokenCalls::ExchangeRateCurrent(decoded));
            }
            if let Ok(decoded) =
                <ExchangeRateStoredCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockCTokenCalls::ExchangeRateStored(decoded));
            }
            if let Ok(decoded) =
                <IncreaseAllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockCTokenCalls::IncreaseAllowance(decoded));
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(MockCTokenCalls::Mint(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(MockCTokenCalls::Name(decoded));
            }
            if let Ok(decoded) =
                <RedeemUnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockCTokenCalls::RedeemUnderlying(decoded));
            }
            if let Ok(decoded) =
                <SetAccrualBlockNumberCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockCTokenCalls::SetAccrualBlockNumber(decoded));
            }
            if let Ok(decoded) =
                <SetBorrowIndexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockCTokenCalls::SetBorrowIndex(decoded));
            }
            if let Ok(decoded) =
                <SetBorrowRatePerBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockCTokenCalls::SetBorrowRatePerBlock(decoded));
            }
            if let Ok(decoded) =
                <SetExchangeRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockCTokenCalls::SetExchangeRate(decoded));
            }
            if let Ok(decoded) =
                <SetSupplyRatePerBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockCTokenCalls::SetSupplyRatePerBlock(decoded));
            }
            if let Ok(decoded) =
                <SupplyRatePerBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockCTokenCalls::SupplyRatePerBlock(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockCTokenCalls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockCTokenCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockCTokenCalls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockCTokenCalls::TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <UnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockCTokenCalls::Underlying(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MockCTokenCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MockCTokenCalls::AccrualBlockNumber(element) => element.encode(),
                MockCTokenCalls::Allowance(element) => element.encode(),
                MockCTokenCalls::Approve(element) => element.encode(),
                MockCTokenCalls::ApproveInternal(element) => element.encode(),
                MockCTokenCalls::BalanceOf(element) => element.encode(),
                MockCTokenCalls::BalanceOfUnderlying(element) => element.encode(),
                MockCTokenCalls::BorrowBalanceCurrent(element) => element.encode(),
                MockCTokenCalls::BorrowIndex(element) => element.encode(),
                MockCTokenCalls::BorrowRatePerBlock(element) => element.encode(),
                MockCTokenCalls::Decimals(element) => element.encode(),
                MockCTokenCalls::DecreaseAllowance(element) => element.encode(),
                MockCTokenCalls::ExchangeRateCurrent(element) => element.encode(),
                MockCTokenCalls::ExchangeRateStored(element) => element.encode(),
                MockCTokenCalls::IncreaseAllowance(element) => element.encode(),
                MockCTokenCalls::Mint(element) => element.encode(),
                MockCTokenCalls::Name(element) => element.encode(),
                MockCTokenCalls::RedeemUnderlying(element) => element.encode(),
                MockCTokenCalls::SetAccrualBlockNumber(element) => element.encode(),
                MockCTokenCalls::SetBorrowIndex(element) => element.encode(),
                MockCTokenCalls::SetBorrowRatePerBlock(element) => element.encode(),
                MockCTokenCalls::SetExchangeRate(element) => element.encode(),
                MockCTokenCalls::SetSupplyRatePerBlock(element) => element.encode(),
                MockCTokenCalls::SupplyRatePerBlock(element) => element.encode(),
                MockCTokenCalls::Symbol(element) => element.encode(),
                MockCTokenCalls::TotalSupply(element) => element.encode(),
                MockCTokenCalls::Transfer(element) => element.encode(),
                MockCTokenCalls::TransferFrom(element) => element.encode(),
                MockCTokenCalls::Underlying(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MockCTokenCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MockCTokenCalls::AccrualBlockNumber(element) => element.fmt(f),
                MockCTokenCalls::Allowance(element) => element.fmt(f),
                MockCTokenCalls::Approve(element) => element.fmt(f),
                MockCTokenCalls::ApproveInternal(element) => element.fmt(f),
                MockCTokenCalls::BalanceOf(element) => element.fmt(f),
                MockCTokenCalls::BalanceOfUnderlying(element) => element.fmt(f),
                MockCTokenCalls::BorrowBalanceCurrent(element) => element.fmt(f),
                MockCTokenCalls::BorrowIndex(element) => element.fmt(f),
                MockCTokenCalls::BorrowRatePerBlock(element) => element.fmt(f),
                MockCTokenCalls::Decimals(element) => element.fmt(f),
                MockCTokenCalls::DecreaseAllowance(element) => element.fmt(f),
                MockCTokenCalls::ExchangeRateCurrent(element) => element.fmt(f),
                MockCTokenCalls::ExchangeRateStored(element) => element.fmt(f),
                MockCTokenCalls::IncreaseAllowance(element) => element.fmt(f),
                MockCTokenCalls::Mint(element) => element.fmt(f),
                MockCTokenCalls::Name(element) => element.fmt(f),
                MockCTokenCalls::RedeemUnderlying(element) => element.fmt(f),
                MockCTokenCalls::SetAccrualBlockNumber(element) => element.fmt(f),
                MockCTokenCalls::SetBorrowIndex(element) => element.fmt(f),
                MockCTokenCalls::SetBorrowRatePerBlock(element) => element.fmt(f),
                MockCTokenCalls::SetExchangeRate(element) => element.fmt(f),
                MockCTokenCalls::SetSupplyRatePerBlock(element) => element.fmt(f),
                MockCTokenCalls::SupplyRatePerBlock(element) => element.fmt(f),
                MockCTokenCalls::Symbol(element) => element.fmt(f),
                MockCTokenCalls::TotalSupply(element) => element.fmt(f),
                MockCTokenCalls::Transfer(element) => element.fmt(f),
                MockCTokenCalls::TransferFrom(element) => element.fmt(f),
                MockCTokenCalls::Underlying(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AccrualBlockNumberCall> for MockCTokenCalls {
        fn from(var: AccrualBlockNumberCall) -> Self {
            MockCTokenCalls::AccrualBlockNumber(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for MockCTokenCalls {
        fn from(var: AllowanceCall) -> Self {
            MockCTokenCalls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for MockCTokenCalls {
        fn from(var: ApproveCall) -> Self {
            MockCTokenCalls::Approve(var)
        }
    }
    impl ::std::convert::From<ApproveInternalCall> for MockCTokenCalls {
        fn from(var: ApproveInternalCall) -> Self {
            MockCTokenCalls::ApproveInternal(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for MockCTokenCalls {
        fn from(var: BalanceOfCall) -> Self {
            MockCTokenCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BalanceOfUnderlyingCall> for MockCTokenCalls {
        fn from(var: BalanceOfUnderlyingCall) -> Self {
            MockCTokenCalls::BalanceOfUnderlying(var)
        }
    }
    impl ::std::convert::From<BorrowBalanceCurrentCall> for MockCTokenCalls {
        fn from(var: BorrowBalanceCurrentCall) -> Self {
            MockCTokenCalls::BorrowBalanceCurrent(var)
        }
    }
    impl ::std::convert::From<BorrowIndexCall> for MockCTokenCalls {
        fn from(var: BorrowIndexCall) -> Self {
            MockCTokenCalls::BorrowIndex(var)
        }
    }
    impl ::std::convert::From<BorrowRatePerBlockCall> for MockCTokenCalls {
        fn from(var: BorrowRatePerBlockCall) -> Self {
            MockCTokenCalls::BorrowRatePerBlock(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for MockCTokenCalls {
        fn from(var: DecimalsCall) -> Self {
            MockCTokenCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<DecreaseAllowanceCall> for MockCTokenCalls {
        fn from(var: DecreaseAllowanceCall) -> Self {
            MockCTokenCalls::DecreaseAllowance(var)
        }
    }
    impl ::std::convert::From<ExchangeRateCurrentCall> for MockCTokenCalls {
        fn from(var: ExchangeRateCurrentCall) -> Self {
            MockCTokenCalls::ExchangeRateCurrent(var)
        }
    }
    impl ::std::convert::From<ExchangeRateStoredCall> for MockCTokenCalls {
        fn from(var: ExchangeRateStoredCall) -> Self {
            MockCTokenCalls::ExchangeRateStored(var)
        }
    }
    impl ::std::convert::From<IncreaseAllowanceCall> for MockCTokenCalls {
        fn from(var: IncreaseAllowanceCall) -> Self {
            MockCTokenCalls::IncreaseAllowance(var)
        }
    }
    impl ::std::convert::From<MintCall> for MockCTokenCalls {
        fn from(var: MintCall) -> Self {
            MockCTokenCalls::Mint(var)
        }
    }
    impl ::std::convert::From<NameCall> for MockCTokenCalls {
        fn from(var: NameCall) -> Self {
            MockCTokenCalls::Name(var)
        }
    }
    impl ::std::convert::From<RedeemUnderlyingCall> for MockCTokenCalls {
        fn from(var: RedeemUnderlyingCall) -> Self {
            MockCTokenCalls::RedeemUnderlying(var)
        }
    }
    impl ::std::convert::From<SetAccrualBlockNumberCall> for MockCTokenCalls {
        fn from(var: SetAccrualBlockNumberCall) -> Self {
            MockCTokenCalls::SetAccrualBlockNumber(var)
        }
    }
    impl ::std::convert::From<SetBorrowIndexCall> for MockCTokenCalls {
        fn from(var: SetBorrowIndexCall) -> Self {
            MockCTokenCalls::SetBorrowIndex(var)
        }
    }
    impl ::std::convert::From<SetBorrowRatePerBlockCall> for MockCTokenCalls {
        fn from(var: SetBorrowRatePerBlockCall) -> Self {
            MockCTokenCalls::SetBorrowRatePerBlock(var)
        }
    }
    impl ::std::convert::From<SetExchangeRateCall> for MockCTokenCalls {
        fn from(var: SetExchangeRateCall) -> Self {
            MockCTokenCalls::SetExchangeRate(var)
        }
    }
    impl ::std::convert::From<SetSupplyRatePerBlockCall> for MockCTokenCalls {
        fn from(var: SetSupplyRatePerBlockCall) -> Self {
            MockCTokenCalls::SetSupplyRatePerBlock(var)
        }
    }
    impl ::std::convert::From<SupplyRatePerBlockCall> for MockCTokenCalls {
        fn from(var: SupplyRatePerBlockCall) -> Self {
            MockCTokenCalls::SupplyRatePerBlock(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for MockCTokenCalls {
        fn from(var: SymbolCall) -> Self {
            MockCTokenCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for MockCTokenCalls {
        fn from(var: TotalSupplyCall) -> Self {
            MockCTokenCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for MockCTokenCalls {
        fn from(var: TransferCall) -> Self {
            MockCTokenCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for MockCTokenCalls {
        fn from(var: TransferFromCall) -> Self {
            MockCTokenCalls::TransferFrom(var)
        }
    }
    impl ::std::convert::From<UnderlyingCall> for MockCTokenCalls {
        fn from(var: UnderlyingCall) -> Self {
            MockCTokenCalls::Underlying(var)
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
    #[doc = "Container type for all return fields from the `balanceOfUnderlying` function with signature `balanceOfUnderlying(address)` and selector `[58, 249, 230, 105]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BalanceOfUnderlyingReturn(pub ethers::core::types::U256);
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
    #[doc = "Container type for all return fields from the `mint` function with signature `mint(address,uint256)` and selector `[64, 193, 15, 25]`"]
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
