<script lang="ts">
    export let param: KNearestNeighborsParam;
</script>

<h4>Hyperparameters</h4>
<label>
    K
    <input type="number" bind:value={param.k} />
</label>
