<script lang="ts">
    import CheckboxInput from "$lib/components/forms/checkbox.svelte"
    import {setTheme, theme} from "$lib/components/theme/index.svelte"
</script>


<CheckboxInput checked={theme() === "dark"} onchange={async () => setTheme(theme() === "light" ? "dark" : "light")}>
    {#snippet before()}
    <span class="label-text">Use Dark Theme?</span>
    {/snippet}
</CheckboxInput>