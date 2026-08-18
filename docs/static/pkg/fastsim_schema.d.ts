/* tslint:disable */
/* eslint-disable */

/**
 * Build a download URL for a vehicle id, defaulting to the GitHub raw-content
 * base URL for `fastsim-vehicles` when `base_url` is not provided.
 */
export function build_download_url(id: string, extension: string, base_url?: string | null): string;

/**
 * Parse `vehicles.jsonl` text (as fetched client-side) into index entries.
 *
 * This is the wasm-facing counterpart to `read_jsonl` — the
 * browser widget should fetch the raw file text and pass it here rather
 * than re-implementing line-splitting/JSON-parsing in JS.
 */
export function parse_vehicles_jsonl(text: string): any;

/**
 * Filter a JSON array of `IndexEntryV1` objects against a JSON-encoded
 * `Query`. Returns matching entries as a JS array, preserving original order.
 *
 * Typical usage: call `parse_vehicles_jsonl` once on page load, keep the
 * resulting entries in JS, and call this on every filter-input change with
 * `JSON.stringify(entries)` and the current query.
 */
export function search_entries(entries_json: string, query_json: string): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly build_download_url: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number, number, number];
    readonly parse_vehicles_jsonl: (a: number, b: number) => [number, number, number];
    readonly search_entries: (a: number, b: number, c: number, d: number) => [number, number, number];
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
