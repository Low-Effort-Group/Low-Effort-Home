use wasm_bindgen::prelude::*;
use bcrypt::hash as bcrypt_hash;
use base64::{engine::general_purpose, Engine as _};

#[wasm_bindgen]
extern "C" {
    pub fn console_log(s: &str);
}

#[wasm_bindgen]
pub fn bcrypt(password: &str, cost: u32) -> String {
    return bcrypt_hash(password, cost).unwrap();
}

fn str_to_bytes(s: &str) -> Vec<u8> { // Consider making this available to the javascript side.
    return s.as_bytes().to_vec();
}

#[wasm_bindgen] 
pub fn base64(data: &[u8]) -> String { // Might be slower than btoa in smaller data inputs however I have more control this way.
    return general_purpose::STANDARD.encode(data);
}

#[wasm_bindgen]
pub fn base64_str(data: &str) -> String { // It's ~1 ms faster than calling str_to_bytes(base64(data)).
    return general_purpose::STANDARD.encode(str_to_bytes(data));
}