use base64::{write::EncoderStringWriter, STANDARD};
use image::DynamicImage;
use image::GrayImage;
use std::io::{Cursor, Write};
use std::path::Path;
use wasm_bindgen::prelude::*;

pub const DATA_SIZE: usize = 28 * 28;
pub const DATA_TYPES: usize = 10;
pub const TRAIN_NUM: usize = 60_000;
pub const TEST_NUM: usize = 10_000;

const DATA_MAGIC_NUMBER: u32 = 2051;
const LABEL_MAGIC_NUMBER: u32 = 2049;

const BLACK_IMG: &'static str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABwAAAAcCAIAAAD9b0jDAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAJcEhZcwAAFiUAABYlAUlSJPAAAAAZSURBVEhL7cExAQAAAMKg9U9tB28gAABONQlMAAEdn/sHAAAAAElFTkSuQmCC";

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn load_mnist_data(data: Vec<u8>, labels: Vec<u8>) -> Dataset {
    Dataset::load(data, labels)
}

#[wasm_bindgen]
pub fn get_nth_image(dataset: &Dataset, n: usize) -> Vec<u8> {
    dataset.iter().nth(n).unwrap().value.to_vec()
}

#[wasm_bindgen]
pub fn data_as_png_base64_string(data: &[u8]) -> String {
    let mut ret = String::from("data:image/png;base64,");
    let mut writer = Cursor::new(Vec::new());

    image::write_buffer_with_format(
        &mut writer,
        data,
        28,
        28,
        image::ColorType::L8,
        image::ImageOutputFormat::Png,
    )
    .unwrap();

    let mut enc = EncoderStringWriter::from(&mut ret, STANDARD);
    enc.write_all(writer.get_ref()).unwrap();
    let _ = enc.into_inner();

    ret
}

#[wasm_bindgen]
pub fn get_black_image() -> String {
    BLACK_IMG.to_string()
}

pub fn save_as_image<P: AsRef<Path>>(path: P, data: &[u8]) {
    let gray = GrayImage::from_raw(28, 28, data.to_vec()).unwrap();
    let img = DynamicImage::ImageLuma8(gray);
    img.save(path).unwrap();
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Dataset {
    /// Number of Data
    pub num: usize,
    /// Size of Data
    pub size: usize,
    data: Vec<u8>,
    labels: Vec<u8>,
}
impl Dataset {
    /// Combines both MNIST raw data and label into a singular dataset
    ///
    pub fn load(data: Vec<u8>, labels: Vec<u8>) -> Self {
        assert_eq!(slice_to_u32(&data[0..4]), DATA_MAGIC_NUMBER);
        assert_eq!(slice_to_u32(&labels[0..4]), LABEL_MAGIC_NUMBER);
        assert_eq!(slice_to_u32(&data[4..8]), slice_to_u32(&labels[4..8]));

        let row = slice_to_u32(&data[8..12]);
        let col = slice_to_u32(&data[12..16]);
        Self {
            num: slice_to_u32(&data[4..8]) as usize,
            size: (row * col) as usize,
            data: data[16..].to_vec(),
            labels: labels[8..].to_vec(),
        }
    }

    pub fn iter(&self) -> DataSetIterator<'_> {
        DataSetIterator {
            size: self.size,
            num: self.num,
            index: 0,
            data: &self.data,
            labels: &self.labels,
        }
    }
}

pub struct DataSetIterator<'a> {
    size: usize,
    num: usize,
    index: usize,
    data: &'a [u8],
    labels: &'a [u8],
}
impl<'a> Iterator for DataSetIterator<'a> {
    type Item = Data<'a, u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.num {
            let range = (self.index * self.size)..((self.index + 1) * self.size);
            let index = self.index;
            self.index += 1;
            Some(Data {
                value: &self.data[range],
                label: self.labels[index],
            })
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Data<'a, T> {
    pub value: &'a [T],
    pub label: u8,
}
impl<'a> Data<'a, u8> {
    pub fn euclidean_distance(&self, other: &Data<u8>) -> f32 {
        self.value
            .iter()
            .zip(other.value.iter())
            .map(|(&a, &b)| (a as f32, b as f32))
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f32>()
            .sqrt()
    }
}

fn slice_to_u32(slice: &[u8]) -> u32 {
    u32::from_be_bytes([slice[0], slice[1], slice[2], slice[3]])
}
