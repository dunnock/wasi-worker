import { IFs } from "memfs";
import { File } from "memfs/lib/node";
import WasmFs from "@wasmer/wasmfs";
import { Volume, IReadStream } from "memfs/lib/volume";


export class WorkerFS {
  wasmFs: WasmFs;
  stdin: BufferedStdin;
  stdout: PipedWriter;
  stderr: PipedWriter;
  output: PipedWriter;

  constructor() {
    this.wasmFs = new WasmFs();

    this.stdin = new BufferedStdin();
    this.stdin.bindToFd(this.wasmFs.volume.fds[0]);

    this.stdout = new PipedWriter(this.wasmFs.volume.fds[1]);

    this.stderr = new PipedWriter(this.wasmFs.volume.fds[2]);
    this.stderr.mapStrFn((str) => console.error("worker error>" + str))

    let outgoingFd = this.wasmFs.fs.openSync("/output.bin", "w+");
    this.output = new PipedWriter(this.wasmFs.volume.fds[outgoingFd]);
    // @ts-ignore
    //console.log(this.wasmFs.volume.getFileByFd(outgoingFd));
  }

  getFs(): IFs {
    return this.wasmFs.fs;
  }
}


class PipedWriter {
  fd: File;
  binFn?: (buffer: Uint8Array) => void;
  strFn?: (msg: String) => void;
  writes = 0;

  constructor(fd: File) {
    this.fd = fd;
    this.fd.node.write = this.write;
  }

  write = (
    stdoutBuffer: Buffer | Uint8Array,
    offset: number = 0,
    length: number = stdoutBuffer.byteLength,
    position?: number
  ) => {
    this.writes ++;
    if (this.binFn) {
      this.binFn(stdoutBuffer);
      return stdoutBuffer.length;
    }

    let dataString = new TextDecoder("utf-8").decode(stdoutBuffer);

    if(this.strFn) {
      this.strFn(dataString);
    } else {
      console.log(dataString);
    }

    // Record all of our stdout to show in the prompt

    return stdoutBuffer.length;
  }

  mapBinFn(fn: (buffer: Uint8Array) => void) {
    this.binFn = fn;
  }

  mapStrFn(fn: (msg: String) => void) {
    this.strFn = fn;
  }
}

class BufferedStdin {
  messages: Array<Uint8Array>;
  lastPosition: number;

  constructor() {
    this.messages = new Array;
    this.lastPosition = 0;
  }

  bindToFd(stdin_fd: File) {
    stdin_fd.node.read = this.read;
  }

  push(message: Uint8Array) {
    this.messages.push(message);
  }

  read = (
    stdinBuffer: Buffer | Uint8Array,
    offset: number = 0,
    length: number = stdinBuffer.byteLength,
    position?: number
  ) => {
    if (this.messages.length === 0) {
      return 0;
    } else if (position && position > 0 && position != this.lastPosition) {
      this.error("BufferedStdin read on position not supported: " + position)
    }
    let message = this.messages.shift();
    if (message && message.length < length) {
      stdinBuffer.set(message);
    } else if (message) {
      this.error("Message does not fit passed stdin.read buffer: " + message.length)
    } else {
      return 0;
    }
    this.lastPosition += message.length;
    return message.length;
  }

  error(message: String) {
    let err = new Error("BufferedStdin error: " + message);
    console.error(err);
    throw err
  }
}
