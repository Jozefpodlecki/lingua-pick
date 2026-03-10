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
            const context = await getAppContext();
            if (!context) throw new Error("Missing context");

            const languages = await getAllLanguages(context.token);

            const source = await getDefaultLanguage(context.token);
            
            pageState = {
                type: "loaded",
                languages,
                source,
                target: null,
                features: []
            };
        } catch (err) {
            console.error(err);
        }
    }

    async function onSubmit() {
        if (pageState.type !== "loaded") return;

        pageState = { type: "submitting" };

        try {
            // submit logic here
        } catch (e) {
            console.error(e);
        }
    }
</script>

<main class="flex min-h-screen items-center justify-center bg-black-50 ">

{#if pageState.type === "loading" || pageState.type === "submitting"}
    <Loader />

{:else if pageState.type === "loaded"}

<div class="w-200 border h-100 space-y-6 rounded bg-black p-8">

    <h1 class="text-xl font-semibold">
        Create Profile
    </h1>

    <div class="space-y-4">

        <div>
            <label class="text-sm font-medium text-gray-600">
                Source language
            </label>

            <LanguageSelector
                items={pageState.languages}
                bind:selected={pageState.source}
            />
        </div>

        <div>
            <label class="text-sm font-medium text-gray-600">
                Target language
            </label>

            <LanguageSelector
                items={pageState.languages}
                bind:selected={pageState.target}
            />
        </div>
        {#if pageState.target}
            {#each pageState.features as feature}
                <div class="">
                    {feature.id} {feature.name} {feature.description}
                </div>
            {/each}
        {/if}
    </div>
    <div class="mt-20">
        <button
            type="button"
            class="w-50 flex items-center justify-center border rounded py-2 text-white disabled:opacity-40"
            disabled={!pageState.source || !pageState.target}
            onclick={onSubmit}
        >
            <span>Create</span>
            <IconSave class="ml-2"/>
        </button>
    </div>
    
</div>

{/if}

</main>