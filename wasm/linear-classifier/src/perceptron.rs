use std::ops::{Add, AddAssign};

use mnist::Dataset;
use ndarray::{Array2, ArrayView2, Zip};

const NUM_OUTPUT: usize = 10;
const DATA_SIZE: usize = mnist::DATA_SIZE;

pub fn dataset_to_arrays(dataset: &Dataset) -> (Array2<f32>, Array2<f32>) {
    let data =
        Array2::from_shape_vec((dataset.num, DATA_SIZE), dataset.to_data_flat_f32_vec()).unwrap();
    let mut label: Array2<f32> = Array2::zeros((NUM_OUTPUT, dataset.num));
    // let label = dataset.to_label_vec();
    println!(
        "{:?}, {:?}, {:?}",
        label.shape(),
        label.row(0).shape(),
        label.column(0).shape()
    );

    label
        .columns_mut()
        .into_iter()
        .zip(dataset.iter())
        .for_each(|(mut c, d)| {
            c[d.label as usize] = 1.0;
        });

    (data, label)
}

pub struct Perceptron {
    weights: Array2<f32>,
    biases: Array2<f32>,
    learning_rate: f32,
}
impl Perceptron {
    pub fn new(learning_rate: f32) -> Self {
        Self {
            weights: Array2::zeros((NUM_OUTPUT, DATA_SIZE)),
            biases: Array2::zeros((NUM_OUTPUT, 1)),
            learning_rate,
        }
    }

    pub fn train(&mut self, data: &Array2<f32>, label: &Array2<f32>) {
        Zip::from(data.rows())
            .and(label.columns())
            .for_each(|input, target| {
                let target = target.into_owned().into_shape((10, 1)).unwrap();
                let output = self.predict(&input.into_shape((1, 784)).unwrap());

                let update = self.learning_rate * (target - output);
                
                println!("update={:?}, weights={:?}, biases={:?}", update.shape(), self.weights.shape(), self.biases.shape());

                // println!("{:?}, {:?}", d.shape(), l.shape());

                self.weights += update.dot(&input);

                self.weights.map_axis_mut(ndarray::Axis(1), |mut row| {
                    row.zip_mut_with(&update, |v, u| {
                        *v += u;
                    })
                });
                self.biases.zip_mut_with(&update, |b , u| {
                    *b += u;
                })
            });

        // println!("{:?}", label.column(0));
        // let output = self.net_input(&data.view());
    }

    pub fn net_input(&self, input: &ArrayView2<f32>) -> Array2<f32> {
        self.weights.dot(&input.t()).add(&self.biases)
    }

    pub fn predict(&self, input: &ArrayView2<f32>) -> Array2<f32> {
        let mut output = self.net_input(input);
        output.map_inplace(|o| {
            if *o >= 0.0 {
                *o = 1.0;
            } else {
                *o = 0.0;
            }
        });
        output
    }
}
