<script lang="ts">
    import type {Snippet} from "svelte";

    interface Props {
        labelText?: string,
        helperText?: string,
        placeholderText?: string,
        disabled?: boolean,
        warningText?: string,
        invalidText?: string
        type?: string

        value?: string
        class?: string

        after?: Snippet
        before?: Snippet
    }

    let {
        class: className,
        labelText,
        helperText,
        placeholderText,
        type,
        disabled,
        warningText,
        invalidText,
        value = $bindable(""),
        after,
        before,
        ...props
    }: Props = $props()
</script>

<label class="form-control w-full">
    {#if !!labelText}
        <div class="label">
            <span class="label-text">{labelText}</span>
        </div>
    {/if}
    <label class="input input-bordered flex items-center gap-2 {className}"
           class:input-error={invalidText !== undefined}>
        {@render before()}
        <input {...props} bind:value={value} class="w-full" {disabled} placeholder={placeholderText} {type}/>
        {#if warningText !== undefined}
            <div class="tooltip" data-tip={warningText}>
                <i class="fa-solid fa-warning text-warning"></i>
            </div>
        {/if}
        {@render after()}
    </label>
    {#if !!helperText || !!invalidText}
        <div class="label pb-0">
            <span class="label-text text-sm"
                  class:text-error={!!invalidText}>{!!invalidText ? invalidText : helperText}</span>
        </div>
    {/if}
</label>