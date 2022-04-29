use linear_classifier::{Adaline, dataset_to_arrays};

use mnist::Dataset;

fn run_adaline(num_iter: usize, learning_rate: f32, train: &Dataset, test: &Dataset) {
    let (train_data, train_label) = dataset_to_arrays(train);
    let (test_data, test_label) = dataset_to_arrays(test);

    let mut a = Adaline::new(learning_rate);

    for i in 0..num_iter {
        println!("Iteration: {}", i);
        let cost = a.train(&train_data, &train_label);
        println!("cost={}", cost);

    //     let num_error = p.test(&test_data, &test_label);
    //     println!(
    //         "{}: Number of Errors = {}, Error Rate = {}",
    //         i,
    //         num_error,
    //         (num_error as f32) / (test.num as f32)
    //     );
    }
}

fn main() {
    let training_dataset = Dataset::load_from_path(
        "../files/decoded/mnist-training-data",
        "../files/decoded/mnist-training-label",
    );
    let test_dataset = Dataset::load_from_path(
        "../files/decoded/mnist-test-data",
        "../files/decoded/mnist-test-label",
    );

    run_adaline(2, 0.01, &training_dataset, &test_dataset);
}