use gz::decode_from_path;
use k_nearest::KNearestNeighbors;
use mnist::{Data, Dataset};

use std::fs;
use std::io::prelude::*;
use std::ops::Div;
fn save_file() {
    let data = decode_from_path("../files/mnist-training-data.gz");
    let label = decode_from_path("../files/mnist-training-label.gz");
    fs::write("../files/decoded/mnist-training-data", data).unwrap();
    fs::write("../files/decoded/mnist-training-label", label).unwrap();
    let data = decode_from_path("../files/mnist-test-data.gz");
    let label = decode_from_path("../files/mnist-test-label.gz");
    fs::write("../files/decoded/mnist-test-data", data).unwrap();
    fs::write("../files/decoded/mnist-test-label", label).unwrap();
}

fn load_mnist_dataset() -> mnist::Dataset {
    let data = decode_from_path("../files/mnist-training-data.gz");
    let label = decode_from_path("../files/mnist-training-label.gz");
    Dataset::load(data, label)
}

fn test_knn(training: Dataset, testing: Dataset, k: usize) -> f32 {
    let mut knn = KNearestNeighbors::new(k);
    knn.train(&training);

    let mut correct: usize = 0;
    for data in testing.iter() {
        let ret = knn.par_find(data.value);
        if let Some(nn) = ret {
            let mut digits: [usize; 10] = [0; 10];
            for d in nn {
                digits[d as usize] += 1;
            }
            let mut ranking: Vec<(usize, usize)> = digits
                .into_iter()
                .enumerate()
                .collect();
            ranking.sort_by(|(_, d1), (_, d2)| d2.cmp(d1));

            if let Some(&(guess, _)) = ranking.first() {
                if guess == data.label as usize {
                    correct += 1;
                }
            }
            // println!("Actual: {}, Ranking: {:?}", data.label, ranking);
        }
    }
    let error_rate: f32 = (testing.num-correct) as f32 / testing.num as f32;
    println!("K={}: Error Rate: {}/{}={}", k, (testing.num-correct), testing.num, error_rate);
    error_rate
}

fn get_training() -> Dataset {
    Dataset::load_from_path(
        "../files/decoded/mnist-training-data",
        "../files/decoded/mnist-training-label",
    )
}

fn get_testing() -> Dataset {
    Dataset::load_from_path(
        "../files/decoded/mnist-test-data",
        "../files/decoded/mnist-test-label",
    )
}

fn main() {
    let mut error_rates = Vec::new();
    let rate = test_knn(get_training(), get_testing(), 10000);
    error_rates.push(rate);
    // for k in 2..10 as usize {
    //     let rate = test_knn(get_training(), get_testing(), k);
    //     error_rates.push(rate);
    // }
    println!("{:?}", error_rates);


    // let training_dataset = Dataset::load_from_path(
    //     "../files/decoded/mnist-training-data",
    //     "../files/decoded/mnist-training-label",
    // );
    // let testing_dataset = Dataset::load_from_path(
    //     "../files/decoded/mnist-test-data",
    //     "../files/decoded/mnist-test-label",
    // );

    // let mut knn = KNearestNeighbors::new(3);
    // knn.load(training_dataset);

    // let mut correct: usize = 0;
    // for data in testing_dataset.iter() {
    //     let ret = knn.par_find(data.value);
    //     if let Some(nn) = ret {
    //         let mut digits: [usize; 10] = [0; 10];
    //         for d in nn {
    //             digits[d as usize] += 1;
    //         }
    //         let mut ranking: Vec<(usize, usize)> = digits
    //             .into_iter()
    //             .enumerate()
    //             .collect();
    //         ranking.sort_by(|(_, d1), (_, d2)| d2.cmp(d1));

    //         if let Some(&(guess, count)) = ranking.first() {
    //             if guess == data.label as usize {
    //                 correct += 1;
    //             }
    //         }
    //         println!("Actual: {}, Ranking: {:?}", data.label, ranking);
    //     }
    // }
    // let error_rate: f32 = 1.0 - ((correct as f32) / (testing_dataset.num as f32));
    // println!("Total: {}, Correct: {}, Error Rate: {}", testing_dataset.num, correct, error_rate);
}
