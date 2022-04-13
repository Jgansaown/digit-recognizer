export enum Command {
    init_wasm,
    new_clusters,
    load_dataset,
    step,
    info,
    free,
    test_one,
}

export interface CommandDto {
    command: Command;
    data?: any[];
    // init?: [k: number, data: Uint8Array, label: Uint8Array];
    // dist?: number;
    // info?: any[];
    // image?: Uint8Array;
    // label?: number;
}