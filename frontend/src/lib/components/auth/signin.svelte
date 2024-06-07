<script lang="ts">
    import {signin} from "$lib/api/auth"
    import TextInput from "$lib/components/forms/text.svelte"
    import PasswordInput from "$lib/components/forms/password.svelte"
    import CheckboxInput from "$lib/components/forms/checkbox.svelte"
    import {isValidEmail} from "$lib/utils/check";
    import {useFetch} from "$lib/spells/fetch.svelte";
    import {browser} from "$app/environment";

    let storedEmail: string | null = null
    if (browser) {
        storedEmail = localStorage.getItem("auth_email")
    }
    let email = $state(storedEmail ?? "")
    let password = $state("")
    let remember = $state(storedEmail !== null)

    let signinRequest = useFetch(signin)

    let emailError: string | undefined = $derived.by(() => {
        if (!isValidEmail(email.trim())) {
            return "Email Adresse is not valid"
        }
    })
    let passwordError: string | undefined = $derived.by(() => {
        if (password.trim().length <= 0) {
            return "Password must not be empty"
        }
    })

    let fieldsValid = $derived(emailError === undefined && passwordError === undefined)

    async function handleSignin(e: SubmitEvent) {
        e.preventDefault()

        signinRequest.reset()
        let result = await signinRequest.send(email.trim(), password.trim(), {remember})

        if (result.error === undefined) {
            if (remember) {
                localStorage.setItem("auth_email", email.trim())
            } else {
                localStorage.removeItem("auth_email")
            }

            onsuccess && onsuccess()
        } else if (result.response === undefined) {
            onfail && onfail(result.error)
        }
    }

    let {onchangesignup, onsuccess, onfail}: {
        onchangesignup?: () => void,
        onsuccess?: () => void
        onfail?: (error: typeof signinRequest.error) => void
    } = $props()
</script>

<form class="card gap-2 max-w-screen-sm" onsubmit={handleSignin}>
    <TextInput bind:value={email} invalidText={signinRequest.error !== undefined ? "" : undefined} labelText="Email"
               placeholderText="example@gmail.com"
               type="email"
               warningText={emailError}/>

    <PasswordInput bind:value={password}
                   invalidText={signinRequest.error}
                   labelText="Password"
                   placeholderText="supersecret"/>


    <div class="w-full flex flex-row justify-between">
        <CheckboxInput bind:checked={remember} class="checkbox-accent">
            {#snippet before()}
            <span class="label-text">Remember me</span>
            {/snippet}
        </CheckboxInput>

        {#if onchangesignup}
            <button type="button" class="link link-secondary" onclick={onchangesignup}>
                Register?
            </button>
        {/if}
    </div>


    <button class="btn btn-primary" disabled={signinRequest.isLoading || !fieldsValid} type="submit">
        {#if signinRequest.isLoading}
            <span class="loading loading-spinner"></span>
        {/if}
        Log In
    </button>
</form>
