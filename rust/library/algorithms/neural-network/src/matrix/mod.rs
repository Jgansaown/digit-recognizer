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

mod layer;

use std::{
    ops::Neg,
    sync::{Arc, Mutex},
};

use itertools::izip;
use ndarray::{Array1, Array2, ArrayView1, ArrayView2};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::base::Activation;

use self::layer::{DenseLayer, Layer};

struct LayerCache<'a, T> {
    layer: Option<&'a dyn Layer>,
    z: Option<Array1<T>>,
    a: Array1<T>,
}
/// Cache for network, used to help with back propagation
///
/// index 0 = input layer
/// index 1 = first layer
/// index cache.len() - 1 = last layer
struct NetworkCache<'a, T> {
    inner: Vec<LayerCache<'a, T>>,
}
impl<'a, T> NetworkCache<'a, T> {
    /// Create a cache for the network
    fn new(input: Array1<T>) -> Self {
        Self {
            inner: vec![LayerCache {
                layer: None,
                z: None,
                a: input,
            }],
        }
    }
    fn push(&mut self, layer: &'a dyn Layer, z: Array1<T>, a: Array1<T>) {
        self.inner.push(LayerCache {
            layer: Some(layer),
            z: Some(z),
            a,
        });
    }
    fn len(&self) -> usize {
        self.inner.len()
    }
    fn get(&self, l: isize) -> Option<&LayerCache<'_, T>> {
        if l >= 0 {
            self.inner.get(l as usize)
        } else {
            self.inner.get(self.inner.len() - l.unsigned_abs())
        }
    }
    fn get_unchecked(&self, l: isize) -> &LayerCache<'_, T> {
        if l >= 0 {
            unsafe { self.inner.get_unchecked(l as usize) }
        } else {
            unsafe {
                self.inner
                    .get_unchecked(self.inner.len() - l.unsigned_abs())
            }
        }
    }
    fn last(&self) -> &LayerCache<'_, T> {
        self.inner.last().unwrap()
    }
}
impl<'a> NetworkCache<'a, f32> {
    fn activation_prime(&self, l: isize) -> Array1<f32> {
        let cache = self.get_unchecked(l);
        let layer = cache.layer.unwrap();
        let z = cache.z.as_ref().unwrap();
        layer.activation_prime(z.view())
    }

    fn delta_last_layer(&self, cost_derivative: Array1<f32>) -> Array1<f32> {
        cost_derivative * self.activation_prime(-1)
    }
    /// delta of layers l < L
    ///
    /// prev_delta = delta^(l+1)
    /// prev_w = w^(l+1)
    fn delta(&self, l: isize, prev_delta: ArrayView1<f32>) -> Array1<f32> {
        let ap: Array1<_> = self.activation_prime(l);
        let prev_w: ArrayView2<_> = self.get_unchecked(l + 1).layer.unwrap().weights();

        // println!("l: {}, prev_delta: {}, prev_w: {}", l, prev_delta, prev_w);
        prev_w.t().dot(&prev_delta) * ap
    }

    fn nabla(&self, l: isize, delta: ArrayView1<f32>) -> (Array2<f32>, Array1<f32>) {
        let db: Array1<_> = delta.to_owned();

        let prev_a: ArrayView1<_> = self.get_unchecked(l - 1).a.view();
        // need to broadcast into 2D vector to properly do matrix dot product
        let delta: ArrayView2<_> = delta.broadcast((1, delta.len())).unwrap();
        let prev_a: ArrayView2<_> = prev_a.broadcast((1, prev_a.len())).unwrap();
        // calc
        let dw: Array2<_> = delta.t().dot(&prev_a);
        (dw, db)
    }
}

pub struct Network {
    layers: Vec<Box<dyn Layer + Send + Sync>>,
    input_size: usize,
    output_size: usize,
}
impl Network {
    pub fn build() -> NetworkBuilder {
        NetworkBuilder::default()
    }

    fn predict(&self, input: ArrayView1<f32>) -> Array1<f32> {
        // feed forward
        self.layers
            .iter()
            .fold(input.to_owned(), |acc: Array1<_>, l| {
                let (_, a): (_, Array1<_>) = l.feed_forward(acc.view());
                a
            })
    }

    fn test(&self, data: &[Array1<f32>], label: &[u8]) -> usize {
        let mut correct = 0;
        for (input, &target) in izip!(data, label) {
            let output = self.predict(input.view());
            // pick predicted digit
            let (p, _) = output
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .unwrap();

            if p as u8 == target {
                correct += 1;
            }
        }

        // TODO: return confusion matrix

        correct
    }

