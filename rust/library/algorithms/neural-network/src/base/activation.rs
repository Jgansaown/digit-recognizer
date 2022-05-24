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
            Self::None => input,
            Self::Sigmoid => self.sigmoid(input),
            Self::Relu => self.relu(input),
        }
    }

    /// https://en.wikipedia.org/wiki/Sigmoid_function
    fn sigmoid(&self, input: f32) -> f32 {
        1.0 / (1.0 + -input.exp())
    }

    /// https://en.wikipedia.org/wiki/Rectifier_(neural_networks)
    fn relu(&self, input: f32) -> f32 {
        input.max(0.0)
    }
}
