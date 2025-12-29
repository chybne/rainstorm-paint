<script lang="ts">
    import { goto } from "$app/navigation";
    import { invoke } from "@tauri-apps/api/core";

    let height = 500;
    let width = 500;

    let error_message = "";

    function onCreate() {
        console.log("hello");
        if (width <= 0 || height <= 0) {
            error_message = "height and width cannot be zero or lower";
        }

        invoke("attach_canvas", {
            width: width,
            height: height,
        });

        goto("/workspace");
        console.log("width: " + width);
        console.log("height: " + height);
    }
</script>

<main class="container">
    <div class="canvas-creation">
        <h1>Create a Canvas</h1>
        <div class="input-group">
            <label for="width">WIDTH: </label>
            <input
                class="textfield"
                type="number"
                id="width"
                min="1"
                max="5000"
                bind:value={width}
            />
        </div>
        <div class="input-group">
            <label for="height">HEIGHT: </label>
            <input
                class="textfield"
                type="number"
                id="height"
                min="1"
                max="5000"
                bind:value={height}
            />
        </div>
        <button type="submit" id="create-button" on:click={onCreate}
            >Create</button
        >

        {#if error_message}
            <p class="error-message">{error_message}</p>
        {/if}
    </div>
</main>

<style>
    /** {
        outline: 1px solid rebeccapurple;
    }*/

    .container {
        background-color: var(--background);

        font-family: var(--font-system);

        width: 100%;
        height: 100%;
    }

    .canvas-creation {
        display: flex;
        flex-direction: column;
        gap: 30px;

        overflow: hidden;
        background-color: var(--background);
        color: var(--text);
        padding-left: 2rem;
    }

    .textfield {
        font-size: 16px;
        font-family: var(--font-system);
        color: var(--text);

        background-color: var(--background);
        border: none;
        border-bottom: 2px solid var(--background-dark);
        outline: none;

        padding: 0.3rem;
    }

    #create-button {
        width: 10rem;

        font-size: 24px;
        color: var(--text);

        background-color: var(--background);
        border: 2px solid var(--text);
        border-radius: 0.5rem;
    }

    #create-button:hover {
        background-color: var(--background-dark);
        cursor: pointer;
    }
</style>
