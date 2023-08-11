use flate2::read::GzDecoder;
use ndarray::Array2;
use std::{
    error::Error,
    fmt::{Display, Formatter},
    io::Read,
};

pub struct Dataset<D, T> {
    pub data: Array2<D>,
    pub targets: Array2<T>,
}

pub fn train() -> Dataset<u8, u8> {
    let data = include_bytes!("../data/train-images-idx3-ubyte.gz");
    let data: Array2<u8> = array_from_gz_idx_ubyte(&data[..], (60000, 28 * 28)).unwrap();

    let targets = include_bytes!("../data/train-labels-idx1-ubyte.gz");
    let targets: Array2<u8> = array_from_gz_idx_ubyte(&targets[..], (60000, 1)).unwrap();

    Dataset { data, targets }
}

pub fn t10k() -> Dataset<u8, u8> {
    let data = include_bytes!("../data/t10k-images-idx3-ubyte.gz");
    let data: Array2<u8> = array_from_gz_idx_ubyte(&data[..], (10000, 28 * 28)).unwrap();

    let targets = include_bytes!("../data/t10k-labels-idx1-ubyte.gz");
    let targets: Array2<u8> = array_from_gz_idx_ubyte(&targets[..], (10000, 1)).unwrap();

    Dataset { data, targets }
}

fn array_from_gz_idx_ubyte<R: Read>(r: R, shape: (usize, usize)) -> Result<Array2<u8>, IdxError> {
    let file = GzDecoder::new(r);
    array_from_idx_ubyte(file, shape)
}

fn array_from_idx_ubyte<R: Read>(mut r: R, shape: (usize, usize)) -> Result<Array2<u8>, IdxError> {
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

    Array2::from_shape_vec(shape, buf).map_err(|e| IdxError::Shape(e))
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
