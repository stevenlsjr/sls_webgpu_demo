import './polyfill'

const loadWasm = import("../pkg/index.js").catch(console.error);
/** @type {import("../pkg/index.js")} */
let slsWasmDemo = undefined;

document.addEventListener("DOMContentLoaded", async () => {
    let wasmAppRoot = document.querySelector("#wgpu-app-root");
    let uiRoot = document.querySelector("#wgpu-ui-root");
    window.SLS_WASM_BINDGEN = await loadWasm;

    await startWgpuApp(wasmAppRoot)
});
const BACKEND_WEBGL = 'WEBGL'
const BACKEND_WEBGPU = 'WEBGPU'
const VALID_BACKENDS = {
    [BACKEND_WEBGL]: "WebGL",
    [BACKEND_WEBGPU]: "WebGPU"
};

/**
 *
 * @param wasmAppRoot
 * @returns {Promise<void>}
 */
async function startWgpuApp(wasmAppRoot) {
    const {SlsWgpuDemo} = await loadWasm;
    const url = new URL(location.href);
    let backendParam = (url.searchParams.get('backend') || BACKEND_WEBGL).toUpperCase();
    let renderer = VALID_BACKENDS[backendParam];
    if (!renderer){
        throw new Error(`backend name ${backendParam} is invalid`)
    }

    const app = new SlsWgpuDemo(wasmAppRoot, {renderer});
    window.$app = app;
    await app.run();
}
