import { fabric } from "fabric";

/**
 * Crops a canvas images and returns its image data.
 * adapted from: https://stackoverflow.com/a/22267731
 */
export function cropImageFromCanvas(
    ctx: CanvasRenderingContext2D
): [number, number, ImageData] {
    const image = ctx.getImageData(0, 0, ctx.canvas.width, ctx.canvas.height);
    const { sx, sy, sw, sh } = find_cropped_image(image);
    return [sw, sh, ctx.getImageData(sx, sy, sw, sh)];
}

function find_cropped_image(img: ImageData) {
    const w = img.width;
    const h = img.height;

    let min_x = w;
    let min_y = h;
    let max_x = 0;
    let max_y = 0;

    // find the min x, y and max x, y points
    for (let y = 0; y < h; y++) {
        for (let x = 0; x < w; x++) {
            const index = (y * w + x) * 4;

            let r = img.data[index];
            let g = img.data[index + 1];
            let b = img.data[index + 2];

            if (Math.min(r, g, b) != 255) {
                if (x < min_x) {
                    min_x = x;
                } else if (x > max_x) {
                    max_x = x;
                }
                if (y < min_y) {
                    min_y = y;
                } else if (y > max_y) {
                    max_y = y;
                }
            }
        }
    }

    return {
        sx: min_x,
        sy: min_y,
        sw: 1 + max_x - min_x,
        sh: 1 + max_y - min_y,
    };
}

// https://en.wikipedia.org/wiki/Grayscale#Luma_coding_in_video_systems
// https://en.wikipedia.org/wiki/Luma_(video)
export function rgba_to_grayscale(rgba: Uint8ClampedArray): Uint8Array {
    let gray = new Uint8Array(rgba.length / 4);

    // Data is stored as [r0,g0,b0,a0, ... r[n],g[n],b[n],a[n]] where n is number of pixels.
    for (let i = 0; i < rgba.length; i += 4) {
        const r = 255 - rgba[i]; // red
        const g = 255 - rgba[i + 1]; // green
        const b = 255 - rgba[i + 2]; // blue
        const a = 255 - rgba[i + 3]; // alpha

        // Use RGB grayscale coefficients (https://imagej.nih.gov/ij/docs/menus/image.html)
        const y = 0.299 * r + 0.587 * g + 0.114 * b;
        gray[i / 4] = y; // 4 times fewer data points but the same number of pixels.
    }

    return gray;
}
