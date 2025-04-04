#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{ 
    to_binary, Binary, Deps, DepsMut, 
    Env, MessageInfo, Response, 
    StdResult, Empty, Uint128, WasmMsg, 
    SubMsg, Reply, from_binary 
};

use cw_storage_plus::Item;
use cw2::set_contract_version;
use cw0::parse_reply_instantiate_data;

use cw20::MinterResponse;


use crate::state::{ 
    State, Vault, UserNftData, STATE, USER_VAULTS, 
    VAULT_LIST, VAULT_VTOKEN 
};

use crate::error::ContractError;

use crate::msg::{ 
    ExecuteMsg, InstantiateMsg, Cw721ReceiveMsg, 
    Cw20ReceiveMsg, CheckCreateVault, NftDataTransfer,
    QueryMsg, GetVaultIdResponse, GetVaultArrayResponse, 
    Cw20Instantiate, GetVTokenAddressResponse
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cosmwasm-nftx";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const USER_ADDRESS: Item<String> = Item::new("user_address");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(_deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let set_owner = State { contract_owner: _info.sender };
    STATE.save(_deps.storage, &set_owner)?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match _msg {
        ExecuteMsg::ReceiveNft(cw721_receive_msg) => execute::execute_receive(_deps, _env, _info, cw721_receive_msg),
        ExecuteMsg::Receive(cw20_receive_msg) => execute::execute_receive_cw20(_deps, _env, _info, cw20_receive_msg),
    }
}

pub mod execute {
    use super::*;

    pub fn execute_receive(_deps: DepsMut, _env: Env, _info: MessageInfo, _trigger_msg: Cw721ReceiveMsg) -> Result<Response, ContractError> {
        let check_create_vault: CheckCreateVault = from_binary(&_trigger_msg.msg)?;
        let message: String = String::from("Create Vault");


        let user_nft_data: UserNftData = UserNftData { nft_id: _trigger_msg.token_id, user_address: _trigger_msg.sender.clone() };

        if check_create_vault.message == message {
            
            let is_vault_available = VAULT_LIST.may_load(_deps.storage)?;
            let _vault_id = match is_vault_available.clone() { Some(vector_id) => vector_id.len() as u64 + 1, None => 1};

            let mut all_nft_data: Vec<UserNftData> = Vec::new();
            all_nft_data.push(user_nft_data);

            let vault = Vault {
                vault_id: _vault_id.to_string(),
                vault_name: check_create_vault.vault_name.clone().unwrap(),
                vault_symbol: check_create_vault.vault_symbol.clone().unwrap(),
                vault_owner: _trigger_msg.sender.clone(),
                vault_nft_address: check_create_vault.nft_asset_address.unwrap(),
                vault_user_nft_data: all_nft_data,
            };

            USER_VAULTS.update(_deps.storage, _trigger_msg.sender.clone(), |vault_array| -> Result<Vec<String>, ContractError> {
                match vault_array {
                    Some(mut vec_arr) => {
                        vec_arr.push(_vault_id.to_string());
                        Ok(vec_arr)
                    },
                    None => {
                        let mut new_vault_id_array: Vec<String> = Vec::new();
                        new_vault_id_array.push(_vault_id.to_string());
                        Ok(new_vault_id_array)
                    }
                }
            })?;

            match is_vault_available {
                Some(_) => {
                    VAULT_LIST.update(_deps.storage, |mut vault_array| -> StdResult<_> {
                        vault_array.push(vault);
                        Ok(vault_array)
                    })?;
                },
                None => {
                    let mut new_vault_array: Vec<Vault> = Vec::new();
                    new_vault_array.push(vault);
                    VAULT_LIST.save(_deps.storage, &new_vault_array)?;
                }
            }

            USER_ADDRESS.save(_deps.storage, &_trigger_msg.sender.clone())?;

            let instantiate_cw20_tx = WasmMsg::Instantiate { 
                admin: None, 
                code_id: 846, 
                msg: to_binary(&Cw20Instantiate {
                    name: check_create_vault.vault_name.unwrap(),
                    symbol: check_create_vault.vault_symbol.unwrap(),
                    decimals: 18,
                    initial_balances: vec![],
                    mint: Some(MinterResponse { minter: _env.contract.address.to_string(), cap: None}),
                    marketing: None,
                })?, 
                funds: vec![], 
                label: "vtoken_created".to_string(),
            };

            const INSTANTIATE_CW20_REPLY_ID: u64 = 1u64;

            let submessage: SubMsg<Empty> = SubMsg::reply_on_success(instantiate_cw20_tx, INSTANTIATE_CW20_REPLY_ID);

            let response = Response::new()
                .add_attribute("method", "cw20_instantiate")
                .add_submessage(submessage);

            Ok(response)
        } else {
            let vtoken_address = VAULT_VTOKEN.load(_deps.storage, check_create_vault.vault_id.clone().unwrap());

            match vtoken_address {
                Ok(addr) => {

                    VAULT_LIST.update(_deps.storage, |mut vector_vault| -> Result<Vec<Vault>, ContractError> {
                        for i in 0..vector_vault.len() {
                            if vector_vault[i].vault_id == check_create_vault.vault_id.clone().unwrap() {
                                vector_vault[i].vault_user_nft_data.push(user_nft_data.clone());
                            }
                        }
                        Ok(vector_vault)
                    })?;

                    let execute_mint_tx = WasmMsg::Execute { 
                        contract_addr: addr, 
                        msg: to_binary(&cw20::Cw20ExecuteMsg::Mint { 
                            recipient: _trigger_msg.sender,                        
                            amount: Uint128::from(1u128)
                        })?,
                        funds: vec![]
                    };
    
                    const EXECUTE_MINT_ID: u64 = 2u64;
                    let submessage: SubMsg<Empty> = SubMsg::reply_on_error(execute_mint_tx, EXECUTE_MINT_ID);
    
                    let response: Response = Response::new()
                        .add_attribute("method", "execute_mint")
                        .add_submessage(submessage);
    
                    Ok(response)
                },
                Err(_) => return Err(ContractError::NftTransferFailed {  })
            }
        }
    }

