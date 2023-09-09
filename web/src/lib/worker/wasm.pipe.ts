interface WorkerMessageInterface<T> {
    postMessage(message: T, options?: StructuredSerializeOptions): void;
    onmessage: ((this: any, ev: MessageEvent<T>) => any) | null;
}

interface ReqMessageData<T> {
    type: "req";
    id: string;
    key: keyof T;
    value: T[keyof T];
}
interface AckMessageData<A> {
    type: "ack";
    id: string;
    key: keyof A;
    value: A[keyof A] | null;
    err?: string;
}

type MessageData<R, A> = ReqMessageData<R> | AckMessageData<A>;

/**
 *
 */
class Pipe<
    W extends WorkerMessageInterface<MessageData<Req, Ack>>,
    Req extends {},
    Ack extends { [A in keyof Req]: unknown }
> {
    private worker: W;
    private count = 0;
    private resolvers: Map<
        string,
        {
            res: (value: any) => void;
            rej: (value: any) => void;
        }
    > = new Map();
    handlers: {
        [K in keyof Req]?: (value: Req[K]) => {
            value: Ack[K];
            transfer?: Transferable[];
        };
    } = {};

    constructor(worker: W) {
        this.worker = worker;
        this.worker.onmessage = ({ data }) => {
            if (data.type == "req") {
                this.handleReq(data);
            } else if (data.type == "ack") {
                this.handleAck(data);
            }
        };
    }

    /**
     * Sends a request to the other side, and returns a promise that will resolve
     * when the other side sends back an acknowledgement
     */
    public request<K extends keyof Req>(
        cmd: K,
        data: Req[K],
        transfer?: Transferable[]
    ): Promise<Ack[K]> {
        const id = join(this.count, cmd);
        this.count++; // increment request count

        this.worker.postMessage(
            { type: "req", id: id, key: cmd, value: data },
            { transfer }
        );
        return new Promise((res, rej) => {
            if (this.resolvers.has(id)) {
                rej(`Duplicate resolver id: ${id}`);
            } else {
                this.resolvers.set(id, { res, rej });
            }
        });
    }

    public handle<K extends keyof Req>(
        cmd: K,
        handler: (data: Req[K]) => {
            value: Ack[K];
            transfer?: Transferable[];
        }
    ) {
        this.handlers[cmd] = handler;
    }

    /**
     * Sends an acknowledgement back to the other side
     *
     * `id` is from request
     */
    private ack<K extends keyof Ack>(
        id: string,
        cmd: K,
        data: Ack[K],
        transfer?: Transferable[]
    ) {
        this.worker.postMessage(
            { type: "ack", id: id, key: cmd, value: data },
            { transfer }
        );
    }

    private handleReq({ id, key, value }: ReqMessageData<Req>) {
        const handler = this.handlers[key];
        if (handler != undefined) {
            const ret = handler(value);
            this.ack(id, key, ret.value, ret.transfer);
        } else {
            // handler not set
            this.worker.postMessage({
                type: "ack",
                id,
                key,
                value: null,
                err: "handler not set",
            });
        }
    }

    private handleAck({ id, key, value, err }: AckMessageData<Ack>) {
        const resolver = this.resolvers.get(id);
        if (resolver === undefined) {
            return;
        }
        if (err === undefined) {
            resolver.res.call(null, value);
        } else {
            resolver.rej.call(null, err);
        }
        this.resolvers.delete(id);
    }
}

function join(...v: any[]) {
    return v.map((v) => String(v)).join(".");
}

export { Pipe };
