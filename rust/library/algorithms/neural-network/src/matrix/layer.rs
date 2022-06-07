use ndarray::{Array1, Array2, ArrayView1, Dimension, IntoDimension, ShapeBuilder, ArrayView2};

use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;

use crate::base::Activation;

/// Layer Trait
pub trait Layer {
    /// Feeds input to the layer and returns the output
    ///
    /// Outputs `(z, a)`: the output before and after activation
    fn feed_forward(&self, input: ArrayView1<f32>) -> (Array1<f32>, Array1<f32>);

    ///
    fn activation_prime(&self, input: ArrayView1<f32>) -> Array1<f32>;

    ///
    fn weights(&self) -> ArrayView2<f32>;
    fn biases(&self) -> ArrayView1<f32>;

    fn update_weights_and_biases(&mut self, dw: ArrayView2<f32>, db: ArrayView1<f32>);

    /// Debug
    fn print(&self);
}
//[row, col]

/// Generic Dense Layer (Fully Connected Layer)
///
pub struct DenseLayer {
    /// [output, input]
    weights: Array2<f32>,
    /// [output]
    biases: Array1<f32>,
    /// activation function
    activation_function: Activation,
}
impl DenseLayer {
    pub fn new(input: usize, output: usize, activation_function: Activation) -> Self {
        Self {
            weights: Array2::zeros((output, input)),
            biases: Array1::zeros(output),
            activation_function,
        }
    }
    pub fn random(input: usize, output: usize, activation_function: Activation) -> Self {
        let weights: Array2<_> = Array2::random((output, input), Uniform::new_inclusive(-1.0, 1.0));
        let biases: Array1<_> = Array1::random(output, Uniform::new_inclusive(-1.0, 1.0));
        Self {
            weights,
            biases,
            activation_function,
        }
    }

    pub fn as_string(&self) -> String {
        format!("weights: {:?}\nbiases: {:?}", self.weights, self.biases)
    }

    fn shapes(&self) -> ((usize, usize), usize) {
        (self.weights.dim(), self.biases.dim())
    }
}
impl Layer for DenseLayer {
    fn feed_forward(&self, input: ArrayView1<f32>) -> (Array1<f32>, Array1<f32>) {
        let z: Array1<_> = self.weights.dot(&input) + &self.biases;
        let a: Array1<_> = z.map(|&v| self.activation_function.apply(v));
        (z, a)
    }

    fn activation_prime(&self, input: ArrayView1<f32>) -> Array1<f32> {
        input.map(|&v| self.activation_function.apply_prime(v))
    }

    fn weights(&self) -> ArrayView2<f32> {
        self.weights.view()
    }
    fn biases(&self) -> ArrayView1<f32> {
        self.biases.view()
    }

    fn update_weights_and_biases(&mut self, dw: ArrayView2<f32>, db: ArrayView1<f32>) {
        self.weights.zip_mut_with(&dw, |w, dw| *w -= dw);
        self.biases.zip_mut_with(&db, |b, db| *b -= db);
    }

    fn print(&self) {
        println!("{:?}", self.weights);
        println!("{:?}", self.biases);
    }
}
