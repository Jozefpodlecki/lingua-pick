<script lang="ts">
    
    interface Props {
        text: string;
        progress: number;
    }
    
    let { text, progress }: Props = $props();

</script>

<div data-tauri-drag-region class="w-full h-8 overflow-hidden relative rounded-md bg-gray-700">
    <div
        data-tauri-drag-region
        class="h-full bg-blue-500/50 origin-left transform transition-transform duration-300"
        style="transform: scaleX({progress})"
    ></div>
    <div data-tauri-drag-region class="absolute inset-0 flex items-center justify-center text-white text-sm select-none">
        {text}
    </div>
</div>