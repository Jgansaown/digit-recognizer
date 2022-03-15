// use super::mnist::Dataset;

use std::io::Write;

type U8Data = Vec<u8>;
type F32Data = Vec<f32>;

pub fn test<const N: usize, const K: usize>(dataset: [[u8; N]; K]) {
    println!("K={}, N={}", dataset.len(), dataset[0].len());
}

fn convert_u8_dataset_to_f32_dataset(dataset: Vec<Vec<u8>>) -> Vec<Vec<f32>> {
    dataset
        .into_iter()
        .map(|c| c.into_iter().map(|v| v as f32).collect())
        .collect()
}
fn convert_f32_dataset_to_u8_dataset(dataset: Vec<Vec<f32>>) -> Vec<Vec<u8>> {
    dataset
        .into_iter()
        .map(|c| c.into_iter().map(|v| v.round() as u8).collect())
        .collect()
}

pub fn k_means_clustering(dataset: Vec<Vec<u8>>, initial: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let centroids = convert_u8_dataset_to_f32_dataset(initial);
    let dataset = convert_u8_dataset_to_f32_dataset(dataset);

    let mut kmc = KMeansCluster {
        centroids,
        assigned: vec![0; dataset.len()],
    };

    loop {
        // (1) Assign each data point in dataset to a cluster
        let mut changed = false;
        for (i, data) in dataset.iter().enumerate() {
            changed = kmc.assign_to_centroid(i, data) || changed;
            if i % 5000 == 0 {
                print!("{}....\r", i);
                std::io::stdout().flush().unwrap();
            }
        }
        // (2) Recalculate the centroid of each cluster
        let done = kmc.recalculate_centroids(&dataset);
        // (3) break the loop if assignment (1) did not change
        if done {
            break;
        }
        // if changed == false {
        //     break;
        // }
        // break;
    }
    // (4) return the cluster centroids
    convert_f32_dataset_to_u8_dataset(kmc.centroids)
}

#[derive(Debug)]
pub struct KMeansCluster {
    pub centroids: Vec<Vec<f32>>,
    pub assigned: Vec<u8>,
}
impl KMeansCluster {
    fn assign_to_centroid(&mut self, i: usize, data: &[f32]) -> bool {
        let closest: u8 = self
            .centroids
            .iter()
            .map(|centroid| euclidean_distance(centroid, data))
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).expect("no NaN"))
            .map(|(i, _)| i)
            .expect("no closest centroid")
            .try_into()
            .unwrap();

        match self.assigned.get(i) {
            Some(&prev) => {
                self.assigned[i] = closest;
                prev != closest
            }
            None => {
                println!("oops");
                self.assigned.push(closest);
                true
            }
        }
    }

    fn recalculate_centroids(&mut self, ds: &Vec<Vec<f32>>) -> bool {
        let mut counts = vec![0u32; self.centroids.len()];
        let mut new_centroids = vec![vec![0f32; self.centroids[0].len()]; self.centroids.len()];

        let iter = self.assigned.iter().zip(ds.iter()).enumerate();
        for (i, (&assigned, data)) in iter {
            for (i, &bytes) in data.iter().enumerate() {
                new_centroids[assigned as usize][i] += bytes as f32;
            }
            counts[assigned as usize] += 1;
        }
        
        let new: Vec<Vec<f32>> = counts
            .iter()
            .zip(new_centroids.iter())
            .enumerate()
            .filter_map(|(i, (&count, centroid))| {
                // TODO: if count is zero then keep the centroid?
                if count == 0 {
                    None
                }
                else {
                    Some(centroid.iter().map(|&v| v / (count as f32)).collect())
                }
            })
            .collect();
        
        let max_diff = self.centroids.iter().zip(new.iter()).map(|(o, n)| {
            euclidean_distance(o, n)
        }).reduce(f32::max).unwrap();
        
        // println!("{:?}", new_centroids);
        println!("{:?}", counts);
        println!("{}", max_diff);
        // println!("{:?}", new[0]);
        self.centroids = new;

        max_diff < 100.0f32
    }
}

fn euclidean_distance(s1: &[f32], s2: &[f32]) -> f32 {
    // https://en.wikipedia.org/wiki/Euclidean_distance
    s1.iter()
        .zip(s2.iter())
        .map(|(&a, &b)| (a - b).powi(2))
        .sum::<f32>()
        .sqrt()
}
