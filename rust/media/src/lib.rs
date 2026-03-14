use wasm_bindgen::prelude::wasm_bindgen;
use std::io::Cursor;
use image::ImageFormat::{Png, Jpeg, WebP, Avif, Gif};

// #[wasm_bindgen]
// extern "C" {
//     pub fn console_log(s: &str);
// }

#[wasm_bindgen]
pub fn transcode(data: &[u8], format: &str) -> Vec<u8> {
    let img = image::load_from_memory(data).unwrap();
    let mut buf = Cursor::new(Vec::new());
    match format {
        "png" => img.write_to(&mut buf, Png).unwrap(),
        "jpeg" => img.write_to(&mut buf, Jpeg).unwrap(),
        "webp" => img.write_to(&mut buf, WebP).unwrap(),
        "avif" => img.write_to(&mut buf, Avif).unwrap(),
        "gif" => img.write_to(&mut buf, Gif).unwrap(),
        _ => panic!("Unknown format"),
    }
    return buf.into_inner();
}