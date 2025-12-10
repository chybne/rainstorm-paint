<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";

    let scale = 1;
    let mouseX = 0.0;
    let mouseY = 0.0;

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

    function handleMouseMove(event: MouseEvent) {
        mouseX = event.offsetX;
        mouseY = event.offsetY;
        console.log("mouseX", mouseX, "mouseY", mouseY);
    }

    function handleWheel(event: WheelEvent) {
        const dpr = window.devicePixelRatio;

        /* for pinch gestures */
        if (event.ctrlKey) {
            event.preventDefault(); // prevent zooming the page
            const zoomFactor = 1 - event.deltaY * 0.01;
            scale *= zoomFactor;
            /* clamp the value Definitely needs to be calculated before hand*/
            scale = Math.min(Math.max(scale, 0.3), 5.0);
            invoke("canvas_zoom", {
                zoom: scale,
                mouseX: mouseX * dpr,
                mouseY: mouseY * dpr,
            });
            console.log("Zoom scale:", ":3 " + scale);
        } else {
            /* Pan gestures */
            event.preventDefault();
            console.log("deltaX", event.deltaX, "deltaY", event.deltaY);
            invoke("canvas_pan", { dx: event.deltaX, dy: event.deltaY });
        }
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

<div
    class="canvas"
    role="application"
    bind:this={element}
    on:wheel={handleWheel}
    on:mousemove={handleMouseMove}
></div>

<style>
    .canvas {
        flex: 1;
        /*background-color: red;*/
    }
</style>
