<script lang="ts">
    import { vec2 } from "gl-matrix";
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";

    let scale = 1.0;
    let offsetX = 0.0;
    let offsetY = 0.0;
    let mouseX = 0.0;
    let mouseY = 0.0;

    // this element is binded to the canvas
    let element: HTMLDivElement;

    function zoomRelativeToPoint(
        zoom: number,
        mouseX: number,
        mouseY: number,
    ): [number, number] {
        const offset = vec2.fromValues(offsetX, offsetY);
        const mousePos = vec2.fromValues(mouseX, mouseY);

        const result = vec2.create();
        vec2.add(result, mousePos, offset);
        vec2.scale(result, result, 1 / scale);

        const newCoords = vec2.create();
        vec2.scale(newCoords, result, zoom);
        vec2.sub(newCoords, newCoords, mousePos);

        return [newCoords[0], newCoords[1]];
    }

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
        // console.log("mouseX", mouseX, "mouseY", mouseY);
    }

    function handleWheel(event: WheelEvent) {
        const dpr = window.devicePixelRatio;

        /* for pinch gestures */
        if (event.ctrlKey) {
            event.preventDefault(); // prevent zooming the page
            const zoomFactor = 1 - event.deltaY * 0.01;
            let newScale = scale * zoomFactor;
            console.log(
                "zoomFactor: ",
                zoomFactor,
                "deltaY",
                event.deltaY,
                "new_scale",
                newScale,
            );
            /* clamp the value Definitely needs to be calculated before hand*/
            newScale = Math.min(Math.max(newScale, 0.3), 5.0);
            const [newOffsetX, newOffsetY] = zoomRelativeToPoint(
                newScale,
                mouseX * dpr,
                mouseY * dpr,
            );

            scale = newScale;
            offsetX = newOffsetX;
            offsetY = newOffsetY;

            invoke("process_canvas_input", {
                input: {
                    type: "zoomCanvas",
                    zoom: scale,
                },
            });
            invoke("process_canvas_input", {
                input: {
                    type: "panCanvas",
                    offsetX,
                    offsetY,
                },
            });

            console.log("Zoom scale:", ":3 " + scale);
        } else {
            /* Pan gestures */
            event.preventDefault();
            offsetX += event.deltaX * 2.0;
            offsetY += event.deltaY * 2.0;
            console.log("offset_x", offsetX, "offset_y", offsetY);

            invoke("process_canvas_input", {
                input: {
                    type: "panCanvas",
                    offsetX,
                    offsetY,
                },
            });
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
