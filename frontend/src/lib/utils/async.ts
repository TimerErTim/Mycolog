const DEF_DELAY = 1000;

export function sleep(ms: number) {
    return new Promise(resolve => setTimeout(resolve, Number.isNaN(Number(ms)) ? DEF_DELAY : Number(ms)));
}

export class AsyncLock {
    private promise: Promise<void>

    constructor() {
        this._disable = () => {
        }
        this.promise = Promise.resolve()
    }

    private _disable: () => void

    get disable() {
        return this._disable
    }

    async enter() {
        await this.promise
        this.promise = new Promise(resolve => this._disable = resolve)
    }
}
