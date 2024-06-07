import {useFetch} from "$lib/spells/fetch.svelte";
import {ensureAuthorized} from "$lib/api/decorators/authorization";
import {query, type QueryResponse} from "$lib/api/data";
import {check} from "$lib/api/auth";
import {dev} from "$app/environment";
import type {ResponseResult} from "$lib/api";

const queries = new Array<QueryHandle<any>>()

export type QueryHandle<T extends object | undefined = undefined> = {
    readonly isLoading: boolean,
    readonly isResolved: boolean,
    readonly response: QueryResponse[] | undefined,
    readonly error: string | undefined,
    resend(params?: T): Promise<ResponseResult<QueryResponse[], string>>,
    setParams(params: T): void,
    getParams(): T,
    getStatement(): string
}

export function useQuery<T extends object | undefined>(statement: string, initialParams: T): QueryHandle<T>
export function useQuery(statement: string): QueryHandle
export function useQuery(statement: string, initialParams: object | undefined = undefined) {
    const queryRequest = useFetch(query)
    let queryParams = initialParams

    const queryHandle = {
        get isLoading() {
            return queryRequest.isLoading
        },
        get isResolved() {
            return queryRequest.isResolved
        },
        get response() {
            return queryRequest.response
        },
        get error() {
            return queryRequest.error
        },

        async resend(params?: typeof queryParams) {
            params && this.setParams(params)
            return queryRequest.send(statement, queryParams)
        },
        setParams(params: typeof queryParams) {
            queryParams = params
        },
        getParams() {
            return queryParams
        },
        getStatement() {
            return statement
        }
    }

    queryHandle.resend()
    $effect.pre(() => {
        queries.push(queryHandle)
        return () => {
            queries.splice(queries.indexOf(queryHandle), 1)
        }
    })
    return queryHandle
}

export function reloadQueries() {
    const futures = queries.map(query => query.resend())
    return Promise.all(futures)
}
