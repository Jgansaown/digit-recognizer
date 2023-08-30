use crate::Dataset;
use knn::KNearestNeighbors as InnerKnn;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct KNearestNeighbors(InnerKnn);

#[wasm_bindgen]
impl KNearestNeighbors {
    #[wasm_bindgen(constructor)]
    pub fn new(k: usize) -> Self {
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