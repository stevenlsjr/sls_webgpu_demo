import {createWgpuContext, webGpuIsAvailable} from "./wgpu";
import DemoUI from './ui'

const loadWasm = import("../pkg/index.js").catch(console.error);

document.addEventListener("DOMContentLoaded", async () => {
    let wasmAppRoot = document.querySelector('#wgpu-app-root');
    let uiRoot = document.querySelector('#wgpu-ui-root');

    const module = await loadWasm
    window.SLS_WASM_BINDGEN = module;
    const {SlsWgpuDemo, features} = module;

    /** @type {Set<string>} */
     const APP_FEATURES = new Set(features());
    const ui = new DemoUI({appRoot: uiRoot, features: APP_FEATURES})
    ui.render();
    window.SLS_APP_FEATURES = APP_FEATURES;
    await startApp({module, features: APP_FEATURES, wasmAppRoot});



})

async function startApp({module, features, wasmAppRoot}){
    const {SlsWgpuDemo} = module;

    const isBuiltWithWgpu = features.has("wgpu_renderer")

    if (isBuiltWithWgpu && webGpuIsAvailable()) {
        window.GPU_API = await createWgpuContext({appRoot: wasmAppRoot});
    } else {
        console.log(`app ${ isBuiltWithWgpu? "is": "is not" } built with webgpu support`);
        console.log(`browser ${webGpuIsAvailable()? "does": "does not"} support webgpu`)
    }
    let app = new SlsWgpuDemo(wasmAppRoot);
    app.on('keyup', (event) => {
        console.log("key up: ", event);

    });

    app.on('keydown', (event) => {
        console.log("key down: ", event);

    });
    window.$app = app;
    app.run();
}