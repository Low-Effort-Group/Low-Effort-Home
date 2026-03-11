use wasm_bindgen::prelude::*;
use base64::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn base64_encode(input: &str) -> String {
    return BASE64_STANDARD.encode(input);
}