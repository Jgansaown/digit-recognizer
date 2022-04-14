import { decode_mnist_gz } from "./workers/load";

const URLS = {
    training: {
        data: "https://raw.githubusercontent.com/Jgansaown/rust-digit-recognition/main/files/mnist-training-data.gz",
        label: "https://raw.githubusercontent.com/Jgansaown/rust-digit-recognition/main/files/mnist-training-label.gz",
    },
    testing: {
        data: "https://raw.githubusercontent.com/Jgansaown/rust-digit-recognition/main/files/mnist-test-data.gz",
        label: "https://raw.githubusercontent.com/Jgansaown/rust-digit-recognition/main/files/mnist-test-label.gz"
    },
};

export interface jsDataset {
  data: Uint8Array;
  label: Uint8Array;
}

export interface MnistDataset {
    training: jsDataset,
    testing: jsDataset,
};

async function fetch_file(url: string): Promise<Uint8Array> {
  let resp = await fetch(url);
  let blob = await resp.blob();
  let buf = await blob.arrayBuffer();
  return new Uint8Array(buf);
}

export default async function load_mnist_dataset(): Promise<MnistDataset> {
  const files = await Promise.all([
    fetch_file(URLS.training.data),
    fetch_file(URLS.training.label),
    fetch_file(URLS.testing.data),
    fetch_file(URLS.testing.label),
  ]);

  return await decode_mnist_gz({
    training: {
      data: files[0],
      label: files[1],
    },
    testing: {
      data: files[2],
      label: files[3],
    },
  });
}
