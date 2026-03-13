<script lang="ts">
    import { getAllLanguages, getAppContext, getDefaultLanguage, getLanguageFeatures } from "$lib/api/app";
    import Loader from "$lib/components/Loader.svelte";
    import LanguageSelector from "$lib/components/LanguageSelector.svelte";
    import type { Language, LanguageFeature } from "$lib/types/app";
    import { onMount } from "svelte";
    import IconSave from "~icons/lucide/save";

    type PageState =
        | { type: "loading" }
        | {
            type: "loaded";
            languages: Language[];
            source: Language | null;
            target: Language | null;
            features: LanguageFeature[];
        }
        | { type: "submitting" };

    let pageState = $state<PageState>({ type: "loading" });

    onMount(onLoad);

    async function onLoad() {
        try {
         
        } catch (err) {
            console.error(err);
        }
    }

</script>

<main class="flex min-h-screen items-center justify-center bg-black-50 ">

{#if pageState.type === "loading" || pageState.type === "submitting"}
    <Loader />

{:else if pageState.type === "loaded"}
    <div></div>
{/if}

</main>