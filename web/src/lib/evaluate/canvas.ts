export function get_cropped_scaled_grayscale_image(
    main: CanvasRenderingContext2D,
    crop: CanvasRenderingContext2D,
    scale: CanvasRenderingContext2D
) {
    const image = get_cropped_scaled_image(main, crop, scale);
    return rgba_to_grayscale(image);
}

function get_cropped_scaled_image(
    main: CanvasRenderingContext2D,
    crop: CanvasRenderingContext2D,
    scale: CanvasRenderingContext2D
) {
    const [w, h, croppedImage] = crops_canvas_image(main);

    crop.fillStyle = "rgba(255, 255, 255, 255)";
    crop.fillRect(0, 0, crop.canvas.width, crop.canvas.height);
    crop.save();

    crop.canvas.width = Math.max(w, h) * 1.2;
    crop.canvas.height = Math.max(w, h) * 1.2;

    const leftPadding = (crop.canvas.width - w) / 2;
    const topPadding = (crop.canvas.height - h) / 2;
    crop.putImageData(croppedImage, leftPadding, topPadding);

    // Copy image data to scale 28x28 canvas
    scale.save();
    scale.clearRect(0, 0, scale.canvas.height, scale.canvas.width);
    scale.fillStyle = "rgba(255, 255, 255, 255)"; // white non-transparent color
    scale.fillRect(0, 0, scale.canvas.width, scale.canvas.height);
    scale.scale(28.0 / crop.canvas.height, 28.0 / crop.canvas.width);
    scale.drawImage(crop.canvas, 0, 0);

    const { data } = scale.getImageData(0, 0, 28, 28)!;

    scale.restore();

    return data;
}

/**
 * Crops a canvas images and returns its image data.
 * adapted from: https://stackoverflow.com/a/22267731
 */
function crops_canvas_image(
    ctx: CanvasRenderingContext2D
): [number, number, ImageData] {
    const image = ctx.getImageData(0, 0, ctx.canvas.width, ctx.canvas.height);
    const { sx, sy, sw, sh } = find_cropped_image(image);
    return [sw, sh, ctx.getImageData(sx, sy, sw, sh)];
}

/**
 * Crops a canvas images and returns its image data.
 * adapted from: https://stackoverflow.com/a/22267731
 */
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
export function rgba_to_grayscale(rgba: Uint8ClampedArray): Float64Array {
    let gray = new Float64Array(rgba.length / 4);

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
