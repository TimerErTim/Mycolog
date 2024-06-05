<script lang="ts">
    interface Props {
        checked: boolean
        class?: string

        before?: Snippet
        after?: Snippet
    }

    let {class: className, checked = $bindable(false), before, after, ...props}: Props = $props()
</script>

<div class="form-control">
    <label class="label cursor-pointer w-fit gap-4">
        {@render before()}
        <input {...props} bind:checked class="checkbox {className}" type="checkbox"/>
        {@render after()}
    </label>
</div>