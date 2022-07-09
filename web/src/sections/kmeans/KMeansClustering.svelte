<script lang="ts">
    // UI
    import { afterUpdate, onDestroy, onMount } from "svelte";
    import DisplayTraining from "./DisplayTraining.svelte";
    import Settings from "./Settings.svelte";
    import Canvas from "../../components/Canvas.svelte";
    // Rust Wasm
    // import { Kmeans } from "./worker/load";
    // import { get_black_image } from "@wasm/kmeans";
    
    import black_square from "/black.png";

    // Types
    import type { jsDataset } from "src/lib/mnist.dataset";
    import type { ClusterInfo } from "./cluster";

    export let js_dataset: jsDataset;

    // const kmeans = new Kmeans();

    let num_k = 10;
    let min_change = 100.0;
    let max_iter = 10;

    let diff: number = 0.0;
    let display_clusters: ClusterInfo[][] = [];
    let iter_count: number = 0;

    let canvas_imagedata: ImageData;

    onMount(async () => {
        // await kmeans.init();
        // await kmeans.load_dataset(js_dataset.data, js_dataset.label);
    });
    onDestroy(async () => {
        // await kmeans.free();
    });

    $: if (num_k && iter_count == 0) {
        display_clusters = [[]];
        for (let i = 0; i < num_k; i++) {
            display_clusters[0].push({
                // img: get_black_image(),
                img: black_square,
                label: 0,
                num_of_data: 0,
            });
        }
    }

    $: if (canvas_imagedata) {
        const new_img = Uint8Array.from(canvas_imagedata.data);
        console.log(canvas_imagedata.data);
        console.log(new_img);
        
        // kmeans
        //     .test_one_rgba(new_img)
        //     .then((label) => {
        //         console.log(label);
        //     });
    }

    let is_running: boolean = false;
    async function start_training(k: number) {
        // await kmeans.new(k);
        // display_clusters = [await kmeans.info()];
        // iter_count = 1;
        // is_running = true;
        // while (is_running) {
        //     diff = await kmeans.step();
        //     display_clusters = [...display_clusters, await kmeans.info()];
        //     iter_count += 1;
        //     if (diff < min_change || iter_count > max_iter) {
        //         break;
        //     }
        // }
        // is_running = false;
    }

    function reset_kmeans() {
        display_clusters = [];
        iter_count = 0;
        is_running = false;
    }
</script>

<h2>K-Means Clustering</h2>

<h3>Training</h3>
<div class="content">
    <div>

        <Settings bind:num_k bind:min_change bind:max_iter />
        
        <button on:click={() => start_training(num_k)}>Start Training</button>
        <button on:click={() => reset_kmeans()}>Reset</button>
    </div>
    <div>
        <DisplayTraining clusters={display_clusters} />
    </div>
</div>

<h3>Testing</h3>
<div class="content">
    <div>

        <p>asd</p>
        
        <button>Start Testing</button>
    </div>
    
    <div>
        <Canvas bind:imagedata={canvas_imagedata} />
    </div>
</div>

<style>
    h3 {
        /* margin: 0px; */
    }

    .content {
        display: flex;
        flex-flow: column nowrap;
    }
</style>
