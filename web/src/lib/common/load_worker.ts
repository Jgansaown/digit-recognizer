import GzWorker from "./workers/gz.worker?worker&inline";
import type { MnistDataset } from "./mnist.dataset";

const GZ_WORKER = new GzWorker();

export function decode_mnist_gz(files: MnistDataset): Promise<MnistDataset> {
    return new Promise((res, rej) => {
        GZ_WORKER.postMessage(files);
        GZ_WORKER.onmessage = (msg) => res(msg.data);
    });
}
