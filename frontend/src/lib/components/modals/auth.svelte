<script lang="ts">
    import * as auth from "$lib/components/modals/authControl.svelte"
    import SigninForm from "$lib/components/auth/signin.svelte"
    import SignupForm from "$lib/components/auth/signup.svelte"

    let dialog: HTMLDialogElement

    $effect(() => {
        if (auth.isOpen()) {
            dialog.showModal()
        } else {
            dialog.close()
        }
    })

    let signingin = $state(true)
</script>

<dialog bind:this={dialog} class="modal" oncancel={(e) => e.preventDefault()}>
    <div class="modal-box">
        <h3 class="font-bold text-lg ml-4">{signingin ? "Sign in" : "Sign up"}!</h3>
        {#if signingin}
            <SigninForm onchangesignup={() => signingin = false} onsuccess={auth.success}/>
        {:else}
            <SignupForm onchangesignin={() => signingin = true} onsuccess={auth.success}/>
        {/if}
    </div>
</dialog>
