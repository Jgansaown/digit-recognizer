mod mnist;
pub use crate::mnist::MnistDataset;

use ndarray::Array1;
use neural_network::base::Activation;
use neural_network::matrix::Network;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::console_log;

#[wasm_bindgen]
pub fn run_network(train: MnistDataset, test: MnistDataset) {
    console_error_panic_hook::set_once();

    console_log!("Loading training and testing dataset...");
    let input: Vec<Array1<f32>> = train.as_array_data();
    let label: Vec<Array1<f32>> = train.as_array_label();
    let test_input: Vec<Array1<f32>> = test.as_array_data();
    let test_label: Vec<u8> = test.as_u8_label();

    console_log!("Building 2-layers neural network...");
    let mut network = Network::build()
        .input(784)
        .dense(16, Activation::Sigmoid)
        .dense(16, Activation::Sigmoid)
        .dense(10, Activation::Sigmoid)
        .done();

    console_log!("Training neural network...");
    let test_n = test_input.len();
    for i in 0..1000 {
        let cost =
            network.stochastic_gradient_descent(input.as_slice(), label.as_slice(), 3.0, 10000);
        let correct = network.test(&test_input, &test_label);
        console_log!("{:<4}: cost={:.5}, test={:>4}/{}", i, cost, correct, test_n);
    }
    console_log!("Done!");
}
