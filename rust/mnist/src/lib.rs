use flate2::read::GzDecoder;
use ndarray::{Array, Array1, Array2, ArrayView1, Dimension, StrideShape};
use std::{
    error::Error,
    fmt::{Display, Formatter},
    io::Read,
};

pub const DATA_SIZE: usize = 28 * 28;
pub const DATA_TYPES: usize = 10;
pub const TRAIN_NUM: usize = 60_000;
pub const TEST_NUM: usize = 10_000;

// const DATA_MAGIC_NUMBER: u32 = 2051;
// const LABEL_MAGIC_NUMBER: u32 = 2049;

pub struct DataView<'a> {
    pub image: ArrayView1<'a, f64>,
    pub label: &'a u8,
}

/// Represents a MNIST Dataset
///
/// Owns the underlying data
pub struct Dataset {
    pub num: usize,
    pub size: usize,
    pub images: Array2<f64>, // Shape: (num, size)
    pub labels: Array1<u8>,
}
impl Dataset {
    /// MNIST Training Dataset
    ///
    /// Number of examples: 60,000
    pub fn training() -> Self {
        let images = include_bytes!("../data/train-images-idx3-ubyte.gz");
        let images: Array2<f64> =
            array_from_gz_idx_ubyte(&images[..], (TRAIN_NUM, DATA_SIZE)).unwrap();

        let labels = include_bytes!("../data/train-labels-idx1-ubyte.gz");
        let labels: Array1<u8> = array_from_gz_idx_ubyte(&labels[..], TRAIN_NUM).unwrap();

        Dataset {
            num: TRAIN_NUM,
            size: DATA_SIZE,
            images,
            labels,
        }
    }
    /// MNIST Testing Dataset
    ///
    /// Number of examples: 10,000
    pub fn testing() -> Self {
        let images = include_bytes!("../data/t10k-images-idx3-ubyte.gz");
        let images: Array2<f64> =
            array_from_gz_idx_ubyte(&images[..], (TEST_NUM, DATA_SIZE)).unwrap();

        let labels = include_bytes!("../data/t10k-labels-idx1-ubyte.gz");
        let labels: Array1<u8> = array_from_gz_idx_ubyte(&labels[..], TEST_NUM).unwrap();

        Dataset {
            num: TEST_NUM,
            size: DATA_SIZE,
            images,
            labels,
        }
    }

    /// Panics if i >= self.num
    pub fn at<'a>(&'a self, i: usize) -> DataView<'a> {
        DataView {
            image: self.images.row(i),
            label: &self.labels[i],
        }
    }
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
    r.read_exact(&mut actual).map_err(|e| IdxError::Io(e))?;
    if actual[0] != expected[0] || actual[1] != expected[1] || actual[2] != expected[2] {
        return Err(IdxError::Magic { expected, actual });
    }

    let mut size = [0; 4];
    for _ in 0..actual[3] {
        r.read_exact(&mut size).map_err(|e| IdxError::Io(e))?;
    }

    let mut buf = Vec::new();
    r.read_to_end(&mut buf).map_err(|e| IdxError::Io(e))?;

    let arr = Array::from_shape_vec(shape, buf).map_err(|e| IdxError::Shape(e))?;

    Ok(arr.mapv_into_any(|v| v.into()))
}

#[derive(Debug)]
enum IdxError {
    Io(std::io::Error),
    Shape(ndarray::ShapeError),
    Magic { expected: [u8; 4], actual: [u8; 4] },
}

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
impl Error for IdxError {}