    pub fn execute_receive_cw20(_deps: DepsMut, _env: Env, _info: MessageInfo, _trigger_msg: Cw20ReceiveMsg) -> Result<Response, ContractError> {
        let nft_transfer_data: NftDataTransfer = from_binary(&_trigger_msg.msg)?;

        let user_nft_data = UserNftData {
            nft_id: nft_transfer_data.nft_id.clone(),
            user_address: _trigger_msg.sender.clone()
        };

        VAULT_LIST.update(_deps.storage, |mut vector_vault| -> Result<Vec<Vault>, ContractError> {
            for i in 0..vector_vault.len() {
                if vector_vault[i].vault_id == nft_transfer_data.vault_id.clone() {
                    for j in 0..vector_vault[i].vault_user_nft_data.len()-1 {
                        if vector_vault[i].vault_user_nft_data[j].nft_id == user_nft_data.nft_id {
                            vector_vault[i].vault_user_nft_data.remove(j);
                            break;
                        }
                    }
                }
            }
            Ok(vector_vault)
        })?;

        let execute_nft_tx = WasmMsg::Execute { 
            contract_addr: nft_transfer_data.nft_asset_address, 
            msg: to_binary(&cw721::Cw721ExecuteMsg::TransferNft { recipient: _trigger_msg.sender, token_id: nft_transfer_data.nft_id })?, 
            funds: vec![] 
        };

        const EXECUTE_TRANSFER_NFT: u64 = 3u64;
        let submessage: SubMsg<Empty> = SubMsg::reply_on_error(execute_nft_tx, EXECUTE_TRANSFER_NFT);

        let response: Response = Response::new()
            .add_attribute("method", "execute_nft_transfer")
            .add_submessage(submessage);

        Ok(response)
    }

}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {

    const INSTANTIATE_CW20_REPLY_ID: u64 = 1u64;

    match msg.id {
        INSTANTIATE_CW20_REPLY_ID => reply::handle_cw20_instantiate(deps, msg),
        _id => return Err(ContractError::VTokenInstantiate { id: _id })
    }
}

