use std::rc::Rc;

use crate::{DebugApi, testing_framework::vm::static_vm_mem_store};
use ed25519_dalek::*;
use mx_sc::{
    api::{
        CryptoApi, CryptoApiImpl, ManagedBufferApi, KECCAK256_RESULT_LEN, RIPEMD_RESULT_LEN,
        SHA256_RESULT_LEN,
    },
    types::{heap::BoxedBytes, MessageHashType},
};
use sha2::Sha256;
use sha3::{Digest, Keccak256};

impl CryptoApi for DebugApi {
    type CryptoApiImpl = DebugApi;

    fn crypto_api_impl() -> Self::CryptoApiImpl {
        DebugApi::new_from_static()
    }
}

impl CryptoApiImpl for DebugApi {
    fn sha256_legacy(&mut self, data: &[u8]) -> [u8; SHA256_RESULT_LEN] {
        // memstore
        // let self_mut = Rc::get_mut(&mut self.0).unwrap();
        // let vm = Rc::get_mut(&mut self_mut.vm).unwrap();

        // TODO: check offset is 0
        // result of sha256 is 32 byte
        // vm.mem_store(0, 32).unwrap();
        unsafe{
            static_vm_mem_store(0,32).unwrap();
        }

        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().into()
    }

    fn sha256_managed(
        &mut self,
        dest: Self::ManagedBufferHandle,
        data_handle: Self::ManagedBufferHandle,
    ) {
        // default implementation used in debugger
        // the VM has a dedicated hook
        let x = self.mb_to_boxed_bytes(data_handle);
        let result_bytes = self.sha256_legacy(x.as_slice());
        self.mb_overwrite(dest, &result_bytes[..]);
    }

    fn keccak256_legacy(&mut self, data: &[u8]) -> [u8; KECCAK256_RESULT_LEN] {
        // memstore
        // let self_mut = Rc::get_mut(&mut self.0).unwrap();
        // let vm = Rc::get_mut(&mut self_mut.vm).unwrap();

        // TODO: check offset is 0
        // result of sha256 is 32 byte
        // vm.mem_store(0, 32).unwrap();
        unsafe{
            static_vm_mem_store(0,32).unwrap();
        }

        let mut hasher = Keccak256::new();
        hasher.update(data);
        hasher.finalize().into()
    }

    fn keccak256_managed(
        &mut self,
        dest: Self::ManagedBufferHandle,
        data_handle: Self::ManagedBufferHandle,
    ) {
        // default implementation used in debugger
        // the VM has a dedicated hook
        let x = self.mb_to_boxed_bytes(data_handle);
        let result_bytes = self.keccak256_legacy(x.as_slice());
        self.mb_overwrite(dest, &result_bytes[..]);
    }

    fn ripemd160_legacy(&mut self, _data: &[u8]) -> [u8; RIPEMD_RESULT_LEN] {
        panic!("ripemd160 not implemented yet!")
    }

    fn ripemd160_managed(
        &mut self,
        _dest: Self::ManagedBufferHandle,
        _data_handle: Self::ManagedBufferHandle,
    ) {
        panic!("ripemd160 not implemented yet!")
    }

    fn verify_bls_legacy(&self, _key: &[u8], _message: &[u8], _signature: &[u8]) -> bool {
        panic!("verify_bls not implemented yet!")
    }

    fn verify_bls_managed(
        &self,
        _key: Self::ManagedBufferHandle,
        _message: Self::ManagedBufferHandle,
        _signature: Self::ManagedBufferHandle,
    ) -> bool {
        panic!("verify_bls not implemented yet!")
    }

    fn verify_ed25519_legacy(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
        let public = PublicKey::from_bytes(key);
        if public.is_err() {
            return false;
        }

        let sig = Signature::from_bytes(signature);
        if sig.is_err() {
            return false;
        }

        public.unwrap().verify(message, &sig.unwrap()).is_ok()
    }

    fn verify_ed25519_managed(
        &mut self,
        key: Self::ManagedBufferHandle,
        message: Self::ManagedBufferHandle,
        signature: Self::ManagedBufferHandle,
    ) -> bool {
        let key = self.mb_to_boxed_bytes(key);
        let message = self.mb_to_boxed_bytes(message);
        let signature = self.mb_to_boxed_bytes(signature);
        self.verify_ed25519_legacy(
            key.as_slice(),
            message.as_slice(),
            signature.as_slice(),
        )
    }

    fn verify_secp256k1_legacy(&self, _key: &[u8], _message: &[u8], _signature: &[u8]) -> bool {
        panic!("verify_secp256k1 not implemented yet!")
    }

    fn verify_secp256k1_managed(
        &self,
        _key: Self::ManagedBufferHandle,
        _message: Self::ManagedBufferHandle,
        _signature: Self::ManagedBufferHandle,
    ) -> bool {
        panic!("verify_secp256k1 not implemented yet!")
    }

    fn verify_custom_secp256k1_legacy(
        &self,
        _key: &[u8],
        _message: &[u8],
        _signature: &[u8],
        _hash_type: MessageHashType,
    ) -> bool {
        panic!("verify_custom_secp256k1 not implemented yet!")
    }

    fn verify_custom_secp256k1_managed(
        &self,
        _key: Self::ManagedBufferHandle,
        _message: Self::ManagedBufferHandle,
        _signature: Self::ManagedBufferHandle,
        _hash_type: MessageHashType,
    ) -> bool {
        panic!("verify_custom_secp256k1 not implemented yet!")
    }

    fn encode_secp256k1_der_signature_legacy(&self, _r: &[u8], _s: &[u8]) -> BoxedBytes {
        panic!("encode_secp256k1_signature not implemented yet!")
    }

    fn encode_secp256k1_der_signature_managed(
        &self,
        _r: Self::ManagedBufferHandle,
        _s: Self::ManagedBufferHandle,
        _dest: Self::ManagedBufferHandle,
    ) {
        panic!("encode_secp256k1_signature not implemented yet!")
    }
}
