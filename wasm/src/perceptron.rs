use crate::Dataset;
use perceptron::Perceptron as InnerPerceptron;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Perceptron(InnerPerceptron);

#[wasm_bindgen]
impl Perceptron {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        todo!()
    }
    pub fn step(&mut self, dataset: &Dataset) -> f64 {
        todo!()
    }
    pub fn evaluate(&self, dataset: &Dataset) -> f64 {
        todo!()
    }
    pub fn predict(&self) {
        todo!()
    }
}
