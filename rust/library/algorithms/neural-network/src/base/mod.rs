//! Contains the building blocks for building neural networks
//!
//! ```
//! use neural_network::base::{Network, Activation};
//!
//! let network = Network::build()
//!     .input(784)
//!     .dense(16, Activation::Relu)
//!     .dense(10, Activation::Relu)
//!     .done();
//!
//! //network.fit(&input);
//! //let output = network.predict(&input);
//! ```
//!

mod activation;
mod layer;
mod node;

use mnist::Dataset;

pub use self::activation::Activation;
use self::layer::{DenseLayer, Layer};

pub struct Network {
    layers: Vec<Box<dyn Layer>>,
    input_size: usize,
    output_size: usize,
}
impl Network {
    pub fn build() -> NetworkBuilder {
        NetworkBuilder::default()
    }
    pub fn fit(&self) {}
    pub fn predict(&self) {}

    pub fn test(&mut self, data: &[Vec<f32>], label: &[u8]) -> usize {
        let num = data.len();
        assert_eq!(num, label.len());

        let mut incorrect = 0;
        for i in 0..num {
            let output = self.evaluate(&data[i]);
            if label[i] != output {
                incorrect += 1;
            }
        }
        incorrect
    }

    pub fn gradient_descent(&mut self, data: &[Vec<f32>], label: &[Vec<f32>], eta: f32) -> f32 {
        let num = data.len();
        assert_eq!(num, label.len());

        let mut cost = 0.0;
        for i in 0..num {
            let input = &data[i];
            let target = &label[i];

            assert_eq!(input.len(), self.input_size);
            assert_eq!(target.len(), self.output_size);

            // Feed forward to find the predicted output
            let output = self.feed_forward(input);
            assert_eq!(output.len(), self.output_size);

            // println!("{:?}, {:?}", target, output);

            // The cost of current weights
            cost += self.cost(target, &output);

            // Back propagate
            self.back_propagate(target, &output);
        }
        self.update_weights(eta, num);
        cost / num as f32
    }

    fn feed_forward(&mut self, input: &[f32]) -> Vec<f32> {
        let output = self
            .layers
            .iter_mut()
            .fold(input.to_vec(), |acc, layer| layer.feed_forward(acc));

        assert_eq!(input.len(), self.input_size);
        assert_eq!(output.len(), self.output_size);
        output
    }

    fn back_propagate(&mut self, target: &[f32], output: &[f32]) {
        // find the 'change' in output layer (partial derivative)
        let change: Vec<f32> = target.iter().zip(output).map(|(y, a)| (a - y)).collect();

        self.layers
            .iter_mut()
            .rev()
            .fold(change, |acc, layer| layer.back_propagate(&acc));
    }

    fn update_weights(&mut self, eta: f32, num: usize) {
        self.layers
            .iter_mut()
            .for_each(|layer| layer.update_weights(eta, num));
    }

    fn cost(&self, target: &[f32], output: &[f32]) -> f32 {
        target
            .iter()
            .zip(output)
            .map(|(t, o)| 0.5 * (t - o).powi(2))
            .sum()
    }

    fn evaluate(&mut self, input: &[f32]) -> u8 {
        let output = self.feed_forward(input);

        let (p, _) = output
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();
        p as u8
    }
}

#[derive(Default)]
pub struct NetworkBuilder {
    input_size: Option<usize>,
    next_size: Option<usize>,
    layers: Vec<Box<dyn Layer>>,
}
impl NetworkBuilder {
    pub fn input(mut self, size: usize) -> Self {
        self.input_size = Some(size);
        self.next_size = Some(size);
        self
    }
    pub fn dense(mut self, output: usize, activation: Activation) -> Self {
        if let Some(input) = self.next_size {
            println!(
                "Adding Dense Layer: input={}, output={}, activation={:?}",
                input, output, activation
            );
            self.layers
                .push(Box::new(DenseLayer::new(input, output, 0.0, activation)));
            self.next_size = Some(output);
        }
        self
    }
    pub fn done(self) -> Network {
        Network {
            input_size: self.input_size.unwrap_or_default(),
            output_size: self.next_size.unwrap_or_default(),
            layers: self.layers,
        }
    }
}

fn load_training() -> (Vec<Vec<f32>>, Vec<Vec<f32>>) {
    let dataset = mnist::Dataset::load_from_path(
        "../../../../files/decoded/mnist-training-data",
        "../../../../files/decoded/mnist-training-label",
    );
    let input: Vec<Vec<f32>> = dataset
        .iter()
        .map(|d| d.value.iter().map(|&v| (v as f32) / (255 as f32)).collect())
        .collect();
    let label: Vec<Vec<f32>> = dataset
        .iter()
        .map(|d| {
            let mut o = vec![0.0; 10];
            o[d.label as usize] = 1.0;
            o
        })
        .collect();
    (input, label)
}
fn load_testing() -> (Vec<Vec<f32>>, Vec<u8>) {
    let dataset = mnist::Dataset::load_from_path(
        "../../../../files/decoded/mnist-test-data",
        "../../../../files/decoded/mnist-test-label",
    );
    let input: Vec<Vec<f32>> = dataset
        .iter()
        .map(|d| d.value.iter().map(|&v| (v as f32) / (255 as f32)).collect())
        .collect();
    let label: Vec<u8> = dataset
        .iter()
        .map(|d| {
            d.label
        })
        .collect();
    (input, label)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_building_network() {
        let mut network = Network::build()
            .input(784)
            .dense(16, Activation::Sigmoid)
            .dense(10, Activation::Sigmoid)
            .done();

        let input = [0.0; 784];
        let output = network.feed_forward(&input);
        // println!("{:?}", output);

        assert_eq!(output.len(), 10);
        assert_eq!(output, [0.5; 10]);
    }

    #[test]
    fn test_1_layer_network() {
        let mut network = Network::build()
            .input(784)
            .dense(10, Activation::Sigmoid)
            .done();

        let (input, label) = load_training();
        for i in 0..100 {
            let cost = network.gradient_descent(&input, &label, 0.01);
            println!("1 layer={}: {}", i, cost);
        }
        let (input, label) = load_testing();
        let incorrect = network.test(&input, &label);
        println!("one layer errors: {}/{}={}", incorrect, input.len(), incorrect as f32 / input.len() as f32)
    }

    #[test]
    fn test_2_layer_network() {
        let mut network = Network::build()
            .input(784)
            .dense(20, Activation::Sigmoid)
            .dense(10, Activation::Sigmoid)
            .done();
        
        let (input, label) = load_training();
        for i in 0..100 {
            let cost = network.gradient_descent(&input, &label, 0.01);
            println!("2 layer={}: {}", i, cost);
        }
        let (input, label) = load_testing();
        let incorrect = network.test(&input, &label);
        println!("two layer errors: {}/{}={}", incorrect, input.len(), incorrect as f32 / input.len() as f32)
    }
}
