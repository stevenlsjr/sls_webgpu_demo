/**
 * UI controller for webgl/gpu demo
 */

/**
 * @property {HTMLElement} appRoot
 * @property {Set<string>} features
 */
export default class DemoUI {
    /**
     *
     * @param {HTMLElement} options.appRoot
     * @param { Set<string> | undefined} options.features
     */
    constructor(options) {
        this.options = options;
        this.appRoot = options.appRoot;
        this.features = options.features || new Set(["opengl_renderer"])
    }

    render(){
        let $select = this.appRoot.querySelector('#backend-select')
        $select.innerHTML = "";
        for (const feature of this.features){
            const elt = document.createElement("option")
            elt.value = feature
            elt.innerText = feature
            $select.appendChild(elt)
        }
    }


}