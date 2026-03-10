<script lang="ts">
    import HanziWriter from "hanzi-writer";
    import { onMount } from "svelte";

    interface Props {
        character: string;
        onComplete(): void;
        locale?: "cn" | "jp";
    }

    let { character, onComplete, locale }: Props = $props();
    let container: HTMLDivElement;
    
    onMount(() => {
        const base =
            locale === "jp"
                // ? "https://cdn.jsdelivr.net/npm/hanzi-writer-data-jp@0"
                ? "https://raw.githubusercontent.com/mnako/hanzi-writer-data-ja/master/data"
                : "https://cdn.jsdelivr.net/npm/hanzi-writer-data@3.7.3";

        const writer = HanziWriter.create(container, character, {
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
        
        writer.quiz();
    })

</script>

<div bind:this={container}></div>