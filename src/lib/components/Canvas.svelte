<script lang="ts">
    import { vec2 } from "gl-matrix";
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { Tool, getActiveTool } from "$lib/context/toolContext";

    let toolState = getActiveTool();

    let activeTool = $derived(toolState.tool);

    let scale = 1.0;
    let offsetX = 0.0;
    let offsetY = 0.0;
    let mouseX = 0.0;
    let mouseY = 0.0;

    let isMouseDown = false;

    // this element is binded to the canvas
    let element: HTMLDivElement;

    function handlePointerEnter(event: PointerEvent) {
        console.log("pointer entered!", event);
    }

    function handlePointerDown(event: PointerEvent) {
        if (activeTool !== Tool.Brush) return;
         
        invoke('process_canvas_input', {
            input: {
                type: "beginStroke",
                posX: event.pageX,
                posY: event.pageY,
                pressure: event.pointerType === "mouse" ? 1.0 : event.pressure
            }
        });
    }
    function handlePointerMove(event: PointerEvent) {
        if (activeTool !== Tool.Brush || !isMouseDown) return;

        invoke('process_canvas_input', {input: {
            type: "continueStroke",
            posX: event.pageX,
            posY: event.pageY,
            pressure: event.pointerType === "mouse" ? 1.0 : event.pressure
        }});
    }

    function handlePointerUp(event: PointerEvent) {
        if(activeTool !== Tool.Brush) return;

        console.log("PointerUpEvent", event.pressure);
        invoke('process_canvas_input', {input: {
            type: "endStroke",
            posX: event.pageX,
            posY: event.pageY,
            pressure: event.pressure
        }})
    }
    

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
        console.log("dpr", dpr);
    }

    function handleMouseDown(_event: MouseEvent) {
        isMouseDown = true;
    }

    function handleMouseLeave(_event: MouseEvent) {
        isMouseDown = false;
    }

    function handleMouseUp(_event: MouseEvent) {
        isMouseDown = false;
    }

    function handleMouseMove(event: MouseEvent) {
        if (isMouseDown && activeTool === Tool.Pan) {
            offsetX -= event.movementX;
            offsetY -= event.movementY;
            invoke("process_canvas_input", {input: {
                type: "panCanvas",
                offsetX,
                offsetY
            }});
        }

        mouseX = event.pageX;
        mouseY = event.pageY;
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
        } else {
            /* Pan gestures */
            event.preventDefault();
            offsetX += event.deltaX * 2.0;
            offsetY += event.deltaY * 2.0;

            invoke("process_canvas_input", {
                input: {
                    type: "panCanvas",
                    offsetX: offsetX,
                    offsetY: offsetY,
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

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<div
    class="canvas"
    role="application"
    aria-label="dd"
    bind:this={element}
    onwheel={handleWheel}
    onmousemove={handleMouseMove}
    onmousedown={handleMouseDown}
    onmouseleave={handleMouseLeave}
    onmouseup={handleMouseUp}
    onpointerenter={handlePointerEnter}
    onpointermove={handlePointerMove}
    onpointerdown={handlePointerDown}
    onpointerup={handlePointerUp}
></div>

<style>
    .canvas {
        flex: 1;
        /*background-color: red;*/
    }
</style>
