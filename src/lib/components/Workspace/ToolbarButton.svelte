<script lang="ts">
    let { Icon, Label, onclick, isActive } = $props();
</script>

<button aria-label={Label} {onclick} class={isActive ? "active" : ""}>
    <Icon />
</button>

<style>
    button {
        display: flex;
        justify-content: center;
        align-items: center;

        color: var(--text);

        width: 2.5rem;
        height: 2.5rem;
        border: none;
        border-radius: 15%;
        background-color: var(--background);
    }

    button:hover {
        background-color: var(--background-dark);
        color: var(--text);
    }

    button.active {
        background-color: var(--text);
        color: var(--background);
    }
</style>
