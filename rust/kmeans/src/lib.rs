mod algorithm;
mod init;
mod iter;

use algorithm::{
    calculate_centroids_info, calculate_centroids_label, find_nearest_centroid, update,
    update_centroids, update_membership,
};
use init::KMeansInit;
use iter::KMeansIter;
use mnist::Dataset;
use ndarray::{Array1, Array2, ArrayBase, Data, Ix1, Zip};
use ndarray_stats::DeviationExt;

/// Hyperparameters for KMeans
pub struct KMeansParam {
    init: KMeansInit,
    n_clusters: usize,
    max_iter: usize,
    min_dist: f64,
}

impl KMeansParam {
    pub fn train(&self, dataset: &Dataset) -> (usize, f64, KMeans) {
        let observations = &dataset.images;

        let mut model = KMeans {
            centroids: self.init.create_centroid((self.n_clusters, dataset.size)),
            centroids_info: Array2::zeros((self.n_clusters, 10)),
            centroids_label: Array1::from_elem(self.n_clusters, None),
        };

        let mut prev_centroids = model.centroids.clone();
        let mut memberships = Array1::zeros(dataset.num);

        let mut n_iter = 0;
        let mut dist;
        loop {
            // Update centroids and memberships
            update(&mut model.centroids, &mut memberships, &observations);

            // Check stop condition
            dist = prev_centroids.l2_dist(&model.centroids).unwrap();
            n_iter += 1;

            if dist < self.min_dist || n_iter >= self.max_iter {
                break;
            }

            prev_centroids.assign(&model.centroids);
        }

        // Calculate centroid info
        calculate_centroids_info(&mut model.centroids_info, &memberships, &dataset.labels);
        calculate_centroids_label(&mut model.centroids_label, &model.centroids_info);

        (n_iter, dist, model)
    }

    pub fn train_iter<'a>(&self, dataset: &'a Dataset) -> KMeansIter<'a> {
        KMeansIter::new(
            self.init.create_centroid((self.n_clusters, dataset.size)),
            dataset,
            self.max_iter,
            self.min_dist,
        )
    }

    pub fn init(mut self, init: KMeansInit) -> Self {
        self.init = init;
        self
    }
    pub fn n_clusters(mut self, n_clusters: usize) -> Self {
        self.n_clusters = n_clusters;
        self
    }
    pub fn max_iter(mut self, max_iter: usize) -> Self {
        self.max_iter = max_iter;
        self
    }
    pub fn min_dist(mut self, min_dist: f64) -> Self {
        self.min_dist = min_dist;
        self
    }
}

pub struct KMeans {
    // Shape: (n_clusters, n_features = data_size = 28 * 28)
    centroids: Array2<f64>,
    // Shape: (n_clusters, 10)
    centroids_info: Array2<usize>,
    // Shape: (n_clusters), Value: (label, num_in_cluster)
    centroids_label: Array1<Option<usize>>,
}

impl KMeans {
    /// Default parameters
    /// ```ignore
    /// KMeansParam {
    ///     init: KMeansInit::Random,
    ///     n_clusters: 10,
    ///     data_size: mnist::DATA_SIZE,
    ///     max_iter: 100,
    ///     min_dist: 10.0,
    /// }
    /// ```
    pub fn with_default_param() -> KMeansParam {
        KMeansParam {
            init: KMeansInit::Random,
            n_clusters: 10,
            max_iter: 100,
            min_dist: 10.0,
        }
    }

    pub fn new(n_clusters: usize, data_size: usize) -> Self {
        KMeans {
            centroids: KMeansInit::Random.create_centroid((n_clusters, data_size)),
            centroids_info: Array2::zeros((n_clusters, 10)),
            centroids_label: Array1::from_elem(n_clusters, None),
        }
    }

    pub fn step(&mut self, dataset: &Dataset) -> f64 {
        // 1. Assignment Step: Assign observations to cluster with nearest centroid
        let mut memberships = Array1::zeros(dataset.num);
        update_membership(&mut memberships, &self.centroids, &dataset.images);

        // 2. Update Step: Recalculate centroid for each cluster
        update_centroids(&mut self.centroids, &dataset.images, &memberships);

        // Calculate centroid info
        calculate_centroids_info(&mut self.centroids_info, &memberships, &dataset.labels);
        calculate_centroids_label(&mut self.centroids_label, &self.centroids_info);

        let err = Zip::from(&dataset.labels)
            .and(&memberships)
            .fold(0.0, |acc, &target, &predict| {
                match self.centroids_label[predict] {
                    Some(predict) if predict != target as usize => acc + 1.0,
                    _ => acc,
                }
            });
        
        err / dataset.num as f64
    }

    /// Evaluate the model and returns the number of correct predictions
    pub fn evaluate(&self, dataset: &Dataset) -> usize {
        let mut memberships = Array1::zeros(dataset.num);
        update_membership(&mut memberships, &self.centroids, &dataset.images);

        Zip::from(&dataset.labels)
            .and(&memberships)
            .fold(0, |acc, &target, &predict| {
                match self.centroids_label[predict] {
                    Some(predict) if predict == target as usize => acc + 1,
                    _ => acc,
                }
            })
    }

    /// Predicts the label of the input observation
    pub fn predict(&self, observation: &ArrayBase<impl Data<Elem = f64>, Ix1>) -> Option<usize> {
        let (membership, _) = find_nearest_centroid(&self.centroids, observation);
        self.centroids_label[membership]
    }
}

#[cfg(test)]
mod tests {
    use ndarray::Array1;
    use ndarray_rand::{rand_distr::Uniform, RandomExt};

    use super::*;

    #[test]
    fn test_hyper_parameter() {
        let param = KMeans::with_default_param()
            .init(KMeansInit::Random)
            .n_clusters(10)
            .max_iter(100)
            .min_dist(0.01);

        assert!(matches!(param.init, KMeansInit::Random));
        assert_eq!(param.n_clusters, 10);
        assert_eq!(param.max_iter, 100);
        assert_eq!(param.min_dist, 0.01);
    }

    #[test]
    fn test_train() {
        let dataset = Dataset {
            num: 10,
            size: 10,
            images: Array2::random((10, 10), Uniform::new(0.0, 255.0)),
            labels: Array1::random(10, Uniform::new(0, 10)),
        };
        let (_, _, model) = KMeans::with_default_param()
            .init(KMeansInit::Random)
            .n_clusters(10)
            .max_iter(100)
            .min_dist(0.01)
            .train(&dataset);

        let _correct = model.evaluate(&dataset);
    }

    #[test]
    fn test_train_iter() {
        let dataset = Dataset {
            num: 10,
            size: 10,
            images: Array2::random((10, 10), Uniform::new(0.0, 255.0)),
            labels: Array1::random(10, Uniform::new(0, 10)),
        };
        let mut iter = KMeans::with_default_param()
            .init(KMeansInit::Random)
            .n_clusters(10)
            .max_iter(100)
            .min_dist(0.01)
            .train_iter(&dataset);

        for _step in &mut iter {}

        let model = iter.into_model();
        let _correct = model.evaluate(&dataset);
    }
}
