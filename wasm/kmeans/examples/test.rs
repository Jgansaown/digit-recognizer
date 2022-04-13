use gz::decode_gz;
use kmeans::KMeansClusters;
use mnist::Dataset;
use std::fs;

fn load_training_data() -> (Vec<u8>, Vec<u8>) {
    let data = fs::read("../files/mnist-training-data.gz")
        .expect("Something went wrong reading the data file");
    let labels = fs::read("../files/mnist-training-label.gz")
        .expect("Something went wrong reading the label file");
    (decode_gz(&data), decode_gz(&labels))
}

fn load_testing_data() -> (Vec<u8>, Vec<u8>) {
    let data = fs::read("../files/mnist-test-data.gz")
        .expect("Something went wrong reading the data file");
    let labels = fs::read("../files/mnist-test-label.gz")
        .expect("Something went wrong reading the label file");
    (decode_gz(&data), decode_gz(&labels))
}

fn kmeans_naive_clustering(dataset: &Dataset, k: usize, min_change: f32) -> KMeansClusters {
    let mut clusters = KMeansClusters::random(k);
    loop {
        // (1) Assign each data point in dataset to a cluster
        println!("Assigning each data point to a cluster...");
        #[cfg(feature = "multithread")]
        clusters.par_assign_dataset(dataset);
        #[cfg(not(feature = "multithread"))]
        clusters.assign_dataset(dataset);

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
    clusters
}

fn main() {
    let training = {
        let (data, labels) = load_training_data();
        Dataset::load(data, labels)
    };
    let testing = {
        let (data, labels) = load_testing_data();
        Dataset::load(data, labels)
    };

    // Train clusters
    let clusters = kmeans_naive_clustering(&training, 15, 50.0);

    // Save clusters
    {
        let folder = "../files/images";
        if std::path::Path::new(folder).exists() {
            fs::remove_dir_all(folder).unwrap();
        }
        fs::create_dir(folder).unwrap();
        for (i, cluster) in clusters.get_clusters().iter().enumerate() {
            let filename = format!("{}/cluster_{}_label_{:?}.png", folder, i, cluster.label);
            mnist::save_as_image(filename, &cluster.centroid);
        }
    }

    let err = clusters.test(&training);
    println!("Training error rate: {}", err);

    let err = clusters.test(&testing);
    println!("Testing error rate: {}", err);

    // for data in testing.iter() {
    //     let labels = clusters.test_data(data.value);
    //     println!("actual: {}, labels: {:?}", data.label, labels);
    //     break;
    // }

    let result = clusters.test_dataset(&testing);

    println!(
        "Total: {}, Correct: {}, Incorrect: {}, Error Rate: {}",
        result.total, result.correct, result.incorrect, result.error_rate
    )
}