    fn gradient_descent(&mut self, data: &[Array1<f32>], label: &[Array1<f32>], eta: f32) -> f32 {
        let mut cost = 0.0;
        // back propagation over all inputs
        let mut nabla: Vec<(Array2<f32>, Array1<f32>)> = self.new_nabla();
        for (input, target) in izip!(data, label) {
            cost += self.back_propagation(input.view(), target.view(), &mut nabla);
        }

        // average nabla
        let n = data.len();
        for (dw, db) in &mut nabla {
            dw.mapv_inplace(|v| eta * v / n as f32);
            db.mapv_inplace(|v| eta * v / n as f32);
        }

        // update weights and biases
        for (layer, (dw, db)) in izip!(&mut self.layers, &nabla) {
            layer.update_weights_and_biases(dw.view(), db.view());
        }

        cost / (2.0 * n as f32)
    }

    /// Run back propagation with the network
    ///
    /// Takes in an input that will be fed into the network,
    /// and the targeted output that will be used to calculate the gradients
    ///
    fn back_propagation<'a>(
        &'a self,
        input: ArrayView1<f32>,
        target: ArrayView1<f32>,
        nabla: &mut [(Array2<f32>, Array1<f32>)],
    ) -> f32 {
        // Storing all the outputs: (layer l, z^l, a^l)
        let mut cache: NetworkCache<'a, f32> = NetworkCache::new(input.to_owned());

        // forward pass and cache the outputs
        for layer in &self.layers {
            let input: ArrayView1<_> = cache.last().a.view();
            let (z, a): (Array1<_>, Array1<_>) = layer.feed_forward(input);
            cache.push(layer.as_ref(), z, a);
        }

        // backward pass
        let mut delta: Array1<_> = Array1::zeros(0);

        for l in (((cache.len() - 1) as isize).neg()..=-1).rev() {
            if l == -1 {
                // calculate delta of last layer
                let output: ArrayView1<_> = cache.last().a.view();
                delta = cache.delta_last_layer(self.cost_derivative(output, target));
            } else {
                // calculate delta of layer based on last delta
                delta = cache.delta(l, delta.view());
            }
            // find the gradients for layer's weights and biases
            let (dw, db): (Array2<f32>, Array1<f32>) = cache.nabla(l, delta.view());

            // update nabla
            let (nabla_w, nabla_b) = nabla.get_mut(nabla.len() - l.unsigned_abs()).unwrap();
            nabla_w.zip_mut_with(&dw, |nabla, w| *nabla += w);
            nabla_b.zip_mut_with(&db, |nabla, b| *nabla += b);
        }

        self.cost(cache.last().a.view(), target)
    }

    fn new_nabla(&self) -> Vec<(Array2<f32>, Array1<f32>)> {
        self.layers
            .iter()
            .map(|l| {
                let dw: Array2<f32> = Array2::zeros(l.weights().raw_dim());
                let db: Array1<f32> = Array1::zeros(l.biases().raw_dim());
                (dw, db)
            })
            .collect()
    }

    /// Cost Function (Loss Function/Objective Function)
    ///
    fn cost(&self, output: ArrayView1<f32>, target: ArrayView1<f32>) -> f32 {
        (&target - &output).mapv(|v| v.powi(2)).sum().sqrt().powi(2)
    }

    fn cost_derivative(&self, output: ArrayView1<f32>, target: ArrayView1<f32>) -> Array1<f32> {
        &output - &target
    }
}
impl Network {
    fn par_gradient_descent(
        &mut self,
        data: &[Array1<f32>],
        label: &[Array1<f32>],
        eta: f32,
    ) -> f32 {
        let cost = Arc::new(Mutex::new(0.0));
        // back propagation over all inputs
        let mut nabla: Vec<(Array2<f32>, Array1<f32>)> = data
            .par_iter()
            .zip(label)
            .fold(
                || self.new_nabla(),
                |mut nabla, (input, target)| {
                    let c = self.back_propagation(input.view(), target.view(), &mut nabla);

                    *cost.clone().lock().unwrap() += c;
                    nabla
                },
            )
            .reduce(
                || self.new_nabla(),
                |mut acc, value| {
                    acc.iter_mut().zip(value).for_each(|(a, v)| {
                        a.0.zip_mut_with(&v.0, |a, v| *a += v);
                        a.1.zip_mut_with(&v.1, |a, v| *a += v);
                    });
                    acc
                },
            );

        // average nabla
        let n = data.len();
        for (dw, db) in &mut nabla {
            dw.mapv_inplace(|v| eta * v / n as f32);
            db.mapv_inplace(|v| eta * v / n as f32);
        }
        // update weights and biases
        for (layer, (dw, db)) in izip!(&mut self.layers, &nabla) {
            layer.update_weights_and_biases(dw.view(), db.view());
        }

        let c = *cost.lock().unwrap();
        c / (2.0 * n as f32)
    }
}

#[derive(Default)]
pub struct NetworkBuilder {
    input_size: Option<usize>,
    next_size: Option<usize>,
    layers: Vec<Box<dyn Layer + Send + Sync>>,
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

            // self.layers
            //     .push(Box::new(DenseLayer::new(input, output, activation)));
            self.layers
                .push(Box::new(DenseLayer::random(input, output, activation)));

            self.next_size = Some(output);
        }
        self
    }
    pub fn custom(mut self, layer: Box<dyn Layer + Send + Sync>, output: usize) -> Self {
        if let Some(input) = self.next_size {
            println!("Adding Custom Layer: input={}", input);
            self.layers.push(layer);
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

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr1;

    fn load_training() -> (Vec<Array1<f32>>, Vec<Array1<f32>>) {
        let dataset = mnist::Dataset::load_from_path(
            "../../../../files/decoded/mnist-training-data",
            "../../../../files/decoded/mnist-training-label",
        );
        let input: Vec<Array1<f32>> = dataset
            .iter()
            .map(|d| {
                arr1(
                    &d.value
                        .iter()
                        .map(|&v| (v as f32) / (255 as f32))
                        .collect::<Vec<_>>(),
                )
            })
            .collect();
        let label: Vec<Array1<f32>> = dataset
            .iter()
            .map(|d| {
                let mut o: Array1<f32> = Array1::zeros(10);
                o[d.label as usize] = 1.0;
                o
            })
            .collect();
        (input, label)
    }
    fn load_testing() -> (Vec<Array1<f32>>, Vec<u8>) {
        let dataset = mnist::Dataset::load_from_path(
            "../../../../files/decoded/mnist-test-data",
            "../../../../files/decoded/mnist-test-label",
        );
        let input: Vec<Array1<f32>> = dataset
            .iter()
            .map(|d| {
                arr1(
                    &d.value
                        .iter()
                        .map(|&v| (v as f32) / (255 as f32))
                        .collect::<Vec<_>>(),
                )
            })
            .collect();
        let label: Vec<u8> = dataset.iter().map(|d| d.label).collect();
        (input, label)
    }

    #[test]
    fn test_building_network() {
        let network = Network::build()
            .input(784)
            .dense(16, Activation::Sigmoid)
            .dense(10, Activation::Sigmoid)
            .done();

        let input: Array1<f32> = Array1::zeros(784);
        let output = network.predict(input.view());
        // println!("{:?}", output);

        assert_eq!(output.len(), 10);
        assert_eq!(output, arr1(&[0.5; 10]));
    }

    #[test]
    fn test_network() {
        let mut network = Network::build()
            .input(784)
            .dense(20, Activation::Sigmoid)
            .dense(20, Activation::Sigmoid)
            .dense(10, Activation::Sigmoid)
            .done();

        let (input, label): (Vec<Array1<f32>>, Vec<Array1<f32>>) = load_training();
        let (test_input, test_label): (Vec<Array1<f32>>, Vec<u8>) = load_testing();
        let test_n = test_input.len();
        for i in 0..1000 {
            // let cost = network.gradient_descent(&input, &label, 1.0);
            let cost = network.par_gradient_descent(&input, &label, 3.0);
            // println!("{}: cost={}", i, cost);
            let correct = network.test(&test_input, &test_label);
            println!("{}: cost={}, test={}/{}", i, cost, correct, test_n);
        }
        for (input, label) in izip!(test_input, test_label) {
            let output = network.predict(input.view());
            println!("{}: {}", label, output);
            break;
        }
    }

    #[test]
    fn test_matrix_layer() {
        let layer = DenseLayer::new(3, 1, Activation::Sigmoid);
        let input: Array1<f32> = Array1::ones(3);

        let (z, a): (Array1<_>, Array1<_>) = layer.feed_forward(input.view());

        println!("{}", layer.as_string());
        println!("z={:?}, a={:?}", z, a);
    }
}
