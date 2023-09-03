use std::time::Instant;

use ndarray::ArrayView1;
use mnist::dataset::{Dataset, MNIST};
use mnist::models::{KMeans, KNearestNeighbors, Model, Perceptron};

fn test_model<M: Model + ?Sized>(
    model: &mut M,
    training: &Dataset,
    testing: &Dataset,
    max_iteration: usize,
) {
    let start = Instant::now();
    for i in 0..max_iteration {
        let err_rate = model.step(&training);
        println!("{}: {}", i, err_rate);
    }
    let duration = start.elapsed();
    println!(
        "Time elapsed in training for {} iteration: {:?}",
        max_iteration, duration
    );

    let start = Instant::now();
    let err_rate = model.evaluate(&testing);
    let duration = start.elapsed();
    println!("Time elapsed in testing: {:?}", duration);

    println!("Testing Result: {}", err_rate);
}

fn print_image(data: ArrayView1<f64>) {
    let data = data.to_shape((28, 28)).unwrap();
    for i in 0..28usize {
        for j in 0..28usize {
            let v = data[(i, j)];
            match v {
                _ if v > 128.0 => print!("⬜"),
                _ => print!("⬛"),
            }
        }
        println!("");
    }
}

fn test_kmeans(training: &Dataset, testing: &Dataset, max_iteration: usize) {
    let mut model = KMeans::new(10, training.data_size());
    test_model(&mut model, &training, &testing, max_iteration);
}

fn test_knn(training: &Dataset, testing: &Dataset, max_iteration: usize) {
    let mut model = KNearestNeighbors::new(10);
    // test_model(&mut model, &training, &testing, max_iteration);

    let start = Instant::now();
    let err_rate = model.step(&training);
    println!("[knn] Training: {}", err_rate);
    let duration = start.elapsed();
    println!(
        "Time elapsed in training for {} iteration: {:?}",
        max_iteration, duration
    );

    let start = Instant::now();
    let err_rate = model.evaluate(&testing);
    let duration = start.elapsed();
    println!("Time elapsed in evaluate: {:?}", duration);
    println!("Testing Result: {}", err_rate);

    let observation = testing.observations().row(0);
    let start = Instant::now();
    let prediction = model.calculate_prediction(&observation);
    let duration = start.elapsed();
    println!("Prediction: {:?}", duration);

    print_image(observation);
    println!("{:?}", prediction);
}

fn test_perceptron(training: &Dataset, testing: &Dataset, max_iteration: usize) {
    let mut model = Perceptron::new(0.01, training.data_size(), training.target_size());
    test_model(&mut model, &training, &testing, max_iteration);
}

fn main() {
    let training = MNIST::training_from_static();
    let testing = MNIST::testing_from_static();

    let max_iteration = 100;

    // test_kmeans(&training, &testing, max_iteration);
    // test_knn(&training, &testing, max_iteration);
    test_perceptron(&training, &testing, max_iteration);
}
