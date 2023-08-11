use mnist::{self, Dataset};
use ndarray::ArrayView1;

fn main() {
    let training = mnist::train();
    let testing = mnist::t10k();

    print_data_at(&training, 0);
    print_data_at(&training, 60_000 - 1);

    print_data_at(&testing, 0);
    print_data_at(&testing, 10_000 - 1)
}

fn print_data_at(dataset: &Dataset<u8, u8>, i: usize) {
    let img_data = dataset.data.row(i);
    let target = dataset.targets.row(i);

    print_image(img_data);
    println!("{}", target);
}

fn print_image(data: ArrayView1<u8>) {
    let data = data.to_shape((28, 28)).unwrap();
    for i in 0..28usize {
        for j in 0..28usize {
            let v = data[(i, j)];
            match v {
                _ if v > 128 => print!("⬜"),
                _ => print!("⬛"),
            }
        }
        println!("");
    }
}
