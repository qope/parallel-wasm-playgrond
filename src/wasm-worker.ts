import { expose } from "comlink";

async function zkHash(numThread: number): Promise<string> {
  const multiThread = await import("wasm-lib");
  await multiThread.default();
  await multiThread.initThreadPool(numThread);
  const ret = multiThread.zk_hash();
  return ret;
}

const exports = {
  zkHash,
};
export type WasmWorker = typeof exports;

expose(exports);
