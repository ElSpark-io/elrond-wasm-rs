pub const ESDT_LOCAL_MINT_FUNC_NAME: &[u8] = b"ESDTLocalMint";
pub const ESDT_LOCAL_BURN_FUNC_NAME: &[u8] = b"ESDTLocalBurn";
pub const ESDT_MULTI_TRANSFER_FUNC_NAME: &[u8] = b"MultiESDTNFTTransfer";
pub const ESDT_NFT_TRANSFER_FUNC_NAME: &[u8] = b"ESDTNFTTransfer";
pub const ESDT_NFT_CREATE_FUNC_NAME: &[u8] = b"ESDTNFTCreate";
pub const ESDT_NFT_ADD_QUANTITY_FUNC_NAME: &[u8] = b"ESDTNFTAddQuantity";
pub const ESDT_NFT_ADD_URI_FUNC_NAME: &[u8] = b"ESDTNFTAddURI";
pub const ESDT_NFT_UPDATE_ATTRIBUTES_FUNC_NAME: &[u8] = b"ESDTNFTUpdateAttributes";
pub const ESDT_NFT_BURN_FUNC_NAME: &[u8] = b"ESDTNFTBurn";
pub const ESDT_TRANSFER_FUNC_NAME: &[u8] = b"ESDTTransfer";
pub const CHANGE_OWNER_BUILTIN_FUNC_NAME: &[u8] = b"ChangeOwnerAddress";
pub const SET_USERNAME_FUNC_NAME: &[u8] = b"SetUserName";
pub const UPGRADE_CONTRACT_FUNC_NAME: &[u8] = b"upgradeContract";

pub fn is_esdt_transfer_built_in_function(func_name: &[u8]) -> bool {
    func_name == ESDT_TRANSFER_FUNC_NAME
        || func_name == ESDT_NFT_TRANSFER_FUNC_NAME
        || func_name == ESDT_MULTI_TRANSFER_FUNC_NAME
}