pub mod reply {
    use super::*;

    pub fn handle_cw20_instantiate(deps: DepsMut, msg: Reply) -> Result<Response, ContractError> {
        let res = parse_reply_instantiate_data(msg);

        match res {
            Ok(data) => {

                let vault_list_len = VAULT_LIST.load(deps.storage);

                match vault_list_len {
                    Ok(vector) => {
                        VAULT_VTOKEN.save(deps.storage, (vector.len()).to_string(), &data.contract_address)?;
                    },
                    Err(_) => {
                        return Err(ContractError::VaultNotExist {  })
                    }
                }

                let user_address = match USER_ADDRESS.load(deps.storage) {
                    Ok(user) => user,
                    Err(_) => return Err(ContractError::UserNotExist {  })
                };

                let execute_mint_tx = WasmMsg::Execute { 
                    contract_addr: data.contract_address.clone(), 
                    msg: to_binary(&cw20::Cw20ExecuteMsg::Mint { 
                        recipient: user_address.clone(),                        
                        amount: Uint128::from(1u128)
                    })?,
                    funds: vec![]
                };

                const EXECUTE_MINT_ID: u64 = 2u64;
                let submessage: SubMsg<Empty> = SubMsg::reply_on_error(execute_mint_tx, EXECUTE_MINT_ID);

                let response: Response = Response::new()
                    .add_attribute("method", "execute_mint")
                    .add_submessage(submessage);
                Ok(response)
            },
            Err(_) => return Err(ContractError::VTokenInsError {  })
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    match _msg {
        QueryMsg::GetVaultArray { } => to_binary(&query::get_vault_array(_deps, _env)),
        QueryMsg::GetVaultId { vault_owner } => to_binary(&query::get_vault_id(_deps, _env, vault_owner)),
        QueryMsg::GetVTokenAddress { vault_id } => to_binary(&query::get_vtoken_address(_deps, _env, vault_id))
    }
}

pub mod query {
    use super::*;

    pub fn get_vault_array(_deps: Deps, _env: Env) -> Result<GetVaultArrayResponse, ContractError> {
        let vault_list = VAULT_LIST.load(_deps.storage);

        match vault_list {
            Ok(vector) => {
                Ok(GetVaultArrayResponse {vault_array: vector})
            },
            Err(_) => {
                return Err(ContractError::NftTransferFailed {  });
            }
        }
    }

    pub fn get_vault_id(_deps: Deps, _env: Env, _owner: String) -> Result<GetVaultIdResponse, ContractError> {
        let user_vault = USER_VAULTS.load(_deps.storage, _owner);

        match user_vault {
            Ok(vector) => {
                Ok(GetVaultIdResponse {vault_id_response: vector})
            },
            Err(_) => {
                return Err(ContractError::NftTransferFailed {  });
            }
        }
    }

    pub fn get_vtoken_address(_deps: Deps, _env: Env, _vault_id: String) -> Result<GetVTokenAddressResponse, ContractError> {
        let vtoken_address = VAULT_VTOKEN.load(_deps.storage, _vault_id);

        match vtoken_address {
            Ok(address) => Ok(GetVTokenAddressResponse { vtoken_address: address }),
            Err(_) => return Err(ContractError::NftTransferFailed {  })
        }
    }
}


// Code stored successfully!! 🎉
// +
// ├── code_id: 1074
// └── instantiate_permission: –


// Contract instantiated successfully!! 🎉 
// +
// ├── label: default
// ├── contract_address: osmo1nm0jey9xha383x93rm20psgnht69fx9uz4gqzwf856e490p2g0tsknh6g5
// ├── code_id: 1074
// ├── creator: osmo1cyyzpxplxdzkeea7kwsydadg87357qnahakaks
// └── admin: -

// beaker wasm deploy nftx --raw '{}' --signer-account test1 --network testnet


// osmo1x77mhnglh406gzsrq6pk0syzj8305524x07n9qwa2dcrcxqpar5seejjhz
