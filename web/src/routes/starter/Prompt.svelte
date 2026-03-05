<script lang="ts">
    import { checkUpdates, install, load } from "$lib/api/app";
    import type { UpdateStatus } from "$lib/types/app";
    import { onMount } from "svelte";
    import { Window } from "@tauri-apps/api/window";
    import Loader from "$lib/components/Loader.svelte";
    import ProgressBar from "./ProgressBar.svelte";

    let prompt = $state("Starting");
    let progress = $state(0);
    let downloading = $state(false);
    let downloaded = $state(false);

    onMount(() => {
        onLoad();
    })

    async function onLoad() {
        let result = await load();
        prompt = "Checking updates";
        
        await checkUpdates(onUpdate);
    }

    async function onUpdate(event: { payload: UpdateStatus }) {
        const { payload } = event;
        console.log(payload)
        switch(payload.type) {
            case "checking":
                prompt = "Checking updates...";
                downloading = false;
                downloaded = false;
                break;
            case "found":
                prompt = `Update found: ${payload.version}`;
                downloading = false;
                downloaded = false;
                break;
            case "downloading":
                downloading = true;
                downloaded = false;
                progress = payload.downloaded / payload.fileSize;
                prompt = `Downloading ${payload.version}… ${Math.floor(progress * 100)}%`;
                break;
            case "downloaded":
                prompt = `Downloaded new version - ${payload.version}`;
                downloading = false;
                downloaded = true;
                progress = 1;
                break;
            case "latest":
                onContinue();
                break;
        }
    }

    async function onInstall() {
        await install();
    }

    async function onContinue() {
        let main = await Window.getByLabel("main");
        await main?.show();

        let current = Window.getCurrent();
        await current.hide();
    }

</script>

<section data-tauri-drag-region class="flex flex-col items-center w-64">
    {#if downloading}
        <ProgressBar
            text={prompt}
            progress={progress}/>
        
    {:else if downloaded}
        <div class="mt-2 text-sm text-center">{prompt}</div>
        <div class="flex flex-col gap-2 w-full">
            <button 
                data-action="install"
                type="button"
                class="px-4 py-2 bg-blue-500/50 text-white rounded hover:bg-blue-600 text-sm"
                onclick={onInstall}>
                Install 
            </button>
            <button 
                data-action="continue"
                type="button"
                class="px-4 py-2 bg-gray-500/50 text-white rounded hover:bg-gray-600 text-sm"
                onclick={onContinue}>
                Continue
            </button>
        </div>
    {:else}
        <Loader/>
    {/if}
</section>
