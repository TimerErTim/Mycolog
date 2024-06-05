import type {ResponseResult} from "$lib/api";

export type FetchHandle<O, E, A extends any[]> = ReturnType<typeof useFetch<O, E, A>>

export function useFetch<O, E, A extends any[]>(fetch: (...args: A) => Promise<ResponseResult<O, E>>) {
    let loading = $state(false)
    let response = $state<O | undefined>(undefined)
    let error = $state<E | undefined>(undefined)

    return {
        get loading() {
            return loading
        },
        get response() {
            return response
        },
        get error() {
            return error
        },

        async run(...args: A) {
            loading = true
            const result = await fetch(...args)
            loading = false

            if (!!result.response) {
                response = result.response
                error = result.error
            } else if (result.error !== undefined) {
                response = undefined
                error = result.error
            }

            return result
        },

        reset() {
            loading = false
            response = undefined
            error = undefined
        }
    }
}