<script lang="ts">
    import TextInput from "$lib/components/forms/text.svelte"

    interface Props {
        labelText?: string,
        helperText?: string,
        placeholderText?: string,
        disabled?: boolean,
        warningText?: string,
        invalidText?: string,
        class?: string

        value?: string,

        before?: Snippet,
    }

    let {value = $bindable(""), before, ...props}: Props = $props()
    let hidden = $state(true)
</script>

<TextInput {...props} bind:value type={hidden ? "password" : "text"}>
    {#snippet after()}
    <button class="btn btn-circle btn-sm" onclick={() => hidden = !hidden} type="button">
        <i class="fa-solid" class:fa-eye={!hidden} class:fa-eye-slash={hidden}></i>
    </button>
    {/snippet}
</TextInput>