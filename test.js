import init, { json_to_v8 } from "./serde_to_v8.js";

await init(await Deno.readFile("./serde_to_v8_bg.wasm"));

const json = Deno.args[0];

function wasmParseJSON(json) {
    return json_to_v8(json);
}

function bench(fn, ...args) {
    let max = 0;
    let baseline = 0;
    let iters = 1e3;
    const start = performance.now();
    for (let i = 0; i < iters; i++) {
        const now = performance.now();
        fn(...args);
        const took = performance.now() - now;
        if (took < baseline || baseline == 0) baseline = took;
        if (took > max) max = took;
    }
    const total = performance.now() - start;
    const itersPerMs = iters / total;
    return `total ${total.toFixed(4)}ms ${itersPerMs.toFixed(4)} ops/ms baseline ${baseline.toFixed(4)}ms ±${(max - baseline).toFixed(4)}ms`;
}

console.log("JSON.parse", bench(JSON.parse, json));
console.log("wasmParseJSON", bench(wasmParseJSON, json));
