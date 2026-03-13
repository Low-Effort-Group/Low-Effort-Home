use wasm_bindgen::prelude::*;
use std::io::Cursor;
use image::ImageFormat;

// #[wasm_bindgen]
// extern "C" {
//     pub fn console_log(s: &str);
// }

#[wasm_bindgen]
pub fn transcode(data: &[u8], format: &str) -> Vec<u8> {
    let img = image::load_from_memory(data).unwrap();
    let mut buf = Cursor::new(Vec::new());
    match format {
        "png" => img.write_to(&mut buf, ImageFormat::Png).unwrap(),
        "jpeg" => img.write_to(&mut buf, ImageFormat::Jpeg).unwrap(),
        "webp" => img.write_to(&mut buf, ImageFormat::WebP).unwrap(),
        "avif" => img.write_to(&mut buf, ImageFormat::Avif).unwrap(),
        "gif" => img.write_to(&mut buf, ImageFormat::Gif).unwrap(),
        "ico" => img.write_to(&mut buf, ImageFormat::Ico).unwrap(),
        _ => panic!("Unknown format"),
    }
    return buf.into_inner();
}