use neural_network::LinearClassifier;
use mnist::Dataset;
use std::fs;
use std::io::prelude::*;

fn iter_test(num_iter: usize, learning_rate: f32, train: &Dataset, test: &Dataset) -> Vec<usize> {
    let mut lc = LinearClassifier::new( 784, 10, learning_rate);
    println!("Learning Rate: {}", learning_rate);
    let mut errors = Vec::new();
    for i in 0..num_iter {
        lc.train(&train);
        let e: usize = lc.test(&test);
        errors.push(e);
        println!(
            "{}: Incorrect Predict={}, Error Rate={}",
            i,
            e,
            (e as f32) / (test.num as f32)
        );
    }
    errors
}

fn main() {
    let training_dataset = Dataset::load_from_path(
        "../files/decoded/mnist-training-data",
        "../files/decoded/mnist-training-label",
    );
    let test_dataset = Dataset::load_from_path(
        "../files/decoded/mnist-test-data",
        "../files/decoded/mnist-test-label",
    );

    let num_iter = 1000;
    let rate = [0.5, 0.1, 0.05, 0.01, 0.005, 0.001];

    let mut file = fs::File::create("../files/lc-output.txt").unwrap();
    file.write(b"total,rate").unwrap();
    for i in 0..num_iter {
        file.write(format!(",{}", i).as_bytes()).unwrap();
    }
    file.write(b"\n").unwrap();
    
    for r in rate {
        let errors = iter_test(num_iter, r, &training_dataset, &test_dataset);
        file.write(format!("{},{}", test_dataset.num, r).as_bytes())
            .unwrap();
        for e in errors {
            file.write(format!(",{}", e).as_bytes()).unwrap();
        }
        file.write(b"\n").unwrap();
    }

    // let mut lc = linear_classifier::LinearClassifier::new(784, 10, 0.01);
    // let mut output_file = fs::File::create("../files/linear-classifier-output.txt").unwrap();
    // for i in 0..100 {
    //     lc.train(&training_dataset);

    //     // let errors: usize = lc.test(&test_dataset);
    //     let errors: usize = lc.par_test(&test_dataset);
    //     output_file
    //         .write(format!("{},{}\n", i, errors).as_bytes())
    //         .unwrap();

    //     println!(
    //         "Iterations={}: Error Rate={}/{}={}",
    //         i,
    //         errors,
    //         test_dataset.num,
    //         (errors as f32) / (test_dataset.num as f32)
    //     );
    // }
}
