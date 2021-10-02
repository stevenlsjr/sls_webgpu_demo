const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");
process.env.RUSTFLAGS = '--cfg=web_sys_unstable_apis'

module.exports = {
    mode: "production",
    entry: {
        index: "./js/index.js"
    },
    output: {
        path: dist,
        filename: "[name].js"
    },
    devServer: {
        contentBase: dist,
    },
    resolve: {
        alias: {
            env$: path.resolve(__dirname, './js/env-shim.js')
        }
    },
    plugins: [
        new CopyPlugin([
            path.resolve(__dirname, "static")
        ]),

        new WasmPackPlugin({
            crateDirectory: __dirname,
            watchDirectories: [
                path.resolve(__dirname, '../../src')
            ],
            extraArgs: '-- --features opengl_renderer --no-default-features'
        }),
    ],
    devtool: 'eval'
};
