import {browser} from "$app/environment";

export function useLocalStorage<T>(key: string, initialValue: T) {
    let value = $state<T>(initialValue)

    $effect.pre(() => {
        let storageContent = localStorage.getItem(key)
        if (storageContent !== null) {
            value = JSON.parse(storageContent)
        }
    })

    $effect(() => {
        localStorage.setItem(key, JSON.stringify(value))
    })

    return {
        get value() {
            return value
        },
        set value(val) {
            value = val
        },
        reset() {
            value = initialValue
        }
    }
}
