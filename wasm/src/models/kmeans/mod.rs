mod algorithm;
mod init;

use crate::dataset::Dataset;
use crate::models::Model;
use algorithm::{
    calculate_centroids_info, calculate_centroids_label, calculate_error_rate, update_centroids,
    update_membership,
};
use init::KMeansInit;
use ndarray::{Array1, Array2};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct KMeans {
    // Shape: (n_clusters, n_features = data_size = 28 * 28)
    centroids: Array2<f64>,
    // Shape: (n_clusters, 10)
    centroids_info: Array2<usize>,
    // Shape: (n_clusters), Value: (label, num_in_cluster)
    centroids_label: Array1<Option<usize>>,
}

#[wasm_bindgen]
impl KMeans {
    #[wasm_bindgen(constructor)]
    pub fn new(n_clusters: usize, n_features: usize) -> Self {
        Self {
            centroids: KMeansInit::Random.create_centroid((n_clusters, n_features)),
            centroids_info: Array2::zeros((n_clusters, 10)),
            centroids_label: Array1::from_elem(n_clusters, None),
        }
    }

    pub fn step(&mut self, dataset: &Dataset) -> f64 {
        // 1. Assignment Step: Assign observations to cluster with nearest centroid
        let mut memberships = Array1::zeros(dataset.n_observations());
        update_membership(&mut memberships, &self.centroids, dataset.observations());

        // 2. Update Step: Recalculate centroid for each cluster
        update_centroids(&mut self.centroids, dataset.observations(), &memberships);

        // 3. Calculate centroid info
        calculate_centroids_info(&mut self.centroids_info, &memberships, dataset.labels());
        calculate_centroids_label(&mut self.centroids_label, &self.centroids_info);

        // 4. Return error rate
        calculate_error_rate(&self.centroids_label, &memberships, dataset.labels())
    }

    pub fn evaluate(&self, dataset: &Dataset) -> f64 {
        // 1. Assignment Step: Assign observations to cluster with nearest centroid
        let mut memberships = Array1::zeros(dataset.n_observations());
        update_membership(&mut memberships, &self.centroids, dataset.observations());

        // 2. Return error rate
        calculate_error_rate(&self.centroids_label, &memberships, dataset.labels())
    }

    pub fn predict(&self) {
        todo!()
    }
}

impl Model for KMeans {
    fn step(&mut self, dataset: &Dataset) -> f64 {
        self.step(dataset)
    }

    fn evaluate(&self, dataset: &Dataset) -> f64 {
        self.evaluate(dataset)
    }
}
