mod mnist;

pub use mnist::MNIST;

use ndarray::{iter::AxisIter, Array1, Array2, ArrayView1, Axis, Ix1, Zip};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Dataset {
    observations: Array2<f64>,
    targets: Array2<f64>,
    labels: Array1<usize>,
}

impl Dataset {
    /// observations and targets should have same number of  (n_observations)
    pub fn new(observations: Array2<f64>, targets: Array2<f64>, labels: Array1<usize>) -> Self {
        assert_eq!(observations.dim().0, targets.dim().0);
        assert_eq!(observations.dim().0, labels.dim());

        Self {
            observations,
            targets,
            labels,
        }
    }

    /// Shape: (n_observations, data_size)
    pub fn observations(&self) -> &Array2<f64> {
        &self.observations
    }

    /// Shape: (n_observations, target_size)
    pub fn targets(&self) -> &Array2<f64> {
        &self.targets
    }

    pub fn labels(&self) -> &Array1<usize> {
        &self.labels
    }

    pub fn n_observations(&self) -> usize {
        self.observations.dim().0
    }

    pub fn data_size(&self) -> usize {
        self.observations.dim().1
    }

    pub fn target_size(&self) -> usize {
        self.targets.dim().1
    }

    pub fn at(&self, i: usize) -> (ArrayView1<f64>, ArrayView1<f64>) {
        (self.observations.row(i), self.targets.row(i))
    }

    pub fn observation_target_iter(&self) -> ObservationTargetIter {
        Zip::from(self.observations.axis_iter(Axis(0))).and(self.targets.axis_iter(Axis(0)))
    }

    pub fn observation_label_iter(&self) -> ObservationLabelIter {
        Zip::from(self.observations().axis_iter(Axis(0))).and(self.labels())
    }
}

type ObservationTargetIter<'a> = Zip<(AxisIter<'a, f64, Ix1>, AxisIter<'a, f64, Ix1>), Ix1>;
type ObservationLabelIter<'a> = Zip<(AxisIter<'a, f64, Ix1>, ArrayView1<'a, usize>), Ix1>;
