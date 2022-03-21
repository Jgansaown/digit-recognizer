<script type="ts">
  import init_gz_wasm, { decode_gz } from "@wasm/gz";
  import mnist_dataset from './mnist_dataset.json';
  import { dataset } from './datastore';

  let dataset_type = 'training';
  let gz_dataset: {
      data: Uint8Array,
      label: Uint8Array
  };

  async function fetch_datasetfile(name: string): Promise<Uint8Array> {
    let resp = await fetch(mnist_dataset[name]);
    let blob = await resp.blob();
    let buf = await blob.arrayBuffer();
    return new Uint8Array(buf);
  }

  async function load_dataset() {
    await init_gz_wasm();
    gz_dataset = {
        data: await fetch_datasetfile(`${dataset_type}-data`),
        label: await fetch_datasetfile(`${dataset_type}-label`)
    };
    
    dataset.set({
        data: decode_gz(gz_dataset.data),
        label: decode_gz(gz_dataset.label),
    });
  }

  function download_file(data: Uint8Array, filename: string) {
    let blob = new Blob([data], { type: "" });
    let url = URL.createObjectURL(blob);
    const link = document.createElement("a");
    link.style.display = "none";
    document.body.appendChild(link);
    link.href = url;
    link.download = filename;
    link.click();
    document.body.removeChild(link);
  }
</script>

<p style="display: inline;">Load MNIST dataset:</p>
<label>
    <input type='radio' bind:group={dataset_type} value={'training'}>
    Training
</label>
<label>
    <input type='radio' bind:group={dataset_type} value={'testing'}>
    Testing
</label>
<button on:click={load_dataset}>Load Dataset</button>
