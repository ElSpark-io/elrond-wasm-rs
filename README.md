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