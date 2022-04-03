import init, { load_mnist_data, KMeansClusters, Dataset } from "@wasm/kmeans";
import { CommandDto, Command } from "./kmeans.dto";

self.onmessage = async function (msg: MessageEvent<CommandDto>) {
  switch (msg.data.command) {
    case Command.init:
      await kmeans_init(...msg.data.init);
      self.postMessage({ command: Command.init });
      break;
    case Command.step:
      const dist = kmeans_step();
      self.postMessage({ command: Command.step, dist: dist });
      break;
    case Command.info:
      const info = kmeans_info();
      self.postMessage({ command: Command.info, info: info });
      break;
    case Command.free:
      kmeans_free();
      self.postMessage({ command: Command.free });
    default:
      break;
  }
};

/* K Means Cluster */
let dataset: Dataset;
let clusters: KMeansClusters;

async function kmeans_init(k: number, data: Uint8Array, label: Uint8Array) {
  await init();
  dataset = load_mnist_data(data, label);
  clusters = KMeansClusters.random(k);
}

function kmeans_step(): number {
  clusters.assign_dataset(dataset);
  return clusters.recalculate_centroids();
}

function kmeans_info() {
  return clusters.get_info();
}

function kmeans_free() {
  if (dataset != undefined) {
    dataset.free();
  }
  if (clusters != undefined) {
    clusters.free();
  }
}
