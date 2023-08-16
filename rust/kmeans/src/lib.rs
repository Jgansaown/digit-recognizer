// use std::ops::AddAssign;

use mnist::Dataset;
use ndarray::{Array1, Array2};
use ndarray_rand::{rand_distr::Uniform, RandomExt};
use ndarray_stats::DeviationExt;

pub struct KMeans {
    param: KMeansHyperParameter,
    centroids: Array2<f64>, // Shape: (n_centroids, 28 * 28)
}
impl KMeans {
    pub fn with_param(
        n_clusters: usize,
        max_iter: usize,
        tolerance: f64,
        init: KMeansInit,
    ) -> KMeansHyperParameter {
        KMeansHyperParameter {
            n_clusters,
            max_iter,
            tolerance,
            init,
        }
    }

    pub fn with_default_param() -> KMeansHyperParameter {
        KMeans::with_param(10, 500, 0.01, KMeansInit::Random)
    }

    /// Evaluate the model
    pub fn evaluate(&self, dataset: &Dataset) {
        todo!("todo")
    }

    fn evaluate_1() {
        todo!("todo")
    }
}
// Private Interface
impl KMeans {
    fn train(param: KMeansHyperParameter, dataset: &Dataset) -> Self {
        let mut centroids: Array2<f64> = Array2::random(
            (param.n_clusters, dataset.images.ncols()),
            Uniform::new(0.0, 255.0),
        );
        // TODO: match param.init {}

        // println!("{:?}", centroids);

        for i in 0usize.. {
            let mut counts: Array1<usize> = Array1::zeros(param.n_clusters);
            let mut new_centroids: Array2<f64> = Array2::zeros(centroids.dim());

            for image in dataset.images.rows() {
                // find closest centroid for each image
                let mut i = 0;
                let mut closest = f64::MAX;
                let mut closest_i = 0;
                for centroid in centroids.rows() {
                    let dist = image.l2_dist(&centroid).unwrap();
                    if dist < closest {
                        closest = dist;
                        closest_i = i;
                    }
                    i += 1;
                }
                
                let mut new_centroid = new_centroids.row_mut(closest_i);
                new_centroid += &image;
                counts[closest_i] += 1;
            }

            for (i, &count) in counts.indexed_iter() {
                if count == 0 {
                    continue;
                }
                new_centroids.row_mut(i).mapv_inplace(|v| v / count as f64);
            }

            let dist = centroids.l2_dist(&new_centroids).unwrap();

            centroids = new_centroids;

            println!("{}: {}", i, dist);
            if dist < param.tolerance {
                break;
            }
            if i >= param.max_iter {
                break;
            }
        }

        KMeans { param, centroids }
    }
}

pub struct KMeansHyperParameter {
    n_clusters: usize,
    max_iter: usize,
    tolerance: f64,
    init: KMeansInit,
}
impl KMeansHyperParameter {
    /// Train the K Means model with `dataset`
    pub fn train(self, dataset: &Dataset) -> KMeans {
        KMeans::train(self, dataset)
    }

    pub fn n_clusters(mut self, n_clusters: usize) -> Self {
        self.n_clusters = n_clusters;
        self
    }
    pub fn max_iter(mut self, max_iter: usize) -> Self {
        self.max_iter = max_iter;
        self
    }
    pub fn tolerance(mut self, tolerance: f64) -> Self {
        self.tolerance = tolerance;
        self
    }
    pub fn init(mut self, init: KMeansInit) -> Self {
        self.init = init;
        self
    }
}

pub enum KMeansInit {
    Random,
    KMeansPlusPlus,
}
