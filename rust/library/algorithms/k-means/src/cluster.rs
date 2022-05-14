use getrandom::getrandom;
use mnist::{DATA_SIZE, DATA_TYPES};

pub struct Cluster {
    /// number of data points in the cluster
    pub num: Option<usize>,
    /// label assigned to the cluster
    pub label: Option<u8>,
    /// centroid of the cluster
    pub centroid: Vec<u8>,
    /// intermediate values cached
    cache: ClusterCache,
}
impl Cluster {
    pub fn new() -> Self {
        Self {
            num: None,
            label: None,
            centroid: vec![0; DATA_SIZE],
            cache: ClusterCache::new(),
        }
    }

    pub fn random() -> Self {
        let mut centroid = vec![0; DATA_SIZE];
        getrandom(&mut centroid).expect("error getting random numbers");
        Self {
            num: None,
            label: None,
            centroid,
            cache: ClusterCache::new(),
        }
    }

    pub fn euclidean_distance(&self, other: &[u8]) -> f32 {
        // https://en.wikipedia.org/wiki/Euclidean_distance
        self.centroid
            .iter()
            .zip(other)
            .map(|(&a, &b)| (a as f32, b as f32))
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f32>()
            .sqrt()
    }

    pub fn recalculate_centroid(&mut self) -> Option<f32> {
        let n = self.cache.indexes.len();
        if n == 0 {
            return None;
        } else {
            let new: Vec<u8> = self
                .cache
                .centroid_sum
                .iter()
                .map(|&c| (c / n as f32) as u8)
                .collect();

            let dist = self.euclidean_distance(&new);
            self.num = Some(n);
            self.label = self.cache.get_sorted_labels().iter().copied().nth(0);
            self.centroid = new;

            Some(dist)
        }
    }

    pub fn add_data(&mut self, index: usize, data: &[u8], label: u8) {
        self.cache.add_data(index, data, label);
    }

    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    pub fn get_assigned_indexes(&self) -> &[usize] {
        &self.cache.indexes
    }
}

#[derive(Default)]
struct ClusterCache {
    /// Sum of centroids
    centroid_sum: Vec<f32>,
    /// List of the indexes of each data point in the cluster
    indexes: Vec<usize>,
    /// Number of occurrence of data point's label in the cluster
    label_counts: [usize; DATA_TYPES],
}
impl ClusterCache {
    fn new() -> Self {
        Self {
            centroid_sum: vec![0.0; DATA_SIZE],
            indexes: Vec::new(),
            label_counts: [0; 10],
        }
    }

    fn clear(&mut self) {
        self.centroid_sum.iter_mut().for_each(|v| *v = 0.0);
        self.indexes.clear();
        self.label_counts.iter_mut().for_each(|v| *v = 0);
    }

    fn add_data(&mut self, index: usize, data: &[u8], label: u8) {
        self.centroid_sum
            .iter_mut()
            .zip(data)
            .for_each(|(v, &d)| *v += d as f32);
        self.indexes.push(index);
        self.label_counts[label as usize] += 1;
    }

    fn get_sorted_labels(&self) -> Vec<u8> {
        let mut temp: Vec<(usize, usize)> = self.label_counts.iter().copied().enumerate().collect();
        temp.sort_by(|(_, a), (_, b)| a.cmp(b));
        temp.into_iter().rev().map(|(i, _)| i as u8).collect()
    }
}
