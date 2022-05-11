use gz::decode_from_path;
use std::fs;

fn save_file() {
    let data = decode_from_path("../files/mnist-training-data.gz");
    let label = decode_from_path("../files/mnist-training-label.gz");
    fs::write("../files/decoded/mnist-training-data", data).unwrap();
    fs::write("../files/decoded/mnist-training-label", label).unwrap();

    let data = decode_from_path("../files/mnist-test-data.gz");
    let label = decode_from_path("../files/mnist-test-label.gz");
    fs::write("../files/decoded/mnist-test-data", data).unwrap();
    fs::write("../files/decoded/mnist-test-label", label).unwrap();
}

fn main() {
    save_file();
}
