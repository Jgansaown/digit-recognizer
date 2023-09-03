<script lang="ts">
    import KMeansParamComponent from "./param/KMeansParam.svelte";
    import KNearestNeighborsParamComponent from "./param/KNearestNeighborsParam.svelte";
    import PerceptronParamComponent from "./param/PerceptronParam.svelte";

    import { onMount } from "svelte";
    import { Chart } from "chart.js/auto";
    import { create_training_chart } from "./chart";
    import { WASMWorker } from "./worker/wasm";

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

    onMount(() => {
        training_chart = create_training_chart("training_chart");
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
