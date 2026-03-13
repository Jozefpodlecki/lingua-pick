<script lang="ts">
    import type { DrawResult } from "$lib/types/app";
    import HanziWriter from "hanzi-writer";
    import { onMount } from "svelte";

    interface Props {
        character: string;
        onComplete(summary: DrawResult): void;
        locale?: "cn" | "jp";
        enabled: boolean;
    }

    let { character, onComplete, locale, enabled }: Props = $props();
    let container: HTMLDivElement;
    let writer: HanziWriter | null = null;

    onMount(() => {
        const base =
            locale === "jp"
                // ? "https://cdn.jsdelivr.net/npm/hanzi-writer-data-jp@0"
                ? "https://raw.githubusercontent.com/mnako/hanzi-writer-data-ja/master/data"
                : "https://cdn.jsdelivr.net/npm/hanzi-writer-data@3.7.3";

        writer = HanziWriter.create(container, character, {
            width: 300,
            height: 300,
            showCharacter: true,
            drawingWidth: 50,
            charDataLoader: (char, onLoad, onError) => {
                fetch(`${base}/${char}.json`)
                .then(res => res.json())
                .then(onLoad)
                .catch(onError);
            },
            onComplete
        });
        
    })

    $effect(() => {
        if(enabled) {
            writer?.quiz();
        }
    })

</script>

<div data-character={character} bind:this={container}></div>