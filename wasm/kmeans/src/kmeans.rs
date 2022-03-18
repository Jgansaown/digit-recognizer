use mnist::Dataset;
use super::cluster::Cluster;
use std::io::Write;

pub fn naive_clustering(dataset: &Dataset, k: usize, min_change: f32) -> KMeansClusters {
    let mut clusters = KMeansClusters::random(k);
    loop {
        // Clear cached data
        clusters.clear_cached();

        // (1) Assign each data point in dataset to a cluster
        println!("Assigning each data point to a cluster...");
        for (i, data) in dataset.iter().enumerate() {
            clusters.assign_to_cluster(i, data.value, data.label);

            if (i + 1) % 5000 == 0 {
                print!("{}....\r", i + 1);
                std::io::stdout().flush().unwrap();
            }
        }
        println!("");

        // Dropping centroids with no match
        clusters.drop_no_match_clusters();

        // (2) Recalculate the centroid of each clusters
        println!("Recalculating the centroids...");
        let diff = clusters.recalculate_centroids();

        // (3) break the loop if centroid does not change much
        println!(
            "Average change in distance between old and new centroids: {}",
            diff
        );
        if diff < min_change {
            break;
        }
    }

    // Find the cluster value using the most common label in the cluster
    clusters.find_labels();

    clusters
}

pub struct KMeansClusters {
    clusters: Vec<Cluster>,
}
impl KMeansClusters {
    pub fn new(k: usize) -> Self {
        Self {
            clusters: (0..k).into_iter().map(|_| Cluster::new()).collect(),
        }
    }
    pub fn random(k: usize) -> Self {
        Self {
            clusters: (0..k).into_iter().map(|_| Cluster::random()).collect(),
        }
    }
    pub fn save(&self) {
        todo!();
    }
    pub fn load() -> Self {
        todo!()
    }

    pub fn clear_cached(&mut self) {
        for cluster in &mut self.clusters {
            cluster.clear_assigned();
        }
    }

    pub fn assign_to_cluster(&mut self, index: usize, data: &[u8], label: u8) {
        let id = self.find_closest_cluster(data);
        self.clusters[id].add_data_to_assigned(index, data, label);
    }

    pub fn drop_no_match_clusters(&mut self) {
        let ids: Vec<usize> = self
            .clusters
            .iter()
            .enumerate()
            .filter_map(|(i, cluster)| match cluster.num() {
                0 => Some(i),
                _ => None,
            })
            .collect();
        for id in ids.into_iter().rev() {
            self.clusters.swap_remove(id);
        }
    }

    pub fn recalculate_centroids(&mut self) -> f32 {
        let sums = self
            .clusters
            .iter_mut()
            .filter_map(|cluster| cluster.recalculate_centroid())
            .sum::<f32>();
        sums / (self.clusters.len() as f32)
    }

    pub fn find_labels(&mut self) {
        for cluster in &mut self.clusters {
            cluster.find_label();
        }
    }

    pub fn get_clusters(&self) -> &[Cluster] {
        &self.clusters
    }

    pub fn test(&self, dataset: &Dataset) -> f32 {
        let correct: usize = dataset.iter().fold(0, |acc, data| {
            let label = self.find_closest_cluster_label(data.value);
            if label == data.label {
                acc + 1
            } else {
                acc
            }
        });
        let err_rate = 1.0 - (correct as f32) / (dataset.num as f32);
        println!(
            "Total: {}, Correct: {}, Error Rate: {}",
            dataset.num, correct, err_rate
        );
        err_rate
    }

    fn find_closest_cluster_label(&self, data: &[u8]) -> u8 {
        let i = self.find_closest_cluster(data);
        self.clusters[i].label().unwrap()
    }

    fn find_closest_cluster(&self, data: &[u8]) -> usize {
        self.clusters
            .iter()
            .map(|c| c.euclidean_distance(data))
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).expect("error comparing floats"))
            .map(|(i, _)| i)
            .expect("clusters empty, cannot find closest cluster")
    }
}
