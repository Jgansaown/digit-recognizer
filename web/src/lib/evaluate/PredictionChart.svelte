<script lang="ts">
    import {
        Chart,
        BarController,
        BarElement,
        CategoryScale,
        Colors,
        Tooltip,
    } from "chart.js";

    Chart.register(BarController, BarElement, CategoryScale, Colors, Tooltip);
    import { onMount } from "svelte";

    let canvas: HTMLCanvasElement;
    let chart: Chart<"bar", number[], number>;

    export let data: number[];

    $: if (data && chart) {
        chart.data.datasets[0].data = data;
        chart.update();
    }

    onMount(() => {
        chart = new Chart(canvas, {
            type: "bar",
            data: {
                labels: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
                datasets: [{ label: "Probability", data }],
            },
            options: {
                responsive: true,
                scales: {
                    x: {
                        type: "category",
                        title: {
                            display: true,
                            text: "Digit",
                        },
                        grid: {
                            display: false,
                        },
                    },
                    y: {
                        type: "linear",
                        min: 0.0,
                        max: 1.0,
                        title: {
                            display: true,
                            text: "Confidence",
                        },
                        grid: {
                            display: false,
                        },
                    },
                },
            },
        });
    });
</script>

<canvas bind:this={canvas} />
