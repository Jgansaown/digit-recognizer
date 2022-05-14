use flate2::read::GzDecoder;
use std::fs;
use std::io::Read;
use std::path::Path;
use tar::Archive;

pub fn untargz(tar_gz: &[u8]) -> [Vec<u8>; 4] {
    let mut raw_buffers = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    let mut archive = Archive::new(GzDecoder::new(tar_gz));

    for file in archive.entries().unwrap() {
        let mut file = file.unwrap();
        if let Ok(p) = file.header().path() {
            let raw = match p.to_str() {
                Some("mnist-test-data") => Some(&mut raw_buffers[0]),
                Some("mnist-test-label") => Some(&mut raw_buffers[1]),
                Some("mnist-training-data") => Some(&mut raw_buffers[2]),
                Some("mnist-training-label") => Some(&mut raw_buffers[3]),
                _ => None,
            };
            if let Some(buf) = raw {
                file.read_to_end(buf).expect("error reading file to buffer");
            }
        }
    }
    raw_buffers
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
