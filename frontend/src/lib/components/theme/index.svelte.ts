import {useLocalStorage} from "$lib/spells/storage.svelte"

let themeStorage: ReturnType<typeof useLocalStorage<null | "light" | "dark">>
$effect.root(() => {
    themeStorage = useLocalStorage<null | "light" | "dark">("theme", null)

    $effect(() => {
        if (themeStorage.value === null) {
            document.body.removeAttribute("data-theme")
        } else {
            document.body.setAttribute("data-theme", themeStorage.value)
        }
    })
})

export function setTheme(theme: "light" | "dark" | null) {
    themeStorage.value = theme
}

export function theme() {
    return themeStorage.value
}
