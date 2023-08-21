use ndarray::{Array1, ArrayBase, Axis, Data, DataMut, Ix1, Ix2, Zip};
use ndarray_stats::DeviationExt;

#[cfg(feature = "rayon")]
pub fn update_membership(
    // shape: (n_observations)
    memberships: &mut ArrayBase<impl DataMut<Elem = usize> + Send + Sync, Ix1>,
    // shape: (n_centroids, n_features)
    centroids: &ArrayBase<impl Data<Elem = f64> + Sync, Ix2>,
    // shape: (n_observations, n_features)
    observations: &ArrayBase<impl Data<Elem = f64>, Ix2>,
) {
    Zip::from(memberships)
        .and(observations.axis_iter(Axis(0)))
        .par_for_each(|membership, observation| {
            *membership = find_nearest_centroid(&centroids, &observation).0;
        });
}

#[cfg(not(feature = "rayon"))]
pub fn update_membership(
    // shape: (n_observations)
    memberships: &mut ArrayBase<impl DataMut<Elem = usize>, Ix1>,
    // shape: (n_centroids, n_features)
    centroids: &ArrayBase<impl Data<Elem = f64>, Ix2>,
    // shape: (n_observations, n_features)
    observations: &ArrayBase<impl Data<Elem = f64>, Ix2>,
) {
    Zip::from(memberships)
        .and(observations.axis_iter(Axis(0)))
        .for_each(|membership, observation| {
            *membership = find_nearest_centroid(&centroids, &observation).0;
        });
}

pub fn find_nearest_centroid(
    // shape: (n_centroids, n_features)
    centroids: &ArrayBase<impl Data<Elem = f64>, Ix2>,
    // shape: (n_features)
    observation: &ArrayBase<impl Data<Elem = f64>, Ix1>,
) -> (usize, f64) {
    centroids
        .axis_iter(Axis(0))
        .map(|centroid| centroid.l2_dist(observation).unwrap())
        .enumerate()
        .min_by(|(_, a), (_, b)| a.total_cmp(b))
        .unwrap()
}

/// Compute the new centroids for each cluster
///
/// using m_k-means instead of the standard algorithm (Lloyd's) to avoid
/// problem with empty clusters
/// - https://docs.rs/linfa-clustering/latest/linfa_clustering/struct.KMeans.html
/// - https://www.researchgate.net/publication/228414762_A_Modified_k-means_Algorithm_to_Avoid_Empty_Clusters
pub fn update_centroids(
    centroids: &mut ArrayBase<impl DataMut<Elem = f64>, Ix2>,
    observations: &ArrayBase<impl Data<Elem = f64>, Ix2>,
    memberships: &ArrayBase<impl Data<Elem = usize>, Ix1>,
) {
    // let mut centroids = prev_centroids.clone();
    let mut counts: Array1<usize> = Array1::ones(centroids.nrows());

    Zip::from(observations.outer_iter())
        .and(memberships)
        .for_each(|image, &centroid_i| {
            let mut centroid = centroids.row_mut(centroid_i);
            centroid += &image;
            counts[centroid_i] += 1;
        });

    Zip::from(centroids.rows_mut())
        .and(&counts)
        .for_each(|mut centroid, &count| {
            centroid /= count as f64;
        });
}

pub fn update(
    // shape: (n_centroids, n_features)
    centroids: &mut ArrayBase<impl DataMut<Elem = f64> + Sync, Ix2>,
    // shape: (n_observations)
    memberships: &mut ArrayBase<impl DataMut<Elem = usize> + Sync + Send, Ix1>,
    // shape: (n_observations, n_features)
    observations: &ArrayBase<impl Data<Elem = f64>, Ix2>,
) {
    // 1. Assignment Step: Assign observations to cluster with nearest centroid
    update_membership(memberships, centroids, observations);
    // 2. Update Step: Recalculate centroid for each cluster
    update_centroids(centroids, observations, memberships);
}

pub fn calculate_centroids_info(
    // shape: (n_clusters, 10)
    centroids_info: &mut ArrayBase<impl DataMut<Elem = usize>, Ix2>,
    // shape: (n_observations)
    memberships: &ArrayBase<impl Data<Elem = usize>, Ix1>,
    // shape: (n_observations)
    labels: &ArrayBase<impl Data<Elem = u8>, Ix1>,
) {
    Zip::from(memberships)
        .and(labels)
        .for_each(|&cluster, &label| {
            centroids_info[(cluster, label as usize)] += 1;
        });
}

pub fn calculate_centroids_label(
    // shape: (n_clusters), data: (label, num)
    centroids_label: &mut ArrayBase<impl DataMut<Elem = Option<usize>>, Ix1>,
    // shape: (n_clusters, 10)
    centroids_info: &ArrayBase<impl Data<Elem = usize>, Ix2>,
) {
    Zip::from(centroids_label)
        .and(centroids_info.axis_iter(Axis(0)))
        .for_each(|label, row| {
            let (l, n) = row.indexed_iter().max_by_key(|&(_, &num)| num).unwrap();
            if *n == 0 {
                *label = None;
            } else {
                *label = Some(l);
            }
        });
}
