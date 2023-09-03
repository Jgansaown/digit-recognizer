<script lang="ts">
    export let param: KMeansParam;
</script>

<label>
    Number of Clusters (K)
    <input type="number" bind:value={param.n_clusters} />
</label>
<label>
    Max Iteration
    <input type="number" bind:value={param.max_iter} />
</label>
