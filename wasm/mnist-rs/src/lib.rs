mod mnist;

pub use crate::mnist::MnistDataset;

use ndarray::Array1;
use neural_network::base::Activation;
use neural_network::matrix::Network;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::console_log;

#[cfg(feature = "multithread")]
pub use wasm_bindgen_rayon::init_thread_pool;

#[cfg(feature = "multithread")]
#[wasm_bindgen]
pub fn par_run_network(train: MnistDataset, test: MnistDataset, iterations: usize) {
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
    for i in 0..iterations {
        let cost =
            network.par_stochastic_gradient_descent(input.as_slice(), label.as_slice(), 3.0, 10000);
        let correct = network.test(&test_input, &test_label);
        console_log!("{:<4}: cost={:.5}, test={:>4}/{}", i, cost, correct, test_n);
    }
    console_log!("Done!");
}

#[wasm_bindgen]
pub fn run_network(train: MnistDataset, test: MnistDataset, iterations: usize) {
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
    for i in 0..iterations {
        let cost =
            network.stochastic_gradient_descent(input.as_slice(), label.as_slice(), 3.0, 10000);
        let correct = network.test(&test_input, &test_label);
        console_log!("{:<4}: cost={:.5}, test={:>4}/{}", i, cost, correct, test_n);
    }
    console_log!("Done!");
}

#[wasm_bindgen]
pub struct NeuralNetwork {
    inner: Network,
}
#[wasm_bindgen]
impl NeuralNetwork {
    #[wasm_bindgen]
    pub fn create_1_layer_network(hidden: usize) -> Self {
        let inner = Network::build()
            .input(784)
            .dense(hidden, Activation::Sigmoid)
            .dense(10, Activation::Sigmoid)
            .done();

        Self { inner }
    }
    #[wasm_bindgen]
    pub fn create_2_layer_network(hidden1: usize, hidden2: usize) -> Self {
        let inner = Network::build()
            .input(784)
            .dense(hidden1, Activation::Sigmoid)
            .dense(hidden2, Activation::Sigmoid)
            .dense(10, Activation::Sigmoid)
            .done();

        Self { inner }
    }

    #[wasm_bindgen]
    pub fn stochastic_gradient_descent(
        &mut self,
        train: MnistDataset,
        learn_rate: f32,
        batch_size: usize,
    ) {
        let input: Vec<Array1<f32>> = train.as_array_data();
        let label: Vec<Array1<f32>> = train.as_array_label();

        self.inner.stochastic_gradient_descent(
            input.as_slice(),
            label.as_slice(),
            learn_rate,
            batch_size,
        );
    }
}
