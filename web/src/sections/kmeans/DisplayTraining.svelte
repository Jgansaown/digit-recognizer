<script lang="ts">
    // UI
    import type { ClusterInfo } from "./cluster";

    export let clusters: ClusterInfo[][] = [];
    
    let follow: boolean = false;

    let iter = 0;

    $: if (clusters.length <= 1) {
        iter = 0;
    }

    $: if (follow) {
        iter = clusters.length - 1;
    }
</script>

<h3>Iteration #{iter}</h3>
<div class="container">
    {#each clusters[iter] as { img, label, num_of_data }, i}
        <div>
            <img src={img} alt={`cluster ${i}`} />
        </div>
    {/each}
</div>

<form class="table">
    <div class="row">
        <!-- <label class="col" for="iter_range"> Show Iteration: </label> -->
        <span class="col">0</span>
        <input
            class="col"
            id="iter_range"
            type="range"
            bind:value={iter}
            min="0"
            max={clusters.length - 1}
        />
        <span class="col">{clusters.length - 1}</span>
    </div>
</form>
<div class="row">
    <label class="col" for="follow">Follow Latest Iteration</label>
    <input class="col" id="follow" type="checkbox" bind:checked={follow} />
</div>

<style>
    .container {
        display: flex;
        flex-flow: row wrap;
        justify-content: center;
        gap: 5px;
    }

    img {
        height: 5em;
        width: 5em;
    }

    form.table {
        display: table;
        margin: auto;
    }
    form.table .row {
        display: table-row;
        vertical-align: middle;
    }
    form.table .col {
        display: table-cell;
        vertical-align: middle;
    }
</style>
