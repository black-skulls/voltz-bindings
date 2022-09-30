pub use community_deployer::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod community_deployer {
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
    #[doc = "CommunityDeployer was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static COMMUNITYDEPLOYER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"contract IVAMM\",\"name\":\"_masterVAMM\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IMarginEngine\",\"name\":\"_masterMarginEngine\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_quorumVotes\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_ownerAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"_merkleRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_blockTimestampVotingEnd\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"numberOfVotes\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bool\",\"name\":\"yesVote\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Voted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"TIMELOCK_PERIOD_IN_SECONDS\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"blockTimestampTimelockEnd\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"blockTimestampVotingEnd\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_index\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_numberOfVotes\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"_yesVote\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bytes32[]\",\"name\":\"_merkleProof\",\"type\":\"bytes32[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"castVote\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deploy\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hasTokenIdVoted\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hasVoted\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isQueued\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"masterMarginEngine\",\"outputs\":[{\"internalType\":\"contract IMarginEngine\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"masterVAMM\",\"outputs\":[{\"internalType\":\"contract IVAMM\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"merkleRoot\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"noVoteCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ownerAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"queue\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"quorumVotes\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"voltzFactory\",\"outputs\":[{\"internalType\":\"contract IFactory\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"yesVoteCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static COMMUNITYDEPLOYER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b5060405161243d38038061243d83398101604081905261002f91610096565b600755600380546001600160a01b03199081166001600160a01b039788161790915560028054821695871695909517909455600192909255600080549093169316929092179055600a556100ff565b6001600160a01b038116811461009357600080fd5b50565b60008060008060008060c087890312156100af57600080fd5b86516100ba8161007e565b60208801519096506100cb8161007e565b6040880151606089015191965094506100e38161007e565b809350506080870151915060a087015190509295509295509295565b61232f8061010e6000396000f3fe608060405234801561001057600080fd5b506004361061010b5760003560e01c806392c7abb7116100a2578063b8f2145f11610071578063b8f2145f146101db578063e10d29ee146101fe578063ecca031f14610206578063fe3b959114610219578063febfe75e1461023157600080fd5b806392c7abb7146101a35780639fe1b354146101b6578063b4395f83146101c9578063b5550397146101d257600080fd5b8063775c300c116100de578063775c300c1461015b57806378d177c0146101655780638f84aa091461016f578063916e62f31461019a57600080fd5b80631b25de8f1461011057806324bc1a64146101325780632eb4a7ab1461014957806344a0b3e714610152575b600080fd5b60095461011d9060ff1681565b60405190151581526020015b60405180910390f35b61013b60015481565b604051908152602001610129565b61013b600a5481565b61013b60045481565b610163610244565b005b61013b6202a30081565b600054610182906001600160a01b031681565b6040516001600160a01b039091168152602001610129565b61013b60055481565b6101636101b13660046107e9565b6103a7565b600254610182906001600160a01b031681565b61013b60075481565b61013b60085481565b61011d6101e9366004610887565b60066020526000908152604090205460ff1681565b610163610597565b61011d610214366004610887565b6106c7565b6009546101829061010090046001600160a01b031681565b600354610182906001600160a01b031681565b60095460ff166102885760405162461bcd60e51b815260206004820152600a6024820152691b9bdd081c5d595d595960b21b60448201526064015b60405180910390fd5b60085442116102cf5760405162461bcd60e51b815260206004820152601360248201527274696d656c6f636b206973206f6e676f696e6760681b604482015260640161027f565b6002546003546040516001600160a01b0392831692909116906102f1906107db565b6001600160a01b03928316815291166020820152604001604051809103906000f080158015610324573d6000803e3d6000fd5b5060098054610100600160a81b0319166101006001600160a01b039384168102919091179182905560005460405163f2fde38b60e01b8152908416600482015291049091169063f2fde38b90602401600060405180830381600087803b15801561038d57600080fd5b505af11580156103a1573d6000803e3d6000fd5b50505050565b6007544211156103ee5760405162461bcd60e51b81526020600482015260126024820152713b37ba34b733903832b934b7b21037bb32b960711b604482015260640161027f565b6103f7856106c7565b156104355760405162461bcd60e51b815260206004820152600e60248201526d6475706c696361746520766f746560901b604482015260640161027f565b60408051602081018790526bffffffffffffffffffffffff193360601b1691810191909152605481018590526000906074016040516020818303038152906040528051906020012090506104c083838080602002602001604051908101604052809392919081815260200183836020028082843760009201919091525050600a549150849050610708565b6105035760405162461bcd60e51b815260206004820152601460248201527334b73b30b634b21036b2b935b63290383937b7b360611b604482015260640161027f565b61050c8661071e565b831561052f57846004600082825461052491906108b6565b909155506105479050565b846005600082825461054191906108b6565b90915550505b6040805187815233602082015290810186905284151560608201527fb7086a9dd618ffa688aa9500720dfe31d3b288daba445664cecceaed4a1562c39060800160405180910390a1505050505050565b60075442116105dc5760405162461bcd60e51b8152602060048201526011602482015270766f74696e67206973206f6e676f696e6760781b604482015260640161027f565b60015460045410156106255760405162461bcd60e51b81526020600482015260126024820152711c5d5bdc9d5b481b9bdd081c995858da195960721b604482015260640161027f565b600554600454116106645760405162461bcd60e51b81526020600482015260096024820152686e6f203e3d2079657360b81b604482015260640161027f565b60095460ff16156106a85760405162461bcd60e51b815260206004820152600e60248201526d185b1c9958591e481c5d595d595960921b604482015260640161027f565b6009805460ff191660011790556106c26202a300426108b6565b600855565b6000806106d6610100846108e4565b905060006106e6610100856108f8565b6000928352600b602052604090922054600190921b9182169091149392505050565b600082610715858461075c565b14949350505050565b600061072c610100836108e4565b9050600061073c610100846108f8565b6000928352600b60205260409092208054600190931b9092179091555050565b600081815b84518110156107a15761078d828683815181106107805761078061090c565b60200260200101516107a9565b91508061079981610922565b915050610761565b509392505050565b60008183106107c55760008281526020849052604090206107d4565b60008381526020839052604090205b9392505050565b6119bc806200093e83390190565b60008060008060006080868803121561080157600080fd5b85359450602086013593506040860135801515811461081f57600080fd5b9250606086013567ffffffffffffffff8082111561083c57600080fd5b818801915088601f83011261085057600080fd5b81358181111561085f57600080fd5b8960208260051b850101111561087457600080fd5b9699959850939650602001949392505050565b60006020828403121561089957600080fd5b5035919050565b634e487b7160e01b600052601160045260246000fd5b600082198211156108c9576108c96108a0565b500190565b634e487b7160e01b600052601260045260246000fd5b6000826108f3576108f36108ce565b500490565b600082610907576109076108ce565b500690565b634e487b7160e01b600052603260045260246000fd5b6000600019821415610936576109366108a0565b506001019056fe60806040523480156200001157600080fd5b50604051620019bc380380620019bc83398101604081905262000034916200018e565b6200003f3362000125565b6001600160a01b0382166200009b5760405162461bcd60e51b815260206004820152601460248201527f6d6173746572206d65206d75737420657869737400000000000000000000000060448201526064015b60405180910390fd5b6001600160a01b038116620000f35760405162461bcd60e51b815260206004820152601660248201527f6d61737465722076616d6d206d75737420657869737400000000000000000000604482015260640162000092565b600180546001600160a01b039384166001600160a01b03199182161790915560028054929093169116179055620001cd565b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b6001600160a01b03811681146200018b57600080fd5b50565b60008060408385031215620001a257600080fd5b8251620001af8162000175565b6020840151909250620001c28162000175565b809150509250929050565b6117df80620001dd6000396000f3fe60806040523480156200001157600080fd5b5060043610620000fd5760003560e01c80639fe1b3541162000097578063ca5183b7116200006e578063ca5183b7146200022b578063db9b71701462000257578063f2fde38b146200026e578063febfe75e146200028557600080fd5b80639fe1b35414620001d8578063a389783e14620001ec578063aeb22934146200021457600080fd5b80638da5cb5b11620000d85780638da5cb5b146200018157806394a9b1f7146200019357806395858f9814620001aa578063973cd93114620001c157600080fd5b80630e8a06481462000102578063715018a6146200014857806377aace1a1462000154575b600080fd5b620001196200011336600462000ec5565b62000299565b604080516001600160a01b03948516815292841660208401529216918101919091526060015b60405180910390f35b6200015262000969565b005b60055462000168906001600160a01b031681565b6040516001600160a01b0390911681526020016200013f565b6000546001600160a01b031662000168565b62000152620001a436600462000f31565b62000981565b62000152620001bb36600462000f68565b62000a11565b62000152620001d236600462000f31565b62000ad8565b60015462000168906001600160a01b031681565b62000203620001fd36600462000fa6565b62000b67565b60405190151581526020016200013f565b620001526200022536600462000f31565b62000c56565b620001686200023c36600462000fd9565b6003602052600090815260409020546001600160a01b031681565b620001526200026836600462000ff9565b62000d19565b620001526200027f36600462000f31565b62000d79565b60025462000168906001600160a01b031681565b6000806000620002a862000df5565b6001546040516000916001600160a01b031690620002c69062000ea1565b6001600160a01b039091168152604060208201819052600090820152606001604051809103906000f08015801562000302573d6000803e3d6000fd5b506002546040519192506000916001600160a01b0390911690620003269062000ea1565b6001600160a01b039091168152604060208201819052600090820152606001604051809103906000f08015801562000362573d6000803e3d6000fd5b5060405163eb990c5960e01b81526001600160a01b038c811660048301528b81166024830152604482018b9052606482018a90529192509083169063eb990c5990608401600060405180830381600087803b158015620003c157600080fd5b505af1158015620003d6573d6000803e3d6000fd5b5050604051631b325b2160e31b81526001600160a01b03858116600483015260028a900b60248301528416925063d992d9089150604401600060405180830381600087803b1580156200042857600080fd5b505af11580156200043d573d6000803e3d6000fd5b50506040516331d81ea760e21b81526001600160a01b0384811660048301528516925063c7607a9c9150602401600060405180830381600087803b1580156200048557600080fd5b505af11580156200049a573d6000803e3d6000fd5b5050505060008990508a6001600160a01b0316816001600160a01b0316636f307dc36040518163ffffffff1660e01b815260040160206040518083038186803b158015620004e757600080fd5b505afa158015620004fc573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019062000522919062001030565b6001600160a01b031614620005745760405162461bcd60e51b81526020600482015260136024820152720a8ded6cadce640c8de40dcdee840dac2e8c6d606b1b60448201526064015b60405180910390fd5b6000816001600160a01b03166322ff65686040518163ffffffff1660e01b815260040160206040518083038186803b158015620005b057600080fd5b505afa158015620005c5573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620005eb919062001050565b60ff81166000908152600360205260408120549192506001600160a01b03909116908115620007825781604051620006239062000ea1565b6001600160a01b039091168152604060208201819052600090820152606001604051809103906000f0801580156200065f573d6000803e3d6000fd5b5060405163485cc95560e01b81526001600160a01b03878116600483015288811660248301529192509082169063485cc95590604401600060405180830381600087803b158015620006b057600080fd5b505af1158015620006c5573d6000803e3d6000fd5b505060405163534d337560e01b81526001600160a01b0384811660048301528916925063534d33759150602401600060405180830381600087803b1580156200070d57600080fd5b505af115801562000722573d6000803e3d6000fd5b505060405163f2fde38b60e01b81523360048201526001600160a01b038416925063f2fde38b9150602401600060405180830381600087803b1580156200076857600080fd5b505af11580156200077d573d6000803e3d6000fd5b505050505b60008e6001600160a01b031663313ce5676040518163ffffffff1660e01b815260040160206040518083038186803b158015620007be57600080fd5b505afa158015620007d3573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620007f9919062001050565b90508d6001600160a01b03168f6001600160a01b03167fe134804702afa0f02bd7f0687d4c2f662a1790b4904d1c2cd6f41fcffbfc05c38f8f8f8c8c898c8a60405162000893989796959493929190978852602088019690965260029490940b60408701526001600160a01b03928316606087015290821660808601521660a084015260ff90811660c08401521660e08201526101000190565b60405180910390a360405163f2fde38b60e01b81523360048201526001600160a01b0387169063f2fde38b90602401600060405180830381600087803b158015620008dd57600080fd5b505af1158015620008f2573d6000803e3d6000fd5b505060405163f2fde38b60e01b81523360048201526001600160a01b038a16925063f2fde38b9150602401600060405180830381600087803b1580156200093857600080fd5b505af11580156200094d573d6000803e3d6000fd5b50989b5096995091975050505050505050955095509592505050565b6200097362000df5565b6200097f600062000e51565b565b6200098b62000df5565b6001600160a01b038116620009dc5760405162461bcd60e51b81526020600482015260166024820152751b585cdd195c881d985b5b481b5d5cdd08195e1a5cdd60521b60448201526064016200056b565b6002546001600160a01b0382811691161462000a0e57600280546001600160a01b0319166001600160a01b0383161790555b50565b62000a1b62000df5565b6001600160a01b03821662000a6b5760405162461bcd60e51b81526020600482015260156024820152741b585cdd195c881998db481b5d5cdd08195e1a5cdd605a1b60448201526064016200056b565b60ff811660008181526003602090815260409182902080546001600160a01b0319166001600160a01b0387169081179091558251908152908101929092527fcece9976caa53e350e3311c87df72b4ed94d768ba03a4688cdf331121bf613c2910160405180910390a15050565b62000ae262000df5565b6001600160a01b03811662000b315760405162461bcd60e51b81526020600482015260146024820152731b585cdd195c881b59481b5d5cdd08195e1a5cdd60621b60448201526064016200056b565b6001546001600160a01b0382811691161462000a0e57600180546001600160a01b0383166001600160a01b031990911617905550565b60006001600160a01b03831662000bb85760405162461bcd60e51b81526020600482015260146024820152731bdddb995c88191bd95cc81b9bdd08195e1a5cdd60621b60448201526064016200056b565b6001600160a01b03821662000c055760405162461bcd60e51b81526020600482015260126024820152711a5b9d08191bd95cc81b9bdd08195e1a5cdd60721b60448201526064016200056b565b6005546001600160a01b038381169116141562000c255750600162000c50565b506001600160a01b0380831660009081526004602090815260408083209385168352929052205460ff165b92915050565b62000c6062000df5565b6001600160a01b03811662000caf5760405162461bcd60e51b81526020600482015260146024820152731c195c9a5c1a195c9e481b5d5cdd08195e1a5cdd60621b60448201526064016200056b565b6005546001600160a01b0382811691161462000a0e57600580546001600160a01b0319166001600160a01b0383169081179091556040519081527f9d41c928e1b7d55eb20c9906b2cc3223035dd4dc2c59e5fcd2282e857db1b0509060200160405180910390a150565b3360008181526004602090815260408083206001600160a01b0387168085529252808320805460ff19168615159081179091559051909391927f1d3e246ebbc933bf65d3290db9f93d67ab91a12d2b19308a35806e04d1c174c591a45050565b62000d8362000df5565b6001600160a01b03811662000dea5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b60648201526084016200056b565b62000a0e8162000e51565b6000546001600160a01b031633146200097f5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016200056b565b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b610739806200107183390190565b6001600160a01b038116811462000a0e57600080fd5b600080600080600060a0868803121562000ede57600080fd5b853562000eeb8162000eaf565b9450602086013562000efd8162000eaf565b935060408601359250606086013591506080860135600281900b811462000f2357600080fd5b809150509295509295909350565b60006020828403121562000f4457600080fd5b813562000f518162000eaf565b9392505050565b60ff8116811462000a0e57600080fd5b6000806040838503121562000f7c57600080fd5b823562000f898162000eaf565b9150602083013562000f9b8162000f58565b809150509250929050565b6000806040838503121562000fba57600080fd5b823562000fc78162000eaf565b9150602083013562000f9b8162000eaf565b60006020828403121562000fec57600080fd5b813562000f518162000f58565b600080604083850312156200100d57600080fd5b82356200101a8162000eaf565b91506020830135801515811462000f9b57600080fd5b6000602082840312156200104357600080fd5b815162000f518162000eaf565b6000602082840312156200106357600080fd5b815162000f518162000f5856fe608060405260405161073938038061073983398101604081905261002291610322565b818161003082826000610039565b5050505061043f565b6100428361006f565b60008251118061004f5750805b1561006a5761006883836100af60201b6100291760201c565b505b505050565b610078816100db565b6040516001600160a01b038216907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b90600090a250565b60606100d48383604051806060016040528060278152602001610712602791396101ad565b9392505050565b6100ee8161022660201b6100551760201c565b6101555760405162461bcd60e51b815260206004820152602d60248201527f455243313936373a206e657720696d706c656d656e746174696f6e206973206e60448201526c1bdd08184818dbdb9d1c9858dd609a1b60648201526084015b60405180910390fd5b8061018c7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc60001b61023560201b6100641760201c565b80546001600160a01b0319166001600160a01b039290921691909117905550565b6060600080856001600160a01b0316856040516101ca91906103f0565b600060405180830381855af49150503d8060008114610205576040519150601f19603f3d011682016040523d82523d6000602084013e61020a565b606091505b50909250905061021c86838387610238565b9695505050505050565b6001600160a01b03163b151590565b90565b606083156102a457825161029d576001600160a01b0385163b61029d5760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e7472616374000000604482015260640161014c565b50816102ae565b6102ae83836102b6565b949350505050565b8151156102c65781518083602001fd5b8060405162461bcd60e51b815260040161014c919061040c565b634e487b7160e01b600052604160045260246000fd5b60005b838110156103115781810151838201526020016102f9565b838111156100685750506000910152565b6000806040838503121561033557600080fd5b82516001600160a01b038116811461034c57600080fd5b60208401519092506001600160401b038082111561036957600080fd5b818501915085601f83011261037d57600080fd5b81518181111561038f5761038f6102e0565b604051601f8201601f19908116603f011681019083821181831017156103b7576103b76102e0565b816040528281528860208487010111156103d057600080fd5b6103e18360208301602088016102f6565b80955050505050509250929050565b600082516104028184602087016102f6565b9190910192915050565b602081526000825180602084015261042b8160408501602087016102f6565b601f01601f19169190910160400192915050565b6102c48061044e6000396000f3fe60806040523661001357610011610017565b005b6100115b610027610022610067565b61009f565b565b606061004e8383604051806060016040528060278152602001610268602791396100c3565b9392505050565b6001600160a01b03163b151590565b90565b600061009a7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc546001600160a01b031690565b905090565b3660008037600080366000845af43d6000803e8080156100be573d6000f35b3d6000fd5b6060600080856001600160a01b0316856040516100e09190610218565b600060405180830381855af49150503d806000811461011b576040519150601f19603f3d011682016040523d82523d6000602084013e610120565b606091505b50915091506101318683838761013b565b9695505050505050565b606083156101ac5782516101a5576001600160a01b0385163b6101a55760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e747261637400000060448201526064015b60405180910390fd5b50816101b6565b6101b683836101be565b949350505050565b8151156101ce5781518083602001fd5b8060405162461bcd60e51b815260040161019c9190610234565b60005b838110156102035781810151838201526020016101eb565b83811115610212576000848401525b50505050565b6000825161022a8184602087016101e8565b9190910192915050565b60208152600082518060208401526102538160408501602087016101e8565b601f01601f1916919091016040019291505056fe416464726573733a206c6f772d6c6576656c2064656c65676174652063616c6c206661696c6564a2646970667358221220c9e739666436e4f4e46675a1f8f09b880cbc87b9dadbd6b81e1e432db303b85564736f6c63430008090033416464726573733a206c6f772d6c6576656c2064656c65676174652063616c6c206661696c6564a2646970667358221220956886011e6f38767996d82c907389cef820eefa65e487621fa9ee4bf050a0d364736f6c63430008090033a2646970667358221220e2888a1c834b3bd3b068c1f6e86c69d8dd0b0a6b760f1382f2e27b27618305d764736f6c63430008090033" . parse () . expect ("invalid bytecode")
        });
    pub struct CommunityDeployer<M>(ethers::contract::Contract<M>);
    impl<M> Clone for CommunityDeployer<M> {
        fn clone(&self) -> Self {
            CommunityDeployer(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for CommunityDeployer<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for CommunityDeployer<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(CommunityDeployer))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> CommunityDeployer<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), COMMUNITYDEPLOYER_ABI.clone(), client)
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
                COMMUNITYDEPLOYER_ABI.clone(),
                COMMUNITYDEPLOYER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `TIMELOCK_PERIOD_IN_SECONDS` (0x78d177c0) function"]
        pub fn timelock_period_in_seconds(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([120, 209, 119, 192], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `blockTimestampTimelockEnd` (0xb5550397) function"]
        pub fn block_timestamp_timelock_end(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([181, 85, 3, 151], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `blockTimestampVotingEnd` (0xb4395f83) function"]
        pub fn block_timestamp_voting_end(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([180, 57, 95, 131], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `castVote` (0x92c7abb7) function"]
        pub fn cast_vote(
            &self,
            index: ethers::core::types::U256,
            number_of_votes: ethers::core::types::U256,
            yes_vote: bool,
            merkle_proof: ::std::vec::Vec<[u8; 32]>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [146, 199, 171, 183],
                    (index, number_of_votes, yes_vote, merkle_proof),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deploy` (0x775c300c) function"]
        pub fn deploy(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([119, 92, 48, 12], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hasTokenIdVoted` (0xb8f2145f) function"]
        pub fn has_token_id_voted(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([184, 242, 20, 95], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hasVoted` (0xecca031f) function"]
        pub fn has_voted(
            &self,
            index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([236, 202, 3, 31], index)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isQueued` (0x1b25de8f) function"]
        pub fn is_queued(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([27, 37, 222, 143], ())
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
        #[doc = "Calls the contract's `merkleRoot` (0x2eb4a7ab) function"]
        pub fn merkle_root(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([46, 180, 167, 171], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `noVoteCount` (0x916e62f3) function"]
        pub fn no_vote_count(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([145, 110, 98, 243], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ownerAddress` (0x8f84aa09) function"]
        pub fn owner_address(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([143, 132, 170, 9], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `queue` (0xe10d29ee) function"]
        pub fn queue(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 13, 41, 238], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `quorumVotes` (0x24bc1a64) function"]
        pub fn quorum_votes(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([36, 188, 26, 100], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `voltzFactory` (0xfe3b9591) function"]
        pub fn voltz_factory(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([254, 59, 149, 145], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `yesVoteCount` (0x44a0b3e7) function"]
        pub fn yes_vote_count(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([68, 160, 179, 231], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Voted` event"]
        pub fn voted_filter(&self) -> ethers::contract::builders::Event<M, VotedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, VotedFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for CommunityDeployer<M>
    {
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
    #[ethevent(name = "Voted", abi = "Voted(uint256,address,uint256,bool)")]
    pub struct VotedFilter {
        pub index: ethers::core::types::U256,
        pub account: ethers::core::types::Address,
        pub number_of_votes: ethers::core::types::U256,
        pub yes_vote: bool,
    }
    #[doc = "Container type for all input parameters for the `TIMELOCK_PERIOD_IN_SECONDS` function with signature `TIMELOCK_PERIOD_IN_SECONDS()` and selector `[120, 209, 119, 192]`"]
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
        name = "TIMELOCK_PERIOD_IN_SECONDS",
        abi = "TIMELOCK_PERIOD_IN_SECONDS()"
    )]
    pub struct TimelockPeriodInSecondsCall;
    #[doc = "Container type for all input parameters for the `blockTimestampTimelockEnd` function with signature `blockTimestampTimelockEnd()` and selector `[181, 85, 3, 151]`"]
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
        name = "blockTimestampTimelockEnd",
        abi = "blockTimestampTimelockEnd()"
    )]
    pub struct BlockTimestampTimelockEndCall;
    #[doc = "Container type for all input parameters for the `blockTimestampVotingEnd` function with signature `blockTimestampVotingEnd()` and selector `[180, 57, 95, 131]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "blockTimestampVotingEnd", abi = "blockTimestampVotingEnd()")]
    pub struct BlockTimestampVotingEndCall;
    #[doc = "Container type for all input parameters for the `castVote` function with signature `castVote(uint256,uint256,bool,bytes32[])` and selector `[146, 199, 171, 183]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "castVote", abi = "castVote(uint256,uint256,bool,bytes32[])")]
    pub struct CastVoteCall {
        pub index: ethers::core::types::U256,
        pub number_of_votes: ethers::core::types::U256,
        pub yes_vote: bool,
        pub merkle_proof: ::std::vec::Vec<[u8; 32]>,
    }
    #[doc = "Container type for all input parameters for the `deploy` function with signature `deploy()` and selector `[119, 92, 48, 12]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "deploy", abi = "deploy()")]
    pub struct DeployCall;
    #[doc = "Container type for all input parameters for the `hasTokenIdVoted` function with signature `hasTokenIdVoted(uint256)` and selector `[184, 242, 20, 95]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "hasTokenIdVoted", abi = "hasTokenIdVoted(uint256)")]
    pub struct HasTokenIdVotedCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `hasVoted` function with signature `hasVoted(uint256)` and selector `[236, 202, 3, 31]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "hasVoted", abi = "hasVoted(uint256)")]
    pub struct HasVotedCall {
        pub index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `isQueued` function with signature `isQueued()` and selector `[27, 37, 222, 143]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "isQueued", abi = "isQueued()")]
    pub struct IsQueuedCall;
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
    #[doc = "Container type for all input parameters for the `merkleRoot` function with signature `merkleRoot()` and selector `[46, 180, 167, 171]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "merkleRoot", abi = "merkleRoot()")]
    pub struct MerkleRootCall;
    #[doc = "Container type for all input parameters for the `noVoteCount` function with signature `noVoteCount()` and selector `[145, 110, 98, 243]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "noVoteCount", abi = "noVoteCount()")]
    pub struct NoVoteCountCall;
    #[doc = "Container type for all input parameters for the `ownerAddress` function with signature `ownerAddress()` and selector `[143, 132, 170, 9]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "ownerAddress", abi = "ownerAddress()")]
    pub struct OwnerAddressCall;
    #[doc = "Container type for all input parameters for the `queue` function with signature `queue()` and selector `[225, 13, 41, 238]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "queue", abi = "queue()")]
    pub struct QueueCall;
    #[doc = "Container type for all input parameters for the `quorumVotes` function with signature `quorumVotes()` and selector `[36, 188, 26, 100]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "quorumVotes", abi = "quorumVotes()")]
    pub struct QuorumVotesCall;
    #[doc = "Container type for all input parameters for the `voltzFactory` function with signature `voltzFactory()` and selector `[254, 59, 149, 145]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "voltzFactory", abi = "voltzFactory()")]
    pub struct VoltzFactoryCall;
    #[doc = "Container type for all input parameters for the `yesVoteCount` function with signature `yesVoteCount()` and selector `[68, 160, 179, 231]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "yesVoteCount", abi = "yesVoteCount()")]
    pub struct YesVoteCountCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum CommunityDeployerCalls {
        TimelockPeriodInSeconds(TimelockPeriodInSecondsCall),
        BlockTimestampTimelockEnd(BlockTimestampTimelockEndCall),
        BlockTimestampVotingEnd(BlockTimestampVotingEndCall),
        CastVote(CastVoteCall),
        Deploy(DeployCall),
        HasTokenIdVoted(HasTokenIdVotedCall),
        HasVoted(HasVotedCall),
        IsQueued(IsQueuedCall),
        MasterMarginEngine(MasterMarginEngineCall),
        MasterVAMM(MasterVAMMCall),
        MerkleRoot(MerkleRootCall),
        NoVoteCount(NoVoteCountCall),
        OwnerAddress(OwnerAddressCall),
        Queue(QueueCall),
        QuorumVotes(QuorumVotesCall),
        VoltzFactory(VoltzFactoryCall),
        YesVoteCount(YesVoteCountCall),
    }
    impl ethers::core::abi::AbiDecode for CommunityDeployerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <TimelockPeriodInSecondsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommunityDeployerCalls::TimelockPeriodInSeconds(decoded));
            }
            if let Ok(decoded) =
                <BlockTimestampTimelockEndCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CommunityDeployerCalls::BlockTimestampTimelockEnd(decoded));
            }
            if let Ok(decoded) =
                <BlockTimestampVotingEndCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommunityDeployerCalls::BlockTimestampVotingEnd(decoded));
            }
            if let Ok(decoded) =
                <CastVoteCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommunityDeployerCalls::CastVote(decoded));
            }
            if let Ok(decoded) = <DeployCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommunityDeployerCalls::Deploy(decoded));
            }
            if let Ok(decoded) =
                <HasTokenIdVotedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommunityDeployerCalls::HasTokenIdVoted(decoded));
            }
            if let Ok(decoded) =
                <HasVotedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommunityDeployerCalls::HasVoted(decoded));
            }
            if let Ok(decoded) =
                <IsQueuedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommunityDeployerCalls::IsQueued(decoded));
            }
            if let Ok(decoded) =
                <MasterMarginEngineCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommunityDeployerCalls::MasterMarginEngine(decoded));
            }
            if let Ok(decoded) =
                <MasterVAMMCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommunityDeployerCalls::MasterVAMM(decoded));
            }
            if let Ok(decoded) =
                <MerkleRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommunityDeployerCalls::MerkleRoot(decoded));
            }
            if let Ok(decoded) =
                <NoVoteCountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommunityDeployerCalls::NoVoteCount(decoded));
            }
            if let Ok(decoded) =
                <OwnerAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommunityDeployerCalls::OwnerAddress(decoded));
            }
            if let Ok(decoded) = <QueueCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommunityDeployerCalls::Queue(decoded));
            }
            if let Ok(decoded) =
                <QuorumVotesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommunityDeployerCalls::QuorumVotes(decoded));
            }
            if let Ok(decoded) =
                <VoltzFactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommunityDeployerCalls::VoltzFactory(decoded));
            }
            if let Ok(decoded) =
                <YesVoteCountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommunityDeployerCalls::YesVoteCount(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for CommunityDeployerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                CommunityDeployerCalls::TimelockPeriodInSeconds(element) => element.encode(),
                CommunityDeployerCalls::BlockTimestampTimelockEnd(element) => element.encode(),
                CommunityDeployerCalls::BlockTimestampVotingEnd(element) => element.encode(),
                CommunityDeployerCalls::CastVote(element) => element.encode(),
                CommunityDeployerCalls::Deploy(element) => element.encode(),
                CommunityDeployerCalls::HasTokenIdVoted(element) => element.encode(),
                CommunityDeployerCalls::HasVoted(element) => element.encode(),
                CommunityDeployerCalls::IsQueued(element) => element.encode(),
                CommunityDeployerCalls::MasterMarginEngine(element) => element.encode(),
                CommunityDeployerCalls::MasterVAMM(element) => element.encode(),
                CommunityDeployerCalls::MerkleRoot(element) => element.encode(),
                CommunityDeployerCalls::NoVoteCount(element) => element.encode(),
                CommunityDeployerCalls::OwnerAddress(element) => element.encode(),
                CommunityDeployerCalls::Queue(element) => element.encode(),
                CommunityDeployerCalls::QuorumVotes(element) => element.encode(),
                CommunityDeployerCalls::VoltzFactory(element) => element.encode(),
                CommunityDeployerCalls::YesVoteCount(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for CommunityDeployerCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CommunityDeployerCalls::TimelockPeriodInSeconds(element) => element.fmt(f),
                CommunityDeployerCalls::BlockTimestampTimelockEnd(element) => element.fmt(f),
                CommunityDeployerCalls::BlockTimestampVotingEnd(element) => element.fmt(f),
                CommunityDeployerCalls::CastVote(element) => element.fmt(f),
                CommunityDeployerCalls::Deploy(element) => element.fmt(f),
                CommunityDeployerCalls::HasTokenIdVoted(element) => element.fmt(f),
                CommunityDeployerCalls::HasVoted(element) => element.fmt(f),
                CommunityDeployerCalls::IsQueued(element) => element.fmt(f),
                CommunityDeployerCalls::MasterMarginEngine(element) => element.fmt(f),
                CommunityDeployerCalls::MasterVAMM(element) => element.fmt(f),
                CommunityDeployerCalls::MerkleRoot(element) => element.fmt(f),
                CommunityDeployerCalls::NoVoteCount(element) => element.fmt(f),
                CommunityDeployerCalls::OwnerAddress(element) => element.fmt(f),
                CommunityDeployerCalls::Queue(element) => element.fmt(f),
                CommunityDeployerCalls::QuorumVotes(element) => element.fmt(f),
                CommunityDeployerCalls::VoltzFactory(element) => element.fmt(f),
                CommunityDeployerCalls::YesVoteCount(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<TimelockPeriodInSecondsCall> for CommunityDeployerCalls {
        fn from(var: TimelockPeriodInSecondsCall) -> Self {
            CommunityDeployerCalls::TimelockPeriodInSeconds(var)
        }
    }
    impl ::std::convert::From<BlockTimestampTimelockEndCall> for CommunityDeployerCalls {
        fn from(var: BlockTimestampTimelockEndCall) -> Self {
            CommunityDeployerCalls::BlockTimestampTimelockEnd(var)
        }
    }
    impl ::std::convert::From<BlockTimestampVotingEndCall> for CommunityDeployerCalls {
        fn from(var: BlockTimestampVotingEndCall) -> Self {
            CommunityDeployerCalls::BlockTimestampVotingEnd(var)
        }
    }
    impl ::std::convert::From<CastVoteCall> for CommunityDeployerCalls {
        fn from(var: CastVoteCall) -> Self {
            CommunityDeployerCalls::CastVote(var)
        }
    }
    impl ::std::convert::From<DeployCall> for CommunityDeployerCalls {
        fn from(var: DeployCall) -> Self {
            CommunityDeployerCalls::Deploy(var)
        }
    }
    impl ::std::convert::From<HasTokenIdVotedCall> for CommunityDeployerCalls {
        fn from(var: HasTokenIdVotedCall) -> Self {
            CommunityDeployerCalls::HasTokenIdVoted(var)
        }
    }
    impl ::std::convert::From<HasVotedCall> for CommunityDeployerCalls {
        fn from(var: HasVotedCall) -> Self {
            CommunityDeployerCalls::HasVoted(var)
        }
    }
    impl ::std::convert::From<IsQueuedCall> for CommunityDeployerCalls {
        fn from(var: IsQueuedCall) -> Self {
            CommunityDeployerCalls::IsQueued(var)
        }
    }
    impl ::std::convert::From<MasterMarginEngineCall> for CommunityDeployerCalls {
        fn from(var: MasterMarginEngineCall) -> Self {
            CommunityDeployerCalls::MasterMarginEngine(var)
        }
    }
    impl ::std::convert::From<MasterVAMMCall> for CommunityDeployerCalls {
        fn from(var: MasterVAMMCall) -> Self {
            CommunityDeployerCalls::MasterVAMM(var)
        }
    }
    impl ::std::convert::From<MerkleRootCall> for CommunityDeployerCalls {
        fn from(var: MerkleRootCall) -> Self {
            CommunityDeployerCalls::MerkleRoot(var)
        }
    }
    impl ::std::convert::From<NoVoteCountCall> for CommunityDeployerCalls {
        fn from(var: NoVoteCountCall) -> Self {
            CommunityDeployerCalls::NoVoteCount(var)
        }
    }
    impl ::std::convert::From<OwnerAddressCall> for CommunityDeployerCalls {
        fn from(var: OwnerAddressCall) -> Self {
            CommunityDeployerCalls::OwnerAddress(var)
        }
    }
    impl ::std::convert::From<QueueCall> for CommunityDeployerCalls {
        fn from(var: QueueCall) -> Self {
            CommunityDeployerCalls::Queue(var)
        }
    }
    impl ::std::convert::From<QuorumVotesCall> for CommunityDeployerCalls {
        fn from(var: QuorumVotesCall) -> Self {
            CommunityDeployerCalls::QuorumVotes(var)
        }
    }
    impl ::std::convert::From<VoltzFactoryCall> for CommunityDeployerCalls {
        fn from(var: VoltzFactoryCall) -> Self {
            CommunityDeployerCalls::VoltzFactory(var)
        }
    }
    impl ::std::convert::From<YesVoteCountCall> for CommunityDeployerCalls {
        fn from(var: YesVoteCountCall) -> Self {
            CommunityDeployerCalls::YesVoteCount(var)
        }
    }
    #[doc = "Container type for all return fields from the `TIMELOCK_PERIOD_IN_SECONDS` function with signature `TIMELOCK_PERIOD_IN_SECONDS()` and selector `[120, 209, 119, 192]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TimelockPeriodInSecondsReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `blockTimestampTimelockEnd` function with signature `blockTimestampTimelockEnd()` and selector `[181, 85, 3, 151]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BlockTimestampTimelockEndReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `blockTimestampVotingEnd` function with signature `blockTimestampVotingEnd()` and selector `[180, 57, 95, 131]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BlockTimestampVotingEndReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `hasTokenIdVoted` function with signature `hasTokenIdVoted(uint256)` and selector `[184, 242, 20, 95]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct HasTokenIdVotedReturn(pub bool);
    #[doc = "Container type for all return fields from the `hasVoted` function with signature `hasVoted(uint256)` and selector `[236, 202, 3, 31]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct HasVotedReturn(pub bool);
    #[doc = "Container type for all return fields from the `isQueued` function with signature `isQueued()` and selector `[27, 37, 222, 143]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsQueuedReturn(pub bool);
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
    #[doc = "Container type for all return fields from the `merkleRoot` function with signature `merkleRoot()` and selector `[46, 180, 167, 171]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MerkleRootReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `noVoteCount` function with signature `noVoteCount()` and selector `[145, 110, 98, 243]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct NoVoteCountReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `ownerAddress` function with signature `ownerAddress()` and selector `[143, 132, 170, 9]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct OwnerAddressReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `quorumVotes` function with signature `quorumVotes()` and selector `[36, 188, 26, 100]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct QuorumVotesReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `voltzFactory` function with signature `voltzFactory()` and selector `[254, 59, 149, 145]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct VoltzFactoryReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `yesVoteCount` function with signature `yesVoteCount()` and selector `[68, 160, 179, 231]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct YesVoteCountReturn(pub ethers::core::types::U256);
}
