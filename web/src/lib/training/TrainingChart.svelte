<script lang="ts">
    import { onMount } from "svelte";
    import {
        Chart,
        LineController,
        LineElement,
        PointElement,
        LinearScale,
        Colors,
        Tooltip,
        Title,
        SubTitle,
        type Point,
    } from "chart.js";

    Chart.register(
        LineController,
        LinearScale,
        LineElement,
        PointElement,
        Colors,
        Tooltip,
        Title,
        SubTitle
    );

    let _this: HTMLCanvasElement;
    let chart: Chart<"line", Point[]>;

    export let data: { x: number; y: number }[];

    // update chart
    $: if (data && chart && chart.data.datasets.at(0)?.data) {
        chart.data.datasets.at(0)!.data = data;

        if (chart.options.plugins?.subtitle?.text && data.at(-1)) {
            const { x, y } = data.at(-1)!;
            const err = Math.trunc(y * 100);
            chart.options.plugins.subtitle.text = `Iteration: ${x}, Error Rate: ${err}%`;
        }

        chart.update();
    }

    onMount(() => {
        chart = new Chart<"line", Point[]>(_this, {
            type: "line",
            data: {
                datasets: [{ label: "Training Dataset", data }],
            },
            options: {
                responsive: true,
                parsing: false,
                scales: {
                    x: {
                        type: "linear",
                        min: 1,
                        suggestedMax: 10,
                        title: {
                            display: true,
                            text: "# of Iteration",
                        },
                    },
                    y: {
                        type: "linear",
                        min: 0.0,
                        max: 1.0,
                        title: {
                            display: true,
                            text: "Error Rate",
                        },
                    },
                },
                plugins: {
                    title: {
                        text: "Training Progress",
                        display: true,
                    },
                    subtitle: {
                        text: "Iteration: 0, Error Rate: 100%",
                        display: true,
                    },
                },
            },
        });
    });
</script>

<canvas bind:this={_this} />
