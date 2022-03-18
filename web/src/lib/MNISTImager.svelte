<script lang="ts">
  import init, {
    get_nth_image,
    as_png_base64_string,
  } from "../../pkg/rust_digit_recognition";

  init().then((wasm) => {
    console.log(wasm);
  });

  let data_files: FileList;
  let label_files: FileList;
  let data: Uint8Array;
  let label: Uint8Array;
  let count: number = 0;
  let image_src: string =
    "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABwAAAAcCAIAAAD9b0jDAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAJcEhZcwAAFiUAABYlAUlSJPAAAAAZSURBVEhL7cExAQAAAMKg9U9tB28gAABONQlMAAEdn/sHAAAAAElFTkSuQmCC";

  $: if (data_files) {
    for (const file of data_files) {
      file.arrayBuffer().then((v) => {
        data = new Uint8Array(v);
      });
    }
  }

  $: if (label_files) {
    for (const file of label_files) {
      file.arrayBuffer().then((v) => {
        label = new Uint8Array(v);
      });
    }
  }

  $: if (count >= 0) {
    if (data && label) {
      let image = get_nth_image(data, label, count);
      image_src = as_png_base64_string(image);
    }
  }

  function inc() {
    count += 1;
  }
  
  function dec() {
    if (count > 0) {
      count -= 1;
    }
  }
</script>

<div>
  <div>
    <label for="data">Select data file:</label>
    <input id="data" bind:files={data_files} type="file" />
  </div>
  <div>
    <label for="label">Select label file:</label>
    <input id="label" bind:files={label_files} type="file" />
  </div>

  <p>Image #{count}</p>
  <div>
    <img src={image_src} alt="handwritten digit" />
  </div>
  <button on:click={dec}>Prev</button>
  <button on:click={inc}>Next</button>
</div>

<style>
  img {
    height: 10rem;
    width: 10rem;
  }
</style>
