use crate::dataset::Dataset;
use ndarray::{Array2, ArrayBase, Axis, Data, Ix2};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Perceptron {
    learning_rate: f64,
    /// shape: (1 + n_input, n_output)
    weights: Array2<f64>,
}

#[wasm_bindgen]
impl Perceptron {
    #[wasm_bindgen(constructor)]
    pub fn new(learning_rate: f64, n_input: usize, n_output: usize) -> Self {
        Self {
            learning_rate,
            weights: Array2::zeros((1 + n_input, n_output)),
        }
    }

    pub fn step(&mut self, dataset: &Dataset) -> f64 {
        let difference = self.calculate_difference(dataset);
        self.update_weights(&(self.learning_rate * &difference), dataset.observations());
        self.calculate_error(&difference) / dataset.n_observations() as f64
    }

    pub fn evaluate(&self, dataset: &Dataset) -> f64 {
        let difference = self.calculate_difference(dataset);
        self.calculate_error(&difference) / dataset.n_observations() as f64
    }

    pub fn predict(&self, observation: Vec<f64>) -> Vec<f64> {
        let observation = Array2::from_shape_vec((1, observation.len()), observation).unwrap();
        self.feed_forward(&observation).into_raw_vec()
    }
}

impl Perceptron {
    fn calculate_difference(&self, dataset: &Dataset) -> Array2<f64> {
        let output = self.feed_forward(dataset.observations());
        dataset.targets() - &output
    }

    /// y = f(x * w + b)
    ///
    /// - f: activation function
    /// - w: weights (data_size, n_output)
    /// - x: inputs (n_observation, data_size)
    /// - b: biases (1, n_output)
    /// - y: output (n_output, n_observations)
    fn feed_forward(
        &self,
        // shape: (n_observations, data_size)
        input: &ArrayBase<impl Data<Elem = f64>, Ix2>,
    ) -> Array2<f64> {
        let (b, w) = self.weights.view().split_at(Axis(0), 1);
        let y = input.dot(&w) + b;

        // activation function
        y.mapv(|v| if v >= 0.0 { 1.0 } else { 0.0 })
    }

    /// Updates weights
    fn update_weights(
        &mut self,
        // shape: (n_observations, n_output)
        update: &ArrayBase<impl Data<Elem = f64>, Ix2>,
        // shape: (n_observations, data_size)
        input: &ArrayBase<impl Data<Elem = f64>, Ix2>,
    ) {
        let (mut b, mut w) = self.weights.view_mut().split_at(Axis(0), 1);
        w += &input.t().dot(update);
        b += &update.sum_axis(Axis(0));
    }

    fn calculate_error(&self, difference: &ArrayBase<impl Data<Elem = f64>, Ix2>) -> f64 {
        difference
            .map(|v| v.abs())
            .sum_axis(Axis(1))
            .map(|&v| if v != 0.0 { 1.0 } else { 0.0 })
            .sum()
    }
}

impl super::Model for Perceptron {
    fn step(&mut self, dataset: &Dataset) -> f64 {
        self.step(dataset)
    }

    fn evaluate(&self, dataset: &Dataset) -> f64 {
        self.evaluate(dataset)
    }
}
