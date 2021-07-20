/**
 *
 * @returns {boolean}
 */
export function webGpuIsAvailable(){
    return (navigator.gpu !== undefined) && (window.GPU !== undefined)
}

/**
 *
 * @param {HTMLElement} appRoot
 * @returns {Promise<void>}
 */
export async function createWgpuContext({appRoot}){
    console.log("creating webgpu ", window.GPU)
    const canvas = document.createElement('canvas');
    appRoot.appendChild(canvas);

    const adapter = await navigator.gpu.requestAdapter();
    if (!adapter){
        throw new Error("could not create webgpu adapter");
    }
    const device = await adapter.requestDevice();
    const context = canvas.getContext('gpupresent');
    const preferredFormat = context.getSwapChainPreferredFormat(adapter);
    return {canvas, device, context, adapter, preferredFormat}


}