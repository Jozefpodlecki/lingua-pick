<script lang="ts">
    import { goto } from "$app/navigation";
    import { getAppContext } from "$lib/api/app";
    import Loader from "$lib/components/Loader.svelte";
    import StrokeWriter from "$lib/components/StrokeWriter.svelte";
    import HanziWriter from "hanzi-writer";
    import { onMount } from "svelte";

    type PageState =
        | { type: "loading" }
        | {
            type: "loaded";
            
        };

    let state = $state<PageState>({ type: "loading" });

    onMount(() => {
        onLoad()
    });

    async function onLoad() {
        const context = await getAppContext();

        if(!context) {
            // state = { type: "login"}
            goto('/login');
            return;
        }

        if(!context.profile) {
            console.log("test");
        }

        // const writer = HanziWriter.create('character-target-div', '测', {
        //     width: 300,
        //     height: 300,
        //     showCharacter: false,
        //     drawingWidth: 50,
        //     onComplete: (data) => {
        //         console.log(data)
        //     }
        // });
        
        // writer.quiz();

        // const writer1 = HanziWriter.create('character-target-div2', '峠', {
        //     width: 300,
        //     height: 300,
        //     showCharacter: false,
        //     drawingWidth: 50,
        //     charDataLoader: (char, onLoad, onError) => {
        //         fetch(`https://cdn.jsdelivr.net/npm/hanzi-writer-data-jp@0/${char}.json`)
        //         .then(res => res.json())
        //         .then(onLoad)
        //         .catch(onError);
        //     },
        //     onComplete: (data) => {
        //         console.log(data)
        //     }
        // });

        // writer1.quiz();
    }

    function onComplete() {

    }

    // "経済学"
    
</script>

<main class="flex h-screen size-full justify-center items-center">
    <StrokeWriter character="経" locale="jp" onComplete={onComplete}/>
    <StrokeWriter character="済" locale="jp" onComplete={onComplete}/>
    <StrokeWriter character="学" locale="jp" onComplete={onComplete}/>
    <!-- <Loader/> -->
</main>