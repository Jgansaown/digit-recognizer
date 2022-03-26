<script lang="ts">
  // UI
  import { afterUpdate, onDestroy, onMount } from "svelte";
  // Rust Wasm
  import {
    load_mnist_data,
    kmeans_new_clusters_random,
    kmeans_assign_dataset_to_clusters,
    kmeans_recalculate_centroids,
    kmeans_get_clusters_info,
  } from "@wasm/kmeans";
  // Types
  import type { KMeansClusters, Dataset } from "@wasm/kmeans";
  import type { jsDataset } from "../common/fetch_dataset";

  interface ClusterInfo {
    img: string;
    label?: number;
    num_of_data?: number;
  }

  export let js_dataset: jsDataset;

  let dataset: Dataset;
  let clusters: KMeansClusters;

  let num_k = 10;
  let min_change = 100.0;
  let max_iter = 10;

  let diff: number = 0.0;
  let display_clusters: ClusterInfo[][] = [];

  $: console.log(display_clusters);
  $: console.log(diff);

  onMount(async () => {
    dataset = load_mnist_data(js_dataset.data, js_dataset.label);
  });
  onDestroy(() => {
    if (dataset != undefined) {
      dataset.free();
    }
    if (clusters != undefined) {
      clusters.free();
    }
  });

  function step_kmeans(): number {
    if (clusters == undefined) { return; }
    kmeans_assign_dataset_to_clusters(dataset, clusters);
    diff = kmeans_recalculate_centroids(clusters);
    display_clusters = [
      ...display_clusters,
      kmeans_get_clusters_info(clusters),
    ];
    return diff;
  }

  async function start_kmeans(k: number) {
    reset_kmeans();
    clusters = kmeans_new_clusters_random(k);
    display_clusters = [
      ...display_clusters,
      kmeans_get_clusters_info(clusters),
    ];
    await new Promise((r) => setTimeout(r, 100));

    while (true) {
      step_kmeans();
      if (diff < 100.0) {
        break;
      }
      await new Promise((r) => setTimeout(r, 100));
    }
  }

  function reset_kmeans() {
    display_clusters = [];
  }
</script>

<h2>K Means Clustering</h2>

<div id="settings">
  <h3>Settings</h3>
  <p>Initial Condition</p>
  <label>
    Number of clusters (K):
    <input type="number" bind:value={num_k} min="0" max="50" step="1" />
  </label>
  <!-- <p>Initial Clusters:</p>
  <label>
    <input type="radio" />
    Random
  </label>
  <label>
    <input type="radio" />
    K Means ++
  </label> -->

  <p>Stop condition</p>
  <label>
    Minimum change in centroid:
    <input type="number" bind:value={min_change} min="0" step="0.01" />
  </label>
  <label>
    Max iterations:
    <input type="number" bind:value={max_iter} min="0" step="1" />
  </label>
</div>

<button on:click={() => start_kmeans(num_k)}>Start</button>
<button on:click={() => step_kmeans()}>Step</button>
<button on:click={() => reset_kmeans()}>Reset</button>

<p>Clusters</p>

{#each display_clusters as cluster_info}
  <div style="display: flex">
    {#each cluster_info as info, i}
      <div style="margin: auto;">
        <img src={info.img} alt="" />
        <p>{info.label}</p>
      </div>
    {/each}
  </div>
{/each}

<!-- {#if info != undefined}
<div  style="display: flex">
  {#each info as {img, label, num_of_data}, i}
  <div style="margin: auto">
    <img src={img} alt="alt" />
    <p>{i}: {label}</p>
  </div>
  {/each}
</div>
{/if} -->
<style>
  img {
    height: 5em;
    width: 5em;
  }

  label {
    display: block;
  }

  #settings {
    width: 50%;
    margin: auto;
    border: 1px solid black;
  }
  #settings label {
    text-align: right;
  }
</style>
