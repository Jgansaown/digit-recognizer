use gz::decode_from_path;
use std::fs;

const FOLDER: &str = "../files"; //path from workspace root

fn decode_and_save(filename: &str) {
    let file = decode_from_path(format!("{}/{}.gz", FOLDER, filename));
    fs::write(format!("{}/{}", FOLDER, filename), file).expect("error while saving file");
}

fn main() {
    // decode_and_save("mnist-training-data");
    // decode_and_save("mnist-training-label");
    // decode_and_save("mnist-test-data");
    // decode_and_save("mnist-test-label");

    let file = fs::read(format!("{}/{}", FOLDER, "mnist-data.tar.gz")).unwrap();

    let ret = gz::untargz(&file);

    for r in ret {
        println!("{}", r.len());
    }
}
