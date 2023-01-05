# elrond-wasm-rs

Rust smart contract library designed for Elrond's VM. Also provides a debugging mode with mocks.

# Examples

For examples on how to use the Elrond WASM framework, see https://github.com/ElrondNetwork/elrond-wasm-rs/tree/master/contracts/examples

# IDE

The framework is designed to be easiest to use with the Elrond IDE VSCode extension: https://marketplace.visualstudio.com/items?itemName=Elrond.vscode-elrond-ide

# Manual build

To build a smart contract without the IDE, run the following command in the contract crate:
```
./build-wasm.sh
```

In case this doesn't work, you might not have rustc configured properly.
Try:
```
rustup toolchain install nightly
rustup default nightly
rustup target add wasm32-unknown-unknown
```

# Debugging

Step-by-step debugging of smart contracts is possible in VSCode. To do this, it is required to have a separate debug crate and to have tasks.json and launch.json in .vscode properly configured. See https://github.com/ElrondNetwork/elrond-wasm-rs/tree/master/contracts/examples for examples on how to set this up. 

# Advanced

To debug macros:
```
cargo +nightly rustc --bin wasm -- -Z unstable-options --pretty=expanded > demacroed.rs
```

To check wasm size:
```
twiggy top -n 20 target/wasm32-unknown-unknown/release/wasm.wasm
```


BigIntGetUnsignedBytes -> bi_get_unsigned_bytes
BigIntGetSignedBytes -> bi_get_signed_bytes
Sha256 -> sha256_legacy result is always 32byte
Keccak256 -> keccak256_legacy 
EncodeSecp256k1DerSignature -> encode_secp256k1_der_signature_legacy
MBufferGetBytes -> mb_to_boxed_bytes, mb_copy_to_slice_pad_right, unsafe_buffer_load_address, unsafe_buffer_load_token_identifier

MBufferGetByteSlice -> mb_load_slice

GetOriginalTxHash -> get_tx_hash_legacy, get_state_root_hash_legacy. tx hash 32 byte
GetReturnData -> get_return_data -> get_return_data_range -> execute_on_dest_context_readonly_raw_legacy, execute_on_same_context_raw_legacy, execute_on_dest_context_raw_legacy, deploy_from_source_contract_legacy, deploy_contract_legacy.

DeployFromSourceContract -> deploy_from_source_contract_legacy

createContract -> deploy_contract_legacy
GetPrevBlockRandomSeed -> get_prev_block_random_seed_legacy
GetStateRootHash -> 
GetBlockRandomSeed -> get_block_random_seed_legacy
GetESDTTokenNameByIndex -> token_by_index
GetESDTValueByIndex -> 
GetCallValue -> 
GetCaller -> get_caller_legacy
StorageLoad -> storage_load_to_heap
StorageLoadFromAddress -> 
GetArgument -> get_argument_boxed_bytes
GetESDTTokenData -> load_esdt_token_data_unmanaged
GetESDTBalance ->
GetBlockHash -> 
GetOwnerAddress -> get_owner_address_legacy
GetSCAddress -> get_sc_address_legacy