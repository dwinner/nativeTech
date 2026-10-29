import { wasm_gaussian_blur, wasm_gaussian_blur2 } from "@wasm-lib/wasm_lib";

import { Tool } from "@paint-tools/tool.ts";
import type { ToolOptions, CanvasPair } from "@paint-tools/tool.ts";

export interface GaussianBlurOptions extends ToolOptions {
    sigma: number;
    mode: "js" | "wasm" | "wasm2"
}

export class GaussianBlur extends Tool {
    protected declare options: GaussianBlurOptions;

    constructor(canvas: CanvasPair, options: Partial<GaussianBlurOptions> = {}) {
        super(canvas, options);
        this.#applyBlur();
    }

    setOptions(options: Partial<GaussianBlurOptions>): GaussianBlurOptions {
        const def = {
            sigma: 10,
            mode: "wasm",
        } satisfies GaussianBlurOptions;

        this.options = {...def, ...this.options, ...options};
        return this.options;
    }

    #applyBlur() {
        console.time("GaussianBlur");

        if (this.options.mode === "js") {
            this.#applyJSBlur();

        } else if (this.options.mode === "wasm2") {
            this.#applyWasmBlur2();

        } else {
            this.#applyWasmBlur();
        }

        console.timeEnd("GaussianBlur");

        this.ctx.saveSnapshot();
    }

    #applyWasmBlur() {
        const bytes = wasm_gaussian_blur(
            new Uint8Array(this.getSnapshot().data.buffer),
            this.canvas.width,
            this.canvas.height,
            this.options.sigma,
        );

        const data = new Uint8ClampedArray(bytes.buffer);

        // @ts-expect-error (Некогда разбираться)
        const imageData = new ImageData(data, this.canvas.width, this.canvas.height);

        this.ctx.putImageData(imageData, 0, 0);
    }

    #applyWasmBlur2() {
        const bytes = wasm_gaussian_blur2(
            new Uint8Array(this.getSnapshot().data.buffer),
            this.canvas.width,
            this.canvas.height,
            this.options.sigma,
        );

        const data = new Uint8ClampedArray(bytes.buffer);

        // @ts-expect-error (Некогда разбираться)
        const imageData = new ImageData(data, this.canvas.width, this.canvas.height);

        this.ctx.putImageData(imageData, 0, 0);
    }

    #applyJSBlur() {
        const blurred = gaussianBlur(this.getSnapshot(), this.options.sigma);
        this.ctx.putImageData(blurred, 0, 0);

        function gaussianBlur(imageData: ImageData, sigma = 2) {
            const { width, height, data } = imageData;
            const radius = Math.ceil(sigma * 3); // разумный радиус для ядра
            const kernelSize = radius * 2 + 1;

            // 1. Генерируем 1D ядро Гаусса
            const kernel = new Float32Array(kernelSize);
            let sum = 0;

            for (let i = -radius; i <= radius; i++) {
                const val = Math.exp(-(i * i) / (2 * sigma * sigma));
                kernel[i + radius] = val;
                sum += val;
            }

            // Нормализация
            for (let i = 0; i < kernelSize; i++) {
                kernel[i] /= sum;
            }

            // 2. Создаём временный и выходной буферы
            const tempData = new Uint8ClampedArray(data.length);
            const outputData = new Uint8ClampedArray(data.length);

            // 3. Горизонтальный проход (по строкам)
            for (let y = 0; y < height; y++) {
                for (let x = 0; x < width; x++) {
                    let r = 0, g = 0, b = 0, a = 0;

                    for (let i = -radius; i <= radius; i++) {
                        const px = Math.max(0, Math.min(width - 1, x + i));
                        const idx = (y * width + px) * 4;
                        const weight = kernel[i + radius];

                        r += data[idx]     * weight;
                        g += data[idx + 1] * weight;
                        b += data[idx + 2] * weight;
                        a += data[idx + 3] * weight;
                    }

                    const outIdx = (y * width + x) * 4;
                    tempData[outIdx]     = r | 0;
                    tempData[outIdx + 1] = g | 0;
                    tempData[outIdx + 2] = b | 0;
                    tempData[outIdx + 3] = a | 0;
                }
            }

            // 4. Вертикальный проход (по столбцам)
            for (let x = 0; x < width; x++) {
                for (let y = 0; y < height; y++) {
                    let r = 0, g = 0, b = 0, a = 0;

                    for (let i = -radius; i <= radius; i++) {
                        const py = Math.max(0, Math.min(height - 1, y + i));
                        const idx = (py * width + x) * 4;
                        const weight = kernel[i + radius];

                        r += tempData[idx]     * weight;
                        g += tempData[idx + 1] * weight;
                        b += tempData[idx + 2] * weight;
                        a += tempData[idx + 3] * weight;
                    }

                    const outIdx = (y * width + x) * 4;
                    outputData[outIdx]     = r | 0;
                    outputData[outIdx + 1] = g | 0;
                    outputData[outIdx + 2] = b | 0;
                    outputData[outIdx + 3] = a | 0;
                }
            }

            return new ImageData(outputData, width, height);
        }
    }
}
