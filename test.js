import init, { json_to_v8 } from "./pkg/serde_to_v8.js";

await init(await Deno.readFile("./pkg/serde_to_v8_bg.wasm"));

const json = Deno.args[0];

function wasmParseJSON(json) {
    const bin = json_to_v8(json);
    return Deno.core.deserialize(bin);
}

const expected = JSON.parse(json);
const bin = json_to_v8(json);

let actual; 
try { actual = Deno.core.deserialize(bin); } catch(e) { actual = e.message; }

console.log("expected:", expected);
console.log("actual:", actual);
console.log("expected bin:", Deno.core.serialize(expected));
console.log("actual bin:", bin);

function bench(fn, ...args) {
    let max = 0;
    let baseline = 0;
    let iters = 1e4;
    const start = performance.now();
    for (let i = 0; i < iters; i++) {
        const now = performance.now();
        fn(...args);
        const took = performance.now() - now;
        if (took < baseline || baseline == 0) baseline = took;
        if (took > max) max = took;
    }
    const end = performance.now() - start;
    const total = Math.abs(end - start);
    const itersPerMs = iters / total;

    return `total ${total.toFixed(4)}ms ${itersPerMs.toFixed(4)} iters/ms baseline ${baseline.toFixed(4)}ms ±${(max - baseline).toFixed(4)}ms`;
}

console.log("wasmParseJSON", bench(wasmParseJSON, json));
console.log("JSON.parse", bench(JSON.parse, json));
