use std::ops::Deref;
use mnist::Dataset as InnerDataset;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Dataset(InnerDataset);

#[wasm_bindgen]
impl Dataset {
    pub fn mnist_training() -> Self {
        Self(InnerDataset::training())
    }
    pub fn mnist_testing() -> Self {
        Self(InnerDataset::testing())
    }
}

impl Deref for Dataset {
    type Target = InnerDataset;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
