pub use actor::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod actor {
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
    #[doc = "Actor was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ACTOR_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"type\":\"error\",\"name\":\"AavePoolGetReserveNormalizedIncomeReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"AavePoolGetReserveNormalizedVariableDebtReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CTokenExchangeRateReturnedZero\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"unlocked\",\"type\":\"bool\",\"components\":[]}],\"type\":\"error\",\"name\":\"CanOnlyTradeIfUnlocked\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CannotLiquidate\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CannotSettleBeforeMaturity\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"DebugError\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"amount0\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"amount1\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"ExpectedOppositeSigns\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint160\",\"name\":\"sqrtPriceX96\",\"type\":\"uint160\",\"components\":[]}],\"type\":\"error\",\"name\":\"ExpectedSqrtPriceZeroBeforeInit\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"IRSNotionalAmountSpecifiedMustBeNonZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidMarginDelta\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"LidoGetPooledEthBySharesReturnedZero\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"type\":\"error\",\"name\":\"LiquidityDeltaMustBePositiveInBurn\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"type\":\"error\",\"name\":\"LiquidityDeltaMustBePositiveInMint\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginLessThanMinimum\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tick\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"variableTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"cumulativeFeeIncurred\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"fixedTokenDeltaUnbalanced\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginRequirementNotMet\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"marginRequirement\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MarginRequirementNotMetFCM\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"requested\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"available\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"NotEnoughFunds\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OOO\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyFCM\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyMarginEngine\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyOwnerCanUpdatePosition\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyVAMM\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PositionNetZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PositionNotSettled\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"RocketPoolGetEthValueReturnedZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"WithdrawalExceedsCurrentMargin\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"closeToOrBeyondMaturity\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"VAMMAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burnViaAMM\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"FCMAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"notional\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceLimitX96\",\"type\":\"uint160\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initiateFullyCollateralisedFixedTakerSwap\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"MEAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"liquidatePosition\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"peripheryAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"struct IPeriphery.MintOrBurnParams\",\"name\":\"params\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"contract IMarginEngine\",\"name\":\"marginEngine\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"notional\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isMint\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"marginDelta\",\"type\":\"int256\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"mintOrBurnViaPeriphery\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"positionMarginRequirement\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"VAMMAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mintViaAMM\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"positionMarginRequirement\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"MEAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"intAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"allowIntegration\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setIntegrationApproval\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"MEAdrress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"settlePosition\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"MEAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"settlePositionViaAMM\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"FCMAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"settleYBATrader\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"VAMMAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"struct IVAMM.SwapParams\",\"name\":\"params\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"amountSpecified\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceLimitX96\",\"type\":\"uint160\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"swapViaAMM\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"_fixedTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_variableTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_cumulativeFeeIncurred\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_fixedTokenDeltaUnbalanced\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_marginRequirement\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"peripheryAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"struct IPeriphery.SwapPeripheryParams\",\"name\":\"params\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"contract IMarginEngine\",\"name\":\"marginEngine\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isFT\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"notional\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceLimitX96\",\"type\":\"uint160\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"marginDelta\",\"type\":\"int256\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"swapViaPeriphery\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"_fixedTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_variableTokenDelta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_cumulativeFeeIncurred\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_fixedTokenDeltaUnbalanced\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_marginRequirement\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"FCMAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"notionalToUnwind\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceLimitX96\",\"type\":\"uint160\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unwindFullyCollateralisedFixedTakerSwap\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"MEAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"marginDelta\",\"type\":\"int256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updatePositionMarginViaAMM\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ACTOR_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b50610fcb806100206000396000f3fe6080604052600436106100c25760003560e01c806369696dbf1161007f5780638a8763b4116100595780638a8763b4146101f7578063cacdd72314610217578063e9ae4bc814610237578063f23543901461025757600080fd5b806369696dbf1461019657806383e345e7146101b6578063840047eb146101d757600080fd5b8063044ed392146100c7578063292a60d514610114578063303958f31461013657806331bdf73e146101565780634028369814610176578063633755e114610136575b600080fd5b3480156100d357600080fd5b506100e76100e23660046109f3565b61026a565b604080519586526020860194909452928401919091526060830152608082015260a0015b60405180910390f35b34801561012057600080fd5b5061013461012f366004610ab9565b610336565b005b34801561014257600080fd5b50610134610151366004610b36565b6103c2565b34801561016257600080fd5b50610134610171366004610b92565b610436565b34801561018257600080fd5b50610134610191366004610c0b565b6104b1565b3480156101a257600080fd5b506101346101b1366004610c52565b61055e565b6101c96101c4366004610c94565b6105ea565b60405190815260200161010b565b3480156101e357600080fd5b506101346101f2366004610d34565b6106af565b34801561020357600080fd5b506101c9610212366004610ab9565b610726565b34801561022357600080fd5b50610134610232366004610d51565b6107b7565b34801561024357600080fd5b50610134610252366004610c52565b61084b565b6100e7610265366004610da2565b61087f565b604080516333bac73760e11b815282516001600160a01b0390811660048301526020840151602483015291830151821660448201526060830151600290810b60648301526080840151900b608482015260009182918291829182918816906367758e6e9060a40160a060405180830381600087803b1580156102eb57600080fd5b505af11580156102ff573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906103239190610e56565b939b929a50909850965090945092505050565b604051631f2f089360e01b81526001600160a01b03861690631f2f089390610368908790879087908790600401610e96565b602060405180830381600087803b15801561038257600080fd5b505af1158015610396573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906103ba9190610ec9565b505050505050565b60405163a725b96560e01b81526001600160a01b038481166004830152600284810b602484015283900b604483015285169063a725b965906064015b600060405180830381600087803b15801561041857600080fd5b505af115801561042c573d6000803e3d6000fd5b5050505050505050565b604051637717797f60e01b81526001600160a01b038581166004830152600285810b602484015284900b604483015260648201839052861690637717797f90608401600060405180830381600087803b15801561049257600080fd5b505af11580156104a6573d6000803e3d6000fd5b505050505050505050565b6000836001600160a01b031663c45a01556040518163ffffffff1660e01b815260040160206040518083038186803b1580156104ec57600080fd5b505afa158015610500573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105249190610ee2565b604051630db9b71760e41b81526001600160a01b03858116600483015284151560248301529192509082169063db9b7170906044016103fe565b6040516355468a8b60e01b8152600481018390526001600160a01b0382811660248301528416906355468a8b906044015b608060405180830381600087803b1580156105a957600080fd5b505af11580156105bd573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105e19190610eff565b50505050505050565b604080516332e00daf60e01b815282516001600160a01b0390811660048301526020840151600290810b60248401529284015190920b60448201526060830151606482015260808301511515608482015260a083015160a48201526000918416906332e00daf90349060c4016020604051808303818588803b15801561066f57600080fd5b505af1158015610683573d6000803e3d6000fd5b50505050506040513d601f19601f820116820180604052508101906106a89190610ec9565b9392505050565b806001600160a01b031663ebc9b02e6040518163ffffffff1660e01b8152600401602060405180830381600087803b1580156106ea57600080fd5b505af11580156106fe573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107229190610ec9565b5050565b604051635c6651a760e11b81526000906001600160a01b0387169063b8cca34e9061075b908890889088908890600401610e96565b602060405180830381600087803b15801561077557600080fd5b505af1158015610789573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107ad9190610ec9565b9695505050505050565b604051630b2b281f60e11b81526001600160a01b038281166004830152600285810b602484015284900b6044830152851690631656503e90606401602060405180830381600087803b15801561080c57600080fd5b505af1158015610820573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906108449190610ec9565b5050505050565b60405162dd089d60e21b8152600481018390526001600160a01b03828116602483015284169063037422749060440161058f565b6040805163932e0eff60e01b815282516001600160a01b0390811660048301526020840151151560248301529183015160448201526060830151821660648201526080830151600290810b608483015260a0840151900b60a482015260c083015160c4820152600091829182918291829188169063932e0eff90349060e40160e0604051808303818588803b15801561091757600080fd5b505af115801561092b573d6000803e3d6000fd5b50505050506040513d601f19601f820116820180604052508101906109509190610f35565b50949c939b50919950975095509350505050565b6001600160a01b038116811461097957600080fd5b50565b60405160c0810167ffffffffffffffff811182821017156109ad57634e487b7160e01b600052604160045260246000fd5b60405290565b60405160e0810167ffffffffffffffff811182821017156109ad57634e487b7160e01b600052604160045260246000fd5b8060020b811461097957600080fd5b60008082840360c0811215610a0757600080fd5b8335610a1281610964565b925060a0601f1982011215610a2657600080fd5b5060405160a0810181811067ffffffffffffffff82111715610a5857634e487b7160e01b600052604160045260246000fd5b6040526020840135610a6981610964565b8152604084013560208201526060840135610a8381610964565b60408201526080840135610a96816109e4565b606082015260a0840135610aa9816109e4565b6080820152919491935090915050565b600080600080600060a08688031215610ad157600080fd5b8535610adc81610964565b94506020860135610aec81610964565b93506040860135610afc816109e4565b92506060860135610b0c816109e4565b915060808601356001600160801b0381168114610b2857600080fd5b809150509295509295909350565b60008060008060808587031215610b4c57600080fd5b8435610b5781610964565b93506020850135610b6781610964565b92506040850135610b77816109e4565b91506060850135610b87816109e4565b939692955090935050565b600080600080600060a08688031215610baa57600080fd5b8535610bb581610964565b94506020860135610bc581610964565b93506040860135610bd5816109e4565b92506060860135610be5816109e4565b949793965091946080013592915050565b80358015158114610c0657600080fd5b919050565b600080600060608486031215610c2057600080fd5b8335610c2b81610964565b92506020840135610c3b81610964565b9150610c4960408501610bf6565b90509250925092565b600080600060608486031215610c6757600080fd5b8335610c7281610964565b9250602084013591506040840135610c8981610964565b809150509250925092565b60008082840360e0811215610ca857600080fd5b8335610cb381610964565b925060c0601f1982011215610cc757600080fd5b50610cd061097c565b6020840135610cde81610964565b81526040840135610cee816109e4565b60208201526060840135610d01816109e4565b604082015260808401356060820152610d1c60a08501610bf6565b608082015260c0939093013560a08401525092909150565b600060208284031215610d4657600080fd5b81356106a881610964565b60008060008060808587031215610d6757600080fd5b8435610d7281610964565b93506020850135610d82816109e4565b92506040850135610d92816109e4565b91506060850135610b8781610964565b600080828403610100811215610db757600080fd5b8335610dc281610964565b925060e0601f1982011215610dd657600080fd5b50610ddf6109b3565b6020840135610ded81610964565b8152610dfb60408501610bf6565b6020820152606084013560408201526080840135610e1881610964565b606082015260a0840135610e2b816109e4565b608082015260c0840135610e3e816109e4565b60a082015260e0939093013560c08401525092909150565b600080600080600060a08688031215610e6e57600080fd5b5050835160208501516040860151606087015160809097015192989197509594509092509050565b6001600160a01b03949094168452600292830b6020850152910b60408301526001600160801b0316606082015260800190565b600060208284031215610edb57600080fd5b5051919050565b600060208284031215610ef457600080fd5b81516106a881610964565b60008060008060808587031215610f1557600080fd5b505082516020840151604085015160609095015191969095509092509050565b600080600080600080600060e0888a031215610f5057600080fd5b875196506020880151955060408801519450606088015193506080880151925060a0880151610f7e816109e4565b8092505060c088015190509295989194975092955056fea26469706673582212206715d8818e261ce0fd8ad10f83e5c6c702e21a02e42edda169bc8990cf15541864736f6c63430008090033" . parse () . expect ("invalid bytecode")
        });
    pub struct Actor<M>(ethers::contract::Contract<M>);
    impl<M> Clone for Actor<M> {
        fn clone(&self) -> Self {
            Actor(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Actor<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for Actor<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Actor))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> Actor<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ACTOR_ABI.clone(), client).into()
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
                ACTOR_ABI.clone(),
                ACTOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `burnViaAMM` (0x292a60d5) function"]
        pub fn burn_via_amm(
            &self,
            vamm_address: ethers::core::types::Address,
            recipient: ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
            amount: u128,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [41, 42, 96, 213],
                    (vamm_address, recipient, tick_lower, tick_upper, amount),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initiateFullyCollateralisedFixedTakerSwap` (0x69696dbf) function"]
        pub fn initiate_fully_collateralised_fixed_taker_swap(
            &self,
            fcm_address: ethers::core::types::Address,
            notional: ethers::core::types::U256,
            sqrt_price_limit_x96: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [105, 105, 109, 191],
                    (fcm_address, notional, sqrt_price_limit_x96),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liquidatePosition` (0xcacdd723) function"]
        pub fn liquidate_position(
            &self,
            me_address: ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
            owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [202, 205, 215, 35],
                    (me_address, tick_lower, tick_upper, owner),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mintOrBurnViaPeriphery` (0x83e345e7) function"]
        pub fn mint_or_burn_via_periphery(
            &self,
            periphery_address: ethers::core::types::Address,
            params: MintOrBurnParams,
        ) -> ethers::contract::builders::ContractCall<M, I256> {
            self.0
                .method_hash([131, 227, 69, 231], (periphery_address, params))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mintViaAMM` (0x8a8763b4) function"]
        pub fn mint_via_amm(
            &self,
            vamm_address: ethers::core::types::Address,
            recipient: ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
            amount: u128,
        ) -> ethers::contract::builders::ContractCall<M, I256> {
            self.0
                .method_hash(
                    [138, 135, 99, 180],
                    (vamm_address, recipient, tick_lower, tick_upper, amount),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setIntegrationApproval` (0x40283698) function"]
        pub fn set_integration_approval(
            &self,
            me_address: ethers::core::types::Address,
            int_address: ethers::core::types::Address,
            allow_integration: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [64, 40, 54, 152],
                    (me_address, int_address, allow_integration),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `settlePosition` (0x633755e1) function"]
        pub fn settle_position(
            &self,
            me_adrress: ethers::core::types::Address,
            recipient: ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [99, 55, 85, 225],
                    (me_adrress, recipient, tick_lower, tick_upper),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `settlePositionViaAMM` (0x303958f3) function"]
        pub fn settle_position_via_amm(
            &self,
            me_address: ethers::core::types::Address,
            owner: ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [48, 57, 88, 243],
                    (me_address, owner, tick_lower, tick_upper),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `settleYBATrader` (0x840047eb) function"]
        pub fn settle_yba_trader(
            &self,
            fcm_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 0, 71, 235], fcm_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapViaAMM` (0x044ed392) function"]
        pub fn swap_via_amm(
            &self,
            vamm_address: ethers::core::types::Address,
            params: SwapParams,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (I256, I256, ethers::core::types::U256, I256, I256),
        > {
            self.0
                .method_hash([4, 78, 211, 146], (vamm_address, params))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapViaPeriphery` (0xf2354390) function"]
        pub fn swap_via_periphery(
            &self,
            periphery_address: ethers::core::types::Address,
            params: SwapPeripheryParams,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (I256, I256, ethers::core::types::U256, I256, I256),
        > {
            self.0
                .method_hash([242, 53, 67, 144], (periphery_address, params))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unwindFullyCollateralisedFixedTakerSwap` (0xe9ae4bc8) function"]
        pub fn unwind_fully_collateralised_fixed_taker_swap(
            &self,
            fcm_address: ethers::core::types::Address,
            notional_to_unwind: ethers::core::types::U256,
            sqrt_price_limit_x96: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [233, 174, 75, 200],
                    (fcm_address, notional_to_unwind, sqrt_price_limit_x96),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updatePositionMarginViaAMM` (0x31bdf73e) function"]
        pub fn update_position_margin_via_amm(
            &self,
            me_address: ethers::core::types::Address,
            owner: ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
            margin_delta: I256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [49, 189, 247, 62],
                    (me_address, owner, tick_lower, tick_upper, margin_delta),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Actor<M> {
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
    pub enum ActorErrors {
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
    impl ethers::core::abi::AbiDecode for ActorErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok (decoded) = < AavePoolGetReserveNormalizedIncomeReturnedZero as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (ActorErrors :: AavePoolGetReserveNormalizedIncomeReturnedZero (decoded)) }
            if let Ok (decoded) = < AavePoolGetReserveNormalizedVariableDebtReturnedZero as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (ActorErrors :: AavePoolGetReserveNormalizedVariableDebtReturnedZero (decoded)) }
            if let Ok(decoded) =
                <CTokenExchangeRateReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ActorErrors::CTokenExchangeRateReturnedZero(decoded));
            }
            if let Ok(decoded) =
                <CanOnlyTradeIfUnlocked as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ActorErrors::CanOnlyTradeIfUnlocked(decoded));
            }
            if let Ok(decoded) =
                <CannotLiquidate as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ActorErrors::CannotLiquidate(decoded));
            }
            if let Ok(decoded) =
                <CannotSettleBeforeMaturity as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ActorErrors::CannotSettleBeforeMaturity(decoded));
            }
            if let Ok(decoded) = <DebugError as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ActorErrors::DebugError(decoded));
            }
            if let Ok(decoded) =
                <ExpectedOppositeSigns as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ActorErrors::ExpectedOppositeSigns(decoded));
            }
            if let Ok(decoded) =
                <ExpectedSqrtPriceZeroBeforeInit as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ActorErrors::ExpectedSqrtPriceZeroBeforeInit(decoded));
            }
            if let Ok(decoded) =
                <IRSNotionalAmountSpecifiedMustBeNonZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ActorErrors::IRSNotionalAmountSpecifiedMustBeNonZero(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <InvalidMarginDelta as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ActorErrors::InvalidMarginDelta(decoded));
            }
            if let Ok(decoded) =
                <LidoGetPooledEthBySharesReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ActorErrors::LidoGetPooledEthBySharesReturnedZero(decoded));
            }
            if let Ok(decoded) =
                <LiquidityDeltaMustBePositiveInBurn as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ActorErrors::LiquidityDeltaMustBePositiveInBurn(decoded));
            }
            if let Ok(decoded) =
                <LiquidityDeltaMustBePositiveInMint as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ActorErrors::LiquidityDeltaMustBePositiveInMint(decoded));
            }
            if let Ok(decoded) =
                <MarginLessThanMinimum as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ActorErrors::MarginLessThanMinimum(decoded));
            }
            if let Ok(decoded) =
                <MarginRequirementNotMet as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ActorErrors::MarginRequirementNotMet(decoded));
            }
            if let Ok(decoded) =
                <MarginRequirementNotMetFCM as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ActorErrors::MarginRequirementNotMetFCM(decoded));
            }
            if let Ok(decoded) =
                <NotEnoughFunds as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ActorErrors::NotEnoughFunds(decoded));
            }
            if let Ok(decoded) = <OOO as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ActorErrors::OOO(decoded));
            }
            if let Ok(decoded) = <OnlyFCM as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ActorErrors::OnlyFCM(decoded));
            }
            if let Ok(decoded) =
                <OnlyMarginEngine as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ActorErrors::OnlyMarginEngine(decoded));
            }
            if let Ok(decoded) =
                <OnlyOwnerCanUpdatePosition as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ActorErrors::OnlyOwnerCanUpdatePosition(decoded));
            }
            if let Ok(decoded) = <OnlyVAMM as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ActorErrors::OnlyVAMM(decoded));
            }
            if let Ok(decoded) =
                <PositionNetZero as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ActorErrors::PositionNetZero(decoded));
            }
            if let Ok(decoded) =
                <PositionNotSettled as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ActorErrors::PositionNotSettled(decoded));
            }
            if let Ok(decoded) =
                <RocketPoolGetEthValueReturnedZero as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ActorErrors::RocketPoolGetEthValueReturnedZero(decoded));
            }
            if let Ok(decoded) =
                <WithdrawalExceedsCurrentMargin as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ActorErrors::WithdrawalExceedsCurrentMargin(decoded));
            }
            if let Ok(decoded) =
                <closeToOrBeyondMaturity as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ActorErrors::closeToOrBeyondMaturity(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ActorErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                ActorErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(element) => {
                    element.encode()
                }
                ActorErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(element) => {
                    element.encode()
                }
                ActorErrors::CTokenExchangeRateReturnedZero(element) => element.encode(),
                ActorErrors::CanOnlyTradeIfUnlocked(element) => element.encode(),
                ActorErrors::CannotLiquidate(element) => element.encode(),
                ActorErrors::CannotSettleBeforeMaturity(element) => element.encode(),
                ActorErrors::DebugError(element) => element.encode(),
                ActorErrors::ExpectedOppositeSigns(element) => element.encode(),
                ActorErrors::ExpectedSqrtPriceZeroBeforeInit(element) => element.encode(),
                ActorErrors::IRSNotionalAmountSpecifiedMustBeNonZero(element) => element.encode(),
                ActorErrors::InvalidMarginDelta(element) => element.encode(),
                ActorErrors::LidoGetPooledEthBySharesReturnedZero(element) => element.encode(),
                ActorErrors::LiquidityDeltaMustBePositiveInBurn(element) => element.encode(),
                ActorErrors::LiquidityDeltaMustBePositiveInMint(element) => element.encode(),
                ActorErrors::MarginLessThanMinimum(element) => element.encode(),
                ActorErrors::MarginRequirementNotMet(element) => element.encode(),
                ActorErrors::MarginRequirementNotMetFCM(element) => element.encode(),
                ActorErrors::NotEnoughFunds(element) => element.encode(),
                ActorErrors::OOO(element) => element.encode(),
                ActorErrors::OnlyFCM(element) => element.encode(),
                ActorErrors::OnlyMarginEngine(element) => element.encode(),
                ActorErrors::OnlyOwnerCanUpdatePosition(element) => element.encode(),
                ActorErrors::OnlyVAMM(element) => element.encode(),
                ActorErrors::PositionNetZero(element) => element.encode(),
                ActorErrors::PositionNotSettled(element) => element.encode(),
                ActorErrors::RocketPoolGetEthValueReturnedZero(element) => element.encode(),
                ActorErrors::WithdrawalExceedsCurrentMargin(element) => element.encode(),
                ActorErrors::closeToOrBeyondMaturity(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ActorErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ActorErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(element) => {
                    element.fmt(f)
                }
                ActorErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(element) => {
                    element.fmt(f)
                }
                ActorErrors::CTokenExchangeRateReturnedZero(element) => element.fmt(f),
                ActorErrors::CanOnlyTradeIfUnlocked(element) => element.fmt(f),
                ActorErrors::CannotLiquidate(element) => element.fmt(f),
                ActorErrors::CannotSettleBeforeMaturity(element) => element.fmt(f),
                ActorErrors::DebugError(element) => element.fmt(f),
                ActorErrors::ExpectedOppositeSigns(element) => element.fmt(f),
                ActorErrors::ExpectedSqrtPriceZeroBeforeInit(element) => element.fmt(f),
                ActorErrors::IRSNotionalAmountSpecifiedMustBeNonZero(element) => element.fmt(f),
                ActorErrors::InvalidMarginDelta(element) => element.fmt(f),
                ActorErrors::LidoGetPooledEthBySharesReturnedZero(element) => element.fmt(f),
                ActorErrors::LiquidityDeltaMustBePositiveInBurn(element) => element.fmt(f),
                ActorErrors::LiquidityDeltaMustBePositiveInMint(element) => element.fmt(f),
                ActorErrors::MarginLessThanMinimum(element) => element.fmt(f),
                ActorErrors::MarginRequirementNotMet(element) => element.fmt(f),
                ActorErrors::MarginRequirementNotMetFCM(element) => element.fmt(f),
                ActorErrors::NotEnoughFunds(element) => element.fmt(f),
                ActorErrors::OOO(element) => element.fmt(f),
                ActorErrors::OnlyFCM(element) => element.fmt(f),
                ActorErrors::OnlyMarginEngine(element) => element.fmt(f),
                ActorErrors::OnlyOwnerCanUpdatePosition(element) => element.fmt(f),
                ActorErrors::OnlyVAMM(element) => element.fmt(f),
                ActorErrors::PositionNetZero(element) => element.fmt(f),
                ActorErrors::PositionNotSettled(element) => element.fmt(f),
                ActorErrors::RocketPoolGetEthValueReturnedZero(element) => element.fmt(f),
                ActorErrors::WithdrawalExceedsCurrentMargin(element) => element.fmt(f),
                ActorErrors::closeToOrBeyondMaturity(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AavePoolGetReserveNormalizedIncomeReturnedZero> for ActorErrors {
        fn from(var: AavePoolGetReserveNormalizedIncomeReturnedZero) -> Self {
            ActorErrors::AavePoolGetReserveNormalizedIncomeReturnedZero(var)
        }
    }
    impl ::std::convert::From<AavePoolGetReserveNormalizedVariableDebtReturnedZero> for ActorErrors {
        fn from(var: AavePoolGetReserveNormalizedVariableDebtReturnedZero) -> Self {
            ActorErrors::AavePoolGetReserveNormalizedVariableDebtReturnedZero(var)
        }
    }
    impl ::std::convert::From<CTokenExchangeRateReturnedZero> for ActorErrors {
        fn from(var: CTokenExchangeRateReturnedZero) -> Self {
            ActorErrors::CTokenExchangeRateReturnedZero(var)
        }
    }
    impl ::std::convert::From<CanOnlyTradeIfUnlocked> for ActorErrors {
        fn from(var: CanOnlyTradeIfUnlocked) -> Self {
            ActorErrors::CanOnlyTradeIfUnlocked(var)
        }
    }
    impl ::std::convert::From<CannotLiquidate> for ActorErrors {
        fn from(var: CannotLiquidate) -> Self {
            ActorErrors::CannotLiquidate(var)
        }
    }
    impl ::std::convert::From<CannotSettleBeforeMaturity> for ActorErrors {
        fn from(var: CannotSettleBeforeMaturity) -> Self {
            ActorErrors::CannotSettleBeforeMaturity(var)
        }
    }
    impl ::std::convert::From<DebugError> for ActorErrors {
        fn from(var: DebugError) -> Self {
            ActorErrors::DebugError(var)
        }
    }
    impl ::std::convert::From<ExpectedOppositeSigns> for ActorErrors {
        fn from(var: ExpectedOppositeSigns) -> Self {
            ActorErrors::ExpectedOppositeSigns(var)
        }
    }
    impl ::std::convert::From<ExpectedSqrtPriceZeroBeforeInit> for ActorErrors {
        fn from(var: ExpectedSqrtPriceZeroBeforeInit) -> Self {
            ActorErrors::ExpectedSqrtPriceZeroBeforeInit(var)
        }
    }
    impl ::std::convert::From<IRSNotionalAmountSpecifiedMustBeNonZero> for ActorErrors {
        fn from(var: IRSNotionalAmountSpecifiedMustBeNonZero) -> Self {
            ActorErrors::IRSNotionalAmountSpecifiedMustBeNonZero(var)
        }
    }
    impl ::std::convert::From<InvalidMarginDelta> for ActorErrors {
        fn from(var: InvalidMarginDelta) -> Self {
            ActorErrors::InvalidMarginDelta(var)
        }
    }
    impl ::std::convert::From<LidoGetPooledEthBySharesReturnedZero> for ActorErrors {
        fn from(var: LidoGetPooledEthBySharesReturnedZero) -> Self {
            ActorErrors::LidoGetPooledEthBySharesReturnedZero(var)
        }
    }
    impl ::std::convert::From<LiquidityDeltaMustBePositiveInBurn> for ActorErrors {
        fn from(var: LiquidityDeltaMustBePositiveInBurn) -> Self {
            ActorErrors::LiquidityDeltaMustBePositiveInBurn(var)
        }
    }
    impl ::std::convert::From<LiquidityDeltaMustBePositiveInMint> for ActorErrors {
        fn from(var: LiquidityDeltaMustBePositiveInMint) -> Self {
            ActorErrors::LiquidityDeltaMustBePositiveInMint(var)
        }
    }
    impl ::std::convert::From<MarginLessThanMinimum> for ActorErrors {
        fn from(var: MarginLessThanMinimum) -> Self {
            ActorErrors::MarginLessThanMinimum(var)
        }
    }
    impl ::std::convert::From<MarginRequirementNotMet> for ActorErrors {
        fn from(var: MarginRequirementNotMet) -> Self {
            ActorErrors::MarginRequirementNotMet(var)
        }
    }
    impl ::std::convert::From<MarginRequirementNotMetFCM> for ActorErrors {
        fn from(var: MarginRequirementNotMetFCM) -> Self {
            ActorErrors::MarginRequirementNotMetFCM(var)
        }
    }
    impl ::std::convert::From<NotEnoughFunds> for ActorErrors {
        fn from(var: NotEnoughFunds) -> Self {
            ActorErrors::NotEnoughFunds(var)
        }
    }
    impl ::std::convert::From<OOO> for ActorErrors {
        fn from(var: OOO) -> Self {
            ActorErrors::OOO(var)
        }
    }
    impl ::std::convert::From<OnlyFCM> for ActorErrors {
        fn from(var: OnlyFCM) -> Self {
            ActorErrors::OnlyFCM(var)
        }
    }
    impl ::std::convert::From<OnlyMarginEngine> for ActorErrors {
        fn from(var: OnlyMarginEngine) -> Self {
            ActorErrors::OnlyMarginEngine(var)
        }
    }
    impl ::std::convert::From<OnlyOwnerCanUpdatePosition> for ActorErrors {
        fn from(var: OnlyOwnerCanUpdatePosition) -> Self {
            ActorErrors::OnlyOwnerCanUpdatePosition(var)
        }
    }
    impl ::std::convert::From<OnlyVAMM> for ActorErrors {
        fn from(var: OnlyVAMM) -> Self {
            ActorErrors::OnlyVAMM(var)
        }
    }
    impl ::std::convert::From<PositionNetZero> for ActorErrors {
        fn from(var: PositionNetZero) -> Self {
            ActorErrors::PositionNetZero(var)
        }
    }
    impl ::std::convert::From<PositionNotSettled> for ActorErrors {
        fn from(var: PositionNotSettled) -> Self {
            ActorErrors::PositionNotSettled(var)
        }
    }
    impl ::std::convert::From<RocketPoolGetEthValueReturnedZero> for ActorErrors {
        fn from(var: RocketPoolGetEthValueReturnedZero) -> Self {
            ActorErrors::RocketPoolGetEthValueReturnedZero(var)
        }
    }
    impl ::std::convert::From<WithdrawalExceedsCurrentMargin> for ActorErrors {
        fn from(var: WithdrawalExceedsCurrentMargin) -> Self {
            ActorErrors::WithdrawalExceedsCurrentMargin(var)
        }
    }
    impl ::std::convert::From<closeToOrBeyondMaturity> for ActorErrors {
        fn from(var: closeToOrBeyondMaturity) -> Self {
            ActorErrors::closeToOrBeyondMaturity(var)
        }
    }
    #[doc = "Container type for all input parameters for the `burnViaAMM` function with signature `burnViaAMM(address,address,int24,int24,uint128)` and selector `[41, 42, 96, 213]`"]
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
        name = "burnViaAMM",
        abi = "burnViaAMM(address,address,int24,int24,uint128)"
    )]
    pub struct BurnViaAMMCall {
        pub vamm_address: ethers::core::types::Address,
        pub recipient: ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub amount: u128,
    }
    #[doc = "Container type for all input parameters for the `initiateFullyCollateralisedFixedTakerSwap` function with signature `initiateFullyCollateralisedFixedTakerSwap(address,uint256,uint160)` and selector `[105, 105, 109, 191]`"]
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
        name = "initiateFullyCollateralisedFixedTakerSwap",
        abi = "initiateFullyCollateralisedFixedTakerSwap(address,uint256,uint160)"
    )]
    pub struct InitiateFullyCollateralisedFixedTakerSwapCall {
        pub fcm_address: ethers::core::types::Address,
        pub notional: ethers::core::types::U256,
        pub sqrt_price_limit_x96: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `liquidatePosition` function with signature `liquidatePosition(address,int24,int24,address)` and selector `[202, 205, 215, 35]`"]
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
        name = "liquidatePosition",
        abi = "liquidatePosition(address,int24,int24,address)"
    )]
    pub struct LiquidatePositionCall {
        pub me_address: ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `mintOrBurnViaPeriphery` function with signature `mintOrBurnViaPeriphery(address,(address,int24,int24,uint256,bool,int256))` and selector `[131, 227, 69, 231]`"]
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
        name = "mintOrBurnViaPeriphery",
        abi = "mintOrBurnViaPeriphery(address,(address,int24,int24,uint256,bool,int256))"
    )]
    pub struct MintOrBurnViaPeripheryCall {
        pub periphery_address: ethers::core::types::Address,
        pub params: MintOrBurnParams,
    }
    #[doc = "Container type for all input parameters for the `mintViaAMM` function with signature `mintViaAMM(address,address,int24,int24,uint128)` and selector `[138, 135, 99, 180]`"]
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
        name = "mintViaAMM",
        abi = "mintViaAMM(address,address,int24,int24,uint128)"
    )]
    pub struct MintViaAMMCall {
        pub vamm_address: ethers::core::types::Address,
        pub recipient: ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub amount: u128,
    }
    #[doc = "Container type for all input parameters for the `setIntegrationApproval` function with signature `setIntegrationApproval(address,address,bool)` and selector `[64, 40, 54, 152]`"]
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
        name = "setIntegrationApproval",
        abi = "setIntegrationApproval(address,address,bool)"
    )]
    pub struct SetIntegrationApprovalCall {
        pub me_address: ethers::core::types::Address,
        pub int_address: ethers::core::types::Address,
        pub allow_integration: bool,
    }
    #[doc = "Container type for all input parameters for the `settlePosition` function with signature `settlePosition(address,address,int24,int24)` and selector `[99, 55, 85, 225]`"]
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
        name = "settlePosition",
        abi = "settlePosition(address,address,int24,int24)"
    )]
    pub struct SettlePositionCall {
        pub me_adrress: ethers::core::types::Address,
        pub recipient: ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
    }
    #[doc = "Container type for all input parameters for the `settlePositionViaAMM` function with signature `settlePositionViaAMM(address,address,int24,int24)` and selector `[48, 57, 88, 243]`"]
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
        name = "settlePositionViaAMM",
        abi = "settlePositionViaAMM(address,address,int24,int24)"
    )]
    pub struct SettlePositionViaAMMCall {
        pub me_address: ethers::core::types::Address,
        pub owner: ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
    }
    #[doc = "Container type for all input parameters for the `settleYBATrader` function with signature `settleYBATrader(address)` and selector `[132, 0, 71, 235]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "settleYBATrader", abi = "settleYBATrader(address)")]
    pub struct SettleYBATraderCall {
        pub fcm_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `swapViaAMM` function with signature `swapViaAMM(address,(address,int256,uint160,int24,int24))` and selector `[4, 78, 211, 146]`"]
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
        name = "swapViaAMM",
        abi = "swapViaAMM(address,(address,int256,uint160,int24,int24))"
    )]
    pub struct SwapViaAMMCall {
        pub vamm_address: ethers::core::types::Address,
        pub params: SwapParams,
    }
    #[doc = "Container type for all input parameters for the `swapViaPeriphery` function with signature `swapViaPeriphery(address,(address,bool,uint256,uint160,int24,int24,int256))` and selector `[242, 53, 67, 144]`"]
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
        name = "swapViaPeriphery",
        abi = "swapViaPeriphery(address,(address,bool,uint256,uint160,int24,int24,int256))"
    )]
    pub struct SwapViaPeripheryCall {
        pub periphery_address: ethers::core::types::Address,
        pub params: SwapPeripheryParams,
    }
    #[doc = "Container type for all input parameters for the `unwindFullyCollateralisedFixedTakerSwap` function with signature `unwindFullyCollateralisedFixedTakerSwap(address,uint256,uint160)` and selector `[233, 174, 75, 200]`"]
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
        name = "unwindFullyCollateralisedFixedTakerSwap",
        abi = "unwindFullyCollateralisedFixedTakerSwap(address,uint256,uint160)"
    )]
    pub struct UnwindFullyCollateralisedFixedTakerSwapCall {
        pub fcm_address: ethers::core::types::Address,
        pub notional_to_unwind: ethers::core::types::U256,
        pub sqrt_price_limit_x96: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `updatePositionMarginViaAMM` function with signature `updatePositionMarginViaAMM(address,address,int24,int24,int256)` and selector `[49, 189, 247, 62]`"]
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
        name = "updatePositionMarginViaAMM",
        abi = "updatePositionMarginViaAMM(address,address,int24,int24,int256)"
    )]
    pub struct UpdatePositionMarginViaAMMCall {
        pub me_address: ethers::core::types::Address,
        pub owner: ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub margin_delta: I256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ActorCalls {
        BurnViaAMM(BurnViaAMMCall),
        InitiateFullyCollateralisedFixedTakerSwap(InitiateFullyCollateralisedFixedTakerSwapCall),
        LiquidatePosition(LiquidatePositionCall),
        MintOrBurnViaPeriphery(MintOrBurnViaPeripheryCall),
        MintViaAMM(MintViaAMMCall),
        SetIntegrationApproval(SetIntegrationApprovalCall),
        SettlePosition(SettlePositionCall),
        SettlePositionViaAMM(SettlePositionViaAMMCall),
        SettleYBATrader(SettleYBATraderCall),
        SwapViaAMM(SwapViaAMMCall),
        SwapViaPeriphery(SwapViaPeripheryCall),
        UnwindFullyCollateralisedFixedTakerSwap(UnwindFullyCollateralisedFixedTakerSwapCall),
        UpdatePositionMarginViaAMM(UpdatePositionMarginViaAMMCall),
    }
    impl ethers::core::abi::AbiDecode for ActorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <BurnViaAMMCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ActorCalls::BurnViaAMM(decoded));
            }
            if let Ok (decoded) = < InitiateFullyCollateralisedFixedTakerSwapCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (ActorCalls :: InitiateFullyCollateralisedFixedTakerSwap (decoded)) }
            if let Ok(decoded) =
                <LiquidatePositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ActorCalls::LiquidatePosition(decoded));
            }
            if let Ok(decoded) =
                <MintOrBurnViaPeripheryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ActorCalls::MintOrBurnViaPeriphery(decoded));
            }
            if let Ok(decoded) =
                <MintViaAMMCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ActorCalls::MintViaAMM(decoded));
            }
            if let Ok(decoded) =
                <SetIntegrationApprovalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ActorCalls::SetIntegrationApproval(decoded));
            }
            if let Ok(decoded) =
                <SettlePositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ActorCalls::SettlePosition(decoded));
            }
            if let Ok(decoded) =
                <SettlePositionViaAMMCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ActorCalls::SettlePositionViaAMM(decoded));
            }
            if let Ok(decoded) =
                <SettleYBATraderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ActorCalls::SettleYBATrader(decoded));
            }
            if let Ok(decoded) =
                <SwapViaAMMCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ActorCalls::SwapViaAMM(decoded));
            }
            if let Ok(decoded) =
                <SwapViaPeripheryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ActorCalls::SwapViaPeriphery(decoded));
            }
            if let Ok (decoded) = < UnwindFullyCollateralisedFixedTakerSwapCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (ActorCalls :: UnwindFullyCollateralisedFixedTakerSwap (decoded)) }
            if let Ok(decoded) =
                <UpdatePositionMarginViaAMMCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ActorCalls::UpdatePositionMarginViaAMM(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ActorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ActorCalls::BurnViaAMM(element) => element.encode(),
                ActorCalls::InitiateFullyCollateralisedFixedTakerSwap(element) => element.encode(),
                ActorCalls::LiquidatePosition(element) => element.encode(),
                ActorCalls::MintOrBurnViaPeriphery(element) => element.encode(),
                ActorCalls::MintViaAMM(element) => element.encode(),
                ActorCalls::SetIntegrationApproval(element) => element.encode(),
                ActorCalls::SettlePosition(element) => element.encode(),
                ActorCalls::SettlePositionViaAMM(element) => element.encode(),
                ActorCalls::SettleYBATrader(element) => element.encode(),
                ActorCalls::SwapViaAMM(element) => element.encode(),
                ActorCalls::SwapViaPeriphery(element) => element.encode(),
                ActorCalls::UnwindFullyCollateralisedFixedTakerSwap(element) => element.encode(),
                ActorCalls::UpdatePositionMarginViaAMM(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ActorCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ActorCalls::BurnViaAMM(element) => element.fmt(f),
                ActorCalls::InitiateFullyCollateralisedFixedTakerSwap(element) => element.fmt(f),
                ActorCalls::LiquidatePosition(element) => element.fmt(f),
                ActorCalls::MintOrBurnViaPeriphery(element) => element.fmt(f),
                ActorCalls::MintViaAMM(element) => element.fmt(f),
                ActorCalls::SetIntegrationApproval(element) => element.fmt(f),
                ActorCalls::SettlePosition(element) => element.fmt(f),
                ActorCalls::SettlePositionViaAMM(element) => element.fmt(f),
                ActorCalls::SettleYBATrader(element) => element.fmt(f),
                ActorCalls::SwapViaAMM(element) => element.fmt(f),
                ActorCalls::SwapViaPeriphery(element) => element.fmt(f),
                ActorCalls::UnwindFullyCollateralisedFixedTakerSwap(element) => element.fmt(f),
                ActorCalls::UpdatePositionMarginViaAMM(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BurnViaAMMCall> for ActorCalls {
        fn from(var: BurnViaAMMCall) -> Self {
            ActorCalls::BurnViaAMM(var)
        }
    }
    impl ::std::convert::From<InitiateFullyCollateralisedFixedTakerSwapCall> for ActorCalls {
        fn from(var: InitiateFullyCollateralisedFixedTakerSwapCall) -> Self {
            ActorCalls::InitiateFullyCollateralisedFixedTakerSwap(var)
        }
    }
    impl ::std::convert::From<LiquidatePositionCall> for ActorCalls {
        fn from(var: LiquidatePositionCall) -> Self {
            ActorCalls::LiquidatePosition(var)
        }
    }
    impl ::std::convert::From<MintOrBurnViaPeripheryCall> for ActorCalls {
        fn from(var: MintOrBurnViaPeripheryCall) -> Self {
            ActorCalls::MintOrBurnViaPeriphery(var)
        }
    }
    impl ::std::convert::From<MintViaAMMCall> for ActorCalls {
        fn from(var: MintViaAMMCall) -> Self {
            ActorCalls::MintViaAMM(var)
        }
    }
    impl ::std::convert::From<SetIntegrationApprovalCall> for ActorCalls {
        fn from(var: SetIntegrationApprovalCall) -> Self {
            ActorCalls::SetIntegrationApproval(var)
        }
    }
    impl ::std::convert::From<SettlePositionCall> for ActorCalls {
        fn from(var: SettlePositionCall) -> Self {
            ActorCalls::SettlePosition(var)
        }
    }
    impl ::std::convert::From<SettlePositionViaAMMCall> for ActorCalls {
        fn from(var: SettlePositionViaAMMCall) -> Self {
            ActorCalls::SettlePositionViaAMM(var)
        }
    }
    impl ::std::convert::From<SettleYBATraderCall> for ActorCalls {
        fn from(var: SettleYBATraderCall) -> Self {
            ActorCalls::SettleYBATrader(var)
        }
    }
    impl ::std::convert::From<SwapViaAMMCall> for ActorCalls {
        fn from(var: SwapViaAMMCall) -> Self {
            ActorCalls::SwapViaAMM(var)
        }
    }
    impl ::std::convert::From<SwapViaPeripheryCall> for ActorCalls {
        fn from(var: SwapViaPeripheryCall) -> Self {
            ActorCalls::SwapViaPeriphery(var)
        }
    }
    impl ::std::convert::From<UnwindFullyCollateralisedFixedTakerSwapCall> for ActorCalls {
        fn from(var: UnwindFullyCollateralisedFixedTakerSwapCall) -> Self {
            ActorCalls::UnwindFullyCollateralisedFixedTakerSwap(var)
        }
    }
    impl ::std::convert::From<UpdatePositionMarginViaAMMCall> for ActorCalls {
        fn from(var: UpdatePositionMarginViaAMMCall) -> Self {
            ActorCalls::UpdatePositionMarginViaAMM(var)
        }
    }
    #[doc = "Container type for all return fields from the `mintOrBurnViaPeriphery` function with signature `mintOrBurnViaPeriphery(address,(address,int24,int24,uint256,bool,int256))` and selector `[131, 227, 69, 231]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MintOrBurnViaPeripheryReturn {
        pub position_margin_requirement: I256,
    }
    #[doc = "Container type for all return fields from the `mintViaAMM` function with signature `mintViaAMM(address,address,int24,int24,uint128)` and selector `[138, 135, 99, 180]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MintViaAMMReturn {
        pub position_margin_requirement: I256,
    }
    #[doc = "Container type for all return fields from the `swapViaAMM` function with signature `swapViaAMM(address,(address,int256,uint160,int24,int24))` and selector `[4, 78, 211, 146]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SwapViaAMMReturn {
        pub fixed_token_delta: I256,
        pub variable_token_delta: I256,
        pub cumulative_fee_incurred: ethers::core::types::U256,
        pub fixed_token_delta_unbalanced: I256,
        pub margin_requirement: I256,
    }
    #[doc = "Container type for all return fields from the `swapViaPeriphery` function with signature `swapViaPeriphery(address,(address,bool,uint256,uint160,int24,int24,int256))` and selector `[242, 53, 67, 144]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SwapViaPeripheryReturn {
        pub fixed_token_delta: I256,
        pub variable_token_delta: I256,
        pub cumulative_fee_incurred: ethers::core::types::U256,
        pub fixed_token_delta_unbalanced: I256,
        pub margin_requirement: I256,
    }
}
