use wasm_bindgen::prelude::*;
use exif::Reader;
use std::io::Cursor;

fn jpeg_tiff(data: &[u8]) -> bool {
    // JPEG: FF D8
    if data.len() > 2 && data[0] == 0xFF && data[1] == 0xD8 {
        return true;
    }
    // TIFF little endian: 49 49 2A 00, big endian: 4D 4D 00 2A
    if data.len() > 4 {
        if (data[0] == 0x49 && data[1] == 0x49 && data[2] == 0x2A && data[3] == 0x00) ||
           (data[0] == 0x4D && data[1] == 0x4D && data[2] == 0x00 && data[3] == 0x2A) {
            return true;
        }
    }
    false
}

#[wasm_bindgen]
pub fn exif(data: &[u8]) -> String {
    if !jpeg_tiff(data) {
        return "Unsupported image format".to_string();
    }

    let mut cursor = Cursor::new(data);
    let mut bufreader = std::io::BufReader::new(&mut cursor);
    let exifreader = Reader::new();

    let exif = exifreader.read_from_container(&mut bufreader).unwrap(); 

    let mut string: String = Default::default();
    for f in exif.fields() {
        let tag = f.tag.to_string();
        let val = f.display_value().to_string();
        string.push_str(&format!("{}: {}\n", tag, val));
    }

    if string.is_empty() {
        string.push_str("No EXIF fields found\n");
    }

    return string;
}