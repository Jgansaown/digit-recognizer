use mnist::Dataset;

#[cfg(feature = "multithread")]
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct Node {
    weights: Vec<f32>,
    bias: f32,
}
impl Node {
    fn new(num_of_weights: usize) -> Self {
        Self {
            weights: vec![0.0; num_of_weights],
            bias: 0.0,
        }
    }

    /// Calculate the output of this node
    ///
    /// data must have same length as weights
    fn output(&self, data: &[u8]) -> f32 {
        assert_eq!(self.weights.len(), data.len());
        let dot_product: f32 = self
            .weights
            .iter()
            .zip(data)
            .map(|(w, x)| w * (*x as f32))
            .sum();
        dot_product + self.bias
    }

    fn update(&mut self, change: f32, data: &[u8]) {
        self.weights.iter_mut().zip(data).for_each(|(w, &d)| {
            *w += change * (d as f32);
        });
        self.bias += change;
    }
}
#[cfg(feature = "multithread")]
impl Node {
    /// Calculate the output of this node
    ///
    /// data must have same length as weights
    fn par_output(&self, data: &[u8]) -> f32 {
        assert_eq!(self.weights.len(), data.len());
        let dot_product: f32 = self
            .weights
            .par_iter()
            .zip(data)
            .map(|(w, x)| w * (*x as f32))
            .sum();
        dot_product + self.bias
    }
}

pub struct LinearClassifier {
    weights: Vec<Node>,
    learning_rate: f32,
}
impl LinearClassifier {
    pub fn new(input_length: usize, num_output: usize, learning_rate: f32) -> Self {
        Self {
            weights: vec![Node::new(input_length); num_output],
            learning_rate,
        }
    }

    pub fn train(&mut self, dataset: &Dataset) {
        for data in dataset.iter() {
            let mut target: [f32; 10] = [0.0; 10];
            target[data.label as usize] = 1.0;

            let predicts: Vec<f32> = self.predict(&data.value);
            let deltas: Vec<f32> = predicts
                .iter()
                .zip(&target)
                .map(|(p, t)| self.learning_rate * (t - p))
                .collect();

            self.weights
                .iter_mut()
                .zip(deltas)
                .for_each(|(node, change)| node.update(change, data.value));
        }
    }

    pub fn test(&self, dataset: &Dataset) -> usize {
        let mut errors: usize = 0;
        for data in dataset.iter() {
            let predicts = self.predict(data.value);
            let predicts: Option<usize> =
                predicts
                    .iter()
                    .enumerate()
                    .find_map(|(i, &p)| if p == 1.0 { Some(i) } else { None });
            match predicts {
                Some(p) => {
                    if p != data.label as usize {
                        errors += 1;
                    }
                }
                None => errors += 1,
            }
        }
        errors
    }

    fn net_input(&self, data: &[u8]) -> Vec<f32> {
        self.weights
            .iter()
            .map(|weights| weights.output(data))
            .collect()
    }

    pub fn predict(&self, data: &[u8]) -> Vec<f32> {
        self.weights
            .iter()
            .map(|weights| weights.output(data))
            .map(|output| if output >= 0.0 { 1.0 } else { 0.0 })
            .collect()
    }
}

#[cfg(feature = "multithread")]
impl LinearClassifier {
    pub fn par_test(&self, dataset: &Dataset) -> usize {
        let errors: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let mut dataset = dataset.to_vec();
        dataset.par_iter_mut().for_each(|(label, data)| {
            let predicts = self.par_predict(data);
            let predicts: Option<usize> =
                predicts
                    .iter()
                    .enumerate()
                    .find_map(|(i, &p)| if p == 1.0 { Some(i) } else { None });
            match predicts {
                Some(p) if p != (*label as usize) => {
                    errors.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                }
                None => {
                    errors.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                }
                _ => {},
            }
        });

        errors.into_inner()
    }
    
    pub fn par_predict(&self, data: &[u8]) -> Vec<f32> {
        self.weights
            .par_iter()
            .map(|weights| weights.par_output(data))
            .map(|output| if output >= 0.0 { 1.0 } else { 0.0 })
            .collect()
    }
}


