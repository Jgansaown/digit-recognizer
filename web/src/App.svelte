<script lang="ts">
    import { fabric } from "fabric";
    import { onMount } from "svelte";
    import { Chart } from "chart.js/auto";

    import ModelTrainer from "./lib/ModelTrainer.svelte";

    onMount(() => {
        // //
        // const probability_data = [
        //     0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9,
        // ];
        // const probability_chart = new Chart("probability_chart", {
        //     type: "bar",
        //     data: {
        //         labels: probability_data.map((_, i) => i),
        //         datasets: [
        //             {
        //                 label: "Probability",
        //                 data: probability_data,
        //             },
        //         ],
        //     },
        //     options: {
        //         responsive: true,
        //     },
        // });

        // const canvas = new fabric.Canvas("canvas", {
        //     isDrawingMode: true,
        // });
        // canvas.freeDrawingBrush.color = "white";
        // canvas.freeDrawingBrush.width = 10;
    });
</script>

<main class="container">
    <h1>Digit Recognizer</h1>

    <!-- <h2>Try a pre-trained model</h2>
    <label>
        <select>
            <option value="model1">model 1</option>
        </select>
    </label>
    <div class="grid">
        <div>
            <p>Draw a digit:</p>
            <canvas id="canvas" />
            <button>Clear</button>
        </div>
        <div>
            <p>Probability Result</p>
            <canvas id="probability_chart" />
        </div>
    </div> -->

    <h2>Train your own model!</h2>
    <ModelTrainer />
</main>

<style>
    /* canvas {
        border: 1px solid white;
    } */
</style>
