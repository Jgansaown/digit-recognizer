use flate2::read::GzDecoder;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use tar::Archive;

use std::collections::HashMap;

pub fn unpack_tar_gz(tar_gz: &[u8]) -> HashMap<PathBuf, Vec<u8>> {
    let mut files = HashMap::new();
    let mut archive = Archive::new(GzDecoder::new(tar_gz));
    for file in archive.entries().unwrap() {
        let mut file = file.unwrap();
        if let Ok(path) = file.header().path().map(|p| p.to_path_buf()) {
            let mut raw = Vec::new();
            file.read_to_end(&mut raw)
                .expect("error reading file to buffer");
            files.insert(path, raw);
        }
    }
    files
}

pub fn decode_gz(gzfile: &[u8]) -> Vec<u8> {
    let mut ret = Vec::new();
    let mut file = GzDecoder::new(gzfile);
    file.read_to_end(&mut ret)
        .expect("Error while reading decoded file");
    ret
}

pub fn decode_from_path<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let data = fs::read(path).expect("Something went wrong reading file from path");
    decode_gz(&data)
}
