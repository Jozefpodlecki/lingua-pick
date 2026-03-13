<script lang="ts">
    import { goto } from "$app/navigation";
    import { getAppContext, getAppDataDir } from "$lib/api/app";
    import StrokeExercise from "$lib/components/exercise/StrokeExercise.svelte";
    import Loader from "$lib/components/Loader.svelte";
    import { convertFileSrc } from '@tauri-apps/api/core';
    import { join } from '@tauri-apps/api/path';
    import { onMount } from "svelte";
    import type { MouseEventHandler } from "svelte/elements";
    import IconHome from "~icons/lucide/home";
    import IconLogOut from "~icons/lucide/log-out";
    import IconChartScatter from "~icons/lucide/chart-scatter";
    import IconSearch from "~icons/lucide/search";
    import IconLanguages from "~icons/lucide/languages";

    type PageState =
        | { type: "loading" }
        | { type: "error", message: string; }
        | {
            type: "loaded";
            backgroundImage: string;
        };

    let pageState = $state<PageState>({ type: "loading" });

    onMount(() => {
        onLoad()
    });

    async function onLoad() {
        try {
            const context = await getAppContext();

            if(!context) {
                // state = { type: "login"}
                goto('/login');
                return;
            }

            if(!context.profile) {
                // console.log("test");
                // goto('/profile');
            }

            const appDataDirPath = await getAppDataDir();
            console.log(appDataDirPath);
            const filePath = await join(appDataDirPath, '/data/aa/background.jpg');
            let backgroundImage = convertFileSrc(filePath);

            pageState = {
                type: "loaded",
                backgroundImage
            }
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
            }
        }
    }

    const onAction: MouseEventHandler<HTMLElement> = (event) => {
        const { action } = event.currentTarget.dataset;

        switch(action) {
            case "profile":
                goto('/profile');
                break;
            case "home":
                goto('/home');
            break;
            case "stats":
                goto('/stats');
                break;
            case "search":
                goto('/search');
                break;
            case "log-out":
                goto('/login');
                break;
        }
    }

</script>

<article class="relative flex flex-col h-screen size-full">
    {#if pageState.type === "loading"}
        <section class="flex h-screen size-full justify-center items-center">
            <main class="flex justify-center items-center bg-gray-900 w-100 h-60">
                <Loader />
            </main>
        </section>
    {:else if pageState.type === "error"}
        <section class="flex h-screen size-full justify-center items-center">
            <main class="flex justify-center items-center bg-gray-900 w-100 h-60">
                {pageState.message}
            </main>
        </section>
    {:else if pageState.type === "loaded"}
        <div
            class="absolute inset-0 -z-1 bg-cover bg-center brightness-25"
            style="background-image: url({pageState.backgroundImage})"
            ></div>
        <nav class="flex h-15 bg-gray-800/50">
            <div class="flex justify-center items-center h-15">
                <button
                    data-action="profile"
                    onclick={onAction}
                    class="flex items-center gap-2 px-4 h-10 ml-3 rounded-lg bg-black/40 hover:bg-black/60 transition"
                    >
                    <IconLanguages class="text-xl" />
                    <span class="font-semibold tracking-wide">EN → AA</span>
                </button>
            </div>
            <button data-action="home" class="flex justify-center items-center h-15 w-15" onclick={onAction}>
                <IconHome class="text-3xl"/>
            </button>
            <button data-action="stats" class="flex justify-center items-center h-15 w-15" onclick={onAction}>
                <IconChartScatter class="text-3xl"/>
            </button>
                <button data-action="search" class="flex justify-center items-center h-15 w-15" onclick={onAction}>
                <IconSearch class="text-3xl"/>
            </button>
        </nav>
        <section class="flex flex-1">
            <aside class="flex items-end justify-center w-20 bg-black-700/20">
                <button data-action="log-out" class="flex justify-center items-center h-15 w-15" onclick={onAction}>
                    <IconLogOut class="text-3xl"/>
                </button>
            </aside>
            <main class="flex-1 bg-black/50">

            </main>
        </section>
    {/if}
</article>