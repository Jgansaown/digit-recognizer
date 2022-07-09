import { expose } from "comlink";
import * as wasm from "@wasm/unpack";

async function unpack_gz(file: Uint8Array): Promise<Uint8Array> {
    await wasm.default();
    return wasm.unpack_gz(file);
}
async function unpack_tar_gz(
    file: Uint8Array
): Promise<Record<string, Uint8Array>> {
    await wasm.default();
    return wasm.unpack_tar_gz(file);
}

const unpack = {
    gz: unpack_gz,
    tar_gz: unpack_tar_gz,
};

expose(unpack);

export type { unpack };