use mnist::Dataset;
use ndarray::{Array2, ArrayBase, Axis, Data, Ix1, Ix2};

pub struct PerceptronParam {
    learning_rate: f64,
    n_input: usize,
    n_output: usize,
    max_iter: usize,
    min_error_rate: f64,
}

impl PerceptronParam {
    pub fn learning_rate(mut self, rate: f64) -> Self {
        self.learning_rate = rate;
        self
    }
    pub fn n_input(mut self, num: usize) -> Self {
        self.n_input = num;
        self
    }
    pub fn n_output(mut self, num: usize) -> Self {
        self.n_output = num;
        self
    }
    pub fn max_iter(mut self, iter: usize) -> Self {
        self.max_iter = iter;
        self
    }
    pub fn min_error_rate(mut self, rate: f64) -> Self {
        self.min_error_rate = rate;
        self
    }

    pub fn train(&self, dataset: &Dataset) -> Perceptron {
        // shape: (n_observations, n_inputs)
        let input = dataset.inputs().view();
        let target = dataset.targets();
        // shape: (n_observations, n_outputs)
        let target = target.view();

        let mut model = Perceptron {
            weights: Array2::zeros((1 + self.n_input, self.n_output)),
        };

        let mut n_iter = 0;

        println!(
            "input: {:?}, target: {:?}, weights: {:?}",
            input.dim(),
            target.dim(),
            model.weights.dim()
        );

        loop {
            let output = model.feed_forward(&input);
            let difference = &target - &output;
            let update = self.learning_rate * &difference;

            model.update_weights(&update, &input);

            let error_rate = model.calculate_error(&difference) / dataset.num as f64;
            n_iter += 1;

            println!("[{}] {:?}", n_iter, error_rate);

            if n_iter >= self.max_iter || error_rate < self.min_error_rate {
                break;
            }
        }

        model
    }
}

pub struct Perceptron {
    /// shape: (1 + n_input, n_output)
    weights: Array2<f64>,
}
impl Perceptron {
    /// Default parameters
    /// ```ignore
    /// PerceptronParam {
    ///     learning_rate: 0.1,
    ///     n_input: 28 * 28,
    ///     n_output: 10,
    ///     max_iter: 100,
    ///     tolerance: 0.001,
    /// }
    /// ```
    pub fn with_default_param() -> PerceptronParam {
        PerceptronParam {
            learning_rate: 0.1,
            n_input: 28 * 28,
            n_output: 10,
            max_iter: 100,
            min_error_rate: 0.001,
        }
    }

    pub fn evaluate(&self, dataset: &Dataset) -> f64 {
        // shape: (n_observations, n_inputs)
        let input = dataset.inputs().view();
        let target = dataset.targets();
        // shape: (n_observations, n_outputs)
        let target = target.view();

        let output = self.feed_forward(&input);
        let difference = &target - &output;
        self.calculate_error(&difference) / dataset.num as f64
    }

    pub fn predict(&self, observation: &ArrayBase<impl Data<Elem = f64>, Ix1>) -> Array2<f64> {
        self.feed_forward(&observation.to_shape((1, observation.dim())).unwrap())
    }
}

impl Perceptron {
    /// y = f(x * w + b)
    ///
    /// - f: activation function
    /// - w: weights (data_size, n_output)
    /// - x: inputs (n_observation, data_size)
    /// - b: biases (1, n_output)
    /// - y: output (n_output, n_observations)
    fn feed_forward(
        &self,
        // shape: (n_observations, data_size)
        input: &ArrayBase<impl Data<Elem = f64>, Ix2>,
    ) -> Array2<f64> {
        let (b, w) = self.weights.view().split_at(Axis(0), 1);
        let y = input.dot(&w) + b;

        // activation function
        y.mapv(|v| if v >= 0.0 { 1.0 } else { 0.0 })
    }

    /// Updates weights
    fn update_weights(
        &mut self,
        // shape: (n_observations, n_output)
        update: &ArrayBase<impl Data<Elem = f64>, Ix2>,
        // shape: (n_observations, data_size)
        input: &ArrayBase<impl Data<Elem = f64>, Ix2>,
    ) {
        let (mut b, mut w) = self.weights.view_mut().split_at(Axis(0), 1);
        w += &input.t().dot(update);
        b += &update.sum_axis(Axis(0));
    }

    fn calculate_error(&self, difference: &ArrayBase<impl Data<Elem = f64>, Ix2>) -> f64 {
        difference
            .map(|v| v.abs())
            .sum_axis(Axis(1))
            .map(|&v| if v != 0.0 { 1.0 } else { 0.0 })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use ndarray::Array1;

    use super::*;

    #[test]
    fn test_() {
        let dataset = Dataset {
            num: 10,
            size: 10,
            images: Array2::zeros((10, 10)),
            labels: Array1::zeros(10),
        };

        let param = Perceptron::with_default_param()
            .n_input(10)
            .n_output(10)
            .learning_rate(0.1)
            .max_iter(1000)
            .min_error_rate(0.30);

        let model = param.train(&dataset);
        model.evaluate(&dataset);
        model.predict(&dataset.at(0).image);
    }
}
