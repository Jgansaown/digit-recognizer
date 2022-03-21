<script lang="ts">
  import { dataset } from "./datastore";
  import init, {
    get_nth_image,
    as_png_base64_string,
  } from "@wasm/kmeans";
  
  init().then((wasm) => {
    console.log(wasm);
  });

  let data: Uint8Array;
  let label: Uint8Array;
  let count: number = 0;
  let image_src: string =
    "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABwAAAAcCAIAAAD9b0jDAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAJcEhZcwAAFiUAABYlAUlSJPAAAAAZSURBVEhL7cExAQAAAMKg9U9tB28gAABONQlMAAEdn/sHAAAAAElFTkSuQmCC";

  let unsubscribe = dataset.subscribe(value => {
    data = value.data;
    label = value.label;
  });

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
  <p>Image #{count}</p>
  <div>
    <img src={image_src} alt="handwritten digit" />
  </div>
  <button on:click={dec}>Prev</button>
  <button on:click={inc}>Next</button>
</div>

<style>
  img {
    height: 5rem;
    width: 5rem;
  }
</style>
