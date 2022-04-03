<script lang="ts">
    // UI
    import { onDestroy, onMount } from "svelte";
    import KMeansClustering from "./lib/kmeans/KMeansClustering.svelte";
    // Rust wasm
    import init_gz from "@wasm/gz";
    import init_kmeans from "@wasm/kmeans";
    // Fetch MNIST dataset
    import load_mnist_dataset from "./lib/common/mnist.dataset";
    import type { jsDataset } from "./lib/common/mnist.dataset";

    // import { get_prime_list } from "./lib/common/load_worker";
    // let k = 1_000_000;
    // let dur = 0;
    // let prime_list = [];

    // async function test_func() {
    //     const ret = await get_prime_list(k);
    //     prime_list = ret.list;
    //     dur = ret.dur;
    // }

    let training_dataset: jsDataset = undefined;
    let testing_dataset: jsDataset = undefined;

    let ml_algro: string = "kmc";

    onMount(async () => {
        // init all wasm modules
        await Promise.all([init_gz(), init_kmeans()]);
        // Loading training and testing dataset
        load_mnist_dataset().then(({ training: ds1, testing: ds2 }) => {
            training_dataset = ds1;
            testing_dataset = ds2;
        });
    });

    onDestroy(() => {});
</script>
<!-- 
<div>
    <p>Number of Prime Numbers from 1 to {k}: {prime_list.length}</p>
    <p>Last 10 Prime Numbers: {prime_list.slice(-10).join(", ")}</p>
    <label>K: <input type="number" bind:value={k} /></label>
    <button on:click={test_func}>Calculate</button>
    <p style="display: inline;">Duration: {dur} ms</p>
</div> -->

<main>
    <h1>Recognizing Handwritten Digits using Machine Learning!</h1>

    <div>
        <p>Select Machine Learning Algorithm:</p>
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
    {#if training_dataset == undefined}
        <p>Loading Training Dataset...</p>
    {:else}
        <div>
            {#if ml_algro == "kmc"}
                <KMeansClustering js_dataset={training_dataset} />
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

    @media (min-width: 480px) {
        h1 {
            max-width: none;
        }
    }
</style>
