<script lang="ts">
    import { greet } from "@wasm/wasm";
    import { fabric } from "fabric";
    import { Chart } from "chart.js/auto";
    import { onMount } from "svelte";

    onMount(() => {
        //
        const probability_data = [
            0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9,
        ];
        const probability_chart = new Chart("probability_chart", {
            type: "bar",
            data: {
                labels: probability_data.map((_, i) => i),
                datasets: [
                    {
                        label: "Probability",
                        data: probability_data,
                    },
                ],
            },
            options: {
                responsive: true,
            },
        });

        //
        const training_data = Array.from({ length: 100 }, (_, i) => 1 / i);
        const training_chart = new Chart("training_chart", {
            type: "line",
            data: {
                labels: training_data.map((_, i) => i),
                datasets: [
                    {
                        label: "Error Rate",
                        data: training_data,
                    },
                ],
            },
        });

        const canvas = new fabric.Canvas("canvas", {
            isDrawingMode: true,
        });
        canvas.freeDrawingBrush.color = "white";
        canvas.freeDrawingBrush.width = 10;
    });

    let selected_model: string = "kmeans";
    let kmeans_param = {
        n_clusters: 10,
        max_iter: 100,
        min_dist: 0.01,
    };
    let knn_param = {
        k: 10,
    };
    let perceptron_param = {
        learning_rate: 0.1,
        max_iter: 10,
        min_error_rate: 0.001,
    };
</script>

<main class="container">
    <h1>Digit Recognizer</h1>

    <h2>Try a pre-trained model</h2>
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
    </div>

    <h2>Train your own model!</h2>
    <label>
        Select a machine learning model to train:
        <select bind:value={selected_model}>
            <option value="kmeans">K Means</option>
            <option value="knn">K Nearest Neighbors</option>
            <option value="perceptron">
                Perceptron (1-layer Neural Network)
            </option>
        </select>
    </label>

    {#if selected_model == "kmeans"}
        <div class="param" id="kmeans_param">
            <h2>K Means Hyperparameters</h2>
            <label>
                Number of Clusters (K)
                <input type="number" bind:value={kmeans_param.n_clusters} />
            </label>
            <label>
                Max Iteration
                <input type="number" bind:value={kmeans_param.max_iter} />
            </label>
            <label>
                Min Distance
                <input type="number" bind:value={kmeans_param.min_dist} />
            </label>
        </div>
    {:else if selected_model == "knn"}
        <div class="param" id="knn_param">
            <h2>K Nearest Neighbors Hyperparameters</h2>
            <label>
                K
                <input type="number" bind:value={knn_param.k} />
            </label>
        </div>
    {:else if selected_model == "perceptron"}
        <div class="param" id="perceptron_param">
            <h2>Perceptron (1-layer Neural Network) Hyperparameters</h2>
            <label>
                Learning Rate
                <input
                    type="number"
                    bind:value={perceptron_param.learning_rate}
                />
            </label>
            <label>
                Max Iteration
                <input type="number" bind:value={perceptron_param.max_iter} />
            </label>
            <label>
                Min Error Rate
                <input
                    type="number"
                    bind:value={perceptron_param.min_error_rate}
                />
            </label>
        </div>
    {/if}

    <div class="grid">
        <button id="start">Start Training</button>
        <button id="stop">Stop</button>
    </div>

    <canvas id="training_chart" />
</main>

<style>
    canvas {
        border: 1px solid white;
    }
</style>
