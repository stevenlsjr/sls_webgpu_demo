/**
 * UI controller for webgl/gpu demo
 */

/**
 * @property {HTMLElement} appRoot
 * @property {Set<string>} features
 * @property {HTMLSelectElement | null} $select
 * @property { (chosenOption: string | undefined  ,event: InputEvent)=>void } _onSelectCallback
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
        this.features = options.features || new Set(["opengl_renderer"]);
        this._onSelectCallback = null;
        this.$select = null
    }

    onSelect(cb){

        this._onSelectCallback = cb;

    }

    get currentBackend(){
        const url = new URL(location.href);
        return url.searchParams.get('backend') || 'opengl_renderer'
    }

    render() {
        let $select = this.appRoot.querySelector('#backend-select')
        $select.innerHTML = "";
        this.$select = $select;
        for (const feature of this.features) {
            const elt = document.createElement("option")
            elt.value = feature
            elt.innerText = feature
            $select.appendChild(elt)
        }

    }


}