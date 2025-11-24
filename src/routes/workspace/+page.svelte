<script>
    import { Brush } from "@lucide/svelte";
    import Canvas from "$lib/components/Canvas.svelte";
    import Button from "$lib/components/Button.svelte";
</script>

<main class="container">
    <div class="toolbar">
        <div class="tools">
            <Button icon={Brush} />
        </div>
    </div>
    <div class="test-base">
        <div class="test"></div>
        <Canvas />
    </div>
    <div class="test2"></div>
</main>

<style>
    .container {
        display: flex;
        flex-direction: column;
        width: 100%;
        height: 100%;
    }

    .toolbar {
        display: flex;
        align-items: center;
        border-bottom: 1px solid var(--background-dark);
        height: 3rem;
        background-color: var(--background);
    }

    .test-base {
        flex: 1;
        display: flex;
    }

    .test {
        width: 3rem;
        border: 1px solid red;
    }

    .test2 {
        height: 3rem;
        border: 1px solid red;
    }
</style>
