use std::fs;

use kmeans::kmeans;
use mnist;

use image::DynamicImage;
use image::GrayImage;

fn load_training_data() -> (Vec<u8>, Vec<u8>) {
    let data = fs::read("./files/mnist-training-data")
        .expect("Something went wrong reading the data file");
    let labels = fs::read("./files/mnist-training-label")
        .expect("Something went wrong reading the label file");
    (data, labels)
}

fn load_testing_data() -> (Vec<u8>, Vec<u8>) {
    let data =
        fs::read("./files/mnist-test-data").expect("Something went wrong reading the data file");
    let labels =
        fs::read("./files/mnist-test-label").expect("Something went wrong reading the label file");
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

    for (i, cluster) in clusters.get_clusters().iter().enumerate() {
        let filename = format!(
            "./images/cluster_{}_label_{}.png",
            i,
            cluster.label().unwrap()
        );
        save_image(filename.as_str(), &cluster.get_centroid());
    }

    let ret = clusters.test(&ds);
}
