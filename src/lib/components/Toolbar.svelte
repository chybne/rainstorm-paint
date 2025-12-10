<script lang="ts">
    import ToolbarButton from "$lib/components/ToolbarButton.svelte";
    import { Brush, Eraser, Lasso, Pipette } from "@lucide/svelte";

    let activeTool: string = "brush";

    const tools = [
        { name: "brush", icon: Brush, label: "Brush" },
        { name: "eraser", icon: Eraser, label: "Eraser" },
        { name: "lasso", icon: Lasso, label: "Lasso" },
        { name: "color-picker", icon: Pipette, label: "Color Picker" },
    ];

    const setActiveTool = (toolName: string) => {
        activeTool = toolName;
    };
</script>

<div class="toolbar" role="toolbar">
    {#each tools as tool}
        <ToolbarButton
            Icon={tool.icon}
            Label={tool.label}
            isActive={tool.name === activeTool}
            onclick={() => {
                setActiveTool(tool.name);
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
