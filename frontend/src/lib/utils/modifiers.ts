export function once<E extends Event>(fn: (e: E) => void) {
    let fun: typeof fn | null = fn
    return (e: E) => {
        if (fun !== null) fn.call(null, e)
        fun = null
    }
}

export function preventDefault<E extends Event>(fn: (e: E) => void) {
    return (e: E) => {
        e.preventDefault()
        fn.call(null, e)
    }
}