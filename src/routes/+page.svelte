<script>
    import { Brush } from "@lucide/svelte";
    import Button from "$lib/components/Button.svelte";
</script>

<main class="container">
    <div class="toolbar">
        <div class="tools">
            <Button icon={Brush} />
        </div>
    </div>
</main>

<style>
    .toolbar {
        display: flex;
        align-items: center;
        border: 0.2rem solid var(--text);
        height: 3rem;
        background-color: var(--background);
    }
</style>
