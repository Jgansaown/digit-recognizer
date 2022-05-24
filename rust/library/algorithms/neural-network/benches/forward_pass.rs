//! # Results
//!
//! ```txt
//! test tests::bench_native_iter_forward_pass_100   ... bench:     549,230 ns/iter (+/- 21,674)
//! test tests::bench_native_iter_forward_pass_1000  ... bench:   5,379,355 ns/iter (+/- 88,027)
//! test tests::bench_native_iter_forward_pass_10000 ... bench:  54,101,239 ns/iter (+/- 856,925)
//! test tests::bench_native_loop_forward_pass_100   ... bench:     536,123 ns/iter (+/- 10,159)
//! test tests::bench_native_loop_forward_pass_1000  ... bench:   5,369,522 ns/iter (+/- 124,040)
//! test tests::bench_native_loop_forward_pass_10000 ... bench:  53,847,854 ns/iter (+/- 646,190)
//! test tests::bench_ndarray_forward_pass_100       ... bench:     545,128 ns/iter (+/- 9,521)
//! test tests::bench_ndarray_forward_pass_1000      ... bench:   5,536,340 ns/iter (+/- 85,426)
//! test tests::bench_ndarray_forward_pass_10000     ... bench:  55,348,706 ns/iter (+/- 735,170)
//! ```

#![feature(test)]

extern crate test;
use ndarray::{arr1, arr2, Array1, Array2};

use mnist::DATA_SIZE;

/// Calculates the result of one layer neural network
///
/// input: \[xi; DATA_SIZE+1\]
/// weights: \[\[wij; DATA_SIZE+1\]; 10\]
///
///
///
fn native_loop_forward_pass(
    weights: &[[f32; DATA_SIZE + 1]; 10],
    input: &[f32; DATA_SIZE + 1],
) -> [f32; 10] {
    let mut output = [0.0; 10];
    for i in 0..10 {
        for j in 0..DATA_SIZE + 1 {
            output[i] += weights[i][j] * input[j];
        }
    }
    output
}

fn native_iter_forward_pass(
    weights: &[[f32; DATA_SIZE + 1]; 10],
    input: &[f32; DATA_SIZE + 1],
) -> [f32; 10] {
    let output = weights
        .iter()
        .map(|weight| weight.iter().zip(input).map(|(w, i)| w * i).sum())
        .collect::<Vec<f32>>()
        .try_into()
        .unwrap();
    output
}

fn native_vec_forward_pass(weights: &[Vec<f32>], input: &[f32]) -> [f32; 10] {
    let output = weights
        .iter()
        .map(|weight| weight.iter().zip(input).map(|(w, i)| w * i).sum())
        .collect::<Vec<f32>>()
        .try_into()
        .unwrap();
    output
}

