export interface ReqMsg {
    // from main
    init_model: ModelParametersUnion;
    free_model: null;
    start_training: null;
    stop_training: null;
    evaluate: null;
    predict: Float64Array;
    // from worker
    step: { i: number; err: number };
}

export interface AckMsg {
    // to main
    init_model: void;
    free_model: void;
    start_training: void;
    stop_training: void;
    evaluate: number;
    predict: Float64Array;
    // to worker
    step: void;
}

/* Helper Types */

/** */
type AsUnion<T extends {}> = {
    [K in keyof T]: { key: K; value: T[K] };
}[keyof T];

/** Filter the key of a key value type */
type FilterKey<T extends {}, F> = {
    [K in keyof T as K extends F ? K : never]: T[K];
};

type StripCmd<T> = T extends `${string}:${infer Cmd}` ? Cmd : never;

type Callbacks<R extends {}, A extends { [A in keyof R]: unknown }> = {
    [K in keyof R]: (value: R[K]) => A[K];
};
