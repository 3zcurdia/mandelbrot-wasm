/* tslint:disable */
/* eslint-disable */
/**
* @param {number} width
* @param {number} height
* @param {number} max_iters
* @param {number} x_min
* @param {number} x_max
* @param {number} y_min
* @param {number} y_max
* @returns {Uint8Array}
*/
export function mandelbrot_pixels(width: number, height: number, max_iters: number, x_min: number, x_max: number, y_min: number, y_max: number): Uint8Array;
/**
* @param {number} width
* @param {number} height
* @param {number} max_iters
* @param {number} x_min
* @param {number} x_max
* @param {number} y_min
* @param {number} y_max
* @returns {Uint32Array}
*/
export function calculate_mandelbrot(width: number, height: number, max_iters: number, x_min: number, x_max: number, y_min: number, y_max: number): Uint32Array;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly mandelbrot_pixels: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => void;
  readonly calculate_mandelbrot: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
