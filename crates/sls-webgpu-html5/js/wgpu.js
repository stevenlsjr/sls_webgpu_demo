/**
 *
 * @returns {boolean}
 */
export function webGpuIsAvailable(){
    return (navigator.gpu !== undefined) && (window.GPU !== undefined)
}
