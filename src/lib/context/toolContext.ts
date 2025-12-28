import { createContext, type Component } from "svelte";
import { Brush, Eraser, Lasso, Pipette, Hand } from "@lucide/svelte";

export enum Tool {
    Brush = "brush",
    Eraser = "eraser",
    Lasso = "lasso",
    ColorPicker = "color-picker",
    Pan = "pan"
}

export const ToolData: Readonly<Array<{tool: Tool, icon: Component; label: string}>> = [
    { tool: Tool.Brush, icon: Brush, label: "Brush" },
    { tool: Tool.Eraser, icon: Eraser, label: "Eraser" },
    { tool: Tool.Lasso, icon: Lasso, label: "Lasso"},
    { tool: Tool.ColorPicker, icon: Pipette, label: "Color Picker"},
    { tool: Tool.Pan, icon: Hand, label: "Pan"},
] as const;  

const [getActiveToolContext, setActiveToolContext] = createContext<{tool: Tool}>();

export function getActiveTool() {
    return getActiveToolContext();
}

export function selectTool(tool: {tool: Tool}) {
    return setActiveToolContext(tool);
}