<!-- Button Component -->
<script>
    export let icon = null;
</script>

<button class="btn">
    <svelte:component this={icon} size="32" />
</button>

<style>
    .btn:hover {
        background-color: var(--background-dark);
    }

    .btn {
        border: 0.2rem solid var(--text);
        border-radius: 1rem;
        margin: 0.2rem;
        background-color: var(--background);
        color: var(--text);
    }
</style>
