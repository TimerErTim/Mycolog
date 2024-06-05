let authModalControl: ReturnType<typeof makeAuthModalControl>

function makeAuthModalControl() {
    let open = $state(false)
    return {
        _callbacks: [] as (() => void)[],
        close() {
            open = false
        },
        show() {
            open = true
        },
        get open() {
            return open
        },
        authorize(callback?: () => void) {
            this.show()
            callback && this._callbacks.push(callback)
        },
        success() {
            for (const callback of this._callbacks) {
                callback()
            }
            this._callbacks.length = 0
            this.close()
        }
    }
}

$effect.root(() => {
    authModalControl = makeAuthModalControl()
})

export function authorize(): Promise<void> {
    return new Promise((resolve) => authModalControl.authorize(() => resolve(undefined)))
}

export function success() {
    authModalControl.success()
}

export function isOpen() {
    return authModalControl.open
}
