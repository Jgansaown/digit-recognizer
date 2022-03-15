use std::fs;

use rust_digit_recognition::mnist;
use rust_digit_recognition::helper::{Clusters, Centroids};

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

    let ds = mnist::Dataset::load(data, labels);
    println!("{}, {}", ds.num, ds.size);

    let mut iter = ds.iter();
    let data1 = iter.next().unwrap();
    let data2 = iter.next().unwrap();
    let dist = data1.euclidean_distance(&data2);
    println!("{:?}", data1);
    println!("{:?}", data2);
    println!("dist={}", dist);

    let initial = Centroids::new(28*28, 10, vec![0; 28 * 28 * 10]);
    let clusters = Clusters::new(ds, initial);

    println!("{:?}", clusters.centroids.iter().next());
}
