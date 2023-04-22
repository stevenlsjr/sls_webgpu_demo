const { series, parallel, watch } = require("gulp");
const path = require("path");
const { spawn } = require("child_process");
const workspaceDir = path.resolve(__dirname, "../..");

const wasmPackDirs = [path.resolve(workspaceDir, "crates/game")];

function createWasmPack({ isDev, cratePath }) {
  return function () {
    console.log(`running wasm-pack in ${isDev ? "dev" : "production"} mode`);
    return spawn(
      "wasm-pack",
      ["build", cratePath, "-t", "web"].concat(isDev ? ["--dev"] : []),
      { stdio: "inherit" }
    );
  };
}

const buildSlsWgpuHtml5 = createWasmPack({
  isDev: false,
  cratePath: "../../crates/sls-webgpu-html5",
});
const buildSlsWgpuHtml5Dev = createWasmPack({
  isDev: true,
  cratePath: "../../crates/sls-webgpu-html5",
});

const watchRust = series(buildSlsWgpuHtml5Dev, function () {
  return watch("../../crates/sls-webgpu-html5/**/*.rs", buildSlsWgpuHtml5Dev);
});
function viteDev() {
  return spawn("yarn", ["dev"], {
    stdio: "inherit",
  });
}

const dev = parallel(viteDev, watchRust);
module.exports = {
  watchRust,
  dev,
  // default: build
};
