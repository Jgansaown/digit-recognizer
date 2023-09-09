<script lang="ts">
    import { onMount, createEventDispatcher } from "svelte";
    import { fabric } from "fabric";
    import { get_cropped_scaled_grayscale_image } from "./canvas";

    const dispatch = createEventDispatcher<{
        fire: Float64Array;
    }>();

    let canvas: fabric.Canvas;

    let mainCanvas: HTMLCanvasElement;
    let cropCanvas: HTMLCanvasElement;
    let scaleCanvas: HTMLCanvasElement;

    let main_ctx: CanvasRenderingContext2D;
    let cropped_ctx: CanvasRenderingContext2D;
    let scaled_ctx: CanvasRenderingContext2D;

    export function clear_canvas() {
        canvas.clear();
        canvas.backgroundColor = "rgba(255, 255, 255, 255)";
        canvas.renderAll();

        main_ctx.clearRect(0, 0, main_ctx.canvas.width, main_ctx.canvas.height);
        cropped_ctx.clearRect(
            0,
            0,
            cropped_ctx.canvas.width,
            cropped_ctx.canvas.height
        );
        scaled_ctx.clearRect(
            0,
            0,
            scaled_ctx.canvas.width,
            scaled_ctx.canvas.height
        );
    }

    onMount(() => {
        main_ctx = mainCanvas.getContext("2d", { willReadFrequently: true })!;
        cropped_ctx = cropCanvas.getContext("2d", {
            willReadFrequently: true,
        })!;
        scaled_ctx = scaleCanvas.getContext("2d", {
            willReadFrequently: true,
        })!;

        canvas = new fabric.Canvas(mainCanvas, {
            isDrawingMode: true,
        });
        canvas.freeDrawingBrush.color = "black";
        canvas.freeDrawingBrush.width = 25;
        canvas.backgroundColor = "rgba(255, 255, 255, 255)";

        let timeoutid: number | undefined = undefined;
        let is_drawing = false;
        let is_timeout = false;

        function fire() {
            clearTimeout(timeoutid);
            timeoutid = setTimeout(() => {
                is_timeout = true;
                (canvas.freeDrawingBrush as any)._finalizeAndAddPath();
                // const data = save_canvas();
                // const gray = rgba_to_grayscale(data);
                const image = get_cropped_scaled_grayscale_image(
                    canvas.getContext(),
                    cropped_ctx,
                    scaled_ctx
                );

                dispatch("fire", image);

                is_timeout = false;
            }, 50);
            is_timeout = true;
        }

        canvas.on("mouse:down", (e) => {
            is_drawing = true;
        });
        canvas.on("mouse:up", (e) => {
            is_drawing = false;
            fire();
        });
        canvas.on("mouse:move", (e) => {
            if (is_drawing && is_timeout == false) {
                fire();
            }
        });
    });
</script>

<div>
    <div id="canvas-wrapper">
        <canvas bind:this={mainCanvas} width="300" height="300" />
    </div>
    <div id="hidden">
        <canvas bind:this={cropCanvas} width="28" height="28" />
        <canvas bind:this={scaleCanvas} width="28" height="28" />
    </div>
</div>

<style>
    #canvas-wrapper {
        border: 1px solid var(--contrast);
        background-color: white;
        width: 310px;
        height: 310px;
        margin: 1em auto;
        padding: 5px;
    }

    #hidden canvas {
        display: none;
        border: 1px solid var(--contrast);
        width: 100px;
        height: 100px;
    }
</style>
