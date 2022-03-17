mod cluster;

use crate::mnist::Dataset;
use cluster::Cluster;
use std::io::Write;

pub fn naive_clustering(dataset: Dataset, k: usize, err: f32) -> Vec<Cluster> {
    let mut clusters: Vec<Cluster> = (0..k).into_iter().map(|_| Cluster::random()).collect();
    loop {
        // Clear cached data
        for cluster in &mut clusters {
            cluster.clear_assigned();
        }

        // (1) Assign each data point in dataset to a cluster
        println!("Assigning each data point to a cluster...");
        for (i, data) in dataset.iter().enumerate() {
            let id = find_closest_cluster(&clusters, data.value);
            clusters[id].add_data_to_assigned(i, data.value, data.label);

            if (i + 1) % 5000 == 0 {
                print!("{}....\r", i + 1);
                std::io::stdout().flush().unwrap();
            }
        }
        println!("");

        // Dropping centroids with no match
        let ids: Vec<usize> = clusters
            .iter()
            .enumerate()
            .filter_map(|(i, cluster)| match cluster.num() {
                0 => Some(i),
                _ => None,
            })
            .collect();
        for id in ids.into_iter().rev() {
            clusters.swap_remove(id);
        }

        // (2) Recalculate the centroid of each clusters
        println!("Recalculating the centroids...");
        let diff = clusters
            .iter_mut()
            .filter_map(|cluster| cluster.recalculate_centroid())
            .sum::<f32>()
            / (k as f32);

        // (3) break the loop if centroid does not change much
        println!(
            "Average euclidean distance between old and new centroids: {}",
            diff
        );
        if diff < err {
            break;
        }
    }

    // Find the cluster value using the most common label in the cluster
    for cluster in &mut clusters {
        cluster.find_label();
    }
    clusters
}

fn find_closest_cluster(clusters: &[Cluster], data: &[u8]) -> usize {
    clusters
        .iter()
        .map(|c| c.euclidean_distance(data))
        .enumerate()
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).expect("error comparing floats"))
        .map(|(i, _)| i)
        .expect("clusters empty, cannot find closest cluster")
}
