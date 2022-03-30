mod cluster;

use crate::cluster::Cluster;
use mnist::{Data, Dataset};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[cfg(feature = "multithread")]
use rayon::prelude::*;

#[wasm_bindgen]
pub struct KMeansClusters {
    inner: Vec<Cluster>,
}
#[wasm_bindgen]
impl KMeansClusters {
    /// Creates `K` clusters with random initial centroids
    pub fn random(k: usize) -> Self {
        Self {
            inner: (0..k).into_iter().map(|_| Cluster::random()).collect(),
        }
    }
    ///
    pub fn assign_dataset(&mut self, dataset: &Dataset) {
        self.clear_cache();
        for (i, data) in dataset.iter().enumerate() {
            let id = self.find_closest_centroid(&data);
            self.inner[id].add_data(i, data.value, data.label);
        }
    }
    ///
    pub fn recalculate_centroids(&mut self) -> f32 {
        let sums = self
            .inner
            .iter_mut()
            .filter_map(|cluster| cluster.recalculate_centroid())
            .sum::<f32>();
        sums / (self.inner.len() as f32)
    }
    ///
    pub fn get_info(&self) -> Vec<JsValue> {
        self.inner
            .iter()
            .map(|c| JsValue::from_serde(&ClusterInfo::from_cluster(c)).unwrap())
            .collect()
    }
}
#[cfg(feature = "multithread")]
impl KMeansClusters {
    ///
    pub fn par_assign_dataset(&mut self, dataset: &Dataset) {
        self.clear_cache();
        let temp: Vec<Data<u8>> = dataset.iter().collect();
        let temp: Vec<(usize, usize, Data<u8>)> = temp
            .into_par_iter()
            .enumerate()
            .map(|(i, data)| (self.find_closest_centroid(&data), i, data))
            .collect();
        for (id, i, data) in temp {
            self.inner[id].add_data(i, data.value, data.label);
        }
    }
}
impl KMeansClusters {
    ///
    pub fn get_clusters(&self) -> &[Cluster] {
        &self.inner
    }
    ///
    pub fn test(&self, dataset: &Dataset) -> f32 {
        let correct: usize = dataset.iter().fold(0, |acc, data| {
            let i = self.find_closest_centroid(&data);
            let label = self.inner[i].label.unwrap();
            if label == data.label {
                acc + 1
            } else {
                acc
            }
        });
        let err_rate = 1.0 - (correct as f32) / (dataset.num as f32);
        println!(
            "Total: {}, Correct: {}, Error Rate: {}",
            dataset.num, correct, err_rate
        );
        err_rate
    }
    ///
    fn clear_cache(&mut self) {
        self.inner.iter_mut().for_each(|c| c.clear_cache());
    }
    ///
    fn find_closest_centroid(&self, data: &Data<u8>) -> usize {
        self.inner
            .iter()
            .map(|c| c.euclidean_distance(data.value))
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).expect("error comparing floats"))
            .map(|(i, _)| i)
            .expect("clusters empty, cannot find closest cluster")
    }
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
