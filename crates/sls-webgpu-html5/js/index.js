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
    /** @type {null | import("../pkg").SlsWgpuApp} */
    let app = null;
    startApp({module, features: APP_FEATURES, wasmAppRoot, backend: ui.currentBackend}).catch((e) => {
        console.error('app could not start: ', e);
    })

    ui.render();
    //


})

/**
 *
 * @param {import('../pkg')} module
 * @param features
 * @param wasmAppRoot
 * @param backend
 * @returns {Promise<import('../pkg).SlsWgpuDemo>}
 */
async function startApp({module, features, wasmAppRoot, backend}) {
    const {SlsWgpuDemo} = module;

    backend = backend || 'opengl_renderer'
    wasmAppRoot.innerHTML = ''

    const isBuiltWithWgpu = features.has("wgpu_renderer")
    if (backend === 'wgpu_renderer') {
        if (isBuiltWithWgpu && webGpuIsAvailable()) {
            window.GPU_API = await createWgpuContext({appRoot: wasmAppRoot});
        } else {
            console.log(`app ${isBuiltWithWgpu ? "is" : "is not"} built with webgpu support`);
            console.log(`browser ${webGpuIsAvailable() ? "does" : "does not"} support webgpu`)
        }
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
    return app;
}