# Digit Recognition using Machine Learning and Rust

[![Rust](https://github.com/Jgansaown/rust-digit-recognition/actions/workflows/rust.yml/badge.svg)](https://github.com/Jgansaown/rust-digit-recognition/actions/workflows/rust.yml) [![Github Pages Deploy](https://github.com/Jgansaown/rust-digit-recognition/actions/workflows/deploy.yml/badge.svg)](https://github.com/Jgansaown/rust-digit-recognition/actions/workflows/deploy.yml)

## Demo Site

Go to the [demo site](https://jgansaown.github.io/rust-digit-recognition) hosted statically on Github Pages  to see it in action!

## Description

Using Rust to implement various machine learning algorithms, compiled into WebAssembly. Which is then consumed by Svelte frontend to execute machine learning training and testing completely on client side.

## To-Dos

### Rust Wasm

- [ ] K-Means Clustering
  - [x] Main algorithm
  - [ ] Implement testing method
- [x] WASM for decompressing MNIST data 
- [ ] K-Nearest Neighbors
- [ ] Neural Network
- [ ] Convolutional Neural Network


### Svelte Frontend

- [x] Run WASM binaries on web workers to free up main thread
- [x] K-Means Clustering UI
- [ ] Testing UI
- [ ] K-Nearest Neighbor UI
- [ ] Neural Network UI
- [ ] Convolutional Neural Network UI