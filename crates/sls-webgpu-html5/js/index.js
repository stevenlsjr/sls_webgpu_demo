import("../pkg/index.js").catch(console.error).then(module => {
    window.SLS_WASM_BINDGEN = module;
    const {SlsWgpuDemo} = module;
    let app = new SlsWgpuDemo();
    app = app.on('handle-input', () => {

    });
    window.$app = app;
    app.run();

});
