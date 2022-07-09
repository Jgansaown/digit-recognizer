<script lang="ts">
    import { onMount } from "svelte";

    // grab list of pre-trained models and load them

    import Canvas2 from "./Canvas2.svelte";

    const models = [
        {
            id: "knn_10",
            name: "K Nearest Neighbours (K=10)",
            details: {
                type: "knn",
            },
        },
        {
            id: "kmc",
            name: "K Means Clustering",
            details: {
                type: "kmc",
            },
        },
        {
            id: "nn",
            name: "Neural Network",
            details: {
                type: "nn",
            },
        },
        {
            id: "cnn",
            name: "Convolutional Neural Network",
            details: {
                type: "cnn",
            },
        },
    ];

    let selected: string;

    $: if (selected) {
        console.log(selected);
    }

    onMount(() => {
    });

    let canvas: Canvas2;
</script>

<div class="container">
    <div class="item">
        <p>Select model:</p>
        <select bind:value={selected}>
            {#each models as { id, name }, i}
                <option value={id}>{name}</option>
            {/each}
        </select>
        <button>Load</button>
    </div>
    <div class="item" id="canvas_container">
        <Canvas2
            bind:this={canvas}
            props={{
                width: 28,
                height: 28,
                background: "white",
                color: "black",
            }}
        />
        <button
            on:click={() => {
                canvas.reset();
            }}>Clear</button
        >
    </div>
    <div class="item">
        <p>Predicted: 0</p>
    </div>
</div>

<style>
    .container { 
        display: flex;
        flex-flow: row wrap;
        justify-content: center;
        align-items: flex-start;

        border: 1px solid red;
    }
    .item {
        flex-grow: 1;
        border: 1px solid blue;
    }
    /* #canvas_container {
        max-width: 300px;
        border: 1px solid black;
    } */
</style>
