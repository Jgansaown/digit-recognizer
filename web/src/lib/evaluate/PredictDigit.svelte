<script lang="ts">
    import DigitCanvas from "./DigitCanvas.svelte";
    import PredictionChart from "./PredictionChart.svelte";

    export let data: number[];
    let canvas: DigitCanvas;

    function clear() {
        canvas.clear_canvas();
        data = [];
    }
</script>

<div class="grid">
    <div>
        <DigitCanvas bind:this={canvas} on:fire />
        <button on:click={clear}>Clear</button>
    </div>
    <div>
        <PredictionChart {data} />
    </div>
</div>
