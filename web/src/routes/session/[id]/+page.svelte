<script lang="ts">
    import { getAppContext, getSession } from "$lib/api/app";
    import { onMount } from "svelte";

    let { id } = $props();

    onMount(() => {
        console.log(id);
        onLoad()
    });

    async function onLoad() {

        try {
            const context = await getAppContext();

            if(!context) {
                console.log("fatal, context not set");
                return;
            }

            const session = await getSession(context.token, id);
        } catch (error) {
            console.log(error);
        }
    }

</script>

<main class="flex h-screen size-full justify-center items-center">

</main>