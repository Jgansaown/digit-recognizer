import init, {
    set_panic_hook,
    Dataset,
    MNIST,
    KMeans,
    KNearestNeighbors,
    Perceptron,
    TemplateModel,
} from "@wasm/mnist";
import { type AckMsg, type ReqMsg } from "./wasm.types";
import { Pipe } from "./wasm.pipe";

function create_new_model({ type, param }: ModelParametersUnion) {
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

class WasmState {
    pipe: Pipe<typeof self, ReqMsg, AckMsg> = new Pipe(self);
    dataset: {
        training: Dataset;
        testing: Dataset;
    };

    model: TemplateModel | undefined = undefined;
    timer: number | undefined = undefined;
    iteration: number = 0;

    constructor() {
        this.dataset = {
            training: MNIST.training_from_static(),
            testing: MNIST.testing_from_static(),
        };

        this.pipe.handle("init_model", (data) => {
            console.log(`[worker][init_model]: ${JSON.stringify(data)}`);
            const ret = this.init_model(data);
            return { value: ret };
        });
        this.pipe.handle("free_model", () => {
            console.log(`[worker][free_model]`);
            const ret = this.free_model();
            return { value: ret };
        });
        this.pipe.handle("start_training", () => {
            console.log(`[worker][start_training]`);
            const ret = this.start_training();
            return { value: ret };
        });
        this.pipe.handle("stop_training", () => {
            console.log(`[worker][stop_training]`);
            const ret = this.stop_training();
            return { value: ret };
        });
        this.pipe.handle("predict", (data) => {
            console.log(`[worker][predict]`);
            return this.predict(data);
        });
        this.pipe.handle("evaluate", () => {
            console.log(`[worker][evaluate]`);
            return this.evaluate();
        })
    }

    init_model(data: ModelParametersUnion) {
        if (this.model == undefined) {
            this.model = create_new_model(data);
        }
    }

    free_model() {
        this.stop_training();
        this.iteration = 0;

        if (this.model != undefined) {
            this.model.free();
            this.model = undefined;
        }
    }

    // can only start if model is created and no timer is currently running
    start_training() {
        this.timer = setTimeout(this.step.bind(this), 0);
    }

    stop_training() {
        clearInterval(this.timer);
        this.timer = undefined;
    }

    predict(data: Float64Array) {
        let prediction = new Float64Array();
        if (this.model) {
            prediction = this.model.predict(data);
        }
        return {
            value: prediction,
            transfer: [prediction.buffer],
        };
    }

    evaluate() {
        let value = NaN;
        if (this.model) {
            value = this.model.evaluate(this.dataset.testing);
        }
        return { value }
    }

    /** Hardcoded maximum of 1,000,000 steps */
    private step() {
        if (this.model && this.iteration <= 1_000_000) {
            const err = this.model.step(this.dataset.training);
            this.iteration++;
            this.pipe.request("step", { i: this.iteration, err });
            this.timer = setTimeout(this.step.bind(this), 0);
        }
    }
}

async function initialize() {
    const start = performance.now();
    await init();
    const end = performance.now();
    console.log(`wasm init took ${end - start} milliseconds`);

    set_panic_hook();

    return new WasmState();
}

let state: WasmState;
initialize().then((_state) => (state = _state));
