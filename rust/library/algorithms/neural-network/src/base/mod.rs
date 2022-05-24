//! Contains the building blocks for building neural networks
//!
//! ```
//! use neural_network::base::{Network, Activation};
//!
//! let network = Network::new()
//!     .input(784)
//!     .dense(16, Activation::Relu)
//!     .dense(10, Activation::Relu)
//!     .build();
//!
//! //network.fit(&input);
//! //let output = network.predict(&input);
//! ```
//!

mod activation;
mod layer;
mod node;

pub use self::activation::Activation;
use self::layer::{DenseLayer, Layer};

pub struct Network {
    layers: Vec<Box<dyn Layer>>,
    input_size: usize,
    output_size: usize,
}
impl Network {
    pub fn new() -> NetworkBuilder {
        NetworkBuilder::default()
    }
    pub fn fit(&self) {}
    pub fn predict(&self) {}

    fn feed_forward(&self, input: &[f32]) -> Vec<f32> {
        let output = self
            .layers
            .iter()
            .fold(input.to_vec(), |acc, layer| layer.forward_pass(&acc));

        assert_eq!(input.len(), self.input_size);
        assert_eq!(output.len(), self.output_size);
        output
    }
    fn loss(&self, output: &[f32], target: &[f32]) -> Vec<f32> {
        output
            .iter()
            .zip(target)
            .map(|(o, t)| 0.5 * (t - o).powi(2))
            .collect()
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
    pub fn build(self) -> Network {
        Network {
            input_size: self.input_size.unwrap_or_default(),
            output_size: self.next_size.unwrap_or_default(),
            layers: self.layers,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_building_network() {
        let network = Network::new()
            .input(784)
            .dense(16, Activation::Relu)
            .dense(10, Activation::Relu)
            .build();

        let input = [0.0; 784];
        let output = network.feed_forward(&input);
        println!("{:?}", output);

        assert_eq!(output.len(), 10);
        assert_eq!(output, [0.0; 10]);
    }
}
