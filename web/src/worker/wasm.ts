//

class WASMWorker extends EventTarget {
    onstep_cb?: (data: any) => void;

    worker = new Worker(new URL("./wasm.worker.ts", import.meta.url), {
        type: "module",
    });

    constructor() {
        super();

        this.worker.onmessage = (e: MessageEvent<WASMEvent<any>>) => {
            const { event, data } = e.data;
            // console.log("from worker:", event, data);
            // console.log(data);

            // this.dispatchEvent(new CustomEvent(event, { detail: data }));
            if (event == "step") {
                this.onstep_cb?.call(null, data);
            }
        };
    }

    start_training(type: string, param: any) {
        if (type == "kmeans") {
            this.send("start_training", param);
        }
    }

    stop_training() {
        this.send("stop_training", {});
    }

    
    public set onstep(cb : (data: any) => void) {
        this.onstep_cb = cb;
    }
    

    private send(event: string, data: any) {
        this.worker.postMessage({ event, data });
    }
}

export { WASMWorker };
