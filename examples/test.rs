use std::fs;

use rust_digit_recognition::mnist;

fn load_training_data() -> (Vec<u8>, Vec<u8>) {
    let data = fs::read("./files/mnist-training-data")
        .expect("Something went wrong reading the data file");
    let labels = fs::read("./files/mnist-training-label")
        .expect("Something went wrong reading the label file");
    (data, labels)
}
fn load_testing_data() -> (Vec<u8>, Vec<u8>) {
    let data =
        fs::read("./files/mnist-test-data").expect("Something went wrong reading the data file");
    let labels =
        fs::read("./files/mnist-test-label").expect("Something went wrong reading the label file");
    (data, labels)
}

fn main() {
    let (data, labels) = load_training_data();

    println!("MNIST test data size = {}", data.len());
    println!("MNIST test label size = {}", labels.len());

    let ds = mnist::Dataset::load(data, labels).expect("Error loading dataset");

    println!("{}, {}, {}", ds.nums, ds.rows, ds.cols);

    for (data, label) in ds.iter() {
        println!("{:?}", label);
    }
}
