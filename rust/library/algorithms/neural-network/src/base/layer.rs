use super::{activation::Activation, node::Node};

/// Layer Trait
pub trait Layer {
    fn feed_forward(&mut self, input: Vec<f32>) -> Vec<f32>;
    fn back_propagate(&mut self, change: &[f32]) -> Vec<f32>;
    fn update_weights(&mut self, eta: f32, num: usize);
}

/// Generic Dense Layer (Fully Connected Layer)
///
#[derive(Debug)]
pub struct DenseLayer {
    input_size: usize,
    output_size: usize,
    nodes: Vec<Node>,
    activator: Activation,

    input: Vec<f32>,
}
impl DenseLayer {
    pub fn new(input: usize, output: usize, initial: f32, activator: Activation) -> Self {
        Self {
            input_size: input,
            output_size: output,
            nodes: vec![Node::new(input, initial); output],
            activator,

            input: Vec::new(),
        }
    }

    pub fn output_error(&self) {}
}
impl Layer for DenseLayer {
    fn feed_forward(&mut self, input: Vec<f32>) -> Vec<f32> {
        // assert_eq!(input.len(), self.input_size);
        self.input = input;
        let output: Vec<f32> = self
            .nodes
            .iter_mut()
            .map(|n| n.feed_forward(&self.input, &self.activator))
            .collect();
        // assert_eq!(output.len(), self.output_size);
        output
    }
    fn back_propagate(&mut self, change: &[f32]) -> Vec<f32> {
        // find the error in layer with change * activation prime
        self.nodes
            .iter_mut()
            .zip(change)
            .map(|(n, c)| n.back_propagate(&self.input, *c, &self.activator))
            .collect()
    }
    fn update_weights(&mut self, eta: f32, num: usize) {
        self.nodes
            .iter_mut()
            .for_each(|n| n.update_weights(eta, num));
    }
}
