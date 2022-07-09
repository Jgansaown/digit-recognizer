use std::ops::Add;

use rand::{thread_rng, Rng};

use super::activation::Activation;

#[derive(Clone, Debug)]
pub struct Node {
    weights: Vec<f32>,
    bias: f32,

    z: f32,
    a: f32,
    delta_w: Vec<f32>,
    delta_b: f32,
}
impl Node {
    pub fn new(size: usize, initial: f32) -> Self {
        // let distr = rand::distributions::Uniform::new_inclusive(-1.0, 1.0);
        // let mut rng = thread_rng();
        // let weights: Vec<f32> = (&mut rng).sample_iter(distr).take(size).collect();
        // print!("{:?}", weights);

        Self {
            weights: vec![initial; size],
            // weights,

            bias: initial,

            z: 0.0,
            a: 0.0,
            delta_w: vec![0.0; size],
            delta_b: 0.0,
        }
    }
    pub fn feed_forward(&mut self, input: &[f32], activation: &Activation) -> f32 {
        self.z = self
            .weights
            .iter()
            .zip(input)
            .map(|(w, i)| w * i)
            .sum::<f32>()
            .add(self.bias);
        self.a = activation.apply(self.z);
        // dbg!(&self.a);
        self.a
    }
    pub fn back_propagate(&mut self, input: &[f32], change: f32, activation: &Activation) -> f32 {
        let error = change * activation.apply_prime(self.z);

        self.delta_w.iter_mut().zip(input).for_each(|(d, a)| {
            *d += error * a;
        });
        self.delta_b += error;

        error
    }
    pub fn update_weights(&mut self, eta: f32, num: usize) {
        self.weights
            .iter_mut()
            .zip(self.delta_w.iter())
            .for_each(|(w, dw)| {
                *w -= eta / (num as f32) * dw;
            });
        self.bias -= eta / (num as f32) * self.delta_b;
        
        // clear cached values
        self.delta_w.iter_mut().for_each(|dw| *dw = 0.0);
        self.delta_b = 0.0;
    }
}
