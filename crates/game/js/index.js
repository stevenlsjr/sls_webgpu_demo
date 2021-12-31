import('../pkg/index').then(async (wasm) => {
  window.WASM = wasm;
  console.log(wasm)
})

console.log('hello!')