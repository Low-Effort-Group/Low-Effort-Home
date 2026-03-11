use wasm_bindgen::prelude::wasm_bindgen;
use bcrypt::hash as bcrypt_hash;
use base64::{engine::general_purpose, Engine as _};
use sha2::{Sha224, Sha256, Sha384, Sha512, Digest as Sha2};
use md5::{Md5};
use hex;

#[wasm_bindgen]
extern "C" {
    pub fn console_log(s: &str);
}

// General utilities
#[wasm_bindgen]
pub fn str_to_bytes(s: &str) -> Vec<u8> {
    return s.as_bytes().to_vec();
}

// Encoding
#[wasm_bindgen] 
pub fn base64(data: &[u8]) -> String { // Might be slower than btoa in smaller data inputs however I have more control this way.
    return general_purpose::STANDARD.encode(data);
}

// Hash algorithms
#[wasm_bindgen]
pub fn bcrypt(password: &str, cost: u32) -> String {
    return bcrypt_hash(password, cost).unwrap();
}

#[wasm_bindgen]
pub fn md5(data: &[u8]) -> String {
    return hex::encode(Md5::digest(data));
}

#[wasm_bindgen]
pub fn sha224(data: &[u8]) -> String {
    return hex::encode(Sha224::digest(data));
}

#[wasm_bindgen]
pub fn sha256(data: &[u8]) -> String {
    return hex::encode(Sha256::digest(data));
}

#[wasm_bindgen]
pub fn sha384(data: &[u8]) -> String {
    return hex::encode(Sha384::digest(data));
}

#[wasm_bindgen]
pub fn sha512(data: &[u8]) -> String {
    return hex::encode(Sha512::digest(data));
}