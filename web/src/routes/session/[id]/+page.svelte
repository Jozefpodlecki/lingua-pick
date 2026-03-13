<script lang="ts">
    import { getAppContext, getSession } from "$lib/api/app";
    import { onMount } from "svelte";
    import { convertFileSrc } from '@tauri-apps/api/core';
    import { appDataDir, join } from '@tauri-apps/api/path';
    import Loader from "$lib/components/Loader.svelte";

    type PageState =
        | { type: "loading" }
        | {
            type: "loaded";
        }
        | { type: "submitting" };

    let { id } = $props();
    let pageState = $state<PageState>({ type: "loading" });
    
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
        } catch (error: unknown) {
            console.log(error);
        }
    }

</script>

<main class="flex h-screen size-full justify-center items-center">
    {#if pageState.type === "loading" || pageState.type === "submitting"}
        <Loader />

    {:else if pageState.type === "loaded"}
        <div></div>
    {/if}
</main>