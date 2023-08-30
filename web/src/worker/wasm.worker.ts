import init, {
    set_panic_hook,
    Dataset,
    KMeans,
    KNearestNeighbors,
    Perceptron,
} from "@wasm/wasm";

let training_dataset: Dataset;
let testing_dataset: Dataset;

async function initialize() {
    const start = performance.now();
    await init();
    const end = performance.now();
    console.log(`wasm init took ${end - start} milliseconds`);

    set_panic_hook();

    training_dataset = Dataset.mnist_training();
    testing_dataset = Dataset.mnist_testing();
}

async function send_message(event: string, data: any) {
    postMessage({ event, data });
}

class Model {
    model?: KMeans;
    interval_id?: number;

    constructor() {}

    start_training(data: any) {
        this.stop_training();

        this.model = new KMeans(data.n_clusters);

        let i = 0;

        this.interval_id = setInterval(() => {
            const training_err = this.model?.step(training_dataset);
            const testing_err = this.model?.evaluate(testing_dataset);

            i++;

            send_message("step", { i, training_err, testing_err });

            if (i >= data.max_iter) {
                // done training
                this.stop_training();
            }
        });
    }

    stop_training() {
        clearInterval(this.interval_id);
    }
}

const init_done = initialize();
const model = new Model();

onmessage = async (e: MessageEvent<WASMEvent<any>>) => {
    await init_done;
    const { event, data } = e.data;

    switch (event) {
        case "start_training":
            console.log("[Worker] Received start training event");
            model.start_training(data);

            break;
        case "stop_training":
            console.log("[Worker] Received stop training event");
            model.stop_training();

            break;
        default:
            break;
    }
};
