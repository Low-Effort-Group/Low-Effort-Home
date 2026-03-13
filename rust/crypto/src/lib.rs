use wasm_bindgen::prelude::wasm_bindgen;
use base64::{engine::general_purpose, Engine as _};
use sha1::Sha1;
use sha2::{Sha256, Sha512, Digest as Sha2};
use md5::{Md5};
use bcrypt::hash as bcrypt_hash;
use hex;

// #[wasm_bindgen]
// extern "C" {
//     pub fn console_log(s: &str);
// }

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

#[wasm_bindgen]
pub fn encode(data: &[u8], cost: u32, file: bool) -> Vec<String> {
    let md5 = hex::encode(Md5::digest(data));
    let sha1 = hex::encode(Sha1::digest(data));
    let sha256 = hex::encode(Sha256::digest(data));
    let sha512 = hex::encode(Sha512::digest(data));
    let bcrypt: String;

    if !file {
        bcrypt = bcrypt_hash(data, cost).unwrap_or_else(|_| "Error".to_string());
    } else {
        bcrypt = "Bcrypt not available in file context".to_string();
    }

    return vec![md5, sha1, sha256, sha512, bcrypt];
}