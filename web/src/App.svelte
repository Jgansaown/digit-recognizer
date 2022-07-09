<script lang="ts">
    // UI
    import { onDestroy, onMount } from "svelte";
    // Dataset
    import { load_all } from "./lib/mnist.dataset";
    import type { MnistDataset } from "./lib/mnist.dataset";

    import KMeansClustering from "./sections/kmeans/KMeansClustering.svelte";

    // Rust wasm
    // import init_gz from "@wasm/gz";
    // import init_kmeans from "@wasm/kmeans";
    // Fetch MNIST dataset
    // import load_mnist_dataset from "./lib/mnist.dataset";
    // import type { jsDataset } from "./lib/mnist.dataset";

    import * as Comlink from "comlink";
    import type { obj } from "./lib/workers/mnist.wasm.worker";
import PreTrainedModels from "./components/PreTrainedModels.svelte";

    let dataset: MnistDataset;

    let ml_algro: string = "kmc";

    onMount(async () => {
        dataset = await load_all();

        const NeuralNetwork = Comlink.wrap<typeof obj>(
            new Worker(
                new URL("./lib/workers/mnist.wasm.worker.ts", import.meta.url),
                {
                    type: "module",
                }
            )
        );
        await NeuralNetwork.init();

    });

    onDestroy(() => {});
</script>

<main>
    <h1>Recognizing Handwritten Digits using Machine Learning!</h1>

    <p>Shows how different machine learning algroithms work in an interactive way</p>

    
    <h2>Try the Pre-Trained Models</h2>
    <PreTrainedModels />
    
    <h2>Train your own models</h2>

    <p>Select Machine Learning Algorithm:</p>
    <div id="algro_select">
        <label>
            <input type="radio" bind:group={ml_algro} value={"kmc"} />
            K-Means Clustering
        </label>
        <label>
            <input type="radio" bind:group={ml_algro} value={"knn"} disabled />
            K-Nearest Neighbours
        </label>
        <label>
            <input type="radio" bind:group={ml_algro} value={"nn"} disabled />
            Neural Network
        </label>
        <label>
            <input type="radio" bind:group={ml_algro} value={"cnn"} disabled />
            Convolutional Neural Network
        </label>
    </div>

    <!-- Training -->
    {#if dataset == undefined}
        <p>Loading Training Dataset...</p>
    {:else}
        <div>
            {#if ml_algro == "kmc"}
                <KMeansClustering js_dataset={dataset.training} />
            {:else if ml_algro == "knn"}
                <h2>K Nearest Neighbours</h2>
            {:else if ml_algro == "nn"}
                <h2>Neural Network</h2>
            {:else if ml_algro == "cnn"}
                <h2>Convolutional Neural Network</h2>
            {/if}
        </div>
    {/if}

    <!-- Testing -->
</main>

<footer>
    <p>
        This site is built using Typescript, Svelte and Vite, all of the machine
        learning is powered by WASM written in Rust
    </p>
</footer>

<style>
    :root {
        font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto,
            Oxygen, Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
    }

    main {
        text-align: center;
        padding: 1em;
        margin: 0 auto;
        width: 80%;
    }

    footer {
        text-align: center;
    }

    h1 {
        font-size: 2rem;
        font-weight: 100;
        line-height: 1.1;
        margin: 2rem auto;
        max-width: 14rem;
    }

    #algro_select {
        display: flex;
        flex-flow: column wrap;
        justify-content: flex-start;
        align-content: space-around;
        align-items: flex-start;
    }

    @media (min-width: 480px) {
        h1 {
            max-width: none;
        }

        #algro_select {
            flex-flow: row wrap;
            justify-content: center;
            gap: 0px 1em;
        }
    }
</style>
