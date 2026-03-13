<script lang="ts">
    import StrokeWriter from "$lib/components/StrokeWriter.svelte";
    import type { DrawResult, Lexeme } from "$lib/types/app";
    import { onMount } from "svelte";
    import Loader from "../Loader.svelte";
    import IconChevronRight from "~icons/lucide/chevron-right";
    import IconXCircle from "~icons/lucide/x-circle";
    import { delay } from "$lib/utils";
    import { _ } from 'svelte-i18n'
    
    type PageState =
        | { type: "loading" }
        | { 
            type: "error";
            message: string;
        }
        | {
            type: "loaded";
            lexeme: Lexeme;
            completed: boolean;
            items: Item[];
        };

    interface Props {
        lexeme: string;
    }

    interface Item {
        index: number;
        character: string;
        enabled: boolean;
        startedOn: Date;
        completedOn: Date | null;
        totalMistakes: number;
    }

    let { lexeme }: Props = $props()
    let pageState = $state<PageState>({ type: "loading" });

    function onComplete(summary: DrawResult) {
        
        if(pageState.type !== "loaded") {
            return;
        }

        console.log(summary);

        const item = pageState.items.find(pr => pr.character === summary.character)!;

        item.completedOn = new Date();
        item.totalMistakes = summary.totalMistakes;
        
        const nextItem = pageState.items.at(item.index + 1);

        if(nextItem) {
            nextItem.startedOn = new Date();
            nextItem.enabled = true;
            return;
        }

        onAllComplete();
    }

    function onAllComplete() {
        console.log("all complete");
    }

    function onContinue() {

    }

    onMount(() => {
        onLoad()
    });

    async function loadLexeme(): Promise<Lexeme> {
        // throw new Error("Could not load lexeme")
        return {
            id: 1,
            createdOn: "",
            text: "経済学",
            languageId: 1,
            normalized: "",
            meanings: [
                {
                    language: "en",
                    value: "Economics",
                    breakdown: [
                        { reading: "けい", romanization: "kei" },
                        { reading: "さい", romanization: "sai" },
                        { reading: "がく", romanization: "gaku" }
                    ]
                },
            ]
        }
    }

    async function onLoad() {
        pageState = { type: "loading" };
        await delay(1000);
        
        try {
            const lexeme = await loadLexeme();
            const items = lexeme.text.split("").map((character, index) => ({
                index,
                character,
                enabled: false,
                startedOn: new Date(),
                completedOn: null,
                totalMistakes: 0
            }));
            items[0].enabled = true;

            pageState = {
                type: "loaded",
                lexeme,
                completed: false,
                items
            };   
        } catch (error: unknown) {
            let message: string;

            if (error instanceof Error) {
                message = error.message;
            } else {
                message = String(error);
            }

            pageState = {
                type: "error",
                message
            };
        }
    }
    
</script>

{#if pageState.type === "loading"}
    <Loader />

{:else if pageState.type === "loaded"}
    <section class="flex flex-col">
        <div class="text-center text-5xl">
            {"Draw"}
        </div>
        <div class="flex mt-4">
            {#each pageState.items as {character, totalMistakes, enabled} }
                <StrokeWriter
                    character={character}
                    locale="jp"
                    onComplete={onComplete}
                    enabled={enabled}/>

                {#if pageState.completed }
                <div>Mistakes {totalMistakes}</div>
                {/if}
            {/each}
        </div>
        {#if pageState.completed }
            <div class="">

            </div>
            <div class="">
                <button type="button" onclick={onContinue} class="">
                    Continue
                    <IconChevronRight/>
                </button>
            </div>
        {/if}
    </section>
{:else if pageState.type === "error"}
    <section class="flex flex-col items-center justify-center p-6 text-center text-red-800">
        <IconXCircle class="w-12 h-12 mb-4" />
        <h2 class="text-2xl font-semibold">Oops! Something went wrong.</h2>
        <p class="mt-2">{pageState.message}</p>
        <button 
            class="mt-4 px-4 py-2 bg-red-800 text-white rounded hover:bg-red-900"
            onclick={onLoad}>
            Retry
        </button>
    </section>
{/if}