import { webgpuTsDemo } from "./webgpu-main"

async function main(){
  const root = document.querySelector('#app')
  if (!root){
    throw new Error("App root not found")
  }

  const canvas = root.querySelector<HTMLCanvasElement>('canvas#render-target')
  const messages = root.querySelector<HTMLDivElement>('#messages')
  if (!canvas || !messages){
    throw new Error("Canvas not found")
  }
  
  
  webgpuTsDemo({canvas, messageElt: messages})
}

main()