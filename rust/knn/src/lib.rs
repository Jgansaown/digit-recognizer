use mnist::Dataset;
use ndarray::{ArrayBase, Data, Ix1, Zip};
use ndarray_stats::DeviationExt;

pub struct KNearestNeighborsParam {
    k: usize,
}
impl KNearestNeighborsParam {
    pub fn k(mut self, k: usize) -> Self {
        self.k = k;
        self
    }

    /// "Trains" the K nearest neighbors algorithm
    ///
    /// In reality it just copies the dataset and saves it to memory
    pub fn train(&self, dataset: &Dataset) -> KNearestNeighbors {
        KNearestNeighbors {
            k: self.k,
            dataset: dataset.clone(),
        }
    }
}

pub struct KNearestNeighbors {
    k: usize,
    dataset: Dataset,
}
impl KNearestNeighbors {
    /// Default parameters
    /// ```ignore
    /// KNearestNeighborsParam {
    ///     k: 10,
    /// }
    /// ```
    pub fn with_default_param() -> KNearestNeighborsParam {
        KNearestNeighborsParam { k: 10 }
    }

    #[cfg(feature = "rayon")]
    /// Evaluate the model and returns the number of correct predictions
    pub fn evaluate(&self, dataset: &Dataset) -> usize {
        Zip::from(dataset.images.outer_iter())
            .and(&dataset.labels)
            .par_map_collect(|img, &label| match self.predict(&img) {
                Some(predict) if predict == label as usize => 1,
                _ => 0,
            })
            .sum()
    }

    /// Evaluate the model and returns the number of correct predictions
    pub fn evaluate(&self, dataset: &Dataset) -> usize {
        Zip::from(dataset.images.outer_iter())
            .and(&dataset.labels)
            .map_collect(|img, &label| match self.predict(&img) {
                Some(predict) if predict == label as usize => 1,
                _ => 0,
            })
            .sum()
    }

    /// Predicts the label of the input observation
    pub fn predict(&self, observation: &ArrayBase<impl Data<Elem = f64>, Ix1>) -> Option<usize> {
        let mut distances = Zip::from(self.dataset.images.outer_iter())
            .and(&self.dataset.labels)
            .map_collect(|img, label| (img.l2_dist(observation).unwrap(), label.clone()))
            .into_raw_vec();

        distances.sort_unstable_by(|(d1, _), (d2, _)| d1.total_cmp(d2));

        let mut counts: [usize; 10] = [0; 10];
        let mut label_majority = None;
        for label in distances[..self.k].iter().map(|v| v.1) {
            counts[label as usize] += 1;

            match label_majority {
                Some(m) if counts[m] >= counts[label as usize] => {}
                _ => label_majority = Some(label as usize),
            }
        }

        label_majority
    }
}
