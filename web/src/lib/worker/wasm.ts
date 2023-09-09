import { type AckMsg, type ReqMsg } from "./wasm.types";
import { Pipe } from "./wasm.pipe";

/**
 * Interface to access the WASM worker from the main thread
 */
export class WasmWorker {
    pipe: Pipe<Worker, ReqMsg, AckMsg>;
    handlers: {
        step?: (value: ReqMsg["step"]) => AckMsg["step"];
    } = {};

    constructor() {
        this.pipe = new Pipe(
            new Worker(new URL("./wasm.worker.ts", import.meta.url), {
                type: "module",
            })
        );
        this.pipe.handle("step", (data) => {
            const ret = this.handlers["step"]?.call(null, data);
            return { value: ret };
        });
    }

    set on_step(cb: (data: ReqMsg["step"]) => AckMsg["step"]) {
        this.handlers["step"] = cb;
    }

    /**
     * Sends message to the web worker
     */
    send<K extends keyof ReqMsg>(
        cmd: K,
        value?: ReqMsg[K],
        transfer?: Transferable[]
    ): Promise<AckMsg[K]> {
        if (value == undefined) {
            return this.pipe.request(cmd, null as ReqMsg[K], transfer);
        } else {
            return this.pipe.request(cmd, value, transfer);
        }
    }
}
