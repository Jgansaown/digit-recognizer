import * as Comlink from "comlink";

import type { unpack as unpack_worker } from "./workers/unpack.worker";

const unpack = Comlink.wrap<typeof unpack_worker>(
    new Worker(new URL("./workers/unpack.worker.ts", import.meta.url), {
        type: "module",
    })
);

const URLS = {
    training: {
        data: "https://raw.githubusercontent.com/Jgansaown/rust-digit-recognition/main/files/mnist-training-data.gz",
        label: "https://raw.githubusercontent.com/Jgansaown/rust-digit-recognition/main/files/mnist-training-label.gz",
        tar_gz: "https://raw.githubusercontent.com/Jgansaown/rust-digit-recognition/main/files/mnist-training.tar.gz",
    },
    testing: {
        data: "https://raw.githubusercontent.com/Jgansaown/rust-digit-recognition/main/files/mnist-test-data.gz",
        label: "https://raw.githubusercontent.com/Jgansaown/rust-digit-recognition/main/files/mnist-test-label.gz",
        tar_gz: "https://raw.githubusercontent.com/Jgansaown/rust-digit-recognition/main/files/mnist-testing.tar.gz",
    },
};

export interface jsDataset {
    data: Uint8Array;
    label: Uint8Array;
}

export interface MnistDataset {
    training: jsDataset;
    testing: jsDataset;
}

async function* streamAsyncIterable(stream: ReadableStream) {
    const reader = stream.getReader();
    try {
        while (true) {
            const { done, value } = await reader.read();
            if (done) return;
            yield value;
        }
    } finally {
        reader.releaseLock();
    }
}

async function fetch_file_progress(url: string) {
    const filename = url.split("/").at(-1);
    const response = await fetch(url);

    let receivedLength = 0;
    const contentLength = +response.headers.get("Content-Length");

    const chunks: Uint8Array[] = [];
    for await (const chunk of streamAsyncIterable(response.body)) {
        chunks.push(chunk);
        // progress
        receivedLength += chunk.length;
        const progress = (receivedLength / contentLength) * 100;
        console.debug(`Downloading: ${filename}, ${progress.toFixed(2)}%`);
    }
    const blob = new Blob(chunks);
    return new Uint8Array(await blob.arrayBuffer());
}

async function fetch_file(url: string): Promise<Uint8Array> {
    return fetch_file_progress(url);
    // let resp = await fetch(url);
    // let blob = await resp.blob();
    // let buf = await blob.arrayBuffer();
    // return new Uint8Array(buf);
}

export async function load_dataset(
    type: "training" | "testing"
): Promise<jsDataset> {
    const gzFiles = await Promise.all([
        fetch_file(URLS[type].data),
        fetch_file(URLS[type].label),
    ]);
    const files = await Promise.all([
        unpack.gz(gzFiles[0]),
        unpack.gz(gzFiles[1]),
    ]);
    return {
        data: files[0],
        label: files[1],
    };
}

export async function load_all(): Promise<MnistDataset> {
    const files = await Promise.all([
        load_dataset("training"),
        load_dataset("testing"),
    ]);
    return {
        training: files[0],
        testing: files[1],
    };
}
