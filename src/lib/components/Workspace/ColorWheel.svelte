<script lang="ts">
    import { getAllWindows } from "@tauri-apps/api/window";
    import { getAngleFromPoint, angleToHue, ColorWheel } from "./color-wheel/utils.svelte";

    let canvas: HTMLCanvasElement;
    
    let hue = $state(0);
    const size = 270;
    const colorWheelDraw = new ColorWheel(size);

    let isPointerDown = false
    function handlePointerDown(event: PointerEvent) {
        isPointerDown = true;
        canvas.setPointerCapture(event.pointerId);

        const rect = canvas.getBoundingClientRect();
        const x = event.clientX - rect.left;
        const y = event.clientY - rect.top;

        const centerX = size / 2;
        const centerY = size / 2;

        const dx = x - centerX;
        const dy = y - centerY;
        const distance = Math.sqrt(dx * dx + dy * dy);

        const outerRadius = size / 2 - 15;
        const wheelThickness = size * 0.1;
        const innerRadius = outerRadius - wheelThickness;

        // Ignore clicks outside the ring
        if (distance < innerRadius || distance > outerRadius) return;

        const angle = getAngleFromPoint(x, y, centerX, centerY);
        hue = angleToHue(angle);

        console.log("Selected color:", hue);
    }
    function handlePointerMove(event: PointerEvent) {
        if (!isPointerDown) return;

        const rect = canvas.getBoundingClientRect();
        const x = event.clientX - rect.left;
        const y = event.clientY - rect.top;

        const centerX = size / 2;
        const centerY = size / 2;

        const angle = getAngleFromPoint(x, y, centerX, centerY);
        hue = angleToHue(angle);

        console.log("Selected color:", hue);
        
    }
    function handlePointerUp(event: PointerEvent) {
        isPointerDown = false;
        canvas.releasePointerCapture(event.pointerId);
    }

    $effect(() => {
        if (!canvas) return;
        let ctx = canvas.getContext("2d");
        if (!ctx) return;

        const dpr = window.devicePixelRatio || 1;
        canvas.width = size * dpr;
        canvas.height = size * dpr;
        ctx.scale(dpr, dpr);

        colorWheelDraw.drawWheel(hue, ctx);
    })
</script>

<div class="container"
    style="--size: {size}px;"
>
    <canvas 
        bind:this={canvas}
        onpointerdown={handlePointerDown}
        onpointermove={handlePointerMove}
        onpointerup={handlePointerUp}
        width={size}
        height={size}
    > </canvas>
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