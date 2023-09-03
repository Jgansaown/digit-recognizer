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

    onstep_cb?: (data: any) => void;

    constructor() {
        this.pipe = get_worker_pipe(
            new Worker(new URL("./wasm.worker.ts", import.meta.url), {
                type: "module",
            })
        );

        this.pipe.handleCommand(({ cmd, data }) => {
            if (cmd == "step") {
                this.onstep_cb?.call(null, data);
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

    public set onstep(cb: (data: any) => void) {
        this.onstep_cb = cb;
    }
}
