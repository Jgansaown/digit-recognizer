use gz;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn unpack_gz(file: Vec<u8>) -> Vec<u8> {
    gz::decode_gz(&file)
}

#[wasm_bindgen]
pub fn unpack_tar_gz(file: Vec<u8>) -> JsValue {
    JsValue::from_serde(&gz::unpack_tar_gz(&file)).unwrap()
}
