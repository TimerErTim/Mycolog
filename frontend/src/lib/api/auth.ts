import {fetchBackend} from "$lib/api/index";

export interface SignInOptions {
    remember?: boolean
}

export async function signin(email: string, password: string, options?: SignInOptions): Promise<null | string> {
    const response = await fetchBackend("/auth/signin", {
        method: "POST",
        params: {
            remember: options?.remember
        },
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            email,
            password
        })
    })

    return response.ok ? null : await response.text()
}

export interface SignUpOptions {

}

export async function signup(email: string, password: string, options?: SignUpOptions): Promise<null | string> {
    const response = await fetchBackend("/auth/signup", {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            email,
            password
        })
    })

    return response.ok ? null : await response.text()
}

export async function check(): Promise<CheckResponse> {
    const response = await fetchBackend("/auth/check", {
        method: "POST"
    })

    return {
        code: response.status,
        text: response.ok ? null : await response.text(),
        get ok(): boolean {
            return response.ok
        }
    }
}

export interface CheckResponse {
    code: number
    text: string | null

    get ok(): boolean
}

export async function logout(): Promise<null | string> {
    const response = await fetchBackend("/auth/logout", {
        method: "POST"
    })

    return response.ok ? null : await response.text()
}