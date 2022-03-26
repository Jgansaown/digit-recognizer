mod cluster;
pub mod kmeans;

use mnist::Dataset;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// 
#[wasm_bindgen]
pub fn kmeans_new_clusters_random(k: usize) -> kmeans::KMeansClusters {
    kmeans::KMeansClusters::random(k)
}

/// Iterates through dataset and assign data to closest cluster
///
#[wasm_bindgen]
pub fn kmeans_assign_dataset_to_clusters(dataset: &Dataset, clusters: &mut kmeans::KMeansClusters) {
    clusters.clear_cached();
    for (i, data) in dataset.iter().enumerate() {
        clusters.assign_to_cluster(i, data.value, data.label);
    }
}

/// Recalculates centroid position
///
#[wasm_bindgen]
pub fn kmeans_recalculate_centroids(clusters: &mut kmeans::KMeansClusters) -> f32 {
    clusters.recalculate_centroids()
}

/// 
#[wasm_bindgen]
pub fn kmeans_get_clusters_info(clusters: &kmeans::KMeansClusters) -> Vec<JsValue> {
    clusters
        .get_clusters()
        .iter()
        .map(|c| JsValue::from_serde(&ClusterInfo::from_cluster(c)).unwrap())
        .collect()
}

#[derive(Serialize, Deserialize)]
pub struct ClusterInfo {
    img: String,
    label: Option<u8>,
    num_of_data: Option<usize>,
}
impl ClusterInfo {
    pub fn from_cluster(cluster: &cluster::Cluster) -> ClusterInfo {
        ClusterInfo {
            img: mnist::data_as_png_base64_string(&cluster.centroid),
            label: cluster.label,
            num_of_data: cluster.num,
        }
    }
}
