import init, {
    load_mnist_data,
    KMeansClusters,
    Dataset,
    rgba_image_to_grayscale_image,
    data_as_png_base64_string,
    init_panic_hook,
} from "@wasm/kmeans";
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
        case Command.test_one:
            const label = kmeans_test_one_rgba(msg.data.image);
            self.postMessage({ command: Command.test_one, label: label });
        default:
            break;
    }
};

/* K Means Cluster */
let dataset: Dataset;
let clusters: KMeansClusters;

async function kmeans_init(k: number, data: Uint8Array, label: Uint8Array) {
    await init();
    init_panic_hook();
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

function kmeans_test_one_rgba(rgba: Uint8Array): number {
    // let data = rgba_image_to_grayscale_image(28, 28, rgba);
    // let image = data_as_png_base64_string(data);
    // console.log(image);
    return clusters.test_rgba_image(rgba);
}
