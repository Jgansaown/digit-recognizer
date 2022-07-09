use base64::{write::EncoderStringWriter, STANDARD};
use image::{DynamicImage, GrayImage, RgbaImage};
use std::io::{Cursor, Write};
use std::path::Path;
use wasm_bindgen::prelude::*;

const BLACK_IMG: &'static str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABwAAAAcCAIAAAD9b0jDAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAJcEhZcwAAFiUAABYlAUlSJPAAAAAZSURBVEhL7cExAQAAAMKg9U9tB28gAABONQlMAAEdn/sHAAAAAElFTkSuQmCC";

#[wasm_bindgen]
pub fn data_as_png_base64_string(data: &[u8]) -> String {
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

#[wasm_bindgen]
pub fn rgba_image_to_grayscale_image(data: &[u8]) -> Vec<u8> {
    let rgba = RgbaImage::from_raw(28, 28, data.to_vec()).unwrap();
    let gray = DynamicImage::ImageRgba8(rgba).into_luma8();
    gray.into_vec()
}

#[wasm_bindgen]
pub fn get_black_image() -> String {
    BLACK_IMG.to_string()
}

pub fn save_as_image<P: AsRef<Path>>(path: P, data: &[u8]) {
    let gray = GrayImage::from_raw(28, 28, data.to_vec()).unwrap();
    let img = DynamicImage::ImageLuma8(gray);
    img.save(path).unwrap();
}
