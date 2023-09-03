use crate::dataset::Dataset;

use crate::models::Model;
use ndarray::{Array1, Array2, ArrayBase, Axis, Data, Ix1};
use ndarray_stats::DeviationExt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct KNearestNeighbors {
    k: usize,
    stored: Dataset,
}

#[wasm_bindgen]
impl KNearestNeighbors {
    #[wasm_bindgen(constructor)]
    pub fn new(k: usize) -> Self {
        Self {
            k,
            stored: Dataset::new(
                Array2::zeros((0, 0)),
                Array2::zeros((0, 0)),
                Array1::zeros(0),
            ),
        }
    }

    pub fn step(&mut self, dataset: &Dataset) -> f64 {
        self.stored = dataset.clone();
        1.0
    }

    pub fn evaluate(&self, dataset: &Dataset) -> f64 {
        let n_error: usize =
            dataset
                .observation_label_iter()
                .fold(0, |n_error, observation, &target| {
                    match self.calculate_prediction(&observation) {
                        (_, Some(predict)) if predict == target => n_error,
                        _ => n_error + 1,
                    }
                });
        n_error as f64 / dataset.n_observations() as f64
    }

    pub fn predict(&self) {
        todo!()
    }
}

impl KNearestNeighbors {
    /// predict by adding the target of k closest observations together
    pub fn calculate_prediction(
        &self,
        observation: &ArrayBase<impl Data<Elem = f64>, Ix1>,
    ) -> (Array1<usize>, Option<usize>) {
        let mut distances: Vec<(usize, f64)> = self
            .stored
            .observations()
            .axis_iter(Axis(0))
            .map(|o| o.l2_dist(observation).unwrap())
            .enumerate()
            .collect();

        distances.sort_unstable_by(|(_, d1), (_, d2)| d1.total_cmp(d2));

        let mut targets = Array1::zeros(self.stored.target_size());
        let mut predict = None;
        for &(i, _) in &distances[..self.k] {
            // targets += &self.stored.targets().row(i);
            let label = self.stored.labels()[i];
            targets[label] += 1;

            match predict {
                Some(p) if targets[p] >= targets[label] => {}
                _ => predict = Some(label),
            }
        }

        (targets, predict)
    }
}

impl Model for KNearestNeighbors {
    fn step(&mut self, dataset: &Dataset) -> f64 {
        self.step(dataset)
    }

    fn evaluate(&self, dataset: &Dataset) -> f64 {
        self.evaluate(dataset)
    }
}
