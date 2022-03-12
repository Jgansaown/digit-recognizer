use super::helper::slice_to_u32;

#[derive(Debug)]
pub struct Dataset {
    pub nums: u32,
    pub rows: u32,
    pub cols: u32,
    data: Vec<u8>,
    labels: Vec<u8>,
}
impl Dataset {
    pub fn load(data: Vec<u8>, labels: Vec<u8>) -> Result<Self, &'static str> {
        let data_magic = slice_to_u32(&data[0..4]);
        let labels_magic = slice_to_u32(&labels[0..4]);
        assert_eq!(data_magic, 2051);
        assert_eq!(labels_magic, 2049);

        let data_nums = slice_to_u32(&data[4..8]);
        let labels_nums = slice_to_u32(&labels[4..8]);
        assert_eq!(data_nums, labels_nums);

        Ok(Self {
            nums: slice_to_u32(&data[4..8]),
            rows: slice_to_u32(&data[8..12]),
            cols: slice_to_u32(&data[12..16]),
            data,
            labels,
        })
    }

    pub fn iter(&self) -> DataIterator<'_> {
        DataIterator {
            data: &self.data[16..],
            labels: &self.labels[8..],
            index: 0,
            size: (self.rows * self.cols) as usize,
            nums: self.nums as usize,
        }
    }
}

pub struct DataIterator<'a> {
    size: usize,
    data: &'a [u8],
    labels: &'a [u8],
    index: usize,
    nums: usize,
}
impl<'a> Iterator for DataIterator<'a> {
    type Item = (&'a [u8], u8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.nums {
            let range = (self.index * self.size)..((self.index + 1) * self.size);
            let index = self.index;
            self.index += 1;
            Some((&self.data[range], self.labels[index]))
        } else {
            None
        }
    }
}
