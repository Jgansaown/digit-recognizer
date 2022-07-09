<script lang="ts">
    import { onMount } from "svelte";

    export let props: {
        width?: number;
        height?: number;
        background?: string;
        color?: string;
    } = {
        width: 28,
        height: 28,
        background: "white",
        color: "black",
    };

    let canvas: HTMLCanvasElement;
    let g: CanvasRenderingContext2D;
    let drawing = false;

    export function reset() {
        g.fillStyle = props.background;
        g.fillRect(0, 0, canvas.clientWidth, canvas.clientHeight);
    }

    function calcPos(e: PointerEvent) {
        const x = (e.offsetX / canvas.clientWidth) * canvas.width;
        const y = (e.offsetY / canvas.clientHeight) * canvas.height;
        return [x, y];
    }

    function draw_start(e: PointerEvent) {
        drawing = true;
        const pt = calcPos(e);
        g.beginPath();
        g.moveTo(pt[0], pt[1]);
        g.stroke();
    }
    function draw_move(e: PointerEvent) {
        if (drawing) {
            const pt = calcPos(e);
            g.lineTo(pt[0], pt[1]);
            g.stroke();
        }
    }
    function draw_stop(e: PointerEvent) {
        drawing = false;
        g.closePath();
    }

    onMount(() => {
        g = canvas.getContext("2d");
        g.strokeStyle = props.color;
        g.lineJoin = "round";
        g.lineWidth = 1;
        // g.filter = "blur(1px)";

        canvas.addEventListener("touchstart", (e) => {
            e.preventDefault();
        });
        canvas.addEventListener("pointerdown", draw_start);
        canvas.addEventListener("pointermove", draw_move);
        canvas.addEventListener("pointerup", draw_stop);
    });
</script>

<canvas bind:this={canvas} width={props.width} height={props.height} />

<style>
    canvas {
        width: 100%;
    }
</style>
