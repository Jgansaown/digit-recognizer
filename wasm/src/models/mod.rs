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
