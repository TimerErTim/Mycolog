import {fetchBackend, type ResponseResult} from "$lib/api/index";
import type {SignUpOptions} from "$lib/api/auth";
import {dev} from "$app/environment";
import {ensureAuthorized} from "$lib/api/decorators/authorization";
import {buffered} from "$lib/api/decorators/buffered";
import {mapRecordValues} from "$lib/utils/conversion";

export type QueryResponse = {
    time: string,
    result: any,
    error?: void
} | {
    time: string,
    result?: void,
    error: string
}

async function query(
    statements: string,
    params?: object
): Promise<ResponseResult<QueryResponse[], string>> {
    const response = await fetchBackend("/data/query", {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            statements,
            variables: params
        })
    })

    return response.ok ? {
        status: response.status,
        response: await response.json(),
    } : {
        status: response.status,
        error: await response.text()
    }
}

const authorizedQuery = ensureAuthorized(query)

async function queryMulti(
    queries: Record<string, {
        statements: string,
        params?: object
    }>
): Promise<ResponseResult<Record<string, QueryResponse[]>, string>> {
    const response = await fetchBackend("/data/multi", {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify(queries)
    })

    return response.ok ? {
        status: response.status,
        response: await response.json(),
    } : {
        status: response.status,
        error: await response.text()
    }
}
const authorizedMulti = ensureAuthorized(queryMulti)

async function mappedMulti(input: Record<string, [string, object | undefined]>) {
    const mappedInput = mapRecordValues(input, ([statements, params]) => ({
        statements,
        params
    }))

    const output = await authorizedMulti(mappedInput)

    const mappedOutput: Record<string, ResponseResult<QueryResponse[], string>> = {}
    for (const key in mappedInput) {
        if (output.response !== undefined) {
            const response = output.response[key]
            if (response !== undefined) {
                mappedOutput[key] = {
                    status: output.status,
                    response
                }
            } else {
                mappedOutput[key] = {
                    status: 400,
                    error: "unknown error: response missing in batched request"
                }
            }
        } else {
            mappedOutput[key] = {
                status: output.status,
                error: output.error
            }
        }
    }
    return mappedOutput
}

const queryBehavior = dev ? authorizedQuery : buffered(mappedMulti, 250)
export {queryBehavior as query}
