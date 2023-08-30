use crate::Dataset;
use kmeans::KMeans as InnerKMeans;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct KMeans(InnerKMeans);

#[wasm_bindgen]
impl KMeans {
    #[wasm_bindgen(constructor)]
    pub fn new(n_clusters: usize) -> Self {
        Self(InnerKMeans::new(n_clusters, 28 * 28))
    }

    pub fn step(&mut self, dataset: &Dataset) -> f64 {
        self.0.step(&dataset)
    }

    pub fn evaluate(&self, dataset: &Dataset) -> f64 {
        let correct = self.0.evaluate(&dataset) as f64;
        let num = dataset.num as f64;
        (num - correct) / num
    }
}
