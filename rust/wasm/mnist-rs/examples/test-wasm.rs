use mnist_rs::MnistDataset;

fn main() {
    let file = std::fs::read("../files/mnist-training.tar.gz")
        .unwrap_or_else(|_| panic!("current dir: {:?}", std::env::current_dir()));
    
    // let file = fs::read("mnist-data.tar.gz").unwrap();
    let ds = MnistDataset::from_tar_gz(file);
    println!("{}", ds.as_ref().num);
}