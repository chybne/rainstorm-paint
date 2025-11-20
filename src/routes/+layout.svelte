<script lang="ts">
    import Titlebar from "$lib/components/Titlebar.svelte";
    import "../app.css";
    let { children } = $props();
</script>

<Titlebar title="Untitled" />
{@render children()}
