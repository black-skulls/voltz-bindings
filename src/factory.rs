pub use factory::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod factory {
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
    #[doc = "Factory was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static FACTORY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"contract IMarginEngine\",\"name\":\"_masterMarginEngine\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IVAMM\",\"name\":\"_masterVAMM\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"AavePoolGetReserveNormalizedIncomeReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"AavePoolGetReserveNormalizedVariableDebtReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CTokenExchangeRateReturnedZero\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"unlocked\",\"type\":\"bool\",\"components\":[]}],\"type\":\"error\",\"name\":\"CanOnlyTradeIfUnlocked\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CannotLiquidate\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CannotSettleBeforeMaturity\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"DebugError\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"amount0\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"amount1\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"ExpectedOppositeSigns\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint160\",\"name\":\"sqrtPriceX96\",\"type\":\"uint160\",\"components\":[]}],\"type\":\"error\",\"name\":\"ExpectedSqrtPriceZeroBeforeInit\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"IRSNotionalAmountSpecifiedMustBeNonZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidMarginDelta\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"LidoGetPooledEthBySharesReturnedZero\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"type\":\"error\",\"name\":\"LiquidityDeltaMustBePositiveInBurn\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"type\":\"error\",\"name\":\"LiquidityDeltaMustBePositiveInMint\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginLessThanMinimum\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tick\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"variableTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"cumulativeFeeIncurred\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenDeltaUnbalanced\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginRequirementNotMet\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginRequirementNotMetFCM\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"requested\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"available\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"NotEnoughFunds\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OOO\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyFCM\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyMarginEngine\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyOwnerCanUpdatePosition\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyVAMM\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PositionNetZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PositionNotSettled\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"RocketPoolGetEthValueReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"WithdrawalExceedsCurrentMargin\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"closeToOrBeyondMaturity\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"intAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"isApproved\",\"type\":\"bool\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract IERC20Minimal\",\"name\":\"underlyingToken\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"contract IRateOracle\",\"name\":\"rateOracle\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"termStartTimestampWad\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"termEndTimestampWad\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"int24\",\"name\":\"tickSpacing\",\"type\":\"int24\",\"components\":[],\"indexed\":false},{\"internalType\":\"contract IMarginEngine\",\"name\":\"marginEngine\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"contract IVAMM\",\"name\":\"vamm\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"contract IFCM\",\"name\":\"fcm\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint8\",\"name\":\"yieldBearingProtocolID\",\"type\":\"uint8\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint8\",\"name\":\"underlyingTokenDecimals\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"IrsInstance\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract IFCM\",\"name\":\"masterFCMAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint8\",\"name\":\"yieldBearingProtocolID\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MasterFCM\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract IPeriphery\",\"name\":\"periphery\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PeripheryUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract IERC20Minimal\",\"name\":\"_underlyingToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IRateOracle\",\"name\":\"_rateOracle\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_termStartTimestampWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_termEndTimestampWad\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"_tickSpacing\",\"type\":\"int24\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deployIrsInstance\",\"outputs\":[{\"internalType\":\"contract IMarginEngine\",\"name\":\"marginEngineProxy\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IVAMM\",\"name\":\"vammProxy\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IFCM\",\"name\":\"fcmProxy\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_intAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isApproved\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"masterFCMs\",\"outputs\":[{\"internalType\":\"contract IFCM\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"masterMarginEngine\",\"outputs\":[{\"internalType\":\"contract IMarginEngine\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"masterVAMM\",\"outputs\":[{\"internalType\":\"contract IVAMM\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"periphery\",\"outputs\":[{\"internalType\":\"contract IPeriphery\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"intAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"allowIntegration\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setApproval\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract IFCM\",\"name\":\"_masterFCM\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"_yieldBearingProtocolID\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setMasterFCM\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract IMarginEngine\",\"name\":\"_masterMarginEngine\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setMasterMarginEngine\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract IVAMM\",\"name\":\"_masterVAMM\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setMasterVAMM\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract IPeriphery\",\"name\":\"_periphery\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPeriphery\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static FACTORY_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60806040523480156200001157600080fd5b50604051620019bc380380620019bc83398101604081905262000034916200018e565b6200003f3362000125565b6001600160a01b0382166200009b5760405162461bcd60e51b815260206004820152601460248201527f6d6173746572206d65206d75737420657869737400000000000000000000000060448201526064015b60405180910390fd5b6001600160a01b038116620000f35760405162461bcd60e51b815260206004820152601660248201527f6d61737465722076616d6d206d75737420657869737400000000000000000000604482015260640162000092565b600180546001600160a01b039384166001600160a01b03199182161790915560028054929093169116179055620001cd565b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b6001600160a01b03811681146200018b57600080fd5b50565b60008060408385031215620001a257600080fd5b8251620001af8162000175565b6020840151909250620001c28162000175565b809150509250929050565b6117df80620001dd6000396000f3fe60806040523480156200001157600080fd5b5060043610620000fd5760003560e01c80639fe1b3541162000097578063ca5183b7116200006e578063ca5183b7146200022b578063db9b71701462000257578063f2fde38b146200026e578063febfe75e146200028557600080fd5b80639fe1b35414620001d8578063a389783e14620001ec578063aeb22934146200021457600080fd5b80638da5cb5b11620000d85780638da5cb5b146200018157806394a9b1f7146200019357806395858f9814620001aa578063973cd93114620001c157600080fd5b80630e8a06481462000102578063715018a6146200014857806377aace1a1462000154575b600080fd5b620001196200011336600462000ec5565b62000299565b604080516001600160a01b03948516815292841660208401529216918101919091526060015b60405180910390f35b6200015262000969565b005b60055462000168906001600160a01b031681565b6040516001600160a01b0390911681526020016200013f565b6000546001600160a01b031662000168565b62000152620001a436600462000f31565b62000981565b62000152620001bb36600462000f68565b62000a11565b62000152620001d236600462000f31565b62000ad8565b60015462000168906001600160a01b031681565b62000203620001fd36600462000fa6565b62000b67565b60405190151581526020016200013f565b620001526200022536600462000f31565b62000c56565b620001686200023c36600462000fd9565b6003602052600090815260409020546001600160a01b031681565b620001526200026836600462000ff9565b62000d19565b620001526200027f36600462000f31565b62000d79565b60025462000168906001600160a01b031681565b6000806000620002a862000df5565b6001546040516000916001600160a01b031690620002c69062000ea1565b6001600160a01b039091168152604060208201819052600090820152606001604051809103906000f08015801562000302573d6000803e3d6000fd5b506002546040519192506000916001600160a01b0390911690620003269062000ea1565b6001600160a01b039091168152604060208201819052600090820152606001604051809103906000f08015801562000362573d6000803e3d6000fd5b5060405163eb990c5960e01b81526001600160a01b038c811660048301528b81166024830152604482018b9052606482018a90529192509083169063eb990c5990608401600060405180830381600087803b158015620003c157600080fd5b505af1158015620003d6573d6000803e3d6000fd5b5050604051631b325b2160e31b81526001600160a01b03858116600483015260028a900b60248301528416925063d992d9089150604401600060405180830381600087803b1580156200042857600080fd5b505af11580156200043d573d6000803e3d6000fd5b50506040516331d81ea760e21b81526001600160a01b0384811660048301528516925063c7607a9c9150602401600060405180830381600087803b1580156200048557600080fd5b505af11580156200049a573d6000803e3d6000fd5b5050505060008990508a6001600160a01b0316816001600160a01b0316636f307dc36040518163ffffffff1660e01b815260040160206040518083038186803b158015620004e757600080fd5b505afa158015620004fc573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019062000522919062001030565b6001600160a01b031614620005745760405162461bcd60e51b81526020600482015260136024820152720a8ded6cadce640c8de40dcdee840dac2e8c6d606b1b60448201526064015b60405180910390fd5b6000816001600160a01b03166322ff65686040518163ffffffff1660e01b815260040160206040518083038186803b158015620005b057600080fd5b505afa158015620005c5573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620005eb919062001050565b60ff81166000908152600360205260408120549192506001600160a01b03909116908115620007825781604051620006239062000ea1565b6001600160a01b039091168152604060208201819052600090820152606001604051809103906000f0801580156200065f573d6000803e3d6000fd5b5060405163485cc95560e01b81526001600160a01b03878116600483015288811660248301529192509082169063485cc95590604401600060405180830381600087803b158015620006b057600080fd5b505af1158015620006c5573d6000803e3d6000fd5b505060405163534d337560e01b81526001600160a01b0384811660048301528916925063534d33759150602401600060405180830381600087803b1580156200070d57600080fd5b505af115801562000722573d6000803e3d6000fd5b505060405163f2fde38b60e01b81523360048201526001600160a01b038416925063f2fde38b9150602401600060405180830381600087803b1580156200076857600080fd5b505af11580156200077d573d6000803e3d6000fd5b505050505b60008e6001600160a01b031663313ce5676040518163ffffffff1660e01b815260040160206040518083038186803b158015620007be57600080fd5b505afa158015620007d3573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620007f9919062001050565b90508d6001600160a01b03168f6001600160a01b03167fe134804702afa0f02bd7f0687d4c2f662a1790b4904d1c2cd6f41fcffbfc05c38f8f8f8c8c898c8a60405162000893989796959493929190978852602088019690965260029490940b60408701526001600160a01b03928316606087015290821660808601521660a084015260ff90811660c08401521660e08201526101000190565b60405180910390a360405163f2fde38b60e01b81523360048201526001600160a01b0387169063f2fde38b90602401600060405180830381600087803b158015620008dd57600080fd5b505af1158015620008f2573d6000803e3d6000fd5b505060405163f2fde38b60e01b81523360048201526001600160a01b038a16925063f2fde38b9150602401600060405180830381600087803b1580156200093857600080fd5b505af11580156200094d573d6000803e3d6000fd5b50989b5096995091975050505050505050955095509592505050565b6200097362000df5565b6200097f600062000e51565b565b6200098b62000df5565b6001600160a01b038116620009dc5760405162461bcd60e51b81526020600482015260166024820152751b585cdd195c881d985b5b481b5d5cdd08195e1a5cdd60521b60448201526064016200056b565b6002546001600160a01b0382811691161462000a0e57600280546001600160a01b0319166001600160a01b0383161790555b50565b62000a1b62000df5565b6001600160a01b03821662000a6b5760405162461bcd60e51b81526020600482015260156024820152741b585cdd195c881998db481b5d5cdd08195e1a5cdd605a1b60448201526064016200056b565b60ff811660008181526003602090815260409182902080546001600160a01b0319166001600160a01b0387169081179091558251908152908101929092527fcece9976caa53e350e3311c87df72b4ed94d768ba03a4688cdf331121bf613c2910160405180910390a15050565b62000ae262000df5565b6001600160a01b03811662000b315760405162461bcd60e51b81526020600482015260146024820152731b585cdd195c881b59481b5d5cdd08195e1a5cdd60621b60448201526064016200056b565b6001546001600160a01b0382811691161462000a0e57600180546001600160a01b0383166001600160a01b031990911617905550565b60006001600160a01b03831662000bb85760405162461bcd60e51b81526020600482015260146024820152731bdddb995c88191bd95cc81b9bdd08195e1a5cdd60621b60448201526064016200056b565b6001600160a01b03821662000c055760405162461bcd60e51b81526020600482015260126024820152711a5b9d08191bd95cc81b9bdd08195e1a5cdd60721b60448201526064016200056b565b6005546001600160a01b038381169116141562000c255750600162000c50565b506001600160a01b0380831660009081526004602090815260408083209385168352929052205460ff165b92915050565b62000c6062000df5565b6001600160a01b03811662000caf5760405162461bcd60e51b81526020600482015260146024820152731c195c9a5c1a195c9e481b5d5cdd08195e1a5cdd60621b60448201526064016200056b565b6005546001600160a01b0382811691161462000a0e57600580546001600160a01b0319166001600160a01b0383169081179091556040519081527f9d41c928e1b7d55eb20c9906b2cc3223035dd4dc2c59e5fcd2282e857db1b0509060200160405180910390a150565b3360008181526004602090815260408083206001600160a01b0387168085529252808320805460ff19168615159081179091559051909391927f1d3e246ebbc933bf65d3290db9f93d67ab91a12d2b19308a35806e04d1c174c591a45050565b62000d8362000df5565b6001600160a01b03811662000dea5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b60648201526084016200056b565b62000a0e8162000e51565b6000546001600160a01b031633146200097f5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016200056b565b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b610739806200107183390190565b6001600160a01b038116811462000a0e57600080fd5b600080600080600060a0868803121562000ede57600080fd5b853562000eeb8162000eaf565b9450602086013562000efd8162000eaf565b935060408601359250606086013591506080860135600281900b811462000f2357600080fd5b809150509295509295909350565b60006020828403121562000f4457600080fd5b813562000f518162000eaf565b9392505050565b60ff8116811462000a0e57600080fd5b6000806040838503121562000f7c57600080fd5b823562000f898162000eaf565b9150602083013562000f9b8162000f58565b809150509250929050565b6000806040838503121562000fba57600080fd5b823562000fc78162000eaf565b9150602083013562000f9b8162000eaf565b60006020828403121562000fec57600080fd5b813562000f518162000f58565b600080604083850312156200100d57600080fd5b82356200101a8162000eaf565b91506020830135801515811462000f9b57600080fd5b6000602082840312156200104357600080fd5b815162000f518162000eaf565b6000602082840312156200106357600080fd5b815162000f518162000f5856fe608060405260405161073938038061073983398101604081905261002291610322565b818161003082826000610039565b5050505061043f565b6100428361006f565b60008251118061004f5750805b1561006a5761006883836100af60201b6100291760201c565b505b505050565b610078816100db565b6040516001600160a01b038216907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b90600090a250565b60606100d48383604051806060016040528060278152602001610712602791396101ad565b9392505050565b6100ee8161022660201b6100551760201c565b6101555760405162461bcd60e51b815260206004820152602d60248201527f455243313936373a206e657720696d706c656d656e746174696f6e206973206e60448201526c1bdd08184818dbdb9d1c9858dd609a1b60648201526084015b60405180910390fd5b8061018c7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc60001b61023560201b6100641760201c565b80546001600160a01b0319166001600160a01b039290921691909117905550565b6060600080856001600160a01b0316856040516101ca91906103f0565b600060405180830381855af49150503d8060008114610205576040519150601f19603f3d011682016040523d82523d6000602084013e61020a565b606091505b50909250905061021c86838387610238565b9695505050505050565b6001600160a01b03163b151590565b90565b606083156102a457825161029d576001600160a01b0385163b61029d5760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e7472616374000000604482015260640161014c565b50816102ae565b6102ae83836102b6565b949350505050565b8151156102c65781518083602001fd5b8060405162461bcd60e51b815260040161014c919061040c565b634e487b7160e01b600052604160045260246000fd5b60005b838110156103115781810151838201526020016102f9565b838111156100685750506000910152565b6000806040838503121561033557600080fd5b82516001600160a01b038116811461034c57600080fd5b60208401519092506001600160401b038082111561036957600080fd5b818501915085601f83011261037d57600080fd5b81518181111561038f5761038f6102e0565b604051601f8201601f19908116603f011681019083821181831017156103b7576103b76102e0565b816040528281528860208487010111156103d057600080fd5b6103e18360208301602088016102f6565b80955050505050509250929050565b600082516104028184602087016102f6565b9190910192915050565b602081526000825180602084015261042b8160408501602087016102f6565b601f01601f19169190910160400192915050565b6102c48061044e6000396000f3fe60806040523661001357610011610017565b005b6100115b610027610022610067565b61009f565b565b606061004e8383604051806060016040528060278152602001610268602791396100c3565b9392505050565b6001600160a01b03163b151590565b90565b600061009a7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc546001600160a01b031690565b905090565b3660008037600080366000845af43d6000803e8080156100be573d6000f35b3d6000fd5b6060600080856001600160a01b0316856040516100e09190610218565b600060405180830381855af49150503d806000811461011b576040519150601f19603f3d011682016040523d82523d6000602084013e610120565b606091505b50915091506101318683838761013b565b9695505050505050565b606083156101ac5782516101a5576001600160a01b0385163b6101a55760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e747261637400000060448201526064015b60405180910390fd5b50816101b6565b6101b683836101be565b949350505050565b8151156101ce5781518083602001fd5b8060405162461bcd60e51b815260040161019c9190610234565b60005b838110156102035781810151838201526020016101eb565b83811115610212576000848401525b50505050565b6000825161022a8184602087016101e8565b9190910192915050565b60208152600082518060208401526102538160408501602087016101e8565b601f01601f1916919091016040019291505056fe416464726573733a206c6f772d6c6576656c2064656c65676174652063616c6c206661696c6564a2646970667358221220c9e739666436e4f4e46675a1f8f09b880cbc87b9dadbd6b81e1e432db303b85564736f6c63430008090033416464726573733a206c6f772d6c6576656c2064656c65676174652063616c6c206661696c6564a2646970667358221220956886011e6f38767996d82c907389cef820eefa65e487621fa9ee4bf050a0d364736f6c63430008090033" . parse () . expect ("invalid bytecode")
        });
    pub struct Factory<M>(ethers::contract::Contract<M>);
    impl<M> Clone for Factory<M> {
        fn clone(&self) -> Self {
            Factory(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Factory<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for Factory<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Factory))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> Factory<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), FACTORY_ABI.clone(), client).into()
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
                FACTORY_ABI.clone(),
                FACTORY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
            p0: u8,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([202, 81, 131, 183], p0)
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
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
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
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
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
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(
            &self,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
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
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PeripheryUpdate` event"]
        pub fn periphery_update_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PeripheryUpdateFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, FactoryEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Factory<M> {
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
    pub enum FactoryErrors {
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
    impl ethers::core::abi::AbiDecode for FactoryErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok (decoded) = < AavePoolGetReserveNormalizedIncomeReturnedZero as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (FactoryErrors :: AavePoolGetReserveNormalizedIncomeReturnedZero (decoded)) }
            if let Ok (decoded) = < AavePoolGetReserveNormalizedVariableDebtReturnedZero as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (FactoryErrors :: AavePoolGetReserveNormalizedVariableDebtReturnedZero (decoded)) }
            if let Ok(decoded) =
                <CTokenExchangeRateReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FactoryErrors::CTokenExchangeRateReturnedZero(decoded));
            }
            if let Ok(decoded) =
                <CanOnlyTradeIfUnlocked as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FactoryErrors::CanOnlyTradeIfUnlocked(decoded));
            }
            if let Ok(decoded) =
                <CannotLiquidate as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FactoryErrors::CannotLiquidate(decoded));
            }
            if let Ok(decoded) =
                <CannotSettleBeforeMaturity as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FactoryErrors::CannotSettleBeforeMaturity(decoded));
            }
            if let Ok(decoded) = <DebugError as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FactoryErrors::DebugError(decoded));
            }
            if let Ok(decoded) =
                <ExpectedOppositeSigns as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FactoryErrors::ExpectedOppositeSigns(decoded));
            }
            if let Ok(decoded) =
                <ExpectedSqrtPriceZeroBeforeInit as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FactoryErrors::ExpectedSqrtPriceZeroBeforeInit(decoded));
            }
            if let Ok(decoded) =
                <IRSNotionalAmountSpecifiedMustBeNonZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FactoryErrors::IRSNotionalAmountSpecifiedMustBeNonZero(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <InvalidMarginDelta as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FactoryErrors::InvalidMarginDelta(decoded));
            }
            if let Ok(decoded) =
                <LidoGetPooledEthBySharesReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FactoryErrors::LidoGetPooledEthBySharesReturnedZero(decoded));
            }
            if let Ok(decoded) =
                <LiquidityDeltaMustBePositiveInBurn as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FactoryErrors::LiquidityDeltaMustBePositiveInBurn(decoded));
            }
            if let Ok(decoded) =
                <LiquidityDeltaMustBePositiveInMint as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FactoryErrors::LiquidityDeltaMustBePositiveInMint(decoded));
            }
            if let Ok(decoded) =
                <MarginLessThanMinimum as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FactoryErrors::MarginLessThanMinimum(decoded));
            }
            if let Ok(decoded) =
                <MarginRequirementNotMet as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FactoryErrors::MarginRequirementNotMet(decoded));
            }
            if let Ok(decoded) =
                <MarginRequirementNotMetFCM as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FactoryErrors::MarginRequirementNotMetFCM(decoded));
            }
            if let Ok(decoded) =
                <NotEnoughFunds as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FactoryErrors::NotEnoughFunds(decoded));
            }
            if let Ok(decoded) = <OOO as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(FactoryErrors::OOO(decoded));
            }
            if let Ok(decoded) = <OnlyFCM as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(FactoryErrors::OnlyFCM(decoded));
            }
            if let Ok(decoded) =
                <OnlyMarginEngine as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FactoryErrors::OnlyMarginEngine(decoded));
            }
            if let Ok(decoded) =
                <OnlyOwnerCanUpdatePosition as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FactoryErrors::OnlyOwnerCanUpdatePosition(decoded));
            }
            if let Ok(decoded) = <OnlyVAMM as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(FactoryErrors::OnlyVAMM(decoded));
            }
            if let Ok(decoded) =
                <PositionNetZero as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FactoryErrors::PositionNetZero(decoded));
            }
            if let Ok(decoded) =
                <PositionNotSettled as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FactoryErrors::PositionNotSettled(decoded));
            }
            if let Ok(decoded) =
                <RocketPoolGetEthValueReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FactoryErrors::RocketPoolGetEthValueReturnedZero(decoded));
            }
            if let Ok(decoded) =
                <WithdrawalExceedsCurrentMargin as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FactoryErrors::WithdrawalExceedsCurrentMargin(decoded));
            }
            if let Ok(decoded) =
                <closeToOrBeyondMaturity as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FactoryErrors::closeToOrBeyondMaturity(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for FactoryErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                FactoryErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(element) => {
                    element.encode()
                }
                FactoryErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(element) => {
                    element.encode()
                }
                FactoryErrors::CTokenExchangeRateReturnedZero(element) => element.encode(),
                FactoryErrors::CanOnlyTradeIfUnlocked(element) => element.encode(),
                FactoryErrors::CannotLiquidate(element) => element.encode(),
                FactoryErrors::CannotSettleBeforeMaturity(element) => element.encode(),
                FactoryErrors::DebugError(element) => element.encode(),
                FactoryErrors::ExpectedOppositeSigns(element) => element.encode(),
                FactoryErrors::ExpectedSqrtPriceZeroBeforeInit(element) => element.encode(),
                FactoryErrors::IRSNotionalAmountSpecifiedMustBeNonZero(element) => element.encode(),
                FactoryErrors::InvalidMarginDelta(element) => element.encode(),
                FactoryErrors::LidoGetPooledEthBySharesReturnedZero(element) => element.encode(),
                FactoryErrors::LiquidityDeltaMustBePositiveInBurn(element) => element.encode(),
                FactoryErrors::LiquidityDeltaMustBePositiveInMint(element) => element.encode(),
                FactoryErrors::MarginLessThanMinimum(element) => element.encode(),
                FactoryErrors::MarginRequirementNotMet(element) => element.encode(),
                FactoryErrors::MarginRequirementNotMetFCM(element) => element.encode(),
                FactoryErrors::NotEnoughFunds(element) => element.encode(),
                FactoryErrors::OOO(element) => element.encode(),
                FactoryErrors::OnlyFCM(element) => element.encode(),
                FactoryErrors::OnlyMarginEngine(element) => element.encode(),
                FactoryErrors::OnlyOwnerCanUpdatePosition(element) => element.encode(),
                FactoryErrors::OnlyVAMM(element) => element.encode(),
                FactoryErrors::PositionNetZero(element) => element.encode(),
                FactoryErrors::PositionNotSettled(element) => element.encode(),
                FactoryErrors::RocketPoolGetEthValueReturnedZero(element) => element.encode(),
                FactoryErrors::WithdrawalExceedsCurrentMargin(element) => element.encode(),
                FactoryErrors::closeToOrBeyondMaturity(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for FactoryErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                FactoryErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(element) => {
                    element.fmt(f)
                }
                FactoryErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(element) => {
                    element.fmt(f)
                }
                FactoryErrors::CTokenExchangeRateReturnedZero(element) => element.fmt(f),
                FactoryErrors::CanOnlyTradeIfUnlocked(element) => element.fmt(f),
                FactoryErrors::CannotLiquidate(element) => element.fmt(f),
                FactoryErrors::CannotSettleBeforeMaturity(element) => element.fmt(f),
                FactoryErrors::DebugError(element) => element.fmt(f),
                FactoryErrors::ExpectedOppositeSigns(element) => element.fmt(f),
                FactoryErrors::ExpectedSqrtPriceZeroBeforeInit(element) => element.fmt(f),
                FactoryErrors::IRSNotionalAmountSpecifiedMustBeNonZero(element) => element.fmt(f),
                FactoryErrors::InvalidMarginDelta(element) => element.fmt(f),
                FactoryErrors::LidoGetPooledEthBySharesReturnedZero(element) => element.fmt(f),
                FactoryErrors::LiquidityDeltaMustBePositiveInBurn(element) => element.fmt(f),
                FactoryErrors::LiquidityDeltaMustBePositiveInMint(element) => element.fmt(f),
                FactoryErrors::MarginLessThanMinimum(element) => element.fmt(f),
                FactoryErrors::MarginRequirementNotMet(element) => element.fmt(f),
                FactoryErrors::MarginRequirementNotMetFCM(element) => element.fmt(f),
                FactoryErrors::NotEnoughFunds(element) => element.fmt(f),
                FactoryErrors::OOO(element) => element.fmt(f),
                FactoryErrors::OnlyFCM(element) => element.fmt(f),
                FactoryErrors::OnlyMarginEngine(element) => element.fmt(f),
                FactoryErrors::OnlyOwnerCanUpdatePosition(element) => element.fmt(f),
                FactoryErrors::OnlyVAMM(element) => element.fmt(f),
                FactoryErrors::PositionNetZero(element) => element.fmt(f),
                FactoryErrors::PositionNotSettled(element) => element.fmt(f),
                FactoryErrors::RocketPoolGetEthValueReturnedZero(element) => element.fmt(f),
                FactoryErrors::WithdrawalExceedsCurrentMargin(element) => element.fmt(f),
                FactoryErrors::closeToOrBeyondMaturity(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AavePoolGetReserveNormalizedIncomeReturnedZero> for FactoryErrors {
        fn from(var: AavePoolGetReserveNormalizedIncomeReturnedZero) -> Self {
            FactoryErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(var)
        }
    }
    impl ::std::convert::From<AavePoolGetReserveNormalizedVariableDebtReturnedZero> for FactoryErrors {
        fn from(var: AavePoolGetReserveNormalizedVariableDebtReturnedZero) -> Self {
            FactoryErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(var)
        }
    }
    impl ::std::convert::From<CTokenExchangeRateReturnedZero> for FactoryErrors {
        fn from(var: CTokenExchangeRateReturnedZero) -> Self {
            FactoryErrors::CTokenExchangeRateReturnedZero(var)
        }
    }
    impl ::std::convert::From<CanOnlyTradeIfUnlocked> for FactoryErrors {
        fn from(var: CanOnlyTradeIfUnlocked) -> Self {
            FactoryErrors::CanOnlyTradeIfUnlocked(var)
        }
    }
    impl ::std::convert::From<CannotLiquidate> for FactoryErrors {
        fn from(var: CannotLiquidate) -> Self {
            FactoryErrors::CannotLiquidate(var)
        }
    }
    impl ::std::convert::From<CannotSettleBeforeMaturity> for FactoryErrors {
        fn from(var: CannotSettleBeforeMaturity) -> Self {
            FactoryErrors::CannotSettleBeforeMaturity(var)
        }
    }
    impl ::std::convert::From<DebugError> for FactoryErrors {
        fn from(var: DebugError) -> Self {
            FactoryErrors::DebugError(var)
        }
    }
    impl ::std::convert::From<ExpectedOppositeSigns> for FactoryErrors {
        fn from(var: ExpectedOppositeSigns) -> Self {
            FactoryErrors::ExpectedOppositeSigns(var)
        }
    }
    impl ::std::convert::From<ExpectedSqrtPriceZeroBeforeInit> for FactoryErrors {
        fn from(var: ExpectedSqrtPriceZeroBeforeInit) -> Self {
            FactoryErrors::ExpectedSqrtPriceZeroBeforeInit(var)
        }
    }
    impl ::std::convert::From<IRSNotionalAmountSpecifiedMustBeNonZero> for FactoryErrors {
        fn from(var: IRSNotionalAmountSpecifiedMustBeNonZero) -> Self {
            FactoryErrors::IRSNotionalAmountSpecifiedMustBeNonZero(var)
        }
    }
    impl ::std::convert::From<InvalidMarginDelta> for FactoryErrors {
        fn from(var: InvalidMarginDelta) -> Self {
            FactoryErrors::InvalidMarginDelta(var)
        }
    }
    impl ::std::convert::From<LidoGetPooledEthBySharesReturnedZero> for FactoryErrors {
        fn from(var: LidoGetPooledEthBySharesReturnedZero) -> Self {
            FactoryErrors::LidoGetPooledEthBySharesReturnedZero(var)
        }
    }
    impl ::std::convert::From<LiquidityDeltaMustBePositiveInBurn> for FactoryErrors {
        fn from(var: LiquidityDeltaMustBePositiveInBurn) -> Self {
            FactoryErrors::LiquidityDeltaMustBePositiveInBurn(var)
        }
    }
    impl ::std::convert::From<LiquidityDeltaMustBePositiveInMint> for FactoryErrors {
        fn from(var: LiquidityDeltaMustBePositiveInMint) -> Self {
            FactoryErrors::LiquidityDeltaMustBePositiveInMint(var)
        }
    }
    impl ::std::convert::From<MarginLessThanMinimum> for FactoryErrors {
        fn from(var: MarginLessThanMinimum) -> Self {
            FactoryErrors::MarginLessThanMinimum(var)
        }
    }
    impl ::std::convert::From<MarginRequirementNotMet> for FactoryErrors {
        fn from(var: MarginRequirementNotMet) -> Self {
            FactoryErrors::MarginRequirementNotMet(var)
        }
    }
    impl ::std::convert::From<MarginRequirementNotMetFCM> for FactoryErrors {
        fn from(var: MarginRequirementNotMetFCM) -> Self {
            FactoryErrors::MarginRequirementNotMetFCM(var)
        }
    }
    impl ::std::convert::From<NotEnoughFunds> for FactoryErrors {
        fn from(var: NotEnoughFunds) -> Self {
            FactoryErrors::NotEnoughFunds(var)
        }
    }
    impl ::std::convert::From<OOO> for FactoryErrors {
        fn from(var: OOO) -> Self {
            FactoryErrors::OOO(var)
        }
    }
    impl ::std::convert::From<OnlyFCM> for FactoryErrors {
        fn from(var: OnlyFCM) -> Self {
            FactoryErrors::OnlyFCM(var)
        }
    }
    impl ::std::convert::From<OnlyMarginEngine> for FactoryErrors {
        fn from(var: OnlyMarginEngine) -> Self {
            FactoryErrors::OnlyMarginEngine(var)
        }
    }
    impl ::std::convert::From<OnlyOwnerCanUpdatePosition> for FactoryErrors {
        fn from(var: OnlyOwnerCanUpdatePosition) -> Self {
            FactoryErrors::OnlyOwnerCanUpdatePosition(var)
        }
    }
    impl ::std::convert::From<OnlyVAMM> for FactoryErrors {
        fn from(var: OnlyVAMM) -> Self {
            FactoryErrors::OnlyVAMM(var)
        }
    }
    impl ::std::convert::From<PositionNetZero> for FactoryErrors {
        fn from(var: PositionNetZero) -> Self {
            FactoryErrors::PositionNetZero(var)
        }
    }
    impl ::std::convert::From<PositionNotSettled> for FactoryErrors {
        fn from(var: PositionNotSettled) -> Self {
            FactoryErrors::PositionNotSettled(var)
        }
    }
    impl ::std::convert::From<RocketPoolGetEthValueReturnedZero> for FactoryErrors {
        fn from(var: RocketPoolGetEthValueReturnedZero) -> Self {
            FactoryErrors::RocketPoolGetEthValueReturnedZero(var)
        }
    }
    impl ::std::convert::From<WithdrawalExceedsCurrentMargin> for FactoryErrors {
        fn from(var: WithdrawalExceedsCurrentMargin) -> Self {
            FactoryErrors::WithdrawalExceedsCurrentMargin(var)
        }
    }
    impl ::std::convert::From<closeToOrBeyondMaturity> for FactoryErrors {
        fn from(var: closeToOrBeyondMaturity) -> Self {
            FactoryErrors::closeToOrBeyondMaturity(var)
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
    #[ethevent(name = "PeripheryUpdate", abi = "PeripheryUpdate(address)")]
    pub struct PeripheryUpdateFilter {
        pub periphery: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum FactoryEvents {
        ApprovalFilter(ApprovalFilter),
        IrsInstanceFilter(IrsInstanceFilter),
        MasterFCMFilter(MasterFCMFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PeripheryUpdateFilter(PeripheryUpdateFilter),
    }
    impl ethers::contract::EthLogDecode for FactoryEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(FactoryEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = IrsInstanceFilter::decode_log(log) {
                return Ok(FactoryEvents::IrsInstanceFilter(decoded));
            }
            if let Ok(decoded) = MasterFCMFilter::decode_log(log) {
                return Ok(FactoryEvents::MasterFCMFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(FactoryEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PeripheryUpdateFilter::decode_log(log) {
                return Ok(FactoryEvents::PeripheryUpdateFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for FactoryEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                FactoryEvents::ApprovalFilter(element) => element.fmt(f),
                FactoryEvents::IrsInstanceFilter(element) => element.fmt(f),
                FactoryEvents::MasterFCMFilter(element) => element.fmt(f),
                FactoryEvents::OwnershipTransferredFilter(element) => element.fmt(f),
                FactoryEvents::PeripheryUpdateFilter(element) => element.fmt(f),
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
    pub struct MasterFCMsCall(pub u8);
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum FactoryCalls {
        DeployIrsInstance(DeployIrsInstanceCall),
        IsApproved(IsApprovedCall),
        MasterFCMs(MasterFCMsCall),
        MasterMarginEngine(MasterMarginEngineCall),
        MasterVAMM(MasterVAMMCall),
        Owner(OwnerCall),
        Periphery(PeripheryCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetApproval(SetApprovalCall),
        SetMasterFCM(SetMasterFCMCall),
        SetMasterMarginEngine(SetMasterMarginEngineCall),
        SetMasterVAMM(SetMasterVAMMCall),
        SetPeriphery(SetPeripheryCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ethers::core::abi::AbiDecode for FactoryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DeployIrsInstanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FactoryCalls::DeployIrsInstance(decoded));
            }
            if let Ok(decoded) =
                <IsApprovedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FactoryCalls::IsApproved(decoded));
            }
            if let Ok(decoded) =
                <MasterFCMsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FactoryCalls::MasterFCMs(decoded));
            }
            if let Ok(decoded) =
                <MasterMarginEngineCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FactoryCalls::MasterMarginEngine(decoded));
            }
            if let Ok(decoded) =
                <MasterVAMMCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FactoryCalls::MasterVAMM(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FactoryCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <PeripheryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FactoryCalls::Periphery(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FactoryCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <SetApprovalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FactoryCalls::SetApproval(decoded));
            }
            if let Ok(decoded) =
                <SetMasterFCMCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FactoryCalls::SetMasterFCM(decoded));
            }
            if let Ok(decoded) =
                <SetMasterMarginEngineCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FactoryCalls::SetMasterMarginEngine(decoded));
            }
            if let Ok(decoded) =
                <SetMasterVAMMCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FactoryCalls::SetMasterVAMM(decoded));
            }
            if let Ok(decoded) =
                <SetPeripheryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FactoryCalls::SetPeriphery(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FactoryCalls::TransferOwnership(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for FactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                FactoryCalls::DeployIrsInstance(element) => element.encode(),
                FactoryCalls::IsApproved(element) => element.encode(),
                FactoryCalls::MasterFCMs(element) => element.encode(),
                FactoryCalls::MasterMarginEngine(element) => element.encode(),
                FactoryCalls::MasterVAMM(element) => element.encode(),
                FactoryCalls::Owner(element) => element.encode(),
                FactoryCalls::Periphery(element) => element.encode(),
                FactoryCalls::RenounceOwnership(element) => element.encode(),
                FactoryCalls::SetApproval(element) => element.encode(),
                FactoryCalls::SetMasterFCM(element) => element.encode(),
                FactoryCalls::SetMasterMarginEngine(element) => element.encode(),
                FactoryCalls::SetMasterVAMM(element) => element.encode(),
                FactoryCalls::SetPeriphery(element) => element.encode(),
                FactoryCalls::TransferOwnership(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for FactoryCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                FactoryCalls::DeployIrsInstance(element) => element.fmt(f),
                FactoryCalls::IsApproved(element) => element.fmt(f),
                FactoryCalls::MasterFCMs(element) => element.fmt(f),
                FactoryCalls::MasterMarginEngine(element) => element.fmt(f),
                FactoryCalls::MasterVAMM(element) => element.fmt(f),
                FactoryCalls::Owner(element) => element.fmt(f),
                FactoryCalls::Periphery(element) => element.fmt(f),
                FactoryCalls::RenounceOwnership(element) => element.fmt(f),
                FactoryCalls::SetApproval(element) => element.fmt(f),
                FactoryCalls::SetMasterFCM(element) => element.fmt(f),
                FactoryCalls::SetMasterMarginEngine(element) => element.fmt(f),
                FactoryCalls::SetMasterVAMM(element) => element.fmt(f),
                FactoryCalls::SetPeriphery(element) => element.fmt(f),
                FactoryCalls::TransferOwnership(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DeployIrsInstanceCall> for FactoryCalls {
        fn from(var: DeployIrsInstanceCall) -> Self {
            FactoryCalls::DeployIrsInstance(var)
        }
    }
    impl ::std::convert::From<IsApprovedCall> for FactoryCalls {
        fn from(var: IsApprovedCall) -> Self {
            FactoryCalls::IsApproved(var)
        }
    }
    impl ::std::convert::From<MasterFCMsCall> for FactoryCalls {
        fn from(var: MasterFCMsCall) -> Self {
            FactoryCalls::MasterFCMs(var)
        }
    }
    impl ::std::convert::From<MasterMarginEngineCall> for FactoryCalls {
        fn from(var: MasterMarginEngineCall) -> Self {
            FactoryCalls::MasterMarginEngine(var)
        }
    }
    impl ::std::convert::From<MasterVAMMCall> for FactoryCalls {
        fn from(var: MasterVAMMCall) -> Self {
            FactoryCalls::MasterVAMM(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for FactoryCalls {
        fn from(var: OwnerCall) -> Self {
            FactoryCalls::Owner(var)
        }
    }
    impl ::std::convert::From<PeripheryCall> for FactoryCalls {
        fn from(var: PeripheryCall) -> Self {
            FactoryCalls::Periphery(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for FactoryCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            FactoryCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<SetApprovalCall> for FactoryCalls {
        fn from(var: SetApprovalCall) -> Self {
            FactoryCalls::SetApproval(var)
        }
    }
    impl ::std::convert::From<SetMasterFCMCall> for FactoryCalls {
        fn from(var: SetMasterFCMCall) -> Self {
            FactoryCalls::SetMasterFCM(var)
        }
    }
    impl ::std::convert::From<SetMasterMarginEngineCall> for FactoryCalls {
        fn from(var: SetMasterMarginEngineCall) -> Self {
            FactoryCalls::SetMasterMarginEngine(var)
        }
    }
    impl ::std::convert::From<SetMasterVAMMCall> for FactoryCalls {
        fn from(var: SetMasterVAMMCall) -> Self {
            FactoryCalls::SetMasterVAMM(var)
        }
    }
    impl ::std::convert::From<SetPeripheryCall> for FactoryCalls {
        fn from(var: SetPeripheryCall) -> Self {
            FactoryCalls::SetPeriphery(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for FactoryCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            FactoryCalls::TransferOwnership(var)
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
    pub struct MasterFCMsReturn(pub ethers::core::types::Address);
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
