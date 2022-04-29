use linear_classifier::{dataset_to_arrays, Adaline};

use mnist::Dataset;

fn run_adaline(num_iter: usize, learning_rate: f32, train: &Dataset, test: &Dataset) {
    let (train_data, train_label) = dataset_to_arrays(train);
    let (test_data, test_label) = dataset_to_arrays(test);

    let mut a = Adaline::new(learning_rate);

    for i in 0..num_iter {
        let cost = a.train(&train_data, &train_label);
        let errors = a.test(&test_data, &test_label);
        println!(
            "Iteration: {}, Training Cost: {}, Number of Errors: {}, Error Rate={}",
            i,
            cost,
            errors,
            (errors as f32) / (test.num as f32)
        );
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

    run_adaline(1000, 0.00001, &training_dataset, &test_dataset);
}
