import init, {
    load_mnist_data,
    KMeansClusters,
    Dataset,
    rgba_image_to_grayscale_image,
    data_as_png_base64_string,
    init_panic_hook,
} from "@wasm/kmeans";

import { CommandDto, Command } from "./dto";

const handler: Record<any, any> = {};
const asyncHandler: Record<any, any> = {};

function Process(cmd: Command) {
    return function (
        target: any,
        propertyKey: string,
        descriptor: PropertyDescriptor
    ) {
        const original = descriptor.value;
        descriptor.value = function (...args: any[]) {
            const result = original.call(this, ...args);
            self.postMessage({ command: cmd, data: result });
        };
        handler[cmd] = descriptor.value;
    };
}
function AsyncProcess(cmd: Command) {
    return function (
        target: any,
        propertyKey: string,
        descriptor: PropertyDescriptor
    ) {
        const original = descriptor.value;
        descriptor.value = async function (...args: any[]) {
            const result = await original.call(this, ...args);
            self.postMessage({ command: cmd, data: result });
        };
        asyncHandler[cmd] = descriptor.value;
    };
}

/* K Means Cluster */
class KMeans {
    private clusters: KMeansClusters;
    private dataset: Dataset;

    async handle_request(dto: CommandDto) {
        if (dto.command in handler) {
            handler[dto.command].call(this, ...dto.data);
        }
        if (dto.command in asyncHandler) {
            await asyncHandler[dto.command].call(this, ...dto.data);
        }
    }

    @AsyncProcess(Command.init_wasm)
    private async init() {
        await init();
        init_panic_hook();
        console.log("loaded kmean wasm");
    }

    @Process(Command.new_clusters)
    private create_clusters(k: number) {
        this.clusters = KMeansClusters.random(k);
    }

    @Process(Command.load_dataset)
    private load_dataset(data: Uint8Array, label: Uint8Array) {
        this.dataset = load_mnist_data(data, label);
    }

    @Process(Command.step)
    private step(): number {
        this.clusters.assign_dataset(this.dataset);
        return this.clusters.recalculate_centroids();
    }

    @Process(Command.info)
    private info() {
        return this.clusters.get_info();
    }

    @Process(Command.free)
    private free() {
        if (this.dataset != undefined) {
            this.dataset.free();
        }
        if (this.clusters != undefined) {
            this.clusters.free();
        }
    }

    @Process(Command.test_one)
    private test() {
        return this.clusters.test_dataset_js(this.dataset);
    }
}

let kmeans = new KMeans();
self.onmessage = function (msg: MessageEvent<CommandDto>) {
    kmeans.handle_request(msg.data);
};
