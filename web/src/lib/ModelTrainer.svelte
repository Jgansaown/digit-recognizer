<script lang="ts">
    import KMeansParamComponent from "./param/KMeansParam.svelte";
    import KNearestNeighborsParamComponent from "./param/KNearestNeighborsParam.svelte";
    import PerceptronParamComponent from "./param/PerceptronParam.svelte";

    import { onMount } from "svelte";
    import { fabric } from "fabric";
    import { Chart } from "chart.js/auto";
    import { create_training_chart } from "./chart";
    import { WASMWorker } from "./worker/wasm";
    import { cropImageFromCanvas, rgba_to_grayscale } from "./canvas";

    const worker = new WASMWorker();

    const state: {
        selected: ModelTypes;
        is_training: boolean;
        param: ModelParameters;
    } = {
        selected: "kmeans",
        is_training: false,
        param: {
            kmeans: {
                max_iter: 100,
                n_clusters: 10,
            },
            knn: {
                k: 10,
            },
            perceptron: {
                max_iter: 100,
                learning_rate: 0.01,
            },
        },
    };

    let training_chart: {
        chart: Chart<"line", number[], number>;
        append(data: number[], label: number): void;
        reset(): void;
    };
    let canvas: fabric.Canvas;

    onMount(() => {
        training_chart = create_training_chart("training_chart");

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

        canvas = new fabric.Canvas("canvas", {
            isDrawingMode: true,
        });
        canvas.freeDrawingBrush.color = "black";
        canvas.freeDrawingBrush.width = 15;
        canvas.backgroundColor = "rgba(255, 255, 255, 255)";

        let timeoutid: number | undefined = undefined;
        let is_drawing = false;
        let is_timeout = false;

        function fire() {
            clearTimeout(timeoutid);
            timeoutid = setTimeout(() => {
                is_timeout = true;
                (canvas.freeDrawingBrush as any)._finalizeAndAddPath();
                const data = save_canvas();
                const gray = rgba_to_grayscale(data);
                
                // TODO: worker.predict(gray);

                is_timeout = false;
            }, 50);
            is_timeout = true;
        }

        canvas.on("mouse:down", (e) => {
            is_drawing = true;
        });
        canvas.on("mouse:up", (e) => {
            is_drawing = false;
            fire();
        });
        canvas.on("mouse:move", (e) => {
            if (is_drawing && is_timeout == false) {
                // console.log(e.pointer?.x, e.pointer?.y);
                fire();
            }
        });
    });

    function start_training() {
        training_chart.reset();
        worker.start_training(state.selected, state.param[state.selected]);
    }
    function stop_training() {
        worker.stop_training();
    }

    worker.onstep = (data: {
        i: number;
        training_err: number;
        testing_err: number;
    }) => {
        console.log(
            `[main][step]: ${data.i}: train: ${data.training_err}, test: ${data.testing_err}`
        );
        training_chart.append([data.training_err, data.testing_err], data.i);
    };

    $: if (
        training_chart &&
        training_chart.chart.options.scales?.x != undefined
    ) {
        let param = state.param[state.selected];
        if ("max_iter" in param) {
            training_chart.chart.options.scales.x.max = param.max_iter;
            training_chart.chart.update();
        }
    }

    function clear_canvas() {
        canvas.clear();
        canvas.backgroundColor = "rgba(255, 255, 255, 255)";
        canvas.renderAll();

        const main = (
            document.getElementById("canvas") as HTMLCanvasElement
        ).getContext("2d", { willReadFrequently: true })!;
        const cropped = (
            document.getElementById("cropped-canvas") as HTMLCanvasElement
        ).getContext("2d", { willReadFrequently: true })!;
        const scaled = (
            document.getElementById("scaled-canvas") as HTMLCanvasElement
        ).getContext("2d", {
            willReadFrequently: true,
        })!;

        main.clearRect(0, 0, main.canvas.width, main.canvas.height);
        cropped.clearRect(0, 0, cropped.canvas.width, cropped.canvas.height);
        scaled.clearRect(0, 0, scaled.canvas.width, scaled.canvas.height);
    }
    function save_canvas() {
        const main = (
            document.getElementById("canvas") as HTMLCanvasElement
        ).getContext("2d", { willReadFrequently: true })!;
        const cropped = (
            document.getElementById("cropped-canvas") as HTMLCanvasElement
        ).getContext("2d", { willReadFrequently: true })!;
        const scaled = (
            document.getElementById("scaled-canvas") as HTMLCanvasElement
        ).getContext("2d", {
            willReadFrequently: true,
        })!;

        cropped.fillStyle = "rgba(255, 255, 255, 255)";
        cropped.fillRect(0, 0, cropped.canvas.width, cropped.canvas.height);
        cropped.save();

        const [w, h, croppedImage] = cropImageFromCanvas(main);
        cropped.canvas.width = Math.max(w, h) * 1.2;
        cropped.canvas.height = Math.max(w, h) * 1.2;
        const leftPadding = (cropped.canvas.width - w) / 2;
        const topPadding = (cropped.canvas.height - h) / 2;
        // console.log(croppedImage);
        cropped.putImageData(croppedImage, leftPadding, topPadding);

        // Copy image data to scale 28x28 canvas
        scaled.save();
        scaled.clearRect(0, 0, scaled.canvas.height, scaled.canvas.width);
        scaled.fillStyle = "rgba(255, 255, 255, 255)"; // white non-transparent color
        scaled.fillRect(0, 0, cropped.canvas.width, cropped.canvas.height);
        scaled.scale(28.0 / cropped.canvas.height, 28.0 / cropped.canvas.width);
        scaled.drawImage(cropped.canvas, 0, 0);

        const { data } = scaled.getImageData(0, 0, 28, 28)!;

        scaled.restore();

        return data;
    }
