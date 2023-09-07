<script lang="ts">
    import KMeansParamComponent from "./param/KMeansParam.svelte";
    import KNearestNeighborsParamComponent from "./param/KNearestNeighborsParam.svelte";
    import PerceptronParamComponent from "./param/PerceptronParam.svelte";

    import { onMount } from "svelte";
    import { fabric } from "fabric";
    import {
        create_prediction_chart,
        create_training_chart,
        type Chart,
    } from "./chart";
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
    let probability_chart: {
        chart: Chart<"bar", number[], number>;
        update(data: number[]): void;
        clear(): void;
    };

    onMount(() => {
        training_chart = create_training_chart("training_chart");

        probability_chart = create_prediction_chart("probability_chart");

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

                worker.predict(gray);

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
        training_chart.append([data.training_err, 0], data.i);

        if (training_chart.chart.options.plugins?.subtitle?.text) {
            const err = Math.trunc(data.training_err * 100);
            training_chart.chart.options.plugins.subtitle.text = `Iteration: ${data.i}, Error Rate: ${err}%`;
        }
    };

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

        probability_chart.clear();
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

    worker.onprediction = (data) => {
        probability_chart.update(data);
    };
</script>

<header>
    <h1>Train your own AI locally in your browser!</h1>
    <label>
        Select a model type to train:
        <select bind:value={state.selected}>
            <option value="kmeans">K Means Clustering</option>
            <option value="knn">K Nearest Neighbors</option>
            <option value="perceptron">
                Perceptron (1-layer Neural Network)
            </option>
        </select>
    </label>
</header>

{#if state.selected == "kmeans"}
    <h3>K Means Clustering</h3>
    <p>
        The K Means clustering is an algorithm that groups data into <i>K</i> number
        of clusters by assigning each observation to a cluster with the closest centroid
        (center of cluster).
    </p>
    <!-- <p>
        Training is done by first generating K number of centroids randomly,
        then iterating through each training observations and assigning them to
        the cluster with closest centroid. Then recalculating each cluster's
        centroid by finding the mean of the cluster. And repeat
    </p>
    <p style="margin-bottom: 0px">References:</p>
    <ul>
        <li>
            <a href="https://en.wikipedia.org/wiki/K-means_clustering"
                >https://en.wikipedia.org/wiki/K-means_clustering</a
            >
        </li>
        <li>
            <a
                href="https://scikit-learn.org/stable/modules/clustering.html#k-means"
                >https://scikit-learn.org/stable/modules/clustering.html#k-means</a
            >
        </li>
    </ul> -->
    <KMeansParamComponent bind:param={state.param.kmeans} />
{:else if state.selected == "knn"}
    <h3>K Nearest Neighbors</h3>
    <KNearestNeighborsParamComponent bind:param={state.param.knn} />
{:else if state.selected == "perceptron"}
    <h3>Perceptron (1-layer Neural Network)</h3>
    <PerceptronParamComponent bind:param={state.param.perceptron} />
{/if}

<div>
    <canvas id="training_chart" />
</div>
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

<footer>
    <h5>
        Evaluate the model using MNSIT testing dataset (10,000 observations):
    </h5>
    <p>Error Rate:</p>
    <p>Actual vs Predicted Matrix</p>
    <button>Evaluate</button>

    <h5>Test the model by drawing a digit yourself:</h5>
    <div class="grid">
        <div>
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
            <p>Your model is "00"% sure this is a "digit"</p>
            <canvas id="probability_chart" />
        </div>
    </div>
</footer>

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
