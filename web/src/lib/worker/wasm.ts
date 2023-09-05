import {
    get_worker_pipe,
    type StartTrainingData,
    type WorkerPipe,
} from "./lib";

/**
 * Interface to access the WASM worker from the main thread
 */
export class WASMWorker {
    pipe: WorkerPipe;
    callbacks: {
        step?: (data: any) => void;
        prediction?: (data: any) => void;
    } = {};

    constructor() {
        this.pipe = get_worker_pipe(
            new Worker(new URL("./wasm.worker.ts", import.meta.url), {
                type: "module",
            })
        );

        this.pipe.handleCommand(({ cmd, data }) => {
            switch (cmd) {
                case "step":
                case "prediction":
                    this.callbacks[cmd]?.call(null, data);
                    break;
                default:
                    break;
            }
        });
    }

    start_training<T extends ModelTypes>(type: T, param: ModelParameters[T]) {
        const data = { type, param } as StartTrainingData;
        this.pipe.sendCommand("start_training", data);
    }

    stop_training() {
        this.pipe.sendCommand("stop_training", null);
    }

    set onstep(cb: (data: any) => void) {
        this.callbacks.step = cb;
    }

    predict(data: Float64Array) {
        this.pipe.sendCommandTransfer("predict", data);
    }

    set onprediction(cb: (data: any) => void) {
        this.callbacks.prediction = cb;
    }
}
