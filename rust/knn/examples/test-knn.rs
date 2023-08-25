use std::time::Instant;

use knn::KNearestNeighbors;
use mnist::Dataset;
use ndarray::ArrayView1;

fn main() {
    let training = Dataset::training();
    let testing = Dataset::testing();

    let model = KNearestNeighbors::with_default_param()
        .k(5)
        .train(&training);

    let start = Instant::now();
    let correct = model.evaluate(&testing);
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);

    println!(
        "Test Result: {} / {} = {}",
        correct,
        testing.num,
        correct as f64 / testing.num as f64
    );

    let observation = testing.at(0);
    let predict = model.predict(&observation.image);
    print_image(observation.image);
    println!(
        "Predicted Label: {:?}, Actual Label: {}",
        predict, observation.label
    );
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
