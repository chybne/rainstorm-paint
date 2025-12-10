<!-- Button Component -->
<script lang="ts">
    import { Brush } from "@lucide/svelte";

    export let icon = Brush;
</script>

<button class="btn" on:click>
    <svelte:component this={icon} size="32" />
</button>

<style>
    .btn:hover {
        background-color: var(--background-dark);
    }

    .btn {
        border: 0.2rem solid var(--text);
        border-radius: 1rem;
        margin: 0.2rem;
        background-color: var(--background);
        color: var(--text);
    }
</style>
