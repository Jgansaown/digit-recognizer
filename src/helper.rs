use super::mnist::Dataset;

pub fn slice_to_u32(slice: &[u8]) -> u32 {
    u32::from_be_bytes([slice[0], slice[1], slice[2], slice[3]])
}

pub struct Clusters {
    pub dataset: Dataset,
    pub centroids: Centroids,
    pub assigned: Vec<u8>,
}
impl Clusters {
    pub fn new(ds: Dataset, initial: Centroids) -> Self {
        let assigned = vec![0; ds.num];
        Self {
            dataset: ds,
            centroids: initial,
            assigned,
        }
    }
}

pub struct Centroids {
    size: usize,
    num: usize,
    base: Vec<u8>,
}
impl Centroids {
    pub fn new(size: usize, num: usize, initial: Vec<u8>) -> Self {
        Self {
            size,
            num,
            base: initial,
        }
    }

    pub fn iter(&self) -> CentroidsIterator {
        CentroidsIterator {
            size: self.size,
            num: self.num,
            index: 0,
            base: &self.base,
        }
    }

    pub fn iter_mut<'a>(&'a mut self) -> core::slice::IterMut<'a, u8> {
        self.base.iter_mut()
    }
}
pub struct CentroidsIterator<'a> {
    size: usize,
    num: usize,
    index: usize,
    base: &'a [u8],
}
impl<'a> Iterator for CentroidsIterator<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let range = (self.index * self.size)..((self.index + 1) * self.size);
        if self.index < self.num {
            Some(&self.base[range])
        } else {
            None
        }
    }
}
