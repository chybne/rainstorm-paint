<script lang="ts">
    import {
        getAngleFromPoint,
        angleToHue,
        ColorWheel,
    } from "./color-wheel/utils.svelte";
    import { appState } from "$lib/state/AppState.svelte";

    let canvas: HTMLCanvasElement;

    let hue = $state(0);
    const size = 270;
    const colorWheelDraw = new ColorWheel(size);

    let isPointerDown = false;
    let isSelectingHue = false;
    function handlePointerDown(event: PointerEvent) {
        isPointerDown = true;
        canvas.setPointerCapture(event.pointerId);

        const rect = canvas.getBoundingClientRect();
        const x = event.clientX - rect.left;
        const y = event.clientY - rect.top;

        let newColor = colorWheelDraw.handleWidgetClick(
            x,
            y,
            appState.getColor(),
        );

        if (newColor) {
            appState.setColor(newColor);
            isSelectingHue = true;
            console.log("Selected color:", hue);
        }
    }
    function handlePointerMove(event: PointerEvent) {
        if (!isPointerDown) return;

        const rect = canvas.getBoundingClientRect();
        const x = event.clientX - rect.left;
        const y = event.clientY - rect.top;

        let newColor = colorWheelDraw.handleWidgetClickMove(
            x,
            y,
            appState.getColor(),
        );

        if (newColor) appState.setColor(newColor);
    }
    function handlePointerUp(event: PointerEvent) {
        isPointerDown = false;
        isSelectingHue = false;
        colorWheelDraw.handleWidgetClickUp();
        canvas.releasePointerCapture(event.pointerId);
    }

    $effect(() => {
        let ctx = canvas.getContext("2d");
        if (!ctx) return;

        const dpr = window.devicePixelRatio || 1;
        canvas.width = size * dpr;
        canvas.height = size * dpr;
        ctx.scale(dpr, dpr);

        canvas.style.width = `${size}px`;
        canvas.style.height = `${size}px`;

        console.log(appState.getColor());
        colorWheelDraw.drawWheel(appState.getColor(), ctx);
    });
</script>

<div class="container" style="--size: {size}px;">
    <canvas
        bind:this={canvas}
        onpointerdown={handlePointerDown}
        onpointermove={handlePointerMove}
        onpointerup={handlePointerUp}
        width={size}
        height={size}
    >
    </canvas>
</div>

<style>
    .container {
        background-color: var(--background);
        outline: 1px solid var(--background-dark);
        width: var(--size);
        height: var(--size);

        display: flex;
        justify-content: center;
        align-items: center;
    }
</style>
