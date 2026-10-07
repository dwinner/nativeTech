import { Tool } from "@paint-tools/tool.ts";
import type { ToolOptions, CanvasPair } from "@paint-tools/tool.ts";

export interface FillOptions extends ToolOptions {
    color: string | CanvasGradient | CanvasPattern;
    opacity: number;
    tolerance: number;
    contiguous: boolean;
}

export class Fill extends Tool {
    protected declare options: FillOptions;

    constructor(canvas: CanvasPair, options: Partial<FillOptions> = {}) {
        super(canvas, options);
        this.canvas.addEventListener('mousedown', this.#onFill, {signal: this.signal});
    }

    setOptions(options: Partial<FillOptions>): FillOptions {
        const def = {
            color: "#ff0000",
            opacity: 1.0,
            tolerance: 0,
            contiguous: true,
        } satisfies FillOptions;

        this.options = {...def, ...this.options, ...options};
        return this.options;
    }

    #onFill = (e: MouseEvent) => {
        if (e.button !== 0) {
            return;
        }

        const [x, y] = this.getMousePosition(e);

        const imageData = this.ctx.getImageData(
            0,
            0,
            this.ctx.canvas.width,
            this.ctx.canvas.height
        );

        const data = imageData.data;
        const targetColor = this.#getPixel(data, x, y, imageData.width);

        const fillColor = this.#parseColor(this.options.color);

        if (this.#colorsEqual(targetColor, fillColor, this.options.tolerance ?? 0)) {
            return;
        }

        this.ctx.globalAlpha = this.options.opacity ?? 1.0;

        this.#floodFill(imageData, x, y, targetColor, fillColor);

        this.ctx.putImageData(imageData, 0, 0);
        this.ctx.saveSnapshot();

        this.ctx.globalAlpha = 1.0;
    };

    #getPixel(data: Uint8ClampedArray, x: number, y: number, w: number): [r: number, g: number, b: number, a: number] {
        const i = (y * w + x) * 4;
        return [data[i], data[i + 1], data[i + 2], data[i + 3]];
    }

    #setPixel(data: Uint8ClampedArray, x: number, y: number, w: number, color: [number, number, number, number]) {
        const i = (y * w + x) * 4;
        data[i]     = color[0];
        data[i + 1] = color[1];
        data[i + 2] = color[2];
        data[i + 3] = color[3];
    }

    #colorsEqual(
        c1: [number, number, number, number],
        c2: [number, number, number, number],
        tolerance: number
    ): boolean {
        return (
            Math.abs(c1[0] - c2[0]) <= tolerance &&
            Math.abs(c1[1] - c2[1]) <= tolerance &&
            Math.abs(c1[2] - c2[2]) <= tolerance &&
            Math.abs(c1[3] - c2[3]) <= tolerance
        );
    }

    #parseColor(color: string | CanvasGradient | CanvasPattern): [number, number, number, number] {
        if (typeof color !== "string") {
            // градиент / паттерн → fallback
            return [0, 0, 0, 255];
        }

        const tempCanvas = document.createElement("canvas");
        tempCanvas.width = tempCanvas.height = 1;

        const tempCtx = tempCanvas.getContext("2d")!;
        tempCtx.fillStyle = color;
        tempCtx.fillRect(0, 0, 1, 1);

        const [r, g, b, a] = tempCtx.getImageData(0, 0, 1, 1).data;
        return [r, g, b, a];
    }

    #floodFill(
        imageData: ImageData,
        startX: number,
        startY: number,
        target: [number, number, number, number],
        fill: [number, number, number, number]
    ) {
        const {width, height, data} = imageData;
        const stack: [number, number][] = [[startX, startY]];

        while (stack.length) {
            const [x, y] = stack.pop()!;

            if (x < 0 || x >= width || y < 0 || y >= height) {
                continue;
            }

            const current = this.#getPixel(data, x, y, width);

            if (!this.#colorsEqual(current, target, this.options.tolerance ?? 0)) {
                continue;
            }

            this.#setPixel(data, x, y, width, fill);

            if (this.options.contiguous) {
                // 4-connected (вверх, вниз, влево, вправо)
                stack.push([x + 1, y]);
                stack.push([x - 1, y]);
                stack.push([x, y + 1]);
                stack.push([x, y - 1]);

            } else {
                // 8-connected (включая диагонали)
                for (let dy = -1; dy <= 1; dy++) {
                    for (let dx = -1; dx <= 1; dx++) {
                        if (dx === 0 && dy === 0) continue;
                        stack.push([x + dx, y + dy]);
                    }
                }
            }
        }
    }
}
