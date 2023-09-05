mod kmeans;
mod knn;
mod perceptron;

use crate::dataset::Dataset;

use wasm_bindgen::prelude::*;

pub use kmeans::KMeans;
pub use knn::KNearestNeighbors;
pub use perceptron::Perceptron;

pub trait Model {
    fn step(&mut self, dataset: &Dataset) -> f64;
    fn evaluate(&self, dataset: &Dataset) -> f64;
    // TODO: fn predict(&self);
}

#[wasm_bindgen]
pub struct TemplateModel();

#[wasm_bindgen]
impl TemplateModel {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        todo!()
    }

    #[allow(unused_variables)]
    pub fn step(&mut self, dataset: &Dataset) -> f64 {
        todo!()
    }

    #[allow(unused_variables)]
    pub fn evaluate(&self, dataset: &Dataset) -> f64 {
        todo!()
    }

    #[allow(unused_variables)]
    pub fn predict(&self, observation: Vec<f64>) -> Vec<f64> {
        todo!()
    }
}

impl Default for TemplateModel {
    fn default() -> Self {
        Self::new()
    }
}
