use ndarray::{Array2, ArrayView2, Zip};

const NUM_OUTPUT: usize = 10;
const DATA_SIZE: usize = mnist::DATA_SIZE;

pub struct Perceptron {
    weights: Array2<f32>,
    biases: Array2<f32>,
    learning_rate: f32,
}
impl Perceptron {
    pub fn new(learning_rate: f32) -> Self {
        Self {
            weights: Array2::zeros((NUM_OUTPUT, DATA_SIZE)),
            biases: Array2::zeros((NUM_OUTPUT, 1)),
            learning_rate,
        }
    }

    pub fn train(&mut self, data: &Array2<f32>, label: &Array2<f32>) -> usize {
        let mut errors = 0;
        Zip::from(data.rows())
            .and(label.columns())
            .for_each(|input, target| {
                let target = target.into_owned().into_shape((10, 1)).unwrap();
                let output = self.predict(&input.into_shape((1, 784)).unwrap());

                // Update weights and biases
                let update = self.learning_rate * (&target - &output);
                self.weights = &self.weights + (&update * &input);
                self.biases = &self.biases + &update;

                let lost_func: u8 = (&target - &output)
                    .mapv(|v| if v != 0.0 { 1 } else { 0 })
                    .sum();
                if lost_func > 0 {
                    errors += 1;
                }
            });
        errors
    }

    pub fn test(&self, data: &Array2<f32>, label: &Array2<f32>) -> usize {
        let mut errors = 0;
        Zip::from(data.rows())
            .and(label.columns())
            .for_each(|input, target| {
                let target = target.into_owned().into_shape((10, 1)).unwrap();
                let output = self.predict(&input.into_shape((1, 784)).unwrap());

                let lost_func: u8 = (&target - &output)
                    .mapv(|v| if v != 0.0 { 1 } else { 0 })
                    .sum();
                if lost_func > 0 {
                    errors += 1;
                }
            });
        errors
    }


    pub fn net_input(&self, input: &ArrayView2<f32>) -> Array2<f32> {
        self.weights.dot(&input.t()) + &self.biases
    }

    pub fn predict(&self, input: &ArrayView2<f32>) -> Array2<f32> {
        let mut output = self.net_input(input);
        output.map_inplace(|o| {
            if *o >= 0.0 {
                *o = 1.0;
            } else {
                *o = 0.0;
            }
        });
        output
    }
}
