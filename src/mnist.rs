use super::helper::slice_to_u32;

pub const DATA_SIZE: usize = 28 * 28;
pub const TRAIN_NUM: usize = 60_000;
pub const TEST_NUM: usize = 10_000;

const DATA_MAGIC_NUMBER: u32 = 2051;
const LABEL_MAGIC_NUMBER: u32 = 2049;

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
        let col =  slice_to_u32(&data[12..16]);
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
