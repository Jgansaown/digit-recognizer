use flate2::read::GzDecoder;
use std::fs;
use std::io::Read;
use std::path::Path;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn decode_gz(gzfile: &[u8]) -> Vec<u8> {
    let mut ret = Vec::new();
    let mut file = GzDecoder::new(gzfile);
    file.read_to_end(&mut ret).unwrap();
    ret
}

pub fn decode_from_path<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let data = fs::read(path).expect("Something went wrong reading file from path");
    decode_gz(&data)
}
