import { createContext, type Component } from "svelte";
import { Brush, Eraser, Lasso, Pipette, Hand, Search } from "@lucide/svelte";

export enum Tool {
    Brush = "brush",
    Eraser = "eraser",
    Lasso = "lasso",
    ColorPicker = "color-picker",
    Pan = "pan",
    Magnify = "search"
}

export const ToolData: Readonly<Array<{tool: Tool, icon: Component; label: string}>> = [
    { tool: Tool.Brush, icon: Brush, label: "Brush" },
    { tool: Tool.Eraser, icon: Eraser, label: "Eraser" },

    /* Not planned for MVP definitely planned futrue feature */
    // { tool: Tool.Lasso, icon: Lasso, label: "Lasso"},
    { tool: Tool.ColorPicker, icon: Pipette, label: "Color Picker"},
    { tool: Tool.Pan, icon: Hand, label: "Pan"},
    { tool : Tool.Magnify, icon: Search, label: "Magnify"}
] as const;  

const [getActiveToolContext, setActiveToolContext] = createContext<{tool: Tool}>();

export function getActiveTool() {
    return getActiveToolContext();
}

export function selectTool(tool: {tool: Tool}) {
    return setActiveToolContext(tool);
}