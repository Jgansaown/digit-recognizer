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

    pub fn test_rgba_image(&self, rgba: Vec<u8>) -> u8 {
        let data = mnist::rgba_image_to_grayscale_image(&rgba);
        let labels = self.test_data(&data);
        match labels.first() {
            Some(&(i, _)) => i,
            None => 255,
        }
    }

    pub fn test_dataset_js(&self, dataset: &Dataset) -> JsValue {
        let result = self.test_dataset(dataset);
        JsValue::from_serde(&result).unwrap()
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
    pub fn test_data(&self, data: &[u8]) -> Vec<(u8, f32)> {
        let mut labeled: Vec<(u8, f32)> = self
            .inner
            .iter()
            .filter_map(|c| match c.label {
                Some(label) => Some((label, c.euclidean_distance(data))),
                None => None,
            })
            .collect();
        labeled.sort_by(|(_, a), (_, b)| a.partial_cmp(b).expect("error comparing floats"));
        labeled
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
    
    pub fn test_dataset(&self, dataset: &Dataset) -> TestDatasetResult {
        let matching: Vec<(usize, usize)> = dataset
            .iter()
            .map(|data| (self.find_closest_centroid(&data), data.label as usize))
            .collect();

        // let matches: Vec<[usize; 10]> = Vec::new();
        let mut matches: Vec<[usize; 10]> = self.get_clusters().iter().map(|_| [0; 10]).collect();
        for (id, label) in matching {
            matches[id][label] += 1;
        }

        for (i, m) in matches.iter().enumerate() {
            println!("label: {:?}, total: {}, {:?}", self.get_clusters()[i].label, m.iter().sum::<usize>(), m);
        }

        TestDatasetResult {
            total: dataset.num,
            correct: 0,
            incorrect: 0,
            error_rate: 0.0,
            matches: Vec::new(),
        }
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

#[derive(Serialize, Deserialize, Default)]
pub struct TestDatasetResult {
    pub total: usize,
    pub correct: usize,
    pub incorrect: usize,
    pub error_rate: f32,

    /// Number of matches for each digit in each cluster
    ///
    /// Tuple contains the label of the cluster,
    /// and then number of matching digits in an array
    pub matches: Vec<[usize; 10]>,
}
