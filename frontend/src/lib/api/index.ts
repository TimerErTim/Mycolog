import {api} from "$lib/config";
import {objectToRecord} from "$lib/utils/conversion";

function baseBackendUrl() {
    return `${api.backendProtocol}://${api.backendHost}:${api.backendPort}${api.backendEndpoint}`
}

export async function fetchBackend(endpoint: string, options?: RequestInit & { params?: object }): Promise<Response> {
    const encodedParams = options?.params ?
        new URLSearchParams(objectToRecord(options.params)).toString() :
        ""
    delete options?.params

    return fetch(`${baseBackendUrl()}${endpoint}?${encodedParams}`, options);
}
