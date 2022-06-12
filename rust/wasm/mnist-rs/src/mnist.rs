//! Relating to load and processing MNIST datasets
//!
use gz::{decode_gz, unpack_tar_gz};
use mnist::Dataset;
use ndarray::{arr1, Array1};
use std::path::PathBuf;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct MnistDataset {
    inner: Dataset,
}
#[wasm_bindgen]
impl MnistDataset {
    /// Load MNIST dataset from uncompressed file
    pub fn from_raw(data: Vec<u8>, label: Vec<u8>) -> Self {
        Self {
            inner: Dataset::load(data, label),
        }
    }

    /// Load MNIST dataset from gz compressed file
    ///
    /// Download them here: http://yann.lecun.com/exdb/mnist/
    pub fn from_gz(data: Vec<u8>, label: Vec<u8>) -> Self {
        let data = decode_gz(&data);
        let label = decode_gz(&label);
        Self::from_raw(data, label)
    }

    /// Load MNIST dataset from .tar.gz file
    ///
    /// The data file must be called `data` and the label file must be called `label`
    pub fn from_tar_gz(file: Vec<u8>) -> Self {
        let mut files = unpack_tar_gz(&file);
        match (
            files.remove(&PathBuf::from("data")),
            files.remove(&PathBuf::from("label")),
        ) {
            (Some(data), Some(label)) => (Self::from_raw(data, label)),
            _ => panic!("data or label file not found"),
        }
    }
}
impl MnistDataset {
    pub fn as_ref(&self) -> &Dataset {
        &self.inner
    }

    /// Get dataset data as Vec<Array1<f32>>
    /// 
    /// data is normalized by 0xFF
    pub fn as_array_data(&self) -> Vec<Array1<f32>> {
        self.inner
            .iter()
            .map(|d| {
                arr1(
                    &d.value
                        .iter()
                        .map(|&v| (v as f32) / (0xFF as f32))
                        .collect::<Vec<_>>(),
                )
            })
            .collect()
    }
    pub fn as_array_label(&self) -> Vec<Array1<f32>> {
        self.inner
            .iter()
            .map(|d| {
                let mut o: Array1<f32> = Array1::zeros(10);
                o[d.label as usize] = 1.0;
                o
            })
            .collect()
    }
    pub fn as_u8_label(&self) -> Vec<u8> {
        self.inner.iter().map(|d| d.label).collect()
    }
}
