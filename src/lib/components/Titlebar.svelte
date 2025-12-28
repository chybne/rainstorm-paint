<script lang="ts">
    import { platform } from '@tauri-apps/plugin-os';
    import { getCurrentWindow } from '@tauri-apps/api/window';
    import { onDestroy, onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/core';

    const os = platform();
    const appWindow = getCurrentWindow();

    // You can expose a prop to change the title dynamically
    let app_name = "rainstorm-paint";
    let { title } = $props();


    let isMaximized = $state(false);

    let unlistenMax: () => void;
    onMount(async () => {
        isMaximized = await appWindow.isMaximized();

        unlistenMax = await appWindow.onResized(async () => {
            isMaximized = await appWindow.isMaximized();
        });
    });

    onDestroy(() => {
        unlistenMax();
    })

    /* 
     * Hopefully a temperary hack,
     * I want to create a tauri plugin that
     * customizes the titlebar natively
     */
    let timer: number;
    function handleOverlayHoverStart() {
        timer = setTimeout(showSnapOverlay, 620);
    }
    function handleOverlayHoverCancel() {
        clearTimeout(timer);
    }

    function showSnapOverlay() {
        appWindow.setFocus().then(() => invoke("show_snap_overlay"));
    }

    function handleToggleMaximize() {
        clearTimeout(timer);
        appWindow.toggleMaximize()
    }

    function handleMinimize() {
        appWindow.minimize()
    }

    function handleClose() {
        appWindow.close()
    }

</script>

<div class="titlebar {os}" data-tauri-drag-region>
    <span class="title-text" data-tauri-drag-region>
        {app_name + " — " + title}
    </span>

    {#if os === "windows"}
        <div class="controls">
            <button class="window-controls" id="titlebar-minimize" title="minimize" onclick={handleMinimize}>
                <!-- <Minus class="icon" size={16}/> -->
                <span>&#xE921;</span>
            </button>
            <button class="window-controls" id="titlebar-maximize" title="maximize" onclick={handleToggleMaximize} onmouseenter={handleOverlayHoverStart} onmouseleave={handleOverlayHoverCancel}>
                {#if isMaximized} 
                    <span>&#xE923;</span>
                {:else}
                    <span>&#xE922;</span>
                {/if}
            </button>
            <button class="window-controls" id="titlebar-close" title="close" onclick={handleClose}>
                <span>&#xE8BB;</span>
            </button>
        </div>
    {/if}
</div>

<style>
    .titlebar {
        height: 34px;
        background-color: var(--background);
        display: flex;
        align-items: center;

        /* Prevent highlighting the title text */
        user-select: none;
        -webkit-user-select: none;

        border-bottom: 1px solid var(--background-dark);
    }
    
    .titlebar.macos {
        /* ADD SPACE FOR MACOS */
        padding-left: 80px;
        padding-right: 16px;
    }

    .title-text {
        color: var(--text);
        font-size: 12px;
        font-weight: 400;
        font-family: var(--font-system);
        letter-spacing: 0.5px;
        margin-left: 15px;
    }
    
    .window-controls {
        height: 100%;
        width: 2.75rem;
        background-color: var(--background);
        color: var(--text);
        border: none;

        font-family: "Segoe Fluent Icons", "Segoe MDL2 Assets", sans-serif;
        font-size: 10px;
    }

    .window-controls:hover {
        background-color: var(--background-dark);
    }

    #titlebar-close:hover {
        background-color: var(--secondary);
    }

    /* If you add buttons inside .controls,
     make sure to set specific drag-region styling so they are clickable
  */
    .controls {
        display: flex;
        height: 100%;
        margin-left: auto; /* Pushes controls to the right */
    }
</style>
