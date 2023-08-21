use std::time::Instant;

use kmeans::KMeans;
use mnist::Dataset;
use ndarray::ArrayView1;

fn main() {
    let training = Dataset::training();
    let testing = Dataset::testing();

    let param = KMeans::with_default_param()
        .n_clusters(100)
        .max_iter(1000)
        .min_dist(0.001);

    // let (model, duration) = {
    //     let start = Instant::now();
    //     let (i, dist, model) = param.train(&training);
    //     let duration = start.elapsed();

    //     println!("{}: {}", i, dist);

    //     (model, duration)
    // };

    let (model, duration) = {
        let start = Instant::now();
        let mut iter = param.train_iter(&training);
        for (i, dist) in &mut iter {
            println!("{}: {}", i, dist);
        }
        let duration = start.elapsed();

        (iter.into_model(), duration)
    };

    println!("Time elapsed in expensive_function() is: {:?}", duration);
    
    let correct = model.evaluate(&testing);

    println!(
        "Test Result: {} / {} = {}",
        correct,
        testing.num,
        correct as f64 / testing.num as f64
    );

    let observation = training.at(0);
    let predict = model.predict(&observation.image);
    print_image(observation.image);
    println!(
        "Predicted Label: {:?}, Actual Label: {}",
        predict, observation.label
    );
}

fn print_data_at(dataset: &Dataset, i: usize) {
    print_image(dataset.at(i).image);
    println!("{}", dataset.at(i).label);
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
