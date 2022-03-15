pub mod helper;
pub mod k_means;
pub mod mnist;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn load_mnist_data(data: Vec<u8>, labels: Vec<u8>) -> String {
    log("Hello from Rust Wasm!");

    let ds = mnist::Dataset::load(data, labels);
    format!("Num: {}, Size: {}", ds.num, ds.size)
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// use getrandom::getrandom;
// #[wasm_bindgen]
// pub fn test_k_means(data: Vec<u8>, labels: Vec<u8>) {
//     let ds = mnist::Dataset::load(data, labels).unwrap();

//     let mut initial = Vec::new();
//     for _ in 0..10 {
//         let mut v = vec![0; (ds.rows * ds.cols) as usize];
//         getrandom(&mut v).unwrap();

//         initial.push(v);
//     }
//     let cluster = k_means::k_means_clustering(&ds, initial);

//     log(&format!("{:?}", &cluster.assigned[..10]));
// }
