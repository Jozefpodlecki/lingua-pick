<script lang="ts">
    import { goto } from "$app/navigation";
    import { getAppContext } from "$lib/api/app";
    import Loader from "$lib/components/Loader.svelte";
    import HanziWriter from "hanzi-writer";
    import { onMount } from "svelte";

    let state = $state({ type: "loading"});

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

        var writer = HanziWriter.create('character-target-div', '测', {
  width: 150,
  height: 150,
  showCharacter: false,
  padding: 5
});
writer.quiz();
    }
    
</script>

<main class="flex h-screen size-full justify-center items-center">
    <div id="character-target-div"></div>
    <!-- <Loader/> -->
</main>