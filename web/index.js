// import wasmInit from "./pkg/rust_digit_recognition.js";
import wasmInit, {
    load_mnist_data,
    as_png_base64_string,
    get_nth_image,
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


let count = 0;
document.getElementById('start_button').addEventListener('click', async (e) => {
  console.log("Starting...");
  console.log('Training Data:');
  console.log(mnist_training_data);
  console.log('Training Label:');
  console.log(mnist_training_label);

  let image = get_nth_image(mnist_training_data, mnist_training_label, count);
  document.getElementById('test_image').src = as_png_base64_string(image);
  count += 1;
});