use ndarray::{Array1, Array2, Axis, Zip};

use crate::{DATA_SIZE, NUM_OUTPUT};

pub struct Adaline {
    /// 10 x 784
    weights: Array2<f32>,
    /// 10 x 1
    biases: Array2<f32>,
    learning_rate: f32,
}
impl Adaline {
    pub fn new(learning_rate: f32) -> Self {
        Self {
            weights: Array2::zeros((NUM_OUTPUT, DATA_SIZE)),
            biases: Array2::zeros((NUM_OUTPUT, 1)),
            learning_rate,
        }
    }

    ///
    /// data: 784 x N
    /// label: 10 x N
    pub fn train(&mut self, data: &Array2<f32>, label: &Array2<f32>) -> f32 {
        // println!("weights={:?}, biases={:?}, data={:?}, label={:?}", self.weights.shape(), self.biases.shape(), data.shape(), label.shape());

        // let output = self.net_input(&data);
        let output = self.sigmoid_net_input(&data);
        let errors = &label.clone() - &output;

        // println!("output={:?}, errors={:?}", output.shape(), errors.shape());
        // println!("output={:?}", output);

        // println!("{:?}, {:?}", self.biases.shape(), errors.sum_axis(Axis(1)).into_shape((NUM_OUTPUT, 1)).unwrap().shape());

        // self.weights = &self.weights + self.learning_rate * &data.dot(&errors.t()).t();
        self.weights = &self.weights + self.learning_rate * &errors.dot(&data.t());

        // println!("{:?}", (self.learning_rate * errors.sum_axis(Axis(1))).shape());
        self.biases = &self.biases
            + self.learning_rate
                * errors
                    .sum_axis(Axis(1))
                    .into_shape((NUM_OUTPUT, 1))
                    .unwrap();

        // println!("biases={:?}", self.biases);
        // println!("{:?}", errors.sum_axis(Axis(1)));

        let cost = errors.mapv(|v| v.powi(2)).sum() / 2.0;
        cost
    }

    pub fn test(&self, data: &Array2<f32>, label: &Array2<f32>) -> usize {
        let output = self.predict(&data);
        let errors = label - &output;
        let mut num_errors = 0;
        for row in errors.columns() {
            if row.sum() != 0.0 {
                num_errors += 1;
            }
        }
        num_errors
    }

    pub fn net_input(&self, input: &Array2<f32>) -> Array2<f32> {
        &self.weights.dot(input) + &self.biases
    }

    pub fn sigmoid_net_input(&self, input: &Array2<f32>) -> Array2<f32> {
        self.net_input(input).mapv(|v| 1.0 / (1.0 + (-v).exp()))
    }

    pub fn predict(&self, input: &Array2<f32>) -> Array2<f32> {
        let mut output = self.net_input(input);
        output.map_inplace(|o| {
            if *o >= 0.0 {
                *o = 1.0;
            } else {
                *o = 0.0;
            }
        });
        output
    }
}
