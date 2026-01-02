<script>
    import Canvas from "$lib/components/Canvas.svelte";
    import Toolbar from "$lib/components/Toolbar.svelte";
    import { Tool, selectTool } from "$lib/context/toolContext";

    let toolState = $state({ tool: Tool.Pan });
    selectTool(toolState);
</script>

<main class="container">
    <!-- <div class="toolbar"></div> -->
    <div class="test-base">
        <Toolbar />
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

    /* .toolbar {
        display: flex;
        align-items: center;
        border-bottom: 1px solid var(--background-dark);
        height: 3rem;
        background-color: var(--background);
    } */

    .test-base {
        flex: 1;
        display: flex;
    }

    .test2 {
        height: 3rem;
        border: 1px solid red;
    }
</style>
