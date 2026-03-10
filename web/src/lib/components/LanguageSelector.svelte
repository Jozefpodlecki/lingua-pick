<script lang="ts">
import type { Language } from "$lib/types/app";
import type { MouseEventHandler } from "svelte/elements";

interface Props {
    items: Language[];
    selected?: Language | null;
}

let { items, selected = $bindable() } = $props<Props>();

let query = $state("");
let open = $state(false);

const filtered = $derived(
    items
        .filter((l) => l.name.toLowerCase().includes(query.toLowerCase()))
        .slice(0, 10)
);

const onSelect: MouseEventHandler<HTMLElement> = (event) => {
    const index = Number(event.currentTarget.dataset.index);
    const lang = filtered[index];
    if (!lang) return;

    selected = lang;
    query = lang.name;
    open = false;
};

function onFocus() {
    query = "";
    open = true;
}

function onBlur(event: FocusEvent) {
    const related = event.relatedTarget as HTMLElement | null;
    if (!related || !section.contains(related)) {
        open = false;
        if (selected) query = selected.name;
    }
}

let section: HTMLElement;

$effect(() => {
    if (!open && selected) query = selected.name;
});
</script>

<section
    class="relative w-full"
    bind:this={section}
    onfocusout={onBlur}
>
    <input
        type="text"
        bind:value={query}
        onfocus={onFocus}
        class="w-full bg-transparent rounded-lg border px-3 py-2 outline-none focus:ring-2 focus:ring-black"
        placeholder="Search language..."
    />

    {#if open && filtered.length > 0}
        <div class="absolute z-10 mt-1 w-full max-h-60 overflow-auto rounded-lg border bg-black">
            {#each filtered as lang, i}
                <div
                    role="button"
                    tabIndex={0}
                    data-index={i}
                    class="cursor-pointer px-3 py-2 hover:bg-gray-900"
                    onmousedown={onSelect}
                >
                    {lang.name}
                </div>
            {/each}
        </div>
    {/if}
</section>