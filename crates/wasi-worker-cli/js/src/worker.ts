import WASI from  "@wasmer/wasi";
import wasiBindings from "@wasmer/wasi/lib/bindings/browser";
import { lowerI64Imports } from "@wasmer/wasm-transformer";
import { WorkerFS } from './fs';

declare var self: Worker;

const workerUrl = "worker.wasm"; //"wabench_app_wasi.wasm"; //

let iamWorker = self;
let instance: any = null;

const workerFs = new WorkerFS();

let wasi = new WASI({
  preopenDirectories: {
    "/": "/"
  },
  args: [],
  env: {},
  bindings: {
    // @ts-ignore
    ...wasiBindings.default,
    fs: workerFs.getFs()
  }
});


const fetchAndTransformWasmBinary = async (url: string) => {
  // Get the original Wasm binary
  const fetchedOriginalWasmBinary = await fetch(url);
  const originalWasmBinaryBuffer = await fetchedOriginalWasmBinary.arrayBuffer();
  const originalWasmBinary = new Uint8Array(originalWasmBinaryBuffer);

  // Initialize our wasm-transformer
  //  await wasmTransformerInit(wasmTransformerUrl);
  // IMPORTANT: This URL points to wherever the wasm-transformer.wasm is hosted

  // Transform the binary, by running the lower_i64_imports from the wasm-transformer
  const transformedBinary = await lowerI64Imports(originalWasmBinary);

  // Compile the transformed binary
  const transformedWasmModule = await WebAssembly.compile(transformedBinary);
  return transformedWasmModule;
};

const startWasiTask = async (file: string) => {
  try {
    // Fetch our Wasm File
    const module = await fetchAndTransformWasmBinary(file);
    console.log("Module transformed and compiled, starting...");

    // Instantiate the WebAssembly file
    instance = await WebAssembly.instantiate(module, {
      wasi_snapshot_preview1: wasi.wasiImport
    });

    // Start the WebAssembly WASI instance!
    wasi.start(instance);
    console.log("worker has started");

    // @ts-ignore
    //workerFs.stdout.fd.write(Uint8Array.from([1,2,3]));

    //stdin.push(Uint8Array.from([1,2,3,4,5,6]));
    //console.log(instance.exports.same(124));

  } catch (e) {
    console.error(e);
    console.error(e.stack);
  }
};


workerFs.output.mapBinFn((buffer: Uint8Array) => {
  console.log("Worker outgoing> " + buffer);
  if (typeof iamWorker.postMessage === "function") {
    iamWorker.postMessage(Array.from(buffer));
  }
})

iamWorker.onmessage = function(event) {
  console.log("Worker incoming> "+ event.data);
  workerFs.stdin.push(event.data);
  console.log(instance.exports.message_ready());
};

startWasiTask(workerUrl);