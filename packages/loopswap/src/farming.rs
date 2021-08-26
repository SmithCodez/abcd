use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::asset::{Asset, AssetInfo};

use cosmwasm_std::Uint128;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateConfig {
        owner: Option<String>,
    },
    Stake{
        asset: Asset
    },
    Unstake{
        asset: Asset
    },
    UpdateReward{
        pool: String,
        rewards: Vec<(Asset, String)>,
    },
    WithdrawReward{
        token: AssetInfo,
    },
    Distribute{
    },
    CaclculateMyReward{
    },
    AddDistributionToken{
        token:AssetInfo,
    },
    AddStakeAbleToken{
        token: AssetInfo,
        pair_add:String,
        pair_add2:String,
    }
    
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    QueryReward
    {
        pool: String,
    },
    QueryUserRewardByPool
    {
        wallet: String,
    },
    QueryUserRewardByDistributionToken
    {
        wallet: String,
        distribution_token:String,
    },
    QueryStakedByUser
    {
        wallet: String,
        staked_token:String,
    },
    QueryStaked
    {
        staked_token:String,
    },
}




// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueryRewardResponse {
    pub info: AssetInfo,
    pub daily_reward: Uint128,
    pub locked_for_distribution: Uint128,
}
// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueryUserRewardByPoolResponse {
    pub pool: AssetInfo,
    pub apr:Uint128,
    pub rewards_info: Vec<Asset>,
}