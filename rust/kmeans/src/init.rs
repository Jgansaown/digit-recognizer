use ndarray::{Array2, Ix2, ShapeBuilder};
use ndarray_rand::{rand_distr::Uniform, RandomExt};

pub enum KMeansInit {
    Random,
    // TODO: KMeansPlusPlus,
}

impl KMeansInit {
    pub fn create_centroid<Sh: ShapeBuilder<Dim = Ix2>>(&self, shape: Sh) -> Array2<f64> {
        match self {
            KMeansInit::Random => Array2::random(shape, Uniform::new(0.0, 255.0)),
        }
    }
}
