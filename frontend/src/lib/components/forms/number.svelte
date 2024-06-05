<script lang="ts">
    import TextInput from "$lib/components/forms/text.svelte"

    interface Props {
        labelText?: string,
        helperText?: string,
        placeholderText?: string,
        disabled?: boolean,
        warning?: boolean,
        invalidText?: string,
        class?: string

        value: number,

        before?: Snippet
    }

    let {value = $bindable(0), ...props}: Props = $props()

    let text = $state(value.toString())
    let invalidNumber = $derived(Number.isNaN(Number(text)))

    async function handleSubmit(clearOnFail: boolean) {
        let newValue = Number(text)
        if (Number.isNaN(newValue)) {
            if (clearOnFail) {
                await handleCancel()
            }
            return
        }
        value = newValue
    }

    async function handleCancel() {
        text = value.toString()
    }
</script>

<TextInput {...props} bind:value={text} onblur={() => handleSubmit(true)} onkeydown={(e: KeyboardEvent) => {
               if (e.key === "Enter") { handleSubmit(false) }
               else if (e.key === "Escape") { handleCancel() }
           }}
           type="number"
           warningText={invalidNumber ? "" : undefined}
/>
