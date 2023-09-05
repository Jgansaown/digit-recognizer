export type StartTrainingData =
    | { type: "kmeans"; param: KMeansParam }
    | { type: "knn"; param: KNearestNeighborsParam }
    | { type: "perceptron"; param: PerceptronParam };

export interface WasmSendMsg {
    start_training: StartTrainingData;
    stop_training: null;
    step: any;
    predict: Float64Array;
    prediction: Float64Array;
}
export type WasmHandleMsg =
    | { cmd: "start_training"; data: WasmSendMsg["start_training"] }
    | { cmd: "stop_training"; data: WasmSendMsg["stop_training"] }
    | { cmd: "step"; data: WasmSendMsg["step"] }
    | { cmd: "predict"; data: WasmSendMsg["predict"] }
    | { cmd: "prediction"; data: WasmSendMsg["prediction"] };

export interface WorkerPipe {
    sendCommand<K extends keyof WasmSendMsg>(
        cmd: K,
        data: WasmSendMsg[K]
    ): void;
    handleCommand(cb: (msg: WasmHandleMsg) => void): void;
    sendCommandTransfer<K extends keyof WasmSendMsg>(
        cmd: K,
        data: WasmSendMsg[K]
    ): void;
}

export function get_worker_pipe(worker: Worker | DedicatedWorkerGlobalScope ): WorkerPipe {
    return {
        sendCommand<K extends keyof WasmSendMsg>(cmd: K, data: WasmSendMsg[K]) {
            worker.postMessage({ cmd, data });
        },
        handleCommand(cb: ({ cmd, data }: WasmHandleMsg) => void) {
            worker.onmessage = (e) => cb(e.data);
        },
        sendCommandTransfer<K extends keyof WasmSendMsg>(
            cmd: K,
            data: Float64Array
        ) {
            worker.postMessage({ cmd, data }, [data.buffer]);
        },
    };
}