fn ndarray_forward_pass(weights: &Array2<f32>, input: &Array1<f32>) -> Array1<f32> {
    weights.t().dot(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_native_loop_forward_pass() {
        let ds = mnist::Dataset::load_from_path(
            "../../../../files/decoded/mnist-training-data",
            "../../../../files/decoded/mnist-training-label",
        );
        let input = ds.get_one_input_data_array();
        let weights = [[1.0; DATA_SIZE + 1]; 10];
        let ret = native_loop_forward_pass(&weights, &input);
        // println!("{:?}", ret);
        assert_eq!(ret, [108.94119; 10]);

        let ret = native_loop_forward_pass(&[[1.0; DATA_SIZE + 1]; 10], &[1.0; DATA_SIZE + 1]);
        // println!("{:?}", ret);
        assert_eq!(ret, [785.0; 10]);
    }
    #[test]
    fn test_native_iter_forward_pass() {
        let ds = mnist::Dataset::load_from_path(
            "../../../../files/decoded/mnist-training-data",
            "../../../../files/decoded/mnist-training-label",
        );
        let input = ds.get_one_input_data_array();
        let weights = [[1.0; DATA_SIZE + 1]; 10];
        let ret = native_iter_forward_pass(&weights, &input);
        // println!("{:?}", ret);
        assert_eq!(ret, [108.94119; 10]);

        let ret = native_iter_forward_pass(&[[1.0; DATA_SIZE + 1]; 10], &[1.0; DATA_SIZE + 1]);
        // println!("{:?}", ret);
        assert_eq!(ret, [785.0; 10]);
    }
    #[test]
    fn test_ndarray_forward_pass() {
        let weights = Array2::ones((DATA_SIZE + 1, 10));
        let input = Array1::ones(DATA_SIZE + 1);
        let ret = ndarray_forward_pass(&weights, &input);
        println!("{:?}", ret);
        assert_eq!(ret, arr1(&[785.0; 10]));
    }

    #[bench]
    fn bench_native_loop_forward_pass_100(bench: &mut Bencher) {
        let weights = [[1.0; DATA_SIZE + 1]; 10];
        let input = [1.0; DATA_SIZE + 1];
        bench.iter(|| {
            for _ in 0..100 {
                native_loop_forward_pass(&weights, &input);
            }
        });
    }
    #[bench]
    fn bench_native_loop_forward_pass_1000(bench: &mut Bencher) {
        let weights = [[1.0; DATA_SIZE + 1]; 10];
        let input = [1.0; DATA_SIZE + 1];
        bench.iter(|| {
            for _ in 0..1_000 {
                native_loop_forward_pass(&weights, &input);
            }
        });
    }
    #[bench]
    fn bench_native_loop_forward_pass_10000(bench: &mut Bencher) {
        let weights = [[1.0; DATA_SIZE + 1]; 10];
        let input = [1.0; DATA_SIZE + 1];
        bench.iter(|| {
            for _ in 0..10_000 {
                native_loop_forward_pass(&weights, &input);
            }
        });
    }

    #[bench]
    fn bench_native_vec_forward_pass_100(bench: &mut Bencher) {
        let weights = vec![vec![1.0; DATA_SIZE + 1]; 10];
        let input = vec![1.0; DATA_SIZE + 1];
        bench.iter(|| {
            for _ in 0..100 {
                native_vec_forward_pass(&weights, &input);
            }
        });
    }

    #[bench]
    fn bench_native_iter_forward_pass_100(bench: &mut Bencher) {
        let weights = [[1.0; DATA_SIZE + 1]; 10];
        let input = [1.0; DATA_SIZE + 1];
        bench.iter(|| {
            for _ in 0..100 {
                native_iter_forward_pass(&weights, &input);
            }
        });
    }
    #[bench]
    fn bench_native_iter_forward_pass_1000(bench: &mut Bencher) {
        let weights = [[1.0; DATA_SIZE + 1]; 10];
        let input = [1.0; DATA_SIZE + 1];
        bench.iter(|| {
            for _ in 0..1_000 {
                native_iter_forward_pass(&weights, &input);
            }
        });
    }
    #[bench]
    fn bench_native_iter_forward_pass_10000(bench: &mut Bencher) {
        let weights = [[1.0; DATA_SIZE + 1]; 10];
        let input = [1.0; DATA_SIZE + 1];
        bench.iter(|| {
            for _ in 0..10_000 {
                native_iter_forward_pass(&weights, &input);
            }
        });
    }

    #[bench]
    fn bench_ndarray_forward_pass_100(bench: &mut Bencher) {
        let weights = Array2::ones((DATA_SIZE + 1, 10));
        let input = Array1::ones(DATA_SIZE + 1);
        bench.iter(|| {
            for _ in 0..100 {
                ndarray_forward_pass(&weights, &input);
            }
        });
    }
    #[bench]
    fn bench_ndarray_forward_pass_1000(bench: &mut Bencher) {
        let weights = Array2::ones((DATA_SIZE + 1, 10));
        let input = Array1::ones(DATA_SIZE + 1);
        bench.iter(|| {
            for _ in 0..1_000 {
                ndarray_forward_pass(&weights, &input);
            }
        });
    }
    #[bench]
    fn bench_ndarray_forward_pass_10000(bench: &mut Bencher) {
        let weights = Array2::ones((DATA_SIZE + 1, 10));
        let input = Array1::ones(DATA_SIZE + 1);
        bench.iter(|| {
            for _ in 0..10_000 {
                ndarray_forward_pass(&weights, &input);
            }
        });
    }
}
