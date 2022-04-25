use mnist::Dataset;

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

pub struct LinearClassifier<const N: usize> {
    weights: Vec<Node>,
    learning_rate: f32,
}
impl<const N: usize> LinearClassifier<N> {
    pub fn new() -> Self {
        Self {
            weights: vec![Node::new(N); 10],
            learning_rate: 0.001,
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

    fn net_input(&self, data: &[u8]) -> Vec<f32> {
        self.weights
            .iter()
            .map(|weights| weights.output(data))
            .collect()
    }

    pub fn predict(&self, data: &[u8]) -> Vec<f32> {
        self.net_input(data)
            .iter()
            .map(|&output| if output >= 0.0 { 1.0 } else { 0.0 })
            .collect()
    }
}
