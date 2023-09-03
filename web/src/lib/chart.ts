import { Chart, type ChartItem } from "chart.js/auto";

export function create_training_chart(item: ChartItem) {
    const chart: Chart<"line", number[], number> = new Chart(item, {
        type: "line",
        data: {
            labels: [],
            datasets: [
                { label: "Training Dataset", data: [] },
                { label: "Testing Dataset", data: [] },
            ],
        },
        options: {
            scales: {
                x: {
                    type: "linear",
                    min: 0,
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
        },
    });

    return {
        chart,
        append(data: number[], label: number) {
            chart.data.datasets.map((dataset, i) => dataset.data.push(data[i]));
            chart.data.labels?.push(label);
            chart.update();
        },
        reset() {
            chart.data.datasets.forEach((dataset) => (dataset.data = []));
            chart.data.labels = [];
            chart.update();
        },
    };
}
