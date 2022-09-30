pub use margin_engine_emergency::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod margin_engine_emergency {
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
    #[doc = "MarginEngineEmergency was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static MARGINENGINEEMERGENCY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyOwnerCanUpdatePosition\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AdminChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"beacon\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"BeaconUpgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"version\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Initialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"implementation\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Upgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MAX_CACHE_MAX_AGE_IN_SECONDS\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MAX_LIQUIDATOR_REWARD_WAD\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MAX_LOOKBACK_WINDOW_IN_SECONDS\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MIN_LOOKBACK_WINDOW_IN_SECONDS\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ONE\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ONE_UINT\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"SECONDS_IN_YEAR\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"cacheMaxAgeInSeconds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"emergencyWithdrawal\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"factory\",\"outputs\":[{\"internalType\":\"contract IFactory\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"fcm\",\"outputs\":[{\"internalType\":\"contract IFCM\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isAlpha\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"liquidatorRewardWad\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lookbackWindowInSeconds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"paused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"proxiableUUID\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"rateOracle\",\"outputs\":[{\"internalType\":\"contract IRateOracle\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"termEndTimestampWad\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"termStartTimestampWad\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"underlyingToken\",\"outputs\":[{\"internalType\":\"contract IERC20Minimal\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newImplementation\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"upgradeTo\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newImplementation\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"upgradeToAndCall\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"vamm\",\"outputs\":[{\"internalType\":\"contract IVAMM\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MARGINENGINEEMERGENCY_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60a06040523060805234801561001457600080fd5b5060805161157d61004c60003960008181610457015281816104a001528181610540015281816105800152610613015261157d6000f3fe6080604052600436106101815760003560e01c806388428752116100d1578063c2ee3a081161008a578063e087caf111610064578063e087caf1146103e1578063e098372c146103f6578063f2fde38b14610414578063f907bd6d1461043457600080fd5b8063c2ee3a0814610297578063c45a0155146103a3578063cdcae73d146103c157600080fd5b806388428752146103105780638da5cb5b1461032857806393edb4541461034657806398f4b1b21461035b5780639cbff18814610379578063b623f5191461038e57600080fd5b80635dcc93911161013e578063652c30b711610118578063652c30b7146102b3578063715018a6146102c857806386b127ee146102dd57806387e16303146102f457600080fd5b80635dcc9391146102625780635f6a3e0c1461028157806363f573811461029757600080fd5b806322d23b21146101865780632495a599146101bd5780633659cfe6146101db5780634f1ef286146101fd57806352d1902d146102105780635c975abb14610233575b600080fd5b34801561019257600080fd5b506004546001600160a01b03165b6040516001600160a01b0390911681526020015b60405180910390f35b3480156101c957600080fd5b506001546001600160a01b03166101a0565b3480156101e757600080fd5b506101fb6101f63660046111bc565b61044c565b005b6101fb61020b3660046111ed565b610535565b34801561021c57600080fd5b50610225610606565b6040519081526020016101b4565b34801561023f57600080fd5b50601f5461025290610100900460ff1681565b60405190151581526020016101b4565b34801561026e57600080fd5b506102256a1a1601fc4ea7109e00000081565b34801561028d57600080fd5b50610225610e1081565b3480156102a357600080fd5b50610225670de0b6b3a764000081565b3480156102bf57600080fd5b50600254610225565b3480156102d457600080fd5b506101fb6106b9565b3480156102e957600080fd5b506102256212750081565b34801561030057600080fd5b50610225670429d069189e000081565b34801561031c57600080fd5b50601f5460ff16610252565b34801561033457600080fd5b506098546001600160a01b03166101a0565b34801561035257600080fd5b50600354610225565b34801561036757600080fd5b50600c546001600160a01b03166101a0565b34801561038557600080fd5b50600754610225565b34801561039a57600080fd5b50600a54610225565b3480156103af57600080fd5b50600b546001600160a01b03166101a0565b3480156103cd57600080fd5b506101fb6103dc3660046112c1565b6106cd565b3480156103ed57600080fd5b50600054610225565b34801561040257600080fd5b506006546001600160a01b03166101a0565b34801561042057600080fd5b506101fb61042f3660046111bc565b6107fd565b34801561044057600080fd5b506102256312cc030081565b306001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016141561049e5760405162461bcd60e51b815260040161049590611304565b60405180910390fd5b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166104e7600080516020611501833981519152546001600160a01b031690565b6001600160a01b03161461050d5760405162461bcd60e51b815260040161049590611350565b61051681610873565b604080516000808252602082019092526105329183919061087b565b50565b306001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016141561057e5760405162461bcd60e51b815260040161049590611304565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166105c7600080516020611501833981519152546001600160a01b031690565b6001600160a01b0316146105ed5760405162461bcd60e51b815260040161049590611350565b6105f682610873565b6106028282600161087b565b5050565b6000306001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146106a65760405162461bcd60e51b815260206004820152603860248201527f555550535570677261646561626c653a206d757374206e6f742062652063616c60448201527f6c6564207468726f7567682064656c656761746563616c6c00000000000000006064820152608401610495565b5060008051602061150183398151915290565b6106c16109fa565b6106cb6000610a54565b565b6001600160a01b0383166107085760405162461bcd60e51b815260206004820152600260248201526104f360f41b6044820152606401610495565b60006107176005858585610aa6565b90506001600160a01b03841633148015906107b25750600b546040516351c4bc1f60e11b81526001600160a01b0386811660048301523360248301529091169063a389783e9060440160206040518083038186803b15801561077857600080fd5b505afa15801561078c573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107b0919061139c565b155b156107d057604051637da45ce760e01b815260040160405180910390fd5b600081600101546107e0906113d4565b90506107ec8282610b16565b6107f63382610b33565b5050505050565b6108056109fa565b6001600160a01b03811661086a5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610495565b61053281610a54565b6105326109fa565b7f4910fdfa16fed3260ed0e7147f7cc6da11a60208b5b9406d12a635614ffd91435460ff16156108b3576108ae83610c93565b505050565b826001600160a01b03166352d1902d6040518163ffffffff1660e01b815260040160206040518083038186803b1580156108ec57600080fd5b505afa92505050801561091c575060408051601f3d908101601f19168201909252610919918101906113f1565b60015b61097f5760405162461bcd60e51b815260206004820152602e60248201527f45524331393637557067726164653a206e657720696d706c656d656e7461746960448201526d6f6e206973206e6f74205555505360901b6064820152608401610495565b60008051602061150183398151915281146109ee5760405162461bcd60e51b815260206004820152602960248201527f45524331393637557067726164653a20756e737570706f727465642070726f786044820152681a58589b195555525160ba1b6064820152608401610495565b506108ae838383610d2f565b6098546001600160a01b031633146106cb5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610495565b609880546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b6000610ab28383610d54565b6040516bffffffffffffffffffffffff19606086901b16602082015260e884811b603483015283901b60378201528590600090603a016040516020818303038152906040528051906020012081526020019081526020016000209050949350505050565b80826001016000828254610b2a919061140a565b90915550505050565b6000811315610b5457600154610602906001600160a01b0316833084610e15565b6001546040516370a0823160e01b81523060048201526000916001600160a01b0316906370a082319060240160206040518083038186803b158015610b9857600080fd5b505afa158015610bac573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610bd091906113f1565b9050600082900381811115610c76578115610c0957610bef828261144b565b600154909150610c09906001600160a01b03168584610e80565b600480546040516318399f4d60e31b81526001600160a01b03878116938201939093526024810184905291169063c1ccfa6890604401600060405180830381600087803b158015610c5957600080fd5b505af1158015610c6d573d6000803e3d6000fd5b50505050610c8d565b600154610c8d906001600160a01b03168583610e80565b50505050565b6001600160a01b0381163b610d005760405162461bcd60e51b815260206004820152602d60248201527f455243313936373a206e657720696d706c656d656e746174696f6e206973206e60448201526c1bdd08184818dbdb9d1c9858dd609a1b6064820152608401610495565b60008051602061150183398151915280546001600160a01b0319166001600160a01b0392909216919091179055565b610d3883610eb0565b600082511180610d455750805b156108ae57610c8d8383610ef0565b8060020b8260020b12610d8f5760405162461bcd60e51b8152602060048201526003602482015262544c5560e81b6044820152606401610495565b62010deb19600283900b1215610dcd5760405162461bcd60e51b8152602060048201526003602482015262544c4d60e81b6044820152606401610495565b610dda62010deb19611462565b60020b8160020b13156106025760405162461bcd60e51b815260206004820152600360248201526254554d60e81b6044820152606401610495565b6040516001600160a01b0380851660248301528316604482015260648101829052610c8d9085906323b872dd60e01b906084015b60408051601f198184030181529190526020810180516001600160e01b03166001600160e01b031990931692909217909152610fe4565b6040516001600160a01b0383166024820152604481018290526108ae90849063a9059cbb60e01b90606401610e49565b610eb981610c93565b6040516001600160a01b038216907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b90600090a250565b60606001600160a01b0383163b610f585760405162461bcd60e51b815260206004820152602660248201527f416464726573733a2064656c65676174652063616c6c20746f206e6f6e2d636f6044820152651b9d1c9858dd60d21b6064820152608401610495565b600080846001600160a01b031684604051610f7391906114b1565b600060405180830381855af49150503d8060008114610fae576040519150601f19603f3d011682016040523d82523d6000602084013e610fb3565b606091505b5091509150610fdb828260405180606001604052806027815260200161152160279139611065565b95945050505050565b600061101083836040518060400160405280600781526020016629aa261032b93960c91b815250611085565b8051909150156108ae578080602001905181019061102e919061139c565b6108ae5760405162461bcd60e51b815260206004820152600860248201526714d5130819985a5b60c21b6044820152606401610495565b6060831561107457508161107e565b61107e838361113d565b9392505050565b6060833b6110c45760405162461bcd60e51b815260206004820152600c60248201526b1b9bdb8b58dbdb9d1c9858dd60a21b6044820152606401610495565b600080856001600160a01b03166000866040516110e191906114b1565b60006040518083038185875af1925050503d806000811461111e576040519150601f19603f3d011682016040523d82523d6000602084013e611123565b606091505b5091509150611133828286611167565b9695505050505050565b81511561114d5781518083602001fd5b8060405162461bcd60e51b815260040161049591906114cd565b6060831561117657508161107e565b8251156111865782518084602001fd5b8160405162461bcd60e51b815260040161049591906114cd565b80356001600160a01b03811681146111b757600080fd5b919050565b6000602082840312156111ce57600080fd5b61107e826111a0565b634e487b7160e01b600052604160045260246000fd5b6000806040838503121561120057600080fd5b611209836111a0565b9150602083013567ffffffffffffffff8082111561122657600080fd5b818501915085601f83011261123a57600080fd5b81358181111561124c5761124c6111d7565b604051601f8201601f19908116603f01168101908382118183101715611274576112746111d7565b8160405282815288602084870101111561128d57600080fd5b8260208601602083013760006020848301015280955050505050509250929050565b8035600281900b81146111b757600080fd5b6000806000606084860312156112d657600080fd5b6112df846111a0565b92506112ed602085016112af565b91506112fb604085016112af565b90509250925092565b6020808252602c908201527f46756e6374696f6e206d7573742062652063616c6c6564207468726f7567682060408201526b19195b1959d85d1958d85b1b60a21b606082015260800190565b6020808252602c908201527f46756e6374696f6e206d7573742062652063616c6c6564207468726f7567682060408201526b6163746976652070726f787960a01b606082015260800190565b6000602082840312156113ae57600080fd5b8151801515811461107e57600080fd5b634e487b7160e01b600052601160045260246000fd5b6000600160ff1b8214156113ea576113ea6113be565b5060000390565b60006020828403121561140357600080fd5b5051919050565b600080821280156001600160ff1b038490038513161561142c5761142c6113be565b600160ff1b8390038412811615611445576114456113be565b50500190565b60008282101561145d5761145d6113be565b500390565b60008160020b627fffff1981141561147c5761147c6113be565b60000392915050565b60005b838110156114a0578181015183820152602001611488565b83811115610c8d5750506000910152565b600082516114c3818460208701611485565b9190910192915050565b60208152600082518060208401526114ec816040850160208701611485565b601f01601f1916919091016040019291505056fe360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc416464726573733a206c6f772d6c6576656c2064656c65676174652063616c6c206661696c6564a2646970667358221220090d0297ef7203e20e1481f2f41e8a4c7ed8719ec0918974f230a890d57b42e464736f6c63430008090033" . parse () . expect ("invalid bytecode")
        });
    pub struct MarginEngineEmergency<M>(ethers::contract::Contract<M>);
    impl<M> Clone for MarginEngineEmergency<M> {
        fn clone(&self) -> Self {
            MarginEngineEmergency(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for MarginEngineEmergency<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for MarginEngineEmergency<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MarginEngineEmergency))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> MarginEngineEmergency<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                MARGINENGINEEMERGENCY_ABI.clone(),
                client,
            )
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
                MARGINENGINEEMERGENCY_ABI.clone(),
                MARGINENGINEEMERGENCY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `MAX_CACHE_MAX_AGE_IN_SECONDS` (0x86b127ee) function"]
        pub fn max_cache_max_age_in_seconds(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([134, 177, 39, 238], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAX_LIQUIDATOR_REWARD_WAD` (0x87e16303) function"]
        pub fn max_liquidator_reward_wad(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([135, 225, 99, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAX_LOOKBACK_WINDOW_IN_SECONDS` (0xf907bd6d) function"]
        pub fn max_lookback_window_in_seconds(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([249, 7, 189, 109], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MIN_LOOKBACK_WINDOW_IN_SECONDS` (0x5f6a3e0c) function"]
        pub fn min_lookback_window_in_seconds(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([95, 106, 62, 12], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ONE` (0xc2ee3a08) function"]
        pub fn one(&self) -> ethers::contract::builders::ContractCall<M, I256> {
            self.0
                .method_hash([194, 238, 58, 8], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ONE_UINT` (0x63f57381) function"]
        pub fn one_uint(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([99, 245, 115, 129], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `SECONDS_IN_YEAR` (0x5dcc9391) function"]
        pub fn seconds_in_year(&self) -> ethers::contract::builders::ContractCall<M, I256> {
            self.0
                .method_hash([93, 204, 147, 145], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `cacheMaxAgeInSeconds` (0xb623f519) function"]
        pub fn cache_max_age_in_seconds(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([182, 35, 245, 25], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `emergencyWithdrawal` (0xcdcae73d) function"]
        pub fn emergency_withdrawal(
            &self,
            owner: ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([205, 202, 231, 61], (owner, tick_lower, tick_upper))
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
        #[doc = "Calls the contract's `fcm` (0x22d23b21) function"]
        pub fn fcm(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([34, 210, 59, 33], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isAlpha` (0x88428752) function"]
        pub fn is_alpha(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([136, 66, 135, 82], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liquidatorRewardWad` (0xe087caf1) function"]
        pub fn liquidator_reward_wad(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([224, 135, 202, 241], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lookbackWindowInSeconds` (0x9cbff188) function"]
        pub fn lookback_window_in_seconds(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([156, 191, 241, 136], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `paused` (0x5c975abb) function"]
        pub fn paused(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([92, 151, 90, 187], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `proxiableUUID` (0x52d1902d) function"]
        pub fn proxiable_uuid(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([82, 209, 144, 45], ())
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
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `termEndTimestampWad` (0x93edb454) function"]
        pub fn term_end_timestamp_wad(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([147, 237, 180, 84], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `termStartTimestampWad` (0x652c30b7) function"]
        pub fn term_start_timestamp_wad(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([101, 44, 48, 183], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(
            &self,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `underlyingToken` (0x2495a599) function"]
        pub fn underlying_token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([36, 149, 165, 153], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `upgradeTo` (0x3659cfe6) function"]
        pub fn upgrade_to(
            &self,
            new_implementation: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 89, 207, 230], new_implementation)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `upgradeToAndCall` (0x4f1ef286) function"]
        pub fn upgrade_to_and_call(
            &self,
            new_implementation: ethers::core::types::Address,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([79, 30, 242, 134], (new_implementation, data))
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
        #[doc = "Gets the contract's `AdminChanged` event"]
        pub fn admin_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AdminChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `BeaconUpgraded` event"]
        pub fn beacon_upgraded_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, BeaconUpgradedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Initialized` event"]
        pub fn initialized_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, InitializedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Upgraded` event"]
        pub fn upgraded_filter(&self) -> ethers::contract::builders::Event<M, UpgradedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, MarginEngineEmergencyEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for MarginEngineEmergency<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
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
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "AdminChanged", abi = "AdminChanged(address,address)")]
    pub struct AdminChangedFilter {
        pub previous_admin: ethers::core::types::Address,
        pub new_admin: ethers::core::types::Address,
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
    #[ethevent(name = "BeaconUpgraded", abi = "BeaconUpgraded(address)")]
    pub struct BeaconUpgradedFilter {
        #[ethevent(indexed)]
        pub beacon: ethers::core::types::Address,
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
    #[ethevent(name = "Initialized", abi = "Initialized(uint8)")]
    pub struct InitializedFilter {
        pub version: u8,
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
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers::core::types::Address,
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
    #[ethevent(name = "Upgraded", abi = "Upgraded(address)")]
    pub struct UpgradedFilter {
        #[ethevent(indexed)]
        pub implementation: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MarginEngineEmergencyEvents {
        AdminChangedFilter(AdminChangedFilter),
        BeaconUpgradedFilter(BeaconUpgradedFilter),
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        UpgradedFilter(UpgradedFilter),
    }
    impl ethers::contract::EthLogDecode for MarginEngineEmergencyEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AdminChangedFilter::decode_log(log) {
                return Ok(MarginEngineEmergencyEvents::AdminChangedFilter(decoded));
            }
            if let Ok(decoded) = BeaconUpgradedFilter::decode_log(log) {
                return Ok(MarginEngineEmergencyEvents::BeaconUpgradedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(MarginEngineEmergencyEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(MarginEngineEmergencyEvents::OwnershipTransferredFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(MarginEngineEmergencyEvents::UpgradedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for MarginEngineEmergencyEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MarginEngineEmergencyEvents::AdminChangedFilter(element) => element.fmt(f),
                MarginEngineEmergencyEvents::BeaconUpgradedFilter(element) => element.fmt(f),
                MarginEngineEmergencyEvents::InitializedFilter(element) => element.fmt(f),
                MarginEngineEmergencyEvents::OwnershipTransferredFilter(element) => element.fmt(f),
                MarginEngineEmergencyEvents::UpgradedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `MAX_CACHE_MAX_AGE_IN_SECONDS` function with signature `MAX_CACHE_MAX_AGE_IN_SECONDS()` and selector `[134, 177, 39, 238]`"]
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
        name = "MAX_CACHE_MAX_AGE_IN_SECONDS",
        abi = "MAX_CACHE_MAX_AGE_IN_SECONDS()"
    )]
    pub struct MaxCacheMaxAgeInSecondsCall;
    #[doc = "Container type for all input parameters for the `MAX_LIQUIDATOR_REWARD_WAD` function with signature `MAX_LIQUIDATOR_REWARD_WAD()` and selector `[135, 225, 99, 3]`"]
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
        name = "MAX_LIQUIDATOR_REWARD_WAD",
        abi = "MAX_LIQUIDATOR_REWARD_WAD()"
    )]
    pub struct MaxLiquidatorRewardWadCall;
    #[doc = "Container type for all input parameters for the `MAX_LOOKBACK_WINDOW_IN_SECONDS` function with signature `MAX_LOOKBACK_WINDOW_IN_SECONDS()` and selector `[249, 7, 189, 109]`"]
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
        name = "MAX_LOOKBACK_WINDOW_IN_SECONDS",
        abi = "MAX_LOOKBACK_WINDOW_IN_SECONDS()"
    )]
    pub struct MaxLookbackWindowInSecondsCall;
    #[doc = "Container type for all input parameters for the `MIN_LOOKBACK_WINDOW_IN_SECONDS` function with signature `MIN_LOOKBACK_WINDOW_IN_SECONDS()` and selector `[95, 106, 62, 12]`"]
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
        name = "MIN_LOOKBACK_WINDOW_IN_SECONDS",
        abi = "MIN_LOOKBACK_WINDOW_IN_SECONDS()"
    )]
    pub struct MinLookbackWindowInSecondsCall;
    #[doc = "Container type for all input parameters for the `ONE` function with signature `ONE()` and selector `[194, 238, 58, 8]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "ONE", abi = "ONE()")]
    pub struct OneCall;
    #[doc = "Container type for all input parameters for the `ONE_UINT` function with signature `ONE_UINT()` and selector `[99, 245, 115, 129]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "ONE_UINT", abi = "ONE_UINT()")]
    pub struct OneUintCall;
    #[doc = "Container type for all input parameters for the `SECONDS_IN_YEAR` function with signature `SECONDS_IN_YEAR()` and selector `[93, 204, 147, 145]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "SECONDS_IN_YEAR", abi = "SECONDS_IN_YEAR()")]
    pub struct SecondsInYearCall;
    #[doc = "Container type for all input parameters for the `cacheMaxAgeInSeconds` function with signature `cacheMaxAgeInSeconds()` and selector `[182, 35, 245, 25]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "cacheMaxAgeInSeconds", abi = "cacheMaxAgeInSeconds()")]
    pub struct CacheMaxAgeInSecondsCall;
    #[doc = "Container type for all input parameters for the `emergencyWithdrawal` function with signature `emergencyWithdrawal(address,int24,int24)` and selector `[205, 202, 231, 61]`"]
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
        name = "emergencyWithdrawal",
        abi = "emergencyWithdrawal(address,int24,int24)"
    )]
    pub struct EmergencyWithdrawalCall {
        pub owner: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `fcm` function with signature `fcm()` and selector `[34, 210, 59, 33]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "fcm", abi = "fcm()")]
    pub struct FcmCall;
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
    #[doc = "Container type for all input parameters for the `liquidatorRewardWad` function with signature `liquidatorRewardWad()` and selector `[224, 135, 202, 241]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "liquidatorRewardWad", abi = "liquidatorRewardWad()")]
    pub struct LiquidatorRewardWadCall;
    #[doc = "Container type for all input parameters for the `lookbackWindowInSeconds` function with signature `lookbackWindowInSeconds()` and selector `[156, 191, 241, 136]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "lookbackWindowInSeconds", abi = "lookbackWindowInSeconds()")]
    pub struct LookbackWindowInSecondsCall;
    #[doc = "Container type for all input parameters for the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    #[doc = "Container type for all input parameters for the `paused` function with signature `paused()` and selector `[92, 151, 90, 187]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "paused", abi = "paused()")]
    pub struct PausedCall;
    #[doc = "Container type for all input parameters for the `proxiableUUID` function with signature `proxiableUUID()` and selector `[82, 209, 144, 45]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "proxiableUUID", abi = "proxiableUUID()")]
    pub struct ProxiableUUIDCall;
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
    #[doc = "Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `[113, 80, 24, 166]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    #[doc = "Container type for all input parameters for the `termEndTimestampWad` function with signature `termEndTimestampWad()` and selector `[147, 237, 180, 84]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "termEndTimestampWad", abi = "termEndTimestampWad()")]
    pub struct TermEndTimestampWadCall;
    #[doc = "Container type for all input parameters for the `termStartTimestampWad` function with signature `termStartTimestampWad()` and selector `[101, 44, 48, 183]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "termStartTimestampWad", abi = "termStartTimestampWad()")]
    pub struct TermStartTimestampWadCall;
    #[doc = "Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `[242, 253, 227, 139]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `underlyingToken` function with signature `underlyingToken()` and selector `[36, 149, 165, 153]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "underlyingToken", abi = "underlyingToken()")]
    pub struct UnderlyingTokenCall;
    #[doc = "Container type for all input parameters for the `upgradeTo` function with signature `upgradeTo(address)` and selector `[54, 89, 207, 230]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "upgradeTo", abi = "upgradeTo(address)")]
    pub struct UpgradeToCall {
        pub new_implementation: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `upgradeToAndCall` function with signature `upgradeToAndCall(address,bytes)` and selector `[79, 30, 242, 134]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "upgradeToAndCall", abi = "upgradeToAndCall(address,bytes)")]
    pub struct UpgradeToAndCallCall {
        pub new_implementation: ethers::core::types::Address,
        pub data: ethers::core::types::Bytes,
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
    pub enum MarginEngineEmergencyCalls {
        MaxCacheMaxAgeInSeconds(MaxCacheMaxAgeInSecondsCall),
        MaxLiquidatorRewardWad(MaxLiquidatorRewardWadCall),
        MaxLookbackWindowInSeconds(MaxLookbackWindowInSecondsCall),
        MinLookbackWindowInSeconds(MinLookbackWindowInSecondsCall),
        One(OneCall),
        OneUint(OneUintCall),
        SecondsInYear(SecondsInYearCall),
        CacheMaxAgeInSeconds(CacheMaxAgeInSecondsCall),
        EmergencyWithdrawal(EmergencyWithdrawalCall),
        Factory(FactoryCall),
        Fcm(FcmCall),
        IsAlpha(IsAlphaCall),
        LiquidatorRewardWad(LiquidatorRewardWadCall),
        LookbackWindowInSeconds(LookbackWindowInSecondsCall),
        Owner(OwnerCall),
        Paused(PausedCall),
        ProxiableUUID(ProxiableUUIDCall),
        RateOracle(RateOracleCall),
        RenounceOwnership(RenounceOwnershipCall),
        TermEndTimestampWad(TermEndTimestampWadCall),
        TermStartTimestampWad(TermStartTimestampWadCall),
        TransferOwnership(TransferOwnershipCall),
        UnderlyingToken(UnderlyingTokenCall),
        UpgradeTo(UpgradeToCall),
        UpgradeToAndCall(UpgradeToAndCallCall),
        Vamm(VammCall),
    }
    impl ethers::core::abi::AbiDecode for MarginEngineEmergencyCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <MaxCacheMaxAgeInSecondsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineEmergencyCalls::MaxCacheMaxAgeInSeconds(decoded));
            }
            if let Ok(decoded) =
                <MaxLiquidatorRewardWadCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineEmergencyCalls::MaxLiquidatorRewardWad(decoded));
            }
            if let Ok(decoded) =
                <MaxLookbackWindowInSecondsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MarginEngineEmergencyCalls::MaxLookbackWindowInSeconds(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <MinLookbackWindowInSecondsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MarginEngineEmergencyCalls::MinLookbackWindowInSeconds(
                    decoded,
                ));
            }
            if let Ok(decoded) = <OneCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(MarginEngineEmergencyCalls::One(decoded));
            }
            if let Ok(decoded) =
                <OneUintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineEmergencyCalls::OneUint(decoded));
            }
            if let Ok(decoded) =
                <SecondsInYearCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineEmergencyCalls::SecondsInYear(decoded));
            }
            if let Ok(decoded) =
                <CacheMaxAgeInSecondsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineEmergencyCalls::CacheMaxAgeInSeconds(decoded));
            }
            if let Ok(decoded) =
                <EmergencyWithdrawalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineEmergencyCalls::EmergencyWithdrawal(decoded));
            }
            if let Ok(decoded) =
                <FactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineEmergencyCalls::Factory(decoded));
            }
            if let Ok(decoded) = <FcmCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(MarginEngineEmergencyCalls::Fcm(decoded));
            }
            if let Ok(decoded) =
                <IsAlphaCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineEmergencyCalls::IsAlpha(decoded));
            }
            if let Ok(decoded) =
                <LiquidatorRewardWadCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineEmergencyCalls::LiquidatorRewardWad(decoded));
            }
            if let Ok(decoded) =
                <LookbackWindowInSecondsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineEmergencyCalls::LookbackWindowInSeconds(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineEmergencyCalls::Owner(decoded));
            }
            if let Ok(decoded) = <PausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineEmergencyCalls::Paused(decoded));
            }
            if let Ok(decoded) =
                <ProxiableUUIDCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineEmergencyCalls::ProxiableUUID(decoded));
            }
            if let Ok(decoded) =
                <RateOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineEmergencyCalls::RateOracle(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineEmergencyCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <TermEndTimestampWadCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineEmergencyCalls::TermEndTimestampWad(decoded));
            }
            if let Ok(decoded) =
                <TermStartTimestampWadCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineEmergencyCalls::TermStartTimestampWad(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineEmergencyCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <UnderlyingTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineEmergencyCalls::UnderlyingToken(decoded));
            }
            if let Ok(decoded) =
                <UpgradeToCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineEmergencyCalls::UpgradeTo(decoded));
            }
            if let Ok(decoded) =
                <UpgradeToAndCallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MarginEngineEmergencyCalls::UpgradeToAndCall(decoded));
            }
            if let Ok(decoded) = <VammCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(MarginEngineEmergencyCalls::Vamm(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MarginEngineEmergencyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MarginEngineEmergencyCalls::MaxCacheMaxAgeInSeconds(element) => element.encode(),
                MarginEngineEmergencyCalls::MaxLiquidatorRewardWad(element) => element.encode(),
                MarginEngineEmergencyCalls::MaxLookbackWindowInSeconds(element) => element.encode(),
                MarginEngineEmergencyCalls::MinLookbackWindowInSeconds(element) => element.encode(),
                MarginEngineEmergencyCalls::One(element) => element.encode(),
                MarginEngineEmergencyCalls::OneUint(element) => element.encode(),
                MarginEngineEmergencyCalls::SecondsInYear(element) => element.encode(),
                MarginEngineEmergencyCalls::CacheMaxAgeInSeconds(element) => element.encode(),
                MarginEngineEmergencyCalls::EmergencyWithdrawal(element) => element.encode(),
                MarginEngineEmergencyCalls::Factory(element) => element.encode(),
                MarginEngineEmergencyCalls::Fcm(element) => element.encode(),
                MarginEngineEmergencyCalls::IsAlpha(element) => element.encode(),
                MarginEngineEmergencyCalls::LiquidatorRewardWad(element) => element.encode(),
                MarginEngineEmergencyCalls::LookbackWindowInSeconds(element) => element.encode(),
                MarginEngineEmergencyCalls::Owner(element) => element.encode(),
                MarginEngineEmergencyCalls::Paused(element) => element.encode(),
                MarginEngineEmergencyCalls::ProxiableUUID(element) => element.encode(),
                MarginEngineEmergencyCalls::RateOracle(element) => element.encode(),
                MarginEngineEmergencyCalls::RenounceOwnership(element) => element.encode(),
                MarginEngineEmergencyCalls::TermEndTimestampWad(element) => element.encode(),
                MarginEngineEmergencyCalls::TermStartTimestampWad(element) => element.encode(),
                MarginEngineEmergencyCalls::TransferOwnership(element) => element.encode(),
                MarginEngineEmergencyCalls::UnderlyingToken(element) => element.encode(),
                MarginEngineEmergencyCalls::UpgradeTo(element) => element.encode(),
                MarginEngineEmergencyCalls::UpgradeToAndCall(element) => element.encode(),
                MarginEngineEmergencyCalls::Vamm(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MarginEngineEmergencyCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MarginEngineEmergencyCalls::MaxCacheMaxAgeInSeconds(element) => element.fmt(f),
                MarginEngineEmergencyCalls::MaxLiquidatorRewardWad(element) => element.fmt(f),
                MarginEngineEmergencyCalls::MaxLookbackWindowInSeconds(element) => element.fmt(f),
                MarginEngineEmergencyCalls::MinLookbackWindowInSeconds(element) => element.fmt(f),
                MarginEngineEmergencyCalls::One(element) => element.fmt(f),
                MarginEngineEmergencyCalls::OneUint(element) => element.fmt(f),
                MarginEngineEmergencyCalls::SecondsInYear(element) => element.fmt(f),
                MarginEngineEmergencyCalls::CacheMaxAgeInSeconds(element) => element.fmt(f),
                MarginEngineEmergencyCalls::EmergencyWithdrawal(element) => element.fmt(f),
                MarginEngineEmergencyCalls::Factory(element) => element.fmt(f),
                MarginEngineEmergencyCalls::Fcm(element) => element.fmt(f),
                MarginEngineEmergencyCalls::IsAlpha(element) => element.fmt(f),
                MarginEngineEmergencyCalls::LiquidatorRewardWad(element) => element.fmt(f),
                MarginEngineEmergencyCalls::LookbackWindowInSeconds(element) => element.fmt(f),
                MarginEngineEmergencyCalls::Owner(element) => element.fmt(f),
                MarginEngineEmergencyCalls::Paused(element) => element.fmt(f),
                MarginEngineEmergencyCalls::ProxiableUUID(element) => element.fmt(f),
                MarginEngineEmergencyCalls::RateOracle(element) => element.fmt(f),
                MarginEngineEmergencyCalls::RenounceOwnership(element) => element.fmt(f),
                MarginEngineEmergencyCalls::TermEndTimestampWad(element) => element.fmt(f),
                MarginEngineEmergencyCalls::TermStartTimestampWad(element) => element.fmt(f),
                MarginEngineEmergencyCalls::TransferOwnership(element) => element.fmt(f),
                MarginEngineEmergencyCalls::UnderlyingToken(element) => element.fmt(f),
                MarginEngineEmergencyCalls::UpgradeTo(element) => element.fmt(f),
                MarginEngineEmergencyCalls::UpgradeToAndCall(element) => element.fmt(f),
                MarginEngineEmergencyCalls::Vamm(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<MaxCacheMaxAgeInSecondsCall> for MarginEngineEmergencyCalls {
        fn from(var: MaxCacheMaxAgeInSecondsCall) -> Self {
            MarginEngineEmergencyCalls::MaxCacheMaxAgeInSeconds(var)
        }
    }
    impl ::std::convert::From<MaxLiquidatorRewardWadCall> for MarginEngineEmergencyCalls {
        fn from(var: MaxLiquidatorRewardWadCall) -> Self {
            MarginEngineEmergencyCalls::MaxLiquidatorRewardWad(var)
        }
    }
    impl ::std::convert::From<MaxLookbackWindowInSecondsCall> for MarginEngineEmergencyCalls {
        fn from(var: MaxLookbackWindowInSecondsCall) -> Self {
            MarginEngineEmergencyCalls::MaxLookbackWindowInSeconds(var)
        }
    }
    impl ::std::convert::From<MinLookbackWindowInSecondsCall> for MarginEngineEmergencyCalls {
        fn from(var: MinLookbackWindowInSecondsCall) -> Self {
            MarginEngineEmergencyCalls::MinLookbackWindowInSeconds(var)
        }
    }
    impl ::std::convert::From<OneCall> for MarginEngineEmergencyCalls {
        fn from(var: OneCall) -> Self {
            MarginEngineEmergencyCalls::One(var)
        }
    }
    impl ::std::convert::From<OneUintCall> for MarginEngineEmergencyCalls {
        fn from(var: OneUintCall) -> Self {
            MarginEngineEmergencyCalls::OneUint(var)
        }
    }
    impl ::std::convert::From<SecondsInYearCall> for MarginEngineEmergencyCalls {
        fn from(var: SecondsInYearCall) -> Self {
            MarginEngineEmergencyCalls::SecondsInYear(var)
        }
    }
    impl ::std::convert::From<CacheMaxAgeInSecondsCall> for MarginEngineEmergencyCalls {
        fn from(var: CacheMaxAgeInSecondsCall) -> Self {
            MarginEngineEmergencyCalls::CacheMaxAgeInSeconds(var)
        }
    }
    impl ::std::convert::From<EmergencyWithdrawalCall> for MarginEngineEmergencyCalls {
        fn from(var: EmergencyWithdrawalCall) -> Self {
            MarginEngineEmergencyCalls::EmergencyWithdrawal(var)
        }
    }
    impl ::std::convert::From<FactoryCall> for MarginEngineEmergencyCalls {
        fn from(var: FactoryCall) -> Self {
            MarginEngineEmergencyCalls::Factory(var)
        }
    }
    impl ::std::convert::From<FcmCall> for MarginEngineEmergencyCalls {
        fn from(var: FcmCall) -> Self {
            MarginEngineEmergencyCalls::Fcm(var)
        }
    }
    impl ::std::convert::From<IsAlphaCall> for MarginEngineEmergencyCalls {
        fn from(var: IsAlphaCall) -> Self {
            MarginEngineEmergencyCalls::IsAlpha(var)
        }
    }
    impl ::std::convert::From<LiquidatorRewardWadCall> for MarginEngineEmergencyCalls {
        fn from(var: LiquidatorRewardWadCall) -> Self {
            MarginEngineEmergencyCalls::LiquidatorRewardWad(var)
        }
    }
    impl ::std::convert::From<LookbackWindowInSecondsCall> for MarginEngineEmergencyCalls {
        fn from(var: LookbackWindowInSecondsCall) -> Self {
            MarginEngineEmergencyCalls::LookbackWindowInSeconds(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for MarginEngineEmergencyCalls {
        fn from(var: OwnerCall) -> Self {
            MarginEngineEmergencyCalls::Owner(var)
        }
    }
    impl ::std::convert::From<PausedCall> for MarginEngineEmergencyCalls {
        fn from(var: PausedCall) -> Self {
            MarginEngineEmergencyCalls::Paused(var)
        }
    }
    impl ::std::convert::From<ProxiableUUIDCall> for MarginEngineEmergencyCalls {
        fn from(var: ProxiableUUIDCall) -> Self {
            MarginEngineEmergencyCalls::ProxiableUUID(var)
        }
    }
    impl ::std::convert::From<RateOracleCall> for MarginEngineEmergencyCalls {
        fn from(var: RateOracleCall) -> Self {
            MarginEngineEmergencyCalls::RateOracle(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for MarginEngineEmergencyCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            MarginEngineEmergencyCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<TermEndTimestampWadCall> for MarginEngineEmergencyCalls {
        fn from(var: TermEndTimestampWadCall) -> Self {
            MarginEngineEmergencyCalls::TermEndTimestampWad(var)
        }
    }
    impl ::std::convert::From<TermStartTimestampWadCall> for MarginEngineEmergencyCalls {
        fn from(var: TermStartTimestampWadCall) -> Self {
            MarginEngineEmergencyCalls::TermStartTimestampWad(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for MarginEngineEmergencyCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            MarginEngineEmergencyCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<UnderlyingTokenCall> for MarginEngineEmergencyCalls {
        fn from(var: UnderlyingTokenCall) -> Self {
            MarginEngineEmergencyCalls::UnderlyingToken(var)
        }
    }
    impl ::std::convert::From<UpgradeToCall> for MarginEngineEmergencyCalls {
        fn from(var: UpgradeToCall) -> Self {
            MarginEngineEmergencyCalls::UpgradeTo(var)
        }
    }
    impl ::std::convert::From<UpgradeToAndCallCall> for MarginEngineEmergencyCalls {
        fn from(var: UpgradeToAndCallCall) -> Self {
            MarginEngineEmergencyCalls::UpgradeToAndCall(var)
        }
    }
    impl ::std::convert::From<VammCall> for MarginEngineEmergencyCalls {
        fn from(var: VammCall) -> Self {
            MarginEngineEmergencyCalls::Vamm(var)
        }
    }
    #[doc = "Container type for all return fields from the `MAX_CACHE_MAX_AGE_IN_SECONDS` function with signature `MAX_CACHE_MAX_AGE_IN_SECONDS()` and selector `[134, 177, 39, 238]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MaxCacheMaxAgeInSecondsReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `MAX_LIQUIDATOR_REWARD_WAD` function with signature `MAX_LIQUIDATOR_REWARD_WAD()` and selector `[135, 225, 99, 3]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MaxLiquidatorRewardWadReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `MAX_LOOKBACK_WINDOW_IN_SECONDS` function with signature `MAX_LOOKBACK_WINDOW_IN_SECONDS()` and selector `[249, 7, 189, 109]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MaxLookbackWindowInSecondsReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `MIN_LOOKBACK_WINDOW_IN_SECONDS` function with signature `MIN_LOOKBACK_WINDOW_IN_SECONDS()` and selector `[95, 106, 62, 12]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MinLookbackWindowInSecondsReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `ONE` function with signature `ONE()` and selector `[194, 238, 58, 8]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct OneReturn(pub I256);
    #[doc = "Container type for all return fields from the `ONE_UINT` function with signature `ONE_UINT()` and selector `[99, 245, 115, 129]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct OneUintReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `SECONDS_IN_YEAR` function with signature `SECONDS_IN_YEAR()` and selector `[93, 204, 147, 145]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SecondsInYearReturn(pub I256);
    #[doc = "Container type for all return fields from the `cacheMaxAgeInSeconds` function with signature `cacheMaxAgeInSeconds()` and selector `[182, 35, 245, 25]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CacheMaxAgeInSecondsReturn(pub ethers::core::types::U256);
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
    #[doc = "Container type for all return fields from the `fcm` function with signature `fcm()` and selector `[34, 210, 59, 33]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct FcmReturn(pub ethers::core::types::Address);
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
    #[doc = "Container type for all return fields from the `liquidatorRewardWad` function with signature `liquidatorRewardWad()` and selector `[224, 135, 202, 241]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LiquidatorRewardWadReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `lookbackWindowInSeconds` function with signature `lookbackWindowInSeconds()` and selector `[156, 191, 241, 136]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LookbackWindowInSecondsReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct OwnerReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `paused` function with signature `paused()` and selector `[92, 151, 90, 187]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PausedReturn(pub bool);
    #[doc = "Container type for all return fields from the `proxiableUUID` function with signature `proxiableUUID()` and selector `[82, 209, 144, 45]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ProxiableUUIDReturn(pub [u8; 32]);
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
    #[doc = "Container type for all return fields from the `termEndTimestampWad` function with signature `termEndTimestampWad()` and selector `[147, 237, 180, 84]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TermEndTimestampWadReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `termStartTimestampWad` function with signature `termStartTimestampWad()` and selector `[101, 44, 48, 183]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TermStartTimestampWadReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `underlyingToken` function with signature `underlyingToken()` and selector `[36, 149, 165, 153]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct UnderlyingTokenReturn(pub ethers::core::types::Address);
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
