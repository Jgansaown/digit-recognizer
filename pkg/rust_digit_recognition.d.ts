/* tslint:disable */
/* eslint-disable */
/**
* @param {Uint8Array} data
* @param {Uint8Array} labels
* @returns {string}
*/
export function load_mnist_data(data: Uint8Array, labels: Uint8Array): string;
/**
* @param {Uint8Array} data
* @param {Uint8Array} labels
* @param {number} n
* @returns {Uint8Array}
*/
export function get_nth_image(data: Uint8Array, labels: Uint8Array, n: number): Uint8Array;
/**
* @param {Uint8Array} data
* @returns {string}
*/
export function as_png_base64_string(data: Uint8Array): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly load_mnist_data: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly get_nth_image: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly as_png_base64_string: (a: number, b: number, c: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
