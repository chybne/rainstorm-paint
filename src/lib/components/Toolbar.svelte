<script lang="ts">
    import ToolbarButton from "$lib/components/ToolbarButton.svelte";
    import { ToolData, Tool, getActiveTool, selectTool } from "$lib/context/toolContext";

    let activeTool = getActiveTool();
   
</script>

<div class="toolbar" role="toolbar">
    {#each ToolData as tool}
        <ToolbarButton
            Icon={tool.icon}
            Label={tool.label}
            isActive={tool.tool === activeTool.tool}
            onclick={() => {
                activeTool.tool = tool.tool;
            }}
        />
    {/each}
</div>

<style>
    .toolbar {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 0.25rem;

        padding: 0.25rem 0;
        outline: 1px solid var(--background-dark);

        width: 3rem;
        height: 100%;
        background-color: var(--background);
    }
</style>
