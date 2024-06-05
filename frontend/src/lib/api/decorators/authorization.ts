import type {ResponseResult} from "$lib/api";
import {AsyncLock} from "$lib/utils/async";
import {authorize} from "$lib/components/modals/authControl.svelte";

const authorizedLock = new AsyncLock()

export function ensureAuthorized<O, E, A extends any[]>(fn: (...args: A) => Promise<ResponseResult<O, E>>) {
    return async (...args: A) => {
        let result = await fn(...args)
        if (result.status !== 401) {
            return result
        }

        await authorizedLock.enter()

        result = await fn(...args)
        if (result.status === 401) {
            await authorize()
            result = await fn(...args)
        }

        authorizedLock.disable()

        return result
    }
}
