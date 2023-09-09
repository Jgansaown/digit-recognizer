<script lang="ts">
    import KMeansParamComponent from "./param/KMeansParam.svelte";
    import KNearestNeighborsParamComponent from "./param/KNearestNeighborsParam.svelte";
    import PerceptronParamComponent from "./param/PerceptronParam.svelte";
    import TrainingChart from "./training/TrainingChart.svelte";
    import TrainingButton from "./training/TrainingButton.svelte";
    import PredictDigit from "./evaluate/PredictDigit.svelte";

    import { WasmWorker } from "./worker/wasm";

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
    let training_data: { x: number; y: number }[] = [];
    let probability_data: number[] = [];

    const worker = new WasmWorker();

    async function start_training() {
        training_data = [];

        const data = {
            type: state.selected,
            param: state.param[state.selected],
        } as ModelParametersUnion;
        await worker.send("init_model", data);
        // start training
        await worker.send("start_training");
    }

    async function stop_training() {
        // pause training
        await worker.send("stop_training", null);
    }

    async function continue_training() {
        // start training
        await worker.send("start_training", null);
    }

    async function reset_training() {
        // clear model
        await worker.send("free_model", null);

        training_data = [];
    }

    async function on_canvas_draw(data: Float64Array) {
        const prediction = await worker.send("predict", data, [data.buffer]);

        probability_data = Array.from(prediction);
    }

    worker.on_step = (data: { i: number; err: number }) => {
        console.log(`[main][step]: ${data.i}: err: ${data.err}`);

        const param = state.param[state.selected];
        const max_iter = "max_iter" in param ? param.max_iter : 1;
        if (data.i >= max_iter) {
            stop_training();
        }
        if (data.i <= max_iter) {
            training_data = [...training_data, { x: data.i, y: data.err }];
        }
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

<!-- <TrainingParam /> -->
<TrainingChart data={training_data} />
<TrainingButton
    on:start={start_training}
    on:stop={stop_training}
    on:continue={continue_training}
    on:reset={reset_training}
/>

<footer>
    <h5>Test the model using MNIST testing dataset (10,000 observations):</h5>
    <p>Error Rate:</p>
    <p>Actual vs Predicted Matrix</p>
    <button>Evaluate</button>

    <h5>Test the model by drawing a digit:</h5>
    <PredictDigit
        on:fire={({ detail }) => on_canvas_draw(detail)}
        data={probability_data}
    />
</footer>
