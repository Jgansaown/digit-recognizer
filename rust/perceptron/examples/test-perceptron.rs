use std::time::Instant;

use mnist::Dataset;
use ndarray::ArrayView1;
use perceptron::Perceptron;

fn main() {
    let training = Dataset::training();
    let testing = Dataset::testing();

    let param = Perceptron::with_default_param()
        .learning_rate(0.1)
        .max_iter(1000)
        .min_error_rate(0.30);

    let start = Instant::now();
    let model = param.train(&training);
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);

    let error_rate = model.evaluate(&testing);

    println!("Test Error Rate: {}", error_rate,);

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
