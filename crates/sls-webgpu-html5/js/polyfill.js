function shimCanvas() {
    const old = {
        getContext: HTMLCanvasElement.prototype.getContext
    }

    /**
     * overrides getContext, since the current version of webgpu uses
     * the deprecated "gpupresent", instead of "webgpu"
     *
     * @param name
     * @param options
     * @returns {*}
     */
    HTMLCanvasElement.prototype.getContext = function (name, options) {
        if (name === 'gpupresent') {
            name = 'webgpu';
        }
        return old.getContext.apply(this, [name, options])
    }

}

shimCanvas();