use super::{activation::Activation, node::Node};

/// Layer Trait
pub trait Layer {
    fn forward_pass(&self, input: &[f32]) -> Vec<f32>;
}

/// Generic Dense Layer (Fully Connected Layer)
///
#[derive(Debug)]
pub struct DenseLayer {
    input_size: usize,
    output_size: usize,
    nodes: Vec<Node>,
    activator: Activation,
}
impl DenseLayer {
    pub fn new(input: usize, output: usize, initial: f32, activator: Activation) -> Self {
        Self {
            input_size: input,
            output_size: output,
            nodes: vec![Node::new(input, initial); output],
            activator,
        }
    }
}
impl Layer for DenseLayer {
    fn forward_pass(&self, input: &[f32]) -> Vec<f32> {
        let output: Vec<f32> = self
            .nodes
            .iter()
            .map(|n| n.output(input, &self.activator))
            .collect();
        assert_eq!(input.len(), self.input_size);
        assert_eq!(output.len(), self.output_size);
        output
    }
}
