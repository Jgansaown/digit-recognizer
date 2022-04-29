use ndarray::{Array2, Axis, Array1};

use crate::{NUM_OUTPUT, DATA_SIZE};


pub struct Adaline {
    weights: Array2<f32>,
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

    pub fn train(&mut self, data: &Array2<f32>, label: &Array2<f32>) -> f32 {
        println!("weights={:?}, biases={:?}, data={:?}, label={:?}", self.weights.shape(), self.biases.shape(), data.shape(), label.shape());

        let output = self.net_input(data);
        let errors = &label.clone() - &output;

        println!("output={:?}, errors={:?}", output.shape(), errors.shape());
        println!("output={:?}", output);

        // println!("{:?}, {:?}", self.biases.shape(), errors.sum_axis(Axis(1)).into_shape((NUM_OUTPUT, 1)).unwrap().shape());
        
        
        self.weights = &self.weights + self.learning_rate * &data.dot(&errors.t()).t();
        
        println!("{:?}", (self.learning_rate * errors.sum_axis(Axis(1))).shape());
        self.biases = &self.biases + self.learning_rate * errors.sum_axis(Axis(1)).into_shape((NUM_OUTPUT, 1)).unwrap();

        println!("biases={:?}", self.biases);
        println!("{:?}", errors.sum_axis(Axis(1)));

        let cost = errors.mapv(|v| v.powi(2)).sum() / 2.0;
        cost
    }

    pub fn test(&self) {

    }

    pub fn net_input(&self, input: &Array2<f32>) -> Array2<f32> {
        &self.weights.dot(input) + &self.biases
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