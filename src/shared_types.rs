#[doc = "`Info(bool,uint128,int256,int256,int256,int256,int256,uint256,uint256,uint256)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct Info {
    pub is_settled: bool,
    pub liquidity: u128,
    pub margin: I256,
    pub fixed_token_growth_inside_last_x128: I256,
    pub variable_token_growth_inside_last_x128: I256,
    pub fixed_token_balance: I256,
    pub variable_token_balance: I256,
    pub fee_growth_inside_last_x128: ethers::core::types::U256,
    pub reward_per_amount: ethers::core::types::U256,
    pub accumulated_fees: ethers::core::types::U256,
}
#[doc = "`SwapPeripheryParams(address,bool,uint256,uint160,int24,int24,int256)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct SwapPeripheryParams {
    pub margin_engine: ethers::core::types::Address,
    pub is_ft: bool,
    pub notional: ethers::core::types::U256,
    pub sqrt_price_limit_x96: ethers::core::types::U256,
    pub tick_lower: i32,
    pub tick_upper: i32,
    pub margin_delta: I256,
}
#[doc = "`MarginCalculatorParameters(uint256,uint256,int256,int256,int256,int256,int256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct MarginCalculatorParameters {
    pub apy_upper_multiplier_wad: ethers::core::types::U256,
    pub apy_lower_multiplier_wad: ethers::core::types::U256,
    pub sigma_squared_wad: I256,
    pub alpha_wad: I256,
    pub beta_wad: I256,
    pub xi_upper_wad: I256,
    pub xi_lower_wad: I256,
    pub t_max_wad: I256,
    pub eta_im_wad: ethers::core::types::U256,
    pub eta_lm_wad: ethers::core::types::U256,
    pub gap_1: ethers::core::types::U256,
    pub gap_2: ethers::core::types::U256,
    pub gap_3: ethers::core::types::U256,
    pub gap_4: ethers::core::types::U256,
    pub gap_5: ethers::core::types::U256,
    pub gap_6: ethers::core::types::U256,
    pub gap_7: ethers::core::types::U256,
    pub min_margin_to_incentivise_liquidators: ethers::core::types::U256,
}
#[doc = "`MintOrBurnParams(address,int24,int24,uint256,bool,int256)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct MintOrBurnParams {
    pub margin_engine: ethers::core::types::Address,
    pub tick_lower: i32,
    pub tick_upper: i32,
    pub notional: ethers::core::types::U256,
    pub is_mint: bool,
    pub margin_delta: I256,
}
#[doc = "`Vammvars(uint160,int24,uint8)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct Vammvars {
    pub sqrt_price_x96: ethers::core::types::U256,
    pub tick: i32,
    pub fee_protocol: u8,
}
#[doc = "`ReserveData((uint256),uint128,uint128,uint128,uint128,uint128,uint40,address,address,address,address,uint8)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct ReserveData {
    pub configuration: ReserveConfigurationMap,
    pub liquidity_index: u128,
    pub variable_borrow_index: u128,
    pub current_liquidity_rate: u128,
    pub current_variable_borrow_rate: u128,
    pub current_stable_borrow_rate: u128,
    pub last_update_timestamp: u64,
    pub a_token_address: ethers::core::types::Address,
    pub stable_debt_token_address: ethers::core::types::Address,
    pub variable_debt_token_address: ethers::core::types::Address,
    pub interest_rate_strategy_address: ethers::core::types::Address,
    pub id: u8,
}
#[doc = "`ModifyPositionParams(address,int24,int24,int128)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct ModifyPositionParams {
    pub owner: ethers::core::types::Address,
    pub tick_lower: i32,
    pub tick_upper: i32,
    pub liquidity_delta: i128,
}
#[doc = "`Info(uint256,int256,int256,bool)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct Info {
    pub margin_in_scaled_yield_bearing_tokens: ethers::core::types::U256,
    pub fixed_token_balance: I256,
    pub variable_token_balance: I256,
    pub is_settled: bool,
}
#[doc = "`ReserveConfigurationMap(uint256)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct ReserveConfigurationMap {
    pub data: ethers::core::types::U256,
}
#[doc = "`SwapPeripheryParams(address,bool,uint256,uint160,int24,int24,uint256)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct SwapPeripheryParams {
    pub margin_engine: ethers::core::types::Address,
    pub is_ft: bool,
    pub notional: ethers::core::types::U256,
    pub sqrt_price_limit_x96: ethers::core::types::U256,
    pub tick_lower: i32,
    pub tick_upper: i32,
    pub margin_delta: ethers::core::types::U256,
}
#[doc = "`Info(uint128,int128,int256,int256,uint256,bool)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct Info {
    pub liquidity_gross: u128,
    pub liquidity_net: i128,
    pub fixed_token_growth_outside_x128: I256,
    pub variable_token_growth_outside_x128: I256,
    pub fee_growth_outside_x128: ethers::core::types::U256,
    pub initialized: bool,
}
#[doc = "`SwapParams(address,int256,uint160,int24,int24)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct SwapParams {
    pub recipient: ethers::core::types::Address,
    pub amount_specified: I256,
    pub sqrt_price_limit_x96: ethers::core::types::U256,
    pub tick_lower: i32,
    pub tick_upper: i32,
}
#[doc = "`MintOrBurnParams(address,int24,int24,uint256,bool,int256)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct MintOrBurnParams {
    pub margin_engine: ethers::core::types::Address,
    pub tick_lower: i32,
    pub tick_upper: i32,
    pub notional: ethers::core::types::U256,
    pub is_mint: bool,
    pub margin_delta: I256,
}
