import { spawn } from "node:child_process";

const isProd = (process.env.BUILD_ENV || "dev").toLowerCase() === "production";
const target = "wasm32-unknown-unknown";

const wasmPath = `../../target/wasm32-unknown-unknown/${isProd? 'release' : 'debug'}/sls_webgpu_html5.wasm`
function spawnScript(shellCommand) {
  console.log("$ %s", shellCommand);
  return new Promise((res, rej) => {
    const pid = spawn(shellCommand, { shell: true, stdio: "inherit" });
    pid.on("exit", (code) => {
      if (code === 0) {
        res();
      } else {
        rej(new Error(`exit code ${code}`));
      }
    });
  });
}
await spawnScript(`cargo build --target ${target}` + (isProd ? "--release" : ""));
await spawnScript(`wasm-bindgen --out-dir pkg --target web ${wasmPath}`)