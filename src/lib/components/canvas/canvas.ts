import { invoke } from "@tauri-apps/api/core";
import { vec2 } from "gl-matrix";
import { Tool } from "$lib/context/toolContext";


export let scale = 1.0;
export let offsetX = 0.0;
export let offsetY = 0.0;
export let mouseX = 0.0;
export let mouseY = 0.0;

export let isPointerDown = false;
export function setIsPointerDown(bool: boolean) {
    isPointerDown = bool;
}

abstract class ToolStrategy {
    handlePointerDown(event: PointerEvent): void {
        console.log(`${this.constructor.name} does not support handlePointerDown`);
    }

    handlePointerMove(event: PointerEvent): void {
        console.log(`${this.constructor.name} does not support handlePointerMoved`);
    }

    handlePointerUp(event: PointerEvent): void {
        console.log(`${this.constructor.name} does not support handlePointerUp`);
    }
}

class BrushToolStrategy extends ToolStrategy {
    handlePointerDown(event: PointerEvent): void {
        isPointerDown = true;

        invoke('process_canvas_input', {
            input: {
                type: "beginStroke",
                posX: event.pageX,
                posY: event.pageY,
                pressure: event.pointerType === "mouse" ? 1.0 : event.pressure
            }
        });
    }
    handlePointerMove(event: PointerEvent): void {
        if (!isPointerDown) return;

        invoke('process_canvas_input', {input: {
            type: "continueStroke",
            posX: event.pageX,
            posY: event.pageY,
            pressure: event.pointerType === "mouse" ? 1.0 : event.pressure
        }});
    }
    handlePointerUp(event: PointerEvent): void {
        isPointerDown = false;
        invoke('process_canvas_input', {input: {
            type: "endStroke",
            posX: event.pageX,
            posY: event.pageY,
            pressure: event.pressure
        }})
    }
}

class PanToolStrategy extends ToolStrategy {
    handlePointerDown(event: PointerEvent): void {
        isPointerDown = true;
    }

    handlePointerMove(event: PointerEvent): void {
        if (!isPointerDown) return;
        offsetX -= event.movementX;
        offsetY -= event.movementY;

        invoke('process_canvas_input', {input: {
            type: "panCanvas",
            offsetX,
            offsetY
        }});
    }
    handlePointerUp(event: PointerEvent): void {
        isPointerDown = false;
    }
}



class UnimplementedToolStrategy extends ToolStrategy {}
export const ToolStrategies: Record<Tool, ToolStrategy> = {
    [Tool.Brush]: new BrushToolStrategy(),
    [Tool.Pan]: new PanToolStrategy(),
    [Tool.ColorPicker]: new UnimplementedToolStrategy(),
    [Tool.Eraser]: new UnimplementedToolStrategy(),
    [Tool.Lasso]: new UnimplementedToolStrategy(),
} as const;



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

export function handleMagnifyGesture(event: WheelEvent) {
    const dpr = window.devicePixelRatio;
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
}


export function handlePanGesture(event: WheelEvent) {
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

