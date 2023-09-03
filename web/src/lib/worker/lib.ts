export type StartTrainingData =
    | { type: "kmeans"; param: KMeansParam }
    | { type: "knn"; param: KNearestNeighborsParam }
    | { type: "perceptron"; param: PerceptronParam };

export interface WasmSendMsg {
    start_training: StartTrainingData;
    stop_training: null;
    step: any;
}
export type WasmHandleMsg =
    | { cmd: "start_training"; data: WasmSendMsg["start_training"] }
    | { cmd: "stop_training"; data: WasmSendMsg["stop_training"] }
    | { cmd: "step"; data: WasmSendMsg["step"] };

export interface WorkerPipe {
    sendCommand<K extends keyof WasmSendMsg>(
        cmd: K,
        data: WasmSendMsg[K]
    ): void;
    handleCommand(cb: (msg: WasmHandleMsg) => void): void;
}

export function get_worker_pipe(worker: Worker | Window): WorkerPipe {
    return {
        sendCommand<K extends keyof WasmSendMsg>(cmd: K, data: WasmSendMsg[K]) {
            worker.postMessage({ cmd, data });
        },
        handleCommand(cb: ({ cmd, data }: WasmHandleMsg) => void) {
            worker.onmessage = (e) => cb(e.data);
        },
    };
}
