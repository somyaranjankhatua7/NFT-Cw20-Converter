use cosmwasm_std::StdError;
use thiserror::Error;
use serde::{Serialize, Serializer};
use crate::msg::CheckCreateVault;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("User Doesn't Have Any Vault")]
    UserVaultNotFound {},

    #[error("Vault Not Found")]
    VaultNotFound {},

    #[error(" Nft Transfer Failed")]
    NftTransferFailed {},

    #[error("vault is not exist")]
    VaultNotExist {},

    #[error("user is not exist")]
    UserNotExist {},

    #[error("VTOKEN INstantiate {}", id)]
    VTokenInstantiate {id: u64},

    #[error(" VTokenInsError")]
    VTokenInsError {},

    #[error("Message Error {:?}", message)]
    MessageError { message: CheckCreateVault },


    #[error("Message Error2 {:?}", message)]
    MessageError2 { message: CheckCreateVault }


}


impl Serialize for ContractError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        serializer.serialize_str("ContractError")   
    }
}
// CustomError { val: "Foo" }, "Custom Error val: Foo"
// #[error("Custom Error val: {val:?}")]
// CustomError { val: String },