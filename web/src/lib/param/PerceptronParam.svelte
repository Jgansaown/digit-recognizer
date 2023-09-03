<script lang="ts">
    export let param: PerceptronParam;
</script>

<label>
    Learning Rate
    <input
        type="number"
        bind:value={param.learning_rate}
        min="0.0"
        step="0.01"
    />
</label>
<label>
    Max Iteration
    <input type="number" bind:value={param.max_iter} min="1" />
</label>
