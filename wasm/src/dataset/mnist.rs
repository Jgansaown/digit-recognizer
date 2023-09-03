use super::Dataset;
use flate2::read::GzDecoder;
use ndarray::{Array, Array1, Array2, Axis, Dimension, StrideShape};
use std::{
    error::Error,
    fmt::{Display, Formatter},
    io::Read,
};
use wasm_bindgen::prelude::*;

const DATA_SIZE: usize = 28 * 28;
const DATA_TYPES: usize = 10;
const TRAIN_NUM: usize = 60_000;
const TEST_NUM: usize = 10_000;

// const DATA_MAGIC_NUMBER: u32 = 2051;
// const LABEL_MAGIC_NUMBER: u32 = 2049;

#[wasm_bindgen]
pub struct MNIST;

#[wasm_bindgen]
impl MNIST {
    /// Get MNIST training dataset (n_observation=60,000) from static binary bundled with the WASM
    pub fn training_from_static() -> Dataset {
        let training_images = include_bytes!("./data/train-images-idx3-ubyte.gz").as_slice();
        let training_labels = include_bytes!("./data/train-labels-idx1-ubyte.gz").as_slice();
        dataset_from_gz_idx_ubyte(training_images, training_labels, TRAIN_NUM).unwrap()
    }

    /// Get MNIST testing dataset (n_observation=10,000) from static binary bundled with the WASM
    pub fn testing_from_static() -> Dataset {
        let testing_images = include_bytes!("./data/t10k-images-idx3-ubyte.gz").as_slice();
        let testing_labels = include_bytes!("./data/t10k-labels-idx1-ubyte.gz").as_slice();
        dataset_from_gz_idx_ubyte(testing_images, testing_labels, TEST_NUM).unwrap()
    }

    /// Decode MNIST training dataset (n_observation=60,000)
    pub fn training_from_gz(images_idx3_ubyte_gz: &[u8], labels_idx1_ubyte_gz: &[u8]) -> Dataset {
        dataset_from_gz_idx_ubyte(images_idx3_ubyte_gz, labels_idx1_ubyte_gz, TRAIN_NUM).unwrap()
    }

    /// Decode MNIST testing dataset (n_observation=10,000)
    pub fn testing_from_gz(images_idx3_ubyte_gz: &[u8], labels_idx1_ubyte_gz: &[u8]) -> Dataset {
        dataset_from_gz_idx_ubyte(images_idx3_ubyte_gz, labels_idx1_ubyte_gz, TEST_NUM).unwrap()
    }
}

fn dataset_from_gz_idx_ubyte<R: Read>(
    images: R,
    labels: R,
    num: usize,
) -> Result<Dataset, IdxError> {
    let observations: Array2<f64> = array_from_gz_idx_ubyte(images, (num, DATA_SIZE))?;
    let labels: Array1<usize> = array_from_gz_idx_ubyte(labels, num)?;
    let mut targets: Array2<f64> = Array2::zeros((num, DATA_TYPES));

    targets
        .axis_iter_mut(Axis(0))
        .zip(&labels)
        .for_each(|(mut t, &l)| t[l] = 1.0);

    Ok(Dataset::new(observations, targets, labels))
}

fn array_from_gz_idx_ubyte<R, Sh, D, A>(r: R, shape: Sh) -> Result<Array<A, D>, IdxError>
where
    R: Read,
    D: Dimension,
    Sh: Into<StrideShape<D>>,
    A: From<u8> + 'static,
{
    let file = GzDecoder::new(r);
    array_from_idx_ubyte(file, shape)
}

fn array_from_idx_ubyte<R, Sh, D, A>(mut r: R, shape: Sh) -> Result<Array<A, D>, IdxError>
where
    R: Read,
    D: Dimension,
    Sh: Into<StrideShape<D>>,
    A: From<u8> + 'static,
{
    let expected = [0, 0, 0x08, 0];
    let mut actual = [0, 0, 0, 0];
    r.read_exact(&mut actual).map_err(IdxError::Io)?;
    if actual[0] != expected[0] || actual[1] != expected[1] || actual[2] != expected[2] {
        return Err(IdxError::Magic { expected, actual });
    }

    let mut size = [0; 4];
    for _ in 0..actual[3] {
        r.read_exact(&mut size).map_err(IdxError::Io)?;
    }

    let mut buf = Vec::new();
    r.read_to_end(&mut buf).map_err(IdxError::Io)?;

    let arr = Array::from_shape_vec(shape, buf).map_err(IdxError::Shape)?;

    Ok(arr.mapv_into_any(|v| v.into()))
}

#[derive(Debug)]
enum IdxError {
    Io(std::io::Error),
    Shape(ndarray::ShapeError),
    Magic { expected: [u8; 4], actual: [u8; 4] },
}
impl Error for IdxError {}

impl Display for IdxError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            IdxError::Io(e) => e.fmt(f),
            IdxError::Shape(e) => e.fmt(f),
            IdxError::Magic { expected, actual } => {
                write!(
                    f,
                    "Expected magic number {:?} but got {:?}",
                    &expected[..3],
                    &actual[..3]
                )
            }
        }
    }
}
