import init_gz, { decode_gz } from "@wasm/gz";
import type { MnistDataset } from "../mnist.dataset";

self.onmessage = async function (msg: MessageEvent<MnistDataset>) {
    await init_gz();

    const ret = {
        training: {
            data: decode_gz(msg.data.training.data),
            label: decode_gz(msg.data.training.label),
        },
        testing: {
            data: decode_gz(msg.data.testing.data),
            label: decode_gz(msg.data.testing.label),
        },
    };
    self.postMessage(ret);
};
