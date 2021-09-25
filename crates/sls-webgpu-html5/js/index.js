const loadWasm = import("../pkg/index.js").catch(console.error);
/** @type {import("../pkg/index.js")} */
let slsWasmDemo = undefined;

document.addEventListener("DOMContentLoaded", async () => {
    let wasmAppRoot = document.querySelector("#wgpu-app-root");
    let uiRoot = document.querySelector("#wgpu-ui-root");
    window.SLS_WASM_BINDGEN = await loadWasm;

    await startWgpuApp(wasmAppRoot)
});

/**
 *
 * @param wasmAppRoot
 * @returns {Promise<void>}
 */
async function startWgpuApp(wasmAppRoot) {
    const {SlsWgpuDemo} = await loadWasm;

    const app = new SlsWgpuDemo(wasmAppRoot, {renderer: "WebGPU"});
    window.$app = app;
    await app.run();
}
