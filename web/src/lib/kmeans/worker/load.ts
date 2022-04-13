import KMeansWorker from "./kmeans.worker?worker";
import { CommandDto, Command } from "./dto";

const callbacks: Record<number, Array<(value?: any) => any>> = {};
const kmeans_worker = new KMeansWorker();
kmeans_worker.onmessage = function (msg: MessageEvent<CommandDto>) {
    callbacks[msg.data.command].forEach((resolve) => resolve(msg.data.data));
};

function Request(cmd: Command) {
    return function (
        target: any,
        propertyKey: string,
        descriptor: PropertyDescriptor
    ) {
        const original = descriptor.value;
        descriptor.value = async function (...args: any[]) {
            return new Promise((res, rej) => {
                kmeans_worker.postMessage({ command: cmd, data: args });
                if (callbacks[cmd] == undefined) {
                    callbacks[cmd] = [];
                }
                callbacks[cmd].push(res);
            });
        };
    };
}

export class Kmeans {
    @Request(Command.init_wasm)
    async init() {}

    @Request(Command.new_clusters)
    async new(k: number) {}

    @Request(Command.load_dataset)
    async load_dataset(data: Uint8Array, label: Uint8Array) {}

    @Request(Command.step)
    async step(): Promise<any> {}

    @Request(Command.info)
    async info(): Promise<any> {}

    @Request(Command.free)
    async free(): Promise<void> {}

    @Request(Command.test_one)
    async test_one_rgba(rgba: Uint8Array) {}
}
