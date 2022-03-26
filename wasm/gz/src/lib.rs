use flate2::read::GzDecoder;
use std::io::Read;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn decode_gz(gzfile: &[u8]) -> Vec<u8> {
    let mut ret = Vec::new();
    let mut file = GzDecoder::new(gzfile);
    file.read_to_end(&mut ret).unwrap();
    ret
}
