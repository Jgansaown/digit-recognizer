use std::fs;

use rust_digit_recognition::k_means;
use rust_digit_recognition::mnist;

use image::DynamicImage;
use image::GrayImage;

use getrandom::getrandom;


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

    println!("MNIST test data size = {}", data.len());
    println!("MNIST test label size = {}", labels.len());

    let ds = mnist::Dataset::load(data, labels).expect("Error loading dataset");

    println!("{}, {}, {}", ds.nums, ds.rows, ds.cols);
    println!("{:?}", ds.iter().next());

    // for (i, (data, label)) in ds.iter().enumerate() {
    //     save_image(&format!("./images/{}_{}.png", i, label), data);
    //     if i > 10 {
    //         break;
    //     }
    // }

    // let k = 20;
    // let mut initial = Vec::new();
    // for _ in 0..k {
    //     let mut v = vec![0; (ds.rows * ds.cols) as usize];
    //     getrandom(&mut v).unwrap();
    //     initial.push(v);
    // }
    // // for (i, data) in initial.iter().enumerate() {
    // //     save_image(&format!("./images/initial_{}.png", i), data.as_slice());
    // // }
    // let dataset: Vec<Vec<u8>> = ds.iter().map(|(d, _)| d.to_vec()).collect();
    // // let cluster = k_means::k_means_clustering(dataset[..10000].to_vec(), initial);
    // let cluster = k_means::k_means_clustering(dataset, initial);

    // println!("{:?}", &cluster[0]);

    // for (i, cluster) in cluster.iter().enumerate() {
    //     save_image(format!("cluster_{}.png", i).as_str(), cluster);
    // }

    k_means::test([[0; 10]; 11]);
}
