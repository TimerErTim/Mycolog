<script lang="ts">
    import ThemeDropdown from "$lib/components/theme/dropdown.svelte"
    import {authorize} from "$lib/components/modals/authControl.svelte"
    import {useQuery} from "$lib/spells/query.svelte";

    console.log("entering component")
    let idfk = useQuery("SELECT * FROM user;")
    $inspect(idfk.loading)
    $inspect(idfk.response)

    let otherquery = useQuery("SELECT * FROM $auth;")
    $inspect(otherquery.loading)
    $inspect(otherquery.response)
</script>

<svelte:head>
    <title>IDFK Mycolog</title>
</svelte:head>

<ThemeDropdown/>

<h1>Login</h1>




