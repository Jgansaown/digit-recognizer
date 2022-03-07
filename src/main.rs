use std::fs;

fn get_mnist_header(bytes: &[u8]) {
    assert!(bytes.len() == 16);

    let magic = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    let nums = u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);

    let (rows, cols): (u32, u32) = match magic {
        2051 => {
            let rows = u32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
            let cols = u32::from_be_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]);
            (rows, cols)
        }
        2049 => (0, 1),
        _ => (0, 0),
    };
    // if magic == 2051 {
    //     let rows = u32::from_be_bytes(
    //         [bytes[8], bytes[9], bytes[10], bytes[11]]
    //     );
    //     let cols = u32::from_be_bytes(
    //         [bytes[12], bytes[13], bytes[14], bytes[15]]
    //     );
    // }
    // else {
    //     let rows = 0;
    //     let cols = 0;
    // }
    println!("Magic Number: {}", magic);
    println!("Number of Images: {}", nums);
    println!("Rows: {}, Cols: {}", rows, cols);
}

fn get_mnist_data(data_path: &str, label_path: &str) {
    let data = fs::read(data_path).expect("Something went wrong reading the data file");
    let label = fs::read(label_path).expect("Something went wrong reading the label file");

    get_mnist_header(&data[..16]);
    get_mnist_header(&label[..16]);
}

fn main() {
    get_mnist_data("files/mnist-training-data", "files/mnist-training-label");
}
