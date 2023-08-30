mod dataset;
mod kmeans;
mod knn;
mod perceptron;

pub use crate::dataset::Dataset;
pub use crate::kmeans::KMeans;
pub use crate::knn::KNearestNeighbors;
pub use crate::perceptron::Perceptron;

use wasm_bindgen::prelude::*;

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

#[wasm_bindgen]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
