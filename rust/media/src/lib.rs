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
    if format == "png" {
        img.write_to(&mut buf, ImageFormat::Png).unwrap();
    } else if format == "jpeg" {
        img.write_to(&mut buf, ImageFormat::Jpeg).unwrap();
    } else if format == "webp" {
        img.write_to(&mut buf, ImageFormat::WebP).unwrap();
    } else if format == "avif" {
        img.write_to(&mut buf, ImageFormat::Avif).unwrap();
    } else if format == "gif" {
        img.write_to(&mut buf, ImageFormat::Gif).unwrap();
    } else if format == "ico" {
        img.write_to(&mut buf, ImageFormat::Ico).unwrap();
    }
    return buf.into_inner();
}