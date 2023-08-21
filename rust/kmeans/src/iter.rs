use mnist::Dataset;
use ndarray::{Array1, Array2};
use ndarray_stats::DeviationExt;

use crate::{
    algorithm::{calculate_centroids_info, calculate_centroids_label, update},
    KMeans,
};

pub struct KMeansIter<'a> {
    dataset: &'a Dataset,

    // Shape: (n_clusters, n_features = data_size = 28 * 28)
    centroids: Array2<f64>,

    // intermediate values
    prev_centroids: Array2<f64>,
    memberships: Array1<usize>,

    max_iter: usize,
    n_iter: usize,
    min_dist: f64,
    dist: f64,
    stop: bool,
}
impl<'a> KMeansIter<'a> {
    pub fn new(
        centroids: Array2<f64>,
        dataset: &'a Dataset,
        max_iter: usize,
        min_dist: f64,
    ) -> Self {
        let prev_centroids = centroids.clone();
        let memberships = Array1::zeros(dataset.num);

        Self {
            dataset,
            centroids,

            prev_centroids,
            memberships,

            n_iter: 0,
            dist: f64::INFINITY,
            max_iter,
            min_dist,
            stop: false,
        }
    }

    pub fn into_model(self) -> KMeans {
        let mut centroids_info = Array2::zeros((self.centroids.nrows(), 10));
        let mut centroids_label = Array1::from_elem(self.centroids.nrows(), None);

        calculate_centroids_info(&mut centroids_info, &self.memberships, &self.dataset.labels);
        calculate_centroids_label(&mut centroids_label, &centroids_info);

        KMeans {
            centroids: self.centroids,
            centroids_info,
            centroids_label,
        }
    }
}

impl<'a> Iterator for &mut KMeansIter<'a> {
    type Item = (usize, f64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.stop {
            return None;
        }

        // Update centroids and memberships
        update(
            &mut self.centroids,
            &mut self.memberships,
            &self.dataset.images,
        );

        // Check stop condition
        self.dist = self.prev_centroids.l2_dist(&self.centroids).unwrap();
        self.n_iter += 1;
        self.prev_centroids.assign(&self.centroids);

        if self.dist < self.min_dist || self.n_iter >= self.max_iter {
            self.stop = true;
        }

        Some((self.n_iter, self.dist))
    }
}
