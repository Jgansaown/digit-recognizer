use std::fs;

mod mnist;
use mnist::Dataset;

fn get_mnist_data(data_path: &str, label_path: &str) {
    let data = fs::read(data_path).expect("Something went wrong reading the data file");
    let label = fs::read(label_path).expect("Something went wrong reading the label file");

    let dataset = Dataset::new(data, label);
    println!(
        "Dataset: nums={}, rows={}, cols={}",
        dataset.nums, dataset.rows, dataset.cols
    );
}

fn main() {
    get_mnist_data("files/mnist-training-data", "files/mnist-training-label");
}
