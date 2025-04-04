use cosmwasm_schema::{ cw_serde, QueryResponses };
use cosmwasm_std::{ Binary, Uint128 };
use cw20::{Cw20Coin, Logo, MinterResponse};
use schemars::JsonSchema;
use std::fmt;
use serde::{ Serialize, Deserialize };

use crate::state::Vault;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub struct InstantiateMarketingInfo {
    pub project: Option<String>,
    pub description: Option<String>,
    pub marketing: Option<String>,
    pub logo: Option<Logo>,
}

#[cw_serde]
pub struct Cw20Instantiate {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub initial_balances: Vec<Cw20Coin>,
    pub mint: Option<MinterResponse>,
    pub marketing: Option<InstantiateMarketingInfo>,
}


#[cw_serde]
pub enum ExecuteMsg {
    ReceiveNft(Cw721ReceiveMsg),
    Receive(Cw20ReceiveMsg),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Cw721ReceiveMsg {
   pub sender: String,
   pub token_id: String,
   pub msg: Binary,
}

impl fmt::Display for Cw721ReceiveMsg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "sender:{} token_id:{} msg:{}",
            self.sender,
            self.token_id,
            self.msg.to_string()
        )
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Cw20ReceiveMsg {
    pub sender: String,
    pub amount: Uint128,
    pub msg: Binary,
}

impl fmt::Display for Cw20ReceiveMsg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "sender:{} amount:{} msg:{}",
            self.sender,
            self.amount,
            self.msg.to_string()
        )
    }
}

#[cw_serde]
pub struct CheckCreateVault {
    pub message: String,
    pub vault_id: Option<String>,
    pub vault_name: Option<String>,
    pub vault_symbol: Option<String>,
    pub nft_asset_address: Option<String>,
}

#[cw_serde]
pub struct NftDataTransfer {
    pub vault_id: String,
    pub nft_asset_address: String,
    pub nft_id: String,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetVaultIdResponse)]
    GetVaultId { vault_owner: String },

    #[returns(GetVaultArrayResponse)]
    GetVaultArray {},

    #[returns(GetVTokenAddressResponse)]
    GetVTokenAddress { vault_id: String }
}

#[cw_serde]
pub struct GetVaultIdResponse {
    pub vault_id_response: Vec<String>,
}

#[cw_serde]
pub struct GetVaultArrayResponse {
    pub vault_array: Vec<Vault>,
}

#[cw_serde] 
pub struct GetVTokenAddressResponse {
    pub vtoken_address: String,
}