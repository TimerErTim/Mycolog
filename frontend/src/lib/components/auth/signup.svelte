<script lang="ts">
    import {signin, signup} from "$lib/api/auth"
    import TextInput from "$lib/components/forms/text.svelte"
    import PasswordInput from "$lib/components/forms/password.svelte"
    import CheckboxInput from "$lib/components/forms/checkbox.svelte"
    import {isValidEmail} from "$lib/utils/check";
    import {useFetch} from "$lib/spells/fetch.svelte";
    import {browser} from "$app/environment";

    let email = $state("")
    let password = $state("")
    let passwordCheck = $state("")

    let signupRequest = useFetch(signup)

    let emailError: string | undefined = $derived.by(() => {
        if (!isValidEmail(email.trim())) {
            return "Email Adresse is not valid"
        }
    })
    let passwordError: string | undefined = $derived.by(() => {
        if (password.trim().length < 8) {
            return "Password must be at least 8 characters long"
        }
    })
    let repeatPasswordError: string | undefined = $derived.by(() => {
        if (password.trim() !== passwordCheck.trim()) {
            return "Passwords must match"
        }
    })

    let fieldsValid = $derived(emailError === undefined
        && passwordError === undefined
        && repeatPasswordError === undefined)

    async function handleSignup(e: SubmitEvent) {
        e.preventDefault()

        signupRequest.reset()
        let result = await signupRequest.run(email.trim(), password.trim())

        if (result.error === undefined) {
            onsuccess && onsuccess()
        } else if (result.response === undefined) {
            onfail && onfail(result.error)
        }
    }

    let {onchangesignin, onsuccess, onfail}: {
        onchangesignin?: () => void,
        onsuccess?: () => void,
        onfail?: (error: typeof signupRequest.error) => void
    } = $props()
</script>

<form class="card gap-2 max-w-screen-sm" onsubmit={handleSignup}>
    <TextInput bind:value={email} invalidText={signupRequest.error !== undefined ? "" : undefined} labelText="Email"
               placeholderText="example@gmail.com"
               type="email"
               warningText={emailError}/>

    <PasswordInput bind:value={password}
                   helperText="Password must be 8+ characters long"
                   invalidText={signupRequest.error && ""}
                   labelText="Password"
                   placeholderText="supersecret"/>

    <PasswordInput bind:value={passwordCheck}
                   invalidText={signupRequest.error ?? repeatPasswordError}
                   labelText="Repeat Password"
                   placeholderText="supersecret"/>


    <div class="w-full flex flex-row justify-between">
        {#if onchangesignin}
            <button type="button" class="link link-secondary my-2" onclick={onchangesignin}>
                Already have an account? Sign In instead.
            </button>
        {/if}
    </div>


    <button class="btn btn-primary" disabled={signupRequest.loading || !fieldsValid} type="submit">
        {#if signupRequest.loading}
            <span class="loading loading-spinner"></span>
        {/if}
        Create Account
    </button>
</form>
