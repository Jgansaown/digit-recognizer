<script lang="ts">
    export let num_k: number = 0;
    export let min_change: number = 0;
    export let max_iter: number = 0;
    export let init_choice: string = "random";
</script>

<div class="setting">
    <div class="col">
        <h4>Initial Condition</h4>
        <label for="k_range"> Number of Clusters (K): </label>
        <label for="k_range">{num_k}</label>
        <input id="k_range" type="range" bind:value={num_k} min="0" max="100" />

        <label for="choice">Choose Initial Centroids</label>
        <div id="choice" style="display: flex;">
            <label>
                <input type="radio" bind:group={init_choice} value={"random"} />
                Random
            </label>
            <label>
                <input
                    type="radio"
                    bind:group={init_choice}
                    value={"k-means++"}
                    disabled={true}
                />
                K-Means++
            </label>
        </div>
    </div>
    <div class="col">
        <h4>Stop Condition</h4>
        <label for="c_range"> Minimum Change in Centroid: </label>
        <label for="c_range">{min_change}</label>
        <input
            id="c_range"
            type="range"
            bind:value={min_change}
            min="0"
            max="100"
            step="0.01"
        />
        <label for="i_range"> Maximum Iterations: </label>
        <label for="i_range">{max_iter}</label>
        <input
            id="i_range"
            type="range"
            bind:value={max_iter}
            min="0"
            max="100"
        />
    </div>
</div>

<style>
    div.setting {
        display: flex;
        flex-flow: row wrap;
        justify-content: center;
        gap: 0px 10px;
    }
    div.col {
        display: flex;
        flex-direction: column;
        align-items: center;
        align-content: center;
        width: 250px;
    }
    input {
        width: 90%;
    }
</style>
