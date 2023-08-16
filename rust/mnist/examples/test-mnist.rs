use mnist::Dataset;
use ndarray::ArrayView1;

fn main() {
    let training = Dataset::training();
    let testing = Dataset::testing();

    print_data_at(&training, 0);
    print_data_at(&training, 60_000 - 1);

    print_data_at(&testing, 0);
    print_data_at(&testing, 10_000 - 1)
}

fn print_data_at(dataset: &Dataset, i: usize) {
    print_image(dataset.at(i).image);
    println!("{}", dataset.at(i).label);
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
