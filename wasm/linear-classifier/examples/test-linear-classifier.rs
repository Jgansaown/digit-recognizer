use std::error;

use linear_classifier;
use mnist::Dataset;

fn main() {
    let training_dataset = Dataset::load_from_path(
        "../files/decoded/mnist-training-data",
        "../files/decoded/mnist-training-label",
    );
    let test_dataset = Dataset::load_from_path(
        "../files/decoded/mnist-test-data",
        "../files/decoded/mnist-test-label",
    );

    let mut lc = linear_classifier::LinearClassifier::<784>::new();

    for i in 0..100 {
        lc.train(&training_dataset);

        let mut errors: usize = 0;
        for data in test_dataset.iter() {
            let predicts = lc.predict(data.value);
            let predicts: Option<usize> = predicts
                .iter()
                .enumerate()
                .filter_map(|(i, &p)| if p == 1.0 { Some(i) } else { None })
                .next();
            match predicts {
                Some(p) => {
                    if p != data.label as usize {
                        errors += 1;
                    }
                }
                None => errors += 1,
            }
            // println!("{}: {:?}", data.label, predicts);
        }
        println!("Iterations={}: Error Rate={}/{}={}", i, errors, test_dataset.num, (errors as f32)/(test_dataset.num as f32));
    }
}