</script>

<label>
    Select a machine learning model to train:
    <select bind:value={state.selected}>
        <option value="kmeans">K Means</option>
        <option value="knn">K Nearest Neighbors</option>
        <option value="perceptron">
            Perceptron (1-layer Neural Network)
        </option>
    </select>
</label>

{#if state.selected == "kmeans"}
    <h3>K Means Hyperparameters</h3>
    <KMeansParamComponent bind:param={state.param.kmeans} />
{:else if state.selected == "knn"}
    <h3>K Nearest Neighbors Hyperparameters</h3>
    <KNearestNeighborsParamComponent bind:param={state.param.knn} />
{:else if state.selected == "perceptron"}
    <h3>Perceptron (1-layer Neural Network) Hyperparameters</h3>
    <PerceptronParamComponent bind:param={state.param.perceptron} />
{/if}

<div class="grid">
    {#if state.is_training == false}
        <button
            on:click={() => {
                state.is_training = true;
                start_training();
            }}>Start Training</button
        >
    {:else}
        <button
            on:click={() => {
                state.is_training = false;
                stop_training();
            }}>Stop Training</button
        >
    {/if}
    <!-- <button class="secondary">Save Model</button> -->
</div>

<p>
    Training {state.selected}, using params: {JSON.stringify(
        state.param[state.selected]
    )}
</p>
<canvas id="training_chart" />

<div class="grid">
    <div>
        <p>Draw a digit:</p>
        <div id="canvas-wrapper">
            <canvas id="canvas" width="300" height="300" />
        </div>
        <button on:click={clear_canvas}>Clear</button>
        <div class="grid" style="display: none;">
            <canvas id="cropped-canvas" width="28" height="28" />
            <canvas id="scaled-canvas" width="28" height="28" />
        </div>
    </div>
    <div>
        <p>Probability Result</p>
        <canvas id="probability_chart" />
    </div>
</div>

<style>
    #canvas-wrapper {
        border: 1px solid var(--contrast);
        background-color: white;
        width: 310px;
        height: 310px;
        margin: 1em auto;
        padding: 5px;
    }
    #cropped-canvas {
        /* display: none; */
        border: 1px solid var(--contrast);
        width: 100px;
        height: 100px;
    }
    #scaled-canvas {
        /* display: none; */
        border: 1px solid var(--contrast);
        width: 100px;
        height: 100px;
    }
</style>
