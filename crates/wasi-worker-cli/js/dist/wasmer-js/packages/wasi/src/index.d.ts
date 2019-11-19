import { BigIntPolyfillType } from "./polyfills/bigint";
import { DataViewPolyfillType } from "./polyfills/dataview";
interface Rights {
    base: BigIntPolyfillType;
    inheriting: BigIntPolyfillType;
}
interface File {
    real: number;
    offset?: bigint;
    filetype?: any;
    rights: Rights;
    path?: any;
    fakePath?: any;
}
declare type Exports = {
    [key: string]: any;
};
export declare type WASIBindings = {
    hrtime: () => bigint;
    exit: (rval: number) => void;
    kill: (signal: string) => void;
    randomFillSync: <T>(buffer: T, offset: number, size: number) => T;
    isTTY: (fd: number) => boolean;
    fs: any;
    path: any;
};
export declare type WASIArgs = string[];
export declare type WASIEnv = {
    [key: string]: string | undefined;
};
export declare type WASIPreopenedDirs = {
    [key: string]: string;
};
export declare type WASIConfig = {
    preopenDirectories?: WASIPreopenedDirs;
    env?: WASIEnv;
    args?: WASIArgs;
    bindings?: WASIBindings;
};
export declare class WASIError extends Error {
    errno: number;
    constructor(errno: number);
}
export declare class WASIExitError extends Error {
    code: number | null;
    constructor(code: number | null);
}
export declare class WASIKillError extends Error {
    signal: string;
    constructor(signal: string);
}
export default class WASIDefault {
    memory: WebAssembly.Memory;
    view: DataViewPolyfillType;
    FD_MAP: Map<number, File>;
    wasiImport: Exports;
    bindings: WASIBindings;
    static defaultBindings: WASIBindings;
    constructor(wasiConfig?: WASIConfig);
    refreshMemory(): void;
    setMemory(memory: WebAssembly.Memory): void;
    start(instance: WebAssembly.Instance): void;
}
export declare const WASI: typeof WASIDefault;
export declare type WASI = WASIDefault;
export {};
