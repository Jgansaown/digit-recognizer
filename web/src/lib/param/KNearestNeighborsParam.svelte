<script lang="ts">
    export let param: KNearestNeighborsParam;
</script>

<label>
    K
    <input type="number" bind:value={param.k} />
</label>
