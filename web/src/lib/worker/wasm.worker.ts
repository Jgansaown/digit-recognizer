import init, {
    set_panic_hook,
    Dataset,
    MNIST,
    KMeans,
    KNearestNeighbors,
    Perceptron,
    TemplateModel,
} from "@wasm/mnist";

import {
    get_worker_pipe,
    type StartTrainingData,
    type WorkerPipe,
} from "./lib";

function create_new_model({ type, param }: StartTrainingData) {
    switch (type) {
        case "kmeans":
            return new KMeans(param.n_clusters, 28 * 28);
        case "knn":
            return new KNearestNeighbors(param.k);
        case "perceptron":
            return new Perceptron(param.learning_rate, 28 * 28, 10);
        default:
            return undefined;
    }
}

class Model {
    pipe: WorkerPipe;
    model?: TemplateModel = undefined;
    interval_id?: number = undefined;

    constructor(pipe: WorkerPipe) {
        this.pipe = pipe;
    }

    start_training(
        data: StartTrainingData,
        dataset: { training: Dataset; testing: Dataset }
    ) {
        this.stop_training();
        if (this.model != undefined) {
            this.model.free();
            this.model = undefined;
        }
        this.model = create_new_model(data);

        let i = 0;

        this.interval_id = setInterval(() => {
            const training_err = this.model?.step(dataset.training);
            const testing_err = this.model?.evaluate(dataset.testing);

            i++;

            this.pipe.sendCommand("step", { i, training_err, testing_err });

            if (!("max_iter" in data.param) || i >= data.param.max_iter) {
                // done training
                this.stop_training();
            }
        });
    }

    stop_training() {
        clearInterval(this.interval_id);
        this.interval_id = undefined;
    }

    predict(data: Float64Array) {
        if (this.model) {
            const prediction = this.model.predict(data);
            this.pipe.sendCommandTransfer("prediction", prediction);
        }
    }
}

async function initialize() {
    const start = performance.now();
    await init();
    const end = performance.now();
    console.log(`wasm init took ${end - start} milliseconds`);

    set_panic_hook();

    const pipe = get_worker_pipe(self as DedicatedWorkerGlobalScope);
    const model = new Model(pipe);

    const dataset = {
        training: MNIST.training_from_static(),
        testing: MNIST.testing_from_static(),
    };

    pipe.handleCommand(({ cmd, data }) => {
        switch (cmd) {
            case "start_training":
                console.log("[Worker] Received start training event");
                console.table(data);
                model.start_training(data, dataset);

                break;
            case "stop_training":
                console.log("[Worker] Received stop training event");
                model.stop_training();

                break;
            case "predict":
                console.log("[Worker] Received predict event");
                console.log(`[Worker] data.length: ${data.length}`);
                model.predict(data);

                break;
            default:
                break;
        }
    });
}

initialize();
