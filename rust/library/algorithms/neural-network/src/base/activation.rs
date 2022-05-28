/// Activation functions
#[derive(Debug)]
pub enum Activation {
    None,
    Sigmoid,
    Relu,
}
impl Activation {
    pub fn apply(&self, input: f32) -> f32 {
        match self {
            Self::Sigmoid => Self::sigmoid(input),
            _ => unimplemented!("activation function not implemented"),
            // Self::None => input,
            // Self::Relu => self.relu(input),
        }
    }
    pub fn apply_prime(&self, input: f32) -> f32 {
        match self {
            Self::Sigmoid => Self::sigmoid_prime(input),
            _ => unimplemented!("activation prime function not implemented"),
        }
    }

    /// https://en.wikipedia.org/wiki/Sigmoid_function
    fn sigmoid(input: f32) -> f32 {
        1.0 / (1.0 + (-input).exp())
    }
    fn sigmoid_prime(input: f32) -> f32 {
        let a = Self::sigmoid(input);
        a * (1.0 - a)
    }

    /// https://en.wikipedia.org/wiki/Rectifier_(neural_networks)
    fn relu(&self, input: f32) -> f32 {
        input.max(0.0)
    }
}
