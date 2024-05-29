import {browser} from "$app/environment";

export function useLocalStorage<T>(key: string, initialValue: T) {
    let value = $state<T>(initialValue)

    $effect.pre(() => {
        if (browser) {
            let storageContent = localStorage.getItem(key)
            if (storageContent !== null) {
                value = JSON.parse(storageContent)
            }
        }
    })

    $effect(() => {
        if (browser) {
            localStorage.setItem(key, JSON.stringify(value))
        }
    })

    return {
        get value() {
            return value
        },
        set value(val) {
            value = val
        }
    }
}
