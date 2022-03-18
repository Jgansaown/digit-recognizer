pub mod kmeans;
mod cluster;

use base64::{write::EncoderStringWriter, STANDARD};
use std::io::{Cursor, Write};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_nth_image(data: Vec<u8>, labels: Vec<u8>, n: usize) -> Vec<u8> {
    let ds = mnist::Dataset::load(data, labels);
    ds.iter().nth(n).unwrap().value.to_vec()
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn as_png_base64_string(data: &[u8]) -> String {
    let mut ret = String::from("data:image/png;base64,");
    let mut writer = Cursor::new(Vec::new());

    image::write_buffer_with_format(
        &mut writer,
        data,
        28,
        28,
        image::ColorType::L8,
        image::ImageOutputFormat::Png,
    )
    .unwrap();

    let mut enc = EncoderStringWriter::from(&mut ret, STANDARD);
    enc.write_all(writer.get_ref()).unwrap();
    let _ = enc.into_inner();

    ret
}
