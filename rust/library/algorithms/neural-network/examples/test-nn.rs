use neural_network::base::Activation;
use neural_network::matrix::Network;

use ndarray::{Array1, arr1};

fn load_training() -> (Vec<Array1<f32>>, Vec<Array1<f32>>) {
    let dataset = mnist::Dataset::load_from_path(
        "../files/decoded/mnist-training-data",
        "../files/decoded/mnist-training-label",
    );
    let input: Vec<Array1<f32>> = dataset
        .iter()
        .map(|d| {
            arr1(
                &d.value
                    .iter()
                    .map(|&v| (v as f32) / (255 as f32))
                    .collect::<Vec<_>>(),
            )
        })
        .collect();
    let label: Vec<Array1<f32>> = dataset
        .iter()
        .map(|d| {
            let mut o: Array1<f32> = Array1::zeros(10);
            o[d.label as usize] = 1.0;
            o
        })
        .collect();
    (input, label)
}
fn load_testing() -> (Vec<Array1<f32>>, Vec<u8>) {
    let dataset = mnist::Dataset::load_from_path(
        "../files/decoded/mnist-test-data",
        "../files/decoded/mnist-test-label",
    );
    let input: Vec<Array1<f32>> = dataset
        .iter()
        .map(|d| {
            arr1(
                &d.value
                    .iter()
                    .map(|&v| (v as f32) / (255 as f32))
                    .collect::<Vec<_>>(),
            )
        })
        .collect();
    let label: Vec<u8> = dataset.iter().map(|d| d.label).collect();
    (input, label)
}

fn main() {
    let mut network = Network::build()
        .input(784)
        .dense(16, Activation::Sigmoid)
        .dense(16, Activation::Sigmoid)
        .dense(10, Activation::Sigmoid)
        .done();

    let (input, label): (Vec<Array1<f32>>, Vec<Array1<f32>>) = load_training();
    let (test_input, test_label): (Vec<Array1<f32>>, Vec<u8>) = load_testing();
    let test_n = test_input.len();
    for i in 0..10000 {
        let cost = network.par_stochastic_gradient_descent(&input, &label, 3.0, 1000);
        // println!("{:<4}: cost={:.5}", i, cost);
        let correct = network.test(&test_input, &test_label);
        println!("{:<4}: cost={:.5}, test={:>4}/{}", i, cost, correct, test_n);
    }
    // for (input, label) in izip!(test_input, test_label) {
    //     let output = network.predict(input.view());
    //     println!("{}: {}", label, output);
    //     break;
    // }
}
