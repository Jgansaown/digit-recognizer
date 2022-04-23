use mnist::{Data, Dataset};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct KNearestNeighbors {
    k: usize,
    dataset: Option<Dataset>,
}
#[wasm_bindgen]
impl KNearestNeighbors {
    pub fn new(k: usize) -> Self {
        Self { k, dataset: None }
    }

    /// "Trains" the K nearest neighbors algorithm
    ///
    /// In reality it just copies the dataset and saves it to memory
    pub fn train(&mut self, dataset: &Dataset) {
        self.dataset = Some(dataset.clone());
    }

    /// Find the label of the K closest neighbors
    /// 
    /// Time Complexity: O(NM) where N is the number of training data (60,000) and M is the number of pixels (28*28)
    pub fn find(&self, data: &[u8]) -> Option<Vec<u8>> {
        match &self.dataset {
            Some(ds) => {
                let mut dist: Vec<(f32, u8)> = ds
                    .iter()
                    .map(|d| (d.euclidean_distance(&data), d.label))
                    .collect();
                dist.sort_by(|(d1, _), (d2, _)| d1.partial_cmp(d2).unwrap());

                Some(
                    dist
                        .get(..self.k)
                        .unwrap()
                        .iter()
                        .map(|(_, i)| *i)
                        .collect(),
                )
            }
            None => None,
        }
    }
}
#[cfg(feature = "multithread")]
use rayon::prelude::*;

#[cfg(feature = "multithread")]
impl KNearestNeighbors {
    ///
    pub fn par_find(&self, data: &[u8]) -> Option<Vec<u8>> {
        match &self.dataset {
            Some(ds) => {
                let ds: Vec<Data<u8>> = ds.iter().collect();
                let mut dist: Vec<(f32, u8)> = ds
                    .into_par_iter()
                    .map(|d| (d.euclidean_distance(&data), d.label))
                    .collect();
                dist.par_sort_by(|(d1, _), (d2, _)| d1.partial_cmp(d2).unwrap());

                Some(
                    dist.get(..self.k)
                        .unwrap()
                        .iter()
                        .map(|(_, i)| *i)
                        .collect(),
                )
            }
            None => None,
        }
    }
}
