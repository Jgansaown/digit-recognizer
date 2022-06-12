use std::path::Path;

pub const DATA_SIZE: usize = 28 * 28;
pub const DATA_TYPES: usize = 10;
pub const TRAIN_NUM: usize = 60_000;
pub const TEST_NUM: usize = 10_000;

const DATA_MAGIC_NUMBER: u32 = 2051;
const LABEL_MAGIC_NUMBER: u32 = 2049;

#[derive(Debug, Clone)]
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
    pub fn load(data: Vec<u8>, label: Vec<u8>) -> Self {
        assert_eq!(slice_to_u32(&data[0..4]), DATA_MAGIC_NUMBER);
        assert_eq!(slice_to_u32(&label[0..4]), LABEL_MAGIC_NUMBER);
        assert_eq!(slice_to_u32(&data[4..8]), slice_to_u32(&label[4..8]));

        let row = slice_to_u32(&data[8..12]);
        let col = slice_to_u32(&data[12..16]);
        Self {
            num: slice_to_u32(&data[4..8]) as usize,
            size: (row * col) as usize,
            data: data[16..].to_vec(),
            labels: label[8..].to_vec(),
        }
    }

    pub fn load_from_path<P: AsRef<Path>>(data: P, labels: P) -> Self {
        let data = std::fs::read(data)
            .unwrap_or_else(|_| panic!("current dir: {:?}", std::env::current_dir()));
        let labels = std::fs::read(labels)
            .unwrap_or_else(|_| panic!("current dir: {:?}", std::env::current_dir()));
        Self::load(data, labels)
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

    pub fn to_vec(&self) -> Vec<(u8, Vec<u8>)> {
        self.iter().map(|d| (d.label, d.value.to_vec())).collect()
    }

    pub fn to_data_flat_iter(&self) -> std::slice::Iter<'_, u8> {
        self.data.iter()
    }

    pub fn to_data_flat_f32_vec(&self) -> Vec<f32> {
        self.data.iter().map(|v| *v as f32).collect()
    }

    pub fn to_label_vec(&self) -> Vec<u8> {
        self.labels.clone()
    }

    pub fn to_normalized_data(&self) -> Vec<f32> {
        self.data
            .iter()
            .map(|v| (*v as f32) / (0xff as f32))
            .collect()
    }

    pub fn get_one_input_data_array(&self) -> [f32; DATA_SIZE + 1] {
        let mut input = [0.0; DATA_SIZE + 1];
        for (i, v) in self.iter().next().unwrap().value.iter().enumerate() {
            input[i] = (*v as f32) / (0xff as f32);
        }
        input[784] = 1.0;
        input
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
    ///
    /// Also known as L2 norm or L2 distance
    pub fn euclidean_distance(&self, other: &[u8]) -> f32 {
        self.value
            .iter()
            .zip(other)
            .map(|(&a, &b)| (a as f32, b as f32))
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f32>()
            .sqrt()
    }
}

fn slice_to_u32(slice: &[u8]) -> u32 {
    u32::from_be_bytes([slice[0], slice[1], slice[2], slice[3]])
}
