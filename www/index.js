// import wasmInit from "./pkg/rust_digit_recognition.js";
import wasmInit, {
    load_mnist_data,
    test_k_means,
  } from "./pkg/rust_digit_recognition.js";

main();

async function main() {
  // Instantiate our wasm module
  const rustWasm = await wasmInit("./pkg/rust_digit_recognition_bg.wasm");

}

async function load_data(data, label) {
  // const rustWasm = await wasmInit("./pkg/rust_digit_recognition_bg.wasm");
  
  let ret = load_mnist_data(data, label);
  console.log(ret);
}

let mnist_training_data = new Uint8Array();
let mnist_training_label = new Uint8Array();


document.getElementById('mnist_training_data_input').addEventListener('change', (e) => {
  const reader = new FileReader();
  reader.readAsArrayBuffer(e.target.files[0]);
  reader.onloadend = (evt) => {
    if (evt.target.readyState === FileReader.DONE) {
      const arrayBuffer = evt.target.result;
      mnist_training_data = new Uint8Array(arrayBuffer);
    }
  }
});

document.getElementById('mnist_training_label_input').addEventListener('change', (e) => {
  const reader = new FileReader();
  reader.readAsArrayBuffer(e.target.files[0]);
  reader.onloadend = (evt) => {
    if (evt.target.readyState === FileReader.DONE) {
      const arrayBuffer = evt.target.result;
      mnist_training_label = new Uint8Array(arrayBuffer);
    }
  }
});

document.getElementById('start_button').addEventListener('click', async (e) => {
  console.log("Starting...");
  console.log('Training Data:');
  console.log(mnist_training_data);
  console.log('Training Label:');
  console.log(mnist_training_label);

  await load_data(mnist_training_data, mnist_training_label);
  
  test_k_means(mnist_training_data, mnist_training_label);
});