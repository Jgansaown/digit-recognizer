use crate::mnist::DATA_SIZE;

use getrandom::getrandom;
pub struct Cluster {
    pub centroid: [f32; DATA_SIZE],
    assigned: Assigned,
}
impl Cluster {
    pub fn new() -> Self {
        Self {
            centroid: [0.0; DATA_SIZE],
            assigned: Assigned::new(),
        }
    }

    pub fn random() -> Self {
        let mut tmp = [0; DATA_SIZE];
        getrandom(&mut tmp).expect("error getting random numbers");

        let mut me = Self::new();
        me.centroid
            .iter_mut()
            .zip(tmp)
            .for_each(|(v, t)| *v = t as f32);
        me
    }

    pub fn euclidean_distance(&self, other: &[u8]) -> f32 {
        // https://en.wikipedia.org/wiki/Euclidean_distance
        self.centroid
            .iter()
            .zip(other.iter())
            .map(|(&a, &b)| (a, b as f32))
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f32>()
            .sqrt()
    }

    pub fn recalculate_centroid(&mut self) -> Option<f32> {
        self.assigned.div_by_num(self.num() as f32);

        let diff = self
            .centroid
            .iter()
            .zip(self.assigned.sum.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f32>()
            .sqrt();

        self.centroid = self.assigned.sum;
        match diff.is_nan() {
            true => None,
            false => Some(diff),
        }
    }

    pub fn add_data_to_assigned(&mut self, index: usize, data: &[u8], label: u8) {
        self.assigned.add_data(index, data, label);
    }

    pub fn clear_assigned(&mut self) {
        self.assigned.reset();
    }

    pub fn num(&self) -> usize {
        self.assigned.indexes.len()
    }

    pub fn list_assigned(&self) -> &[usize] {
        &self.assigned.indexes
    }

    pub fn get_centroid(&self) -> Vec<u8> {
        self.centroid.iter().map(|&v| v as u8).collect()
    }

    pub fn find_label(&mut self) {
        self.assigned.calc_value();
    }

    pub fn label(&self) -> Option<u8> {
        self.assigned.value
    }
}

struct Assigned {
    sum: [f32; DATA_SIZE],
    indexes: Vec<usize>,
    labels: Vec<u8>,
    value: Option<u8>,
}
impl Assigned {
    fn new() -> Self {
        Self {
            sum: [0.0; DATA_SIZE],
            indexes: Vec::new(),
            labels: Vec::new(),
            value: None,
        }
    }

    fn reset(&mut self) {
        self.sum = [0.0; DATA_SIZE];
        self.indexes.clear();
        self.labels.clear();
    }

    fn add_data(&mut self, index: usize, data: &[u8], label: u8) {
        self.sum
            .iter_mut()
            .zip(data.iter())
            .for_each(|(v, &d)| *v += d as f32);
        self.indexes.push(index);
        self.labels.push(label);
    }

    fn div_by_num(&mut self, num: f32) {
        for v in &mut self.sum {
            *v /= num;
        }
    }

    fn count_labels(&self) -> [usize; 10] {
        let mut counts = [0; 10];
        for &label in &self.labels {
            counts[label as usize] += 1;
        }
        counts
    }

    fn calc_value(&mut self) {
        let counts = self.count_labels();

        let (i, c) = counts
            .into_iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap();

        self.value = Some(i as u8);
    }
}
