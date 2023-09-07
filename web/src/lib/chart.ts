import {
    Chart,
    LineController,
    LineElement,
    PointElement,
    BarController,
    BarElement,
    LinearScale,
    CategoryScale,
    Colors,
    Tooltip,
    Title,
    SubTitle,
    Decimation,
    type ChartItem,
} from "chart.js";
// import { MatrixController, MatrixElement } from "chartjs-chart-matrix";

Chart.register(
    LineController,
    LinearScale,
    LineElement,
    PointElement,
    BarController,
    BarElement,
    CategoryScale,
    Colors,
    Tooltip,
    Title,
    SubTitle,
    Decimation
);

function getStyle(property: string): string {
    return getComputedStyle(document.body).getPropertyValue(property);
}

Chart.defaults.color = () => getStyle("--color");
Chart.defaults.borderColor = () => getStyle("--muted-color");

export function create_training_chart(item: ChartItem) {
    const chart: Chart<"line", number[], number> = new Chart(item, {
        type: "line",
        data: {
            labels: [],
            datasets: [{ label: "Training Dataset", data: [] }],
        },
        options: {
            responsive: true,
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

// reexport Chart
export { Chart } from "chart.js";
