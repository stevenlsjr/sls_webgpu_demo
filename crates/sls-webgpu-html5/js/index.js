const loadWasm = import("../pkg/index.js").catch(console.error);

document.addEventListener("DOMContentLoaded", async () => {
    console.log('hello')

    window.sc_internal = await window.sc_internal_wrapper();
    const module = await loadWasm
    window.SLS_WASM_BINDGEN = module;
    const {SlsWgpuDemo} = module;
    let wasmApp = document.querySelector('#wgpu-app-root');
    let app = new SlsWgpuDemo(wasmApp);
    app.on('keyup', (event) => {
        console.log("key up: ", event);

    });

    app.on('keydown', (event) => {
        console.log("key down: ", event);

    });
    window.$app = app;
    app.run();

})


