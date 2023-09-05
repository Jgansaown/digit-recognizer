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

export function create_prediction_chart(item: ChartItem) {
    const chart: Chart<"bar", number[], number> = new Chart(item, {
        type: "bar",
        data: {
            labels: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            datasets: [{ label: "Probability", data: [] }],
        },
        options: {
            responsive: true,
            scales: {
                x: {
                    type: "linear",
                    min: 0,
                    max: 9,
                    title: {
                        display: true,
                        text: "Digit",
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
                },
            },
        },
    });

    return {
        chart,
        update(data: number[]) {
            chart.data.datasets[0].data = data;
            chart.update();
        },
        clear() {
            chart.data.datasets[0].data = [];
            chart.update();
        },
    };
}
