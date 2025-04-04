use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{ Item, Map };

#[cw_serde]
pub struct State {
    pub contract_owner: Addr,
}

pub const STATE: Item<State> = Item::new("contract_owner");

#[cw_serde]
pub struct UserNftData {
    pub nft_id: String,
    pub user_address: String,
}

#[cw_serde] 
pub struct Vault {
    pub vault_id: String,
    pub vault_name: String,
    pub vault_symbol: String,
    pub vault_owner: String,
    pub vault_nft_address: String,
    pub vault_user_nft_data: Vec<UserNftData>,
}

/// @dev define USER VAULTS for mapping user address to array of vault id
pub const USER_VAULTS: Map<String, Vec<String>> = Map::new("user_vaults");

/// @dev define VAULT_LIST for storing all the vault in the nftx.
pub const VAULT_LIST: Item<Vec<Vault>> = Item::new("vault_list");

/// @dev define VAULT_VTOKEN for mapping vault id to vtoken contract address
pub const VAULT_VTOKEN: Map<String, String> = Map::new("user_vtoken");