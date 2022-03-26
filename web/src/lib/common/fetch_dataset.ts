import init_gz, { decode_gz } from "@wasm/gz";
import mnist_dataset from "./mnist_dataset.json";

export interface jsDataset {
  data: Uint8Array;
  label: Uint8Array;
}

async function fetch_file(name: string): Promise<Uint8Array> {
  let resp = await fetch(mnist_dataset[name]);
  let blob = await resp.blob();
  let buf = await blob.arrayBuffer();
  return new Uint8Array(buf);
}

export default async function load_dataset(type: string): Promise<jsDataset> {
  await init_gz();

  let r = await Promise.all([
    fetch_file(`${type}-data`).then((data) => decode_gz(data)),
    fetch_file(`${type}-label`).then((data) => decode_gz(data)),
  ]);

  return {
    data: r[0],
    label: r[1],
  };
}
