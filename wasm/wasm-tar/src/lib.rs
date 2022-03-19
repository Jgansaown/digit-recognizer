use wasm_bindgen::prelude::*;

use flate2::read::GzDecoder;
use tar::Archive;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn test_tar(bytes: Vec<u8>) {
    let tar = GzDecoder::new(&bytes[..]);
    let mut archive = Archive::new(tar);
    
    for e in archive.entries().unwrap() {
        let entry = e.unwrap();
        log(format!("{:?}", entry.path()).as_str());
    }
}
