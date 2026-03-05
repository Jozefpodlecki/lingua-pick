<script lang="ts">
    import { appName } from "$lib/constants";
    import Prompt from "./Prompt.svelte";

</script>

<img
    data-tauri-drag-region
    class="absolute inset-0 w-full h-full object-cover brightness-50 -z-0"
    alt="background"
    src="starter.avif"
/>

<div
    data-tauri-drag-region
    class="relative z-10 flex items-center justify-center h-screen w-screen"
>
    <div class="bg-black/80 text-center px-20 py-8 rounded-lg flex flex-col items-center gap-2">
        <div class="flex justify-center items-center">
            <img
                data-tauri-drag-region
                class="brightness-75 px-4"
                alt="logo"
                src="32x32.png"
            />
            <div data-tauri-drag-region class="text-3xl font-cascadia-mono text-gray-300 font-bold cursor-none">{appName}</div>
        </div>
        <Prompt/>
    </div>
</div>