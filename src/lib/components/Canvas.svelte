<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";

    // this element is binded to the canvas
    let element: HTMLDivElement;

    function updateRect() {
        if (!element) {
            return;
        }
        const rect = element.getBoundingClientRect();
        const dpr = window.devicePixelRatio;

        invoke("set_view", {
            x: rect.x * dpr,
            y: rect.y * dpr,
            width: window.innerWidth * dpr,
            height: window.innerHeight * dpr,
        });
    }

    onMount(() => {
        updateRect();

        const resizeObserver = new ResizeObserver(updateRect);
        resizeObserver.observe(element);

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

<div class="canvas" bind:this={element}></div>

<style>
    .canvas {
        flex: 1;
        /*background-color: red;*/
    }
</style>
