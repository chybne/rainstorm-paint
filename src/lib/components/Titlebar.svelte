<script lang="ts">
    // You can expose a prop to change the title dynamically
    let app_name = "rainstorm-paint";
    export let title = "Untitled";
</script>

<div class="titlebar" data-tauri-drag-region>
    <span class="title-text" data-tauri-drag-region>
        {app_name + " — " + title}
    </span>

    <div class="controls"></div>
</div>

<style>
    .titlebar {
        height: 34px; /* Matches the slightly taller Zed/Editor feel */
        background-color: var(
            --background
        ); /* Match your app background or slightly lighter */
        display: flex;
        align-items: center;
        justify-content: center; /* Centers the title */

        /* CRITICAL: Reserve space for macOS Traffic Lights */
        padding-left: 80px;
        padding-right: 16px;

        /* Prevent highlighting the title text */
        user-select: none;
        -webkit-user-select: none;

        border-bottom: 1px solid var(--background-dark);
    }

    .title-text {
        color: var(--text);
        font-size: 12px;
        font-weight: 400;
        font-family: var(--font-system);
        letter-spacing: 0.5px;
    }

    /* If you add buttons inside .controls,
     make sure to set specific drag-region styling so they are clickable
  */
    .controls {
        margin-left: auto; /* Pushes controls to the right */
    }
</style>
