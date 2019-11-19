/// <reference types="node" />
import { IFs } from "memfs";
import { File } from "memfs/lib/node";
import WasmFs from "@wasmer/wasmfs";
export declare class WorkerFS {
    wasmFs: WasmFs;
    stdin: BufferedStdin;
    stdout: PipedWriter;
    stderr: PipedWriter;
    output: PipedWriter;
    constructor();
    getFs(): IFs;
}
declare class PipedWriter {
    fd: File;
    binFn?: (buffer: Uint8Array) => void;
    strFn?: (msg: String) => void;
    writes: number;
    constructor(fd: File);
    write: (stdoutBuffer: Buffer | Uint8Array, offset?: number, length?: number, position?: number | undefined) => number;
    mapBinFn(fn: (buffer: Uint8Array) => void): void;
    mapStrFn(fn: (msg: String) => void): void;
}
declare class BufferedStdin {
    messages: Array<Uint8Array>;
    lastPosition: number;
    constructor();
    bindToFd(stdin_fd: File): void;
    push(message: Uint8Array): void;
    read: (stdinBuffer: Buffer | Uint8Array, offset?: number, length?: number, position?: number | undefined) => number;
    error(message: String): void;
}
export {};
