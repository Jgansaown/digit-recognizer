<script lang="ts">
    import { onMount } from "svelte";

    export let imagedata: ImageData = undefined;

    let canvas: HTMLCanvasElement;
    let ctx: CanvasRenderingContext2D;
    let is_down: boolean = false;

    let m = { x: 0, y: 0 };

    onMount(() => {
        ctx = canvas.getContext("2d");
        canvas.height = canvas.width * 1.0;
        ctx.fillStyle = "#000";
        // ctx.filter = 'grayscale(1)';
        ctx.fillRect(0, 0, canvas.width, canvas.height);

        console.log(canvas.width, canvas.height);
        console.log(canvas.clientHeight, canvas.clientWidth);
    });
    function mousedown(e: MouseEvent) {
        is_down = true;
        ctx.beginPath();

        const canvasX = (e.offsetX / canvas.clientWidth) * 28;
        const canvasY = (e.offsetY / canvas.clientHeight) * 28;
        ctx.moveTo(canvasX, canvasY);
    }
    function mousemove(e: MouseEvent) {
        if (is_down) {
            const canvasX = (e.offsetX / canvas.clientWidth) * 28;
            const canvasY = (e.offsetY / canvas.clientHeight) * 28;

            ctx.lineTo(canvasX, canvasY);
            ctx.strokeStyle = "#fff";
            ctx.stroke();
        }
        m.x = e.offsetX;
        m.y = e.offsetY;
    }
    function mouseup(e: MouseEvent) {
        is_down = false;
        ctx.closePath();
        imagedata = ctx.getImageData(0, 0, canvas.width, canvas.height);
    }
</script>

<!-- <p>The mouse position is {m.x} x {m.y}</p> -->
<div>
    <canvas
        bind:this={canvas}
        width={28}
        height={28}
        on:mousedown={mousedown}
        on:mousemove={mousemove}
        on:mouseup={mouseup}
    />
    <button
        on:click={() => {
            ctx.fillRect(0, 0, canvas.width, canvas.height);
        }}>Clear Canvas</button
    >
</div>

<style>
    div {
        display: flex;
        flex-flow: column wrap;
        align-items: center;
    }
    canvas {
        width: 100%;
        max-width: 300px;
        /* height: 100%;
        max-height: 300px; */
    }
</style>
