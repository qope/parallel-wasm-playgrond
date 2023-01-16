import React, { useEffect, useState } from "react";
import "./App.css";
import { wrap } from "comlink";
import { threads } from "wasm-feature-detect";

function App() {
  const [result, setResult] = useState("");
  const [time, setTime] = useState(0);
  const [isLoading, setIsLoading] = useState(false);
  const [numThreads, setNumThreads] = useState(1);
  const [wasmThreadSupport, setWasmThreadSupport] = useState(false);

  const worker = new Worker(new URL("./wasm-worker", import.meta.url), {
    name: "wasm-worker",
    type: "module",
  });
  const workerApi = wrap<import("./wasm-worker").WasmWorker>(worker);

  useEffect(() => {
    (async () => {
      setWasmThreadSupport(await threads());
    })();
  }, []);

  const doFunc = (f: any) => {
    (async () => {
      setResult("");
      setTime(0);
      setIsLoading(true);
      const start = performance.now();
      const r = await f();
      const end = performance.now();
      setResult(r);
      setTime(Math.round(end - start));
      setIsLoading(false);
    })();
  };

  return (
    <div className="App">
      <p>Cross origin isolation: {crossOriginIsolated.toString()}</p>
      <p>Wasm threads support: {wasmThreadSupport.toString()}</p>
      <p>
        Number of threads{" "}
        <select onChange={(e) => setNumThreads(Number(e.target.value))}>
          {Array.from(Array(navigator.hardwareConcurrency).keys()).map(
            (_, i) => (
              <option
                key={i + 1}
                value={i + 1}
                selected={i + 1 === navigator.hardwareConcurrency}
              >
                {i + 1}
              </option>
            )
          )}
        </select>
        / {navigator.hardwareConcurrency}
      </p>
      <p>
        <button
          onClick={() => doFunc(() => workerApi.zkHash(numThreads))}
          disabled={isLoading}
        >
          zk hash
        </button>
      </p>
      <form>
        <input type="text" value={result} readOnly></input>
      </form>
      <p>time: {time} ms</p>
    </div>
  );
}

export default App;
