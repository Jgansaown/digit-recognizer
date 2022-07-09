import { expose } from "comlink";
import { threads } from "wasm-feature-detect";
import init, {
    MnistDataset,
    run_network,
    NeuralNetwork as NN,
} from "@wasm/mnist-rs";

async function init_wasm() {
    if (await threads()) {
        console.log("wasm threads supported 🙌");
    } else {
        console.log("wasm threads not supported 😢");
    }
    await init();
}

class NeuralNetwork {
    dataset: {
        train: MnistDataset;
        test: MnistDataset;
    };

    load_datasets(
        training: { data: Uint8Array; label: Uint8Array },
        testing: { data: Uint8Array; label: Uint8Array }
    ) {
        this.dataset = {
            train: MnistDataset.from_raw(training.data, training.label),
            test: MnistDataset.from_raw(testing.data, testing.label),
        };
        console.log("done loading datasets");
    }

    run_network() {
        const start = performance.now();
        run_network(this.dataset.train, this.dataset.test, 10);
        const end = performance.now();
        console.log(`took ${(end - start) / 1000} seconds`);
    }
}

const obj = {
    init: init_wasm,
    cls: NeuralNetwork,
    nn: NN,
};

async function new_network() {
    await init();

    const n1 = NN.create_1_layer(16);
    const n2 = NN.create_2_layers(16, 16);

    return new NeuralNetwork();
}

export type { NeuralNetwork };

export type { obj };

expose(obj);
