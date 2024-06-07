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

<dialog bind:this={dialog} class="modal modal-bottom sm:modal-middle" oncancel={(e) => e.preventDefault()}>
    <div class="modal-box bg-base-200">
        <div class="hero-content flex-col p-0">
            <div class="text-center">
                <h1 class="text-5xl font-bold">{signingin ? "Login" : "Register"} now!</h1>
                <p class="py-6">You need to authenticate in order to use this website. The data you store will be linked
                    to your account.</p>
            </div>
            <div class="card shrink-0 w-full max-w-screen-sm shadow-2xl bg-base-100">
                <div class="card-body">
                    {#if signingin}
                        <SigninForm onchangesignup={() => signingin = false} onsuccess={auth.success}/>
                    {:else}
                        <SignupForm onchangesignin={() => signingin = true} onsuccess={auth.success}/>
                    {/if}
                </div>
            </div>
        </div>
    </div>
</dialog>
