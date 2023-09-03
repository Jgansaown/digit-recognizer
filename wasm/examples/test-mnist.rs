use ndarray::ArrayView1;
use mnist::dataset::{Dataset, MNIST};

fn main() {
    
    let start = std::time::Instant::now();
    let training = MNIST::training_from_static();
    let testing = MNIST::testing_from_static();
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);

    print_data_at(&training, 0);
    print_data_at(&training, 60_000 - 1);

    print_data_at(&testing, 0);
    print_data_at(&testing, 10_000 - 1)
}

fn print_data_at(dataset: &Dataset, i: usize) {
    let (observation, target) = dataset.at(i);

    print_image(observation);
    println!(
        "{:?}",
        target.iter().enumerate().collect::<Vec<(usize, &f64)>>()
    );
}

fn print_image(data: ArrayView1<f64>) {
    let data = data.to_shape((28, 28)).unwrap();
    for i in 0..28usize {
        for j in 0..28usize {
            let v = data[(i, j)];
            match v {
                _ if v > 128.0 => print!("⬜"),
                _ => print!("⬛"),
            }
        }
        println!("");
    }
}
