<script lang="ts">
    import { checkUpdates, install, load } from "$lib/api/app";
    import type { UpdateStatus } from "$lib/types/app";
    import { onMount } from "svelte";
    import { Window } from "@tauri-apps/api/window";
    import Loader from "$lib/components/Loader.svelte";
    import ProgressBar from "./ProgressBar.svelte";

    type UpdateState =
        | { status: 'initial' }
        | { status: 'checking' }
        | { status: 'latest' }
        | { status: 'found'; version: string }
        | { status: 'downloading'; version: string; downloaded: number; fileSize: number; progress: number }
        | { status: 'downloaded'; version: string }
        | { status: 'error'; message: string };

    let state = $state<UpdateState>({ status: 'initial' });

    onMount(() => {
        onLoad();
    })

    async function onLoad() {
        state = { status: 'checking' };
        let result = await load();
        await checkUpdates(onUpdate);
    }

    function getPrompt(state: UpdateState): string {
        switch (state.status) {
            case 'initial':
            case 'checking':
                return "Checking updates...";
            case 'found':
                return `Update found: ${state.version}`;
            case 'downloading':
                return `Downloading ${state.version}… ${Math.floor(state.progress * 100)}%`;
            case 'downloaded':
                return `Downloaded new version - ${state.version}`;
            case 'latest':
                return "Already up to date";
            case 'error':
                return state.message;
        }
    }

    async function onUpdate(event: { payload: UpdateStatus }) {
        const { payload } = event;
        console.log(payload);
        
        switch(payload.type) {
            case "checking":
                state = { status: 'checking' };
                break;
            case "found":
                state = { status: 'found', version: payload.version };
                break;
            case "downloading":
                state = {
                    status: 'downloading',
                    version: payload.version,
                    downloaded: payload.downloaded,
                    fileSize: payload.fileSize,
                    progress: payload.downloaded / payload.fileSize
                };
                break;
            case "downloaded":
                state = { status: 'downloaded', version: payload.version };
                break;
            case "error":
                state = { status: 'error', message: payload.message };
                break;
            case "latest":
                state = { status: 'latest' };
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

<section data-tauri-drag-region class="flex flex-col items-center w-64 gap-4">
    {#if state.status === 'downloading'}
        <ProgressBar
            text={getPrompt(state)}
            progress={state.progress}/>
        
    {:else if state.status === 'downloaded'}
        <div class="mt-2 text-sm text-center">{getPrompt(state)}</div>
        <div class="flex flex-col gap-2 w-full">
            <button 
                data-action="install"
                type="button"
                class="px-4 py-2 bg-blue-500/50 text-white rounded hover:bg-blue-600/50 text-sm"
                onclick={onInstall}>
                Install 
            </button>
            <button 
                data-action="continue"
                type="button"
                class="px-4 py-2 bg-gray-500/50 text-white rounded hover:bg-gray-600/50 text-sm"
                onclick={onContinue}>
                Continue
            </button>
        </div>
        
    {:else if state.status === 'error'}
        <div class="text-xs text-red-500 text-center">{getPrompt(state)}</div>
        <div class="flex gap-2">
            <button 
                data-action="retry"        
                type="button"
                class="px-4 py-2 w-30 bg-gray-500/50 text-white rounded hover:bg-gray-600/50 text-sm"
                onclick={onLoad}>
                Retry
            </button>
            <button 
                data-action="continue"
                type="button"
                class="px-4 py-2 w-30 bg-gray-500/50 text-white rounded hover:bg-gray-600/50 text-sm"
                onclick={onContinue}>
                Continue
            </button>
        </div>
    {:else}
        <Loader/>
    {/if}
</section>