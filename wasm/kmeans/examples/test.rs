use std::fs;

use kmeans::kmeans;
use mnist;

use image::DynamicImage;
use image::GrayImage;

fn load_training_data() -> (Vec<u8>, Vec<u8>) {
    let data = fs::read("../files/mnist-training-data")
        .expect("Something went wrong reading the data file");
    let labels = fs::read("../files/mnist-training-label")
        .expect("Something went wrong reading the label file");
    (data, labels)
}

fn load_testing_data() -> (Vec<u8>, Vec<u8>) {
    let data =
        fs::read("../files/mnist-test-data").expect("Something went wrong reading the data file");
    let labels =
        fs::read("../files/mnist-test-label").expect("Something went wrong reading the label file");
    (data, labels)
}

fn save_image(filename: &str, data: &[u8]) {
    let gray = GrayImage::from_raw(28, 28, data.to_vec()).unwrap();
    let img = DynamicImage::ImageLuma8(gray);
    img.save(filename).unwrap();
}

fn main() {
    let (data, labels) = load_training_data();
    // let (data, labels) = load_testing_data();

    println!("data size = {}", data.len());
    println!("label size = {}", labels.len());

    let ds = mnist::Dataset::load(data, labels);
    println!("{}, {}", ds.num, ds.size);

    let clusters = kmeans::naive_clustering(&ds, 20, 10.0);

    let folder = "../files/images";
    if std::path::Path::new(folder).exists() {
        fs::remove_dir_all(folder).unwrap();
        fs::create_dir(folder).unwrap();
    }

    for (i, cluster) in clusters.get_clusters().iter().enumerate() {
        let filename = format!("{}/cluster_{}_label_{:?}.png", folder, i, cluster.label);
        save_image(filename.as_str(), &cluster.centroid);
    }

    let err = clusters.test(&ds);
    println!("Training error rate: {}", err);

    let (d, l) = load_testing_data();
    let test = mnist::Dataset::load(d, l);
    let err = clusters.test(&test);
    println!("Testing error rate: {}", err);
}
