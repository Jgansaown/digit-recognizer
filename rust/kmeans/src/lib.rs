use mnist::Dataset;
use ndarray::{Array1, Array2, ArrayView1, Ix2, ShapeBuilder, Zip};
use ndarray_rand::{
    rand::{rngs::SmallRng, SeedableRng},
    rand_distr::Uniform,
    RandomExt,
};
use ndarray_stats::DeviationExt;

pub struct KMeans {
    pub param: KMeansHyperParameter,
    pub centroids: Array2<f64>,  // Shape: (n_centroids, 28 * 28)
    pub clusters: Array1<usize>, // which centroid each observation belong to

    prev_centroids: Array2<f64>,
    iter: usize,
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
        let mut model = KMeans {
            centroids: KMeans::init_centroids(&param.init, (param.n_clusters, dataset.size)),
            prev_centroids: Array2::zeros((param.n_clusters, dataset.size)),
            clusters: Array1::zeros(dataset.num),
            iter: 0,
            param,
        };

        loop {
            let info = model.step(dataset);

            println!("{}: {}", info.iter, info.dist);

            if info.dist < info.model.param.tolerance || info.iter >= info.model.param.max_iter {
                break;
            }
        }

        model
    }

    fn step<'a>(&'a mut self, dataset: &'a Dataset) -> KMeansStepInfo<'a> {
        self.assign_to_cluster(dataset);
        self.update_centroids(dataset);

        let dist = self.centroids.l2_dist(&self.prev_centroids).unwrap();
        self.iter += 1;
        self.prev_centroids.assign(&self.centroids);

        KMeansStepInfo {
            iter: self.iter,
            dist,
            model: self,
            dataset,
        }
    }

    // Initialize centroids based on hyperparameter
    fn init_centroids<Sh>(init: &KMeansInit, shape: Sh) -> Array2<f64>
    where
        Sh: ShapeBuilder<Dim = Ix2>,
    {
        let mut rng = SmallRng::seed_from_u64(1);
        match init {
            // KMeansInit::Random => Array2::random(shape, Uniform::new(0.0, 255.0)),
            KMeansInit::Random => Array2::random_using(shape, Uniform::new(0.0, 255.0), &mut rng),
        }
    }

    // Assign each observation to nearest cluster
    fn assign_to_cluster(&mut self, dataset: &Dataset) {
        for (image_i, image) in dataset.images.outer_iter().enumerate() {
            // find closest centroid based on the l2 distance
            let (nearest_centroid, _) = self
                .centroids
                .outer_iter()
                .enumerate()
                .map(|(i, centroid)| (i, centroid.l2_dist(&image).unwrap()))
                .min_by(|(_, a), (_, b)| a.total_cmp(b))
                .unwrap();

            self.clusters[image_i] = nearest_centroid;
        }
    }

    /// Recompute the centroids for each cluster
    ///
    /// using m_k-means instead of the standard algorithm (Lloyd's) to avoid
    /// problem with empty clusters
    ///
    /// https://docs.rs/linfa-clustering/latest/linfa_clustering/struct.KMeans.html
    /// https://www.researchgate.net/publication/228414762_A_Modified_k-means_Algorithm_to_Avoid_Empty_Clusters
    fn update_centroids(&mut self, dataset: &Dataset) {
        let mut counts: Array1<usize> = Array1::ones(self.centroids.nrows());

        Zip::from(dataset.images.outer_iter())
            .and(&self.clusters)
            .for_each(|image, &centroid_i| {
                let mut centroid = self.centroids.row_mut(centroid_i);
                centroid += &image;
                counts[centroid_i] += 1;
            });

        Zip::from(self.centroids.rows_mut())
            .and(&counts)
            .for_each(|mut centroid, &count| {
                centroid /= count as f64;
            });
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
    // TODO: KMeansPlusPlus,
}

pub struct KMeansStepInfo<'a> {
    iter: usize,
    dist: f64,
    model: &'a KMeans,
    dataset: &'a Dataset,
}
