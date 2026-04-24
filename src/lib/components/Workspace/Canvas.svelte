<script lang="ts">
    import { onMount } from "svelte";
    import { getActiveTool, Tool } from "$lib/context/toolContext";
    import {
        ToolStrategies,
        handleMagnifyGesture,
        handlePanGesture,
        getIsPointerDown,
        setIsPointerDown,
        fitToView,
    } from "./canvas/toolStrategies.svelte";
    let toolState = getActiveTool();

    let activeTool = $derived(toolState.tool);

    // this element is binded to the canvas
    let canvasElement: HTMLDivElement;

    $effect(() => {
        canvasElement.style.cursor =
            activeTool === Tool.Pan
                ? getIsPointerDown()
                    ? "grabbing"
                    : "grab"
                : "default";
    });

    function handlePointerEnter(event: PointerEvent) {
        console.log("pointer entered!", event);
    }

    function handlePointerDown(event: PointerEvent) {
        const MOUSE_LEFT = 0;
        if (event.button != MOUSE_LEFT) return;
        event.preventDefault();

        canvasElement.setPointerCapture(event.pointerId);

        setIsPointerDown(true);
        console.log("pointer down");
        ToolStrategies[activeTool]?.handlePointerDown(event);
    }
    function handlePointerMove(event: PointerEvent) {
        event.preventDefault();

        console.log("pointer moved");
        ToolStrategies[activeTool]?.handlePointerMove(event);
    }

    function handlePointerUp(event: PointerEvent) {
        event.preventDefault();

        canvasElement.releasePointerCapture(event.pointerId);

        setIsPointerDown(false);
        console.log("pointer up");
        ToolStrategies[activeTool]?.handlePointerUp(event);
    }

    function updateRect() {
        if (!canvasElement) {
            return;
        }

        /*  update when canvas element size changes */
    }

    function handleWheel(event: WheelEvent) {
        event.preventDefault();

        if (event.ctrlKey) {
            handleMagnifyGesture(event);
        } else {
            handlePanGesture(event);
        }
    }

    onMount(() => {
        updateRect();
        fitToView(canvasElement);

        const resizeObserver = new ResizeObserver(updateRect);
        resizeObserver.observe(canvasElement);

        // track movement (layout shifts)
        const mutationObserver = new MutationObserver(updateRect);
        mutationObserver.observe(document.body, {
            attributes: true,
            childList: true,
            subtree: true,
        });

        return () => {
            resizeObserver.disconnect();
            mutationObserver.disconnect();
        };
    });
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<div
    class="canvas"
    role="application"
    aria-label="dd"
    bind:this={canvasElement}
    onwheel={handleWheel}
    onpointerenter={handlePointerEnter}
    onpointermove={handlePointerMove}
    onpointerdown={handlePointerDown}
    onpointerup={handlePointerUp}
></div>

<style>
    .canvas {
        flex: 1;
        touch-action: none;
        /*background-color: red;*/
    }
</style>
