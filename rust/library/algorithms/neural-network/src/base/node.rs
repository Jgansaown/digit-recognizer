use std::ops::Add;

use super::activation::Activation;

#[derive(Clone, Debug)]
pub struct Node {
    weights: Vec<f32>,
    bias: f32,
}
impl Node {
    pub fn new(size: usize, initial: f32) -> Self {
        Self {
            weights: vec![initial; size],
            bias: initial,
        }
    }
    pub fn output(&self, input: &[f32], activation: &Activation) -> f32 {
        let out = self
            .weights
            .iter()
            .zip(input)
            .map(|(w, i)| w * i)
            .sum::<f32>()
            .add(self.bias);
        activation.apply(out)
    }
}
