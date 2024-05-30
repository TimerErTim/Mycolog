import {fetchBackend, type ResponseResult} from "$lib/api/index";

export interface SignInOptions {
    remember?: boolean
}

export async function signin(email: string, password: string, options?: SignInOptions): Promise<ResponseResult<string, string>> {
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

    return response.ok ? {
        response: await response.text(),
    } : {
        error: await response.text()
    }
}

export interface SignUpOptions {

}

export async function signup(email: string, password: string, options?: SignUpOptions) {
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

    return response.ok ? {
        response: await response.text(),
    } : {
        error: await response.text()
    }
}

export async function check() {
    const response = await fetchBackend("/auth/check", {
        method: "POST"
    })

    return response.ok ? {
        response: await response.text(),
    } : {
        error: {
            code: response.status,
            text: await response.text()
        }
    }
}

export async function logout() {
    const response = await fetchBackend("/auth/logout", {
        method: "POST"
    })

    return response.ok ? {
        response: await response.text(),
    } : {
        error: await response.text()
    }
}