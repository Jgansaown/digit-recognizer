<script lang="ts">
    // UI
    import { afterUpdate, onDestroy, onMount } from "svelte";
    import DisplayTraining from "./DisplayTraining.svelte";
    import Settings from "./Settings.svelte";
    // Rust Wasm
    import { Kmeans } from "../common/load_worker";
    import { get_black_image } from "@wasm/kmeans";
    // Types
    import type { jsDataset } from "../common/mnist.dataset";
    import type { ClusterInfo } from "./cluster";

    export let js_dataset: jsDataset;

    const kmeans = new Kmeans();

    let num_k = 10;
    let min_change = 100.0;
    let max_iter = 10;

    let diff: number = 0.0;
    let display_clusters: ClusterInfo[][] = [];
    let iter_count: number = 0;

    onMount(async () => {});
    onDestroy(async () => {
        await kmeans.free();
    });

    $: if (num_k && iter_count == 0) {
        display_clusters = [[]];
        for (let i = 0; i < num_k; i++) {
            display_clusters[0].push({
                img: get_black_image(),
                label: 0,
                num_of_data: 0,
            });
        }
    }

    let is_running: boolean = false;
    async function start_training(k: number) {
        await kmeans.init(k, js_dataset.data, js_dataset.label);
        display_clusters = [await kmeans.info()];
        iter_count = 1;
        is_running = true;
        while (is_running) {
            diff = await kmeans.step();
            display_clusters = [...display_clusters, await kmeans.info()];
            iter_count += 1;
            if (diff < min_change || iter_count > max_iter) {
                break;
            }
        }
        is_running = false;
    }

    function reset_kmeans() {
        display_clusters = [];
        iter_count = 0;
        is_running = false;
    }
</script>


<h2>K-Means Clustering</h2>

<h3>Training Settings</h3>
<Settings bind:num_k bind:min_change bind:max_iter />

<button on:click={() => start_training(num_k)}>Start Training</button>
<button on:click={() => reset_kmeans()}>Reset</button>

<DisplayTraining clusters={display_clusters}/>

<style>
    h3 {
        margin: 0px;
    }
</style>
