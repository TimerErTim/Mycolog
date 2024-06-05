import {api} from "$lib/config";
import {objectToRecord} from "$lib/utils/conversion";
import {dev} from "$app/environment";

export type ResponseResult<O, E> = {
    status: number,
    response: O,
    error?: undefined
} | {
    status: number,
    response?: undefined,
    error: E
}

function baseBackendUrl() {
    return `${api.backendProtocol}://${api.backendHost}:${api.backendPort}${api.backendEndpoint}`
}

export async function fetchBackend(endpoint: string, options?: RequestInit & { params?: object }): Promise<Response> {
    const encodedParams = options?.params ?
        "?" + new URLSearchParams(objectToRecord(options.params)).toString() :
        ""
    delete options?.params

    // Include credentials and cookies always in dev env, otherwise only for hosted domain
    const optionsCredentials: RequestInit = {credentials: dev ? "include" : "same-origin", ...options}
    return fetch(`${baseBackendUrl()}${endpoint}${encodedParams}`, optionsCredentials);
}
