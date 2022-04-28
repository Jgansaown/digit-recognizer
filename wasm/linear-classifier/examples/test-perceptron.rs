use linear_classifier::perceptron::dataset_to_arrays;
use linear_classifier::Perceptron;
use mnist::Dataset;

use mnist::save_as_image;

fn main() {
    let training_dataset = Dataset::load_from_path(
        "../files/decoded/mnist-training-data",
        "../files/decoded/mnist-training-label",
    );

    let mut p = Perceptron::new(0.01);

    let (data, label) = dataset_to_arrays(&training_dataset);
    p.train(&data, &label);
}
