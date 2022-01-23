import './style.css'
import wasm from 'sls-webgpu-game/sls_webgpu_game_bg.wasm?url'
import init, * as wasmModule from 'sls-webgpu-game';

const app = document.querySelector<HTMLDivElement>('#app')!;

init(wasm).then(() => {
    (window as any).WASM = wasmModule;
    const game = new wasmModule.GameRef();
    console.log(game, game.toString());
    (window as any).GAME = game;
})

app.innerHTML = `
  <h1>Hello Vite!!!!</h1>
  <a href="https://vitejs.dev/guide/features.html" target="_blank">Documentation</a>
`
