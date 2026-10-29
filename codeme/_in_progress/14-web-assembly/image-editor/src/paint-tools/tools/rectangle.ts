import { Tool } from "@paint-tools/tool.ts";
import type { ToolOptions, CanvasPair } from "@paint-tools/tool.ts";

export interface RectangleOptions extends ToolOptions {
    lineWidth: number;
    strokeColor: string | CanvasGradient | CanvasPattern;
    strokeStyle: CanvasLineCap;
    fillColor?: string | CanvasGradient | CanvasPattern;
}

export class Rectangle extends Tool {
    protected declare options: RectangleOptions;

    #startX = 0;
    #startY = 0;
    #isDrawing = false;
    #tmpImageData: ImageData | null = null;

    constructor(canvas: CanvasPair, options: Partial<RectangleOptions>) {
        super(canvas, options);

        this.canvas.addEventListener("mousedown", this.#onDrawStart, {signal: this.signal});
        this.canvas.addEventListener("mousemove", this.#onDraw, {signal: this.signal});
        this.canvas.addEventListener("mouseup", this.#onDrawEnd, {signal: this.signal});
        this.canvas.addEventListener("mouseout", this.#onDrawEnd, {signal: this.signal});
    }

    setOptions(options: Partial<RectangleOptions>): RectangleOptions {
        const def = {
            lineWidth: 2,
            strokeColor: "#000",
            strokeStyle: "butt"
        } satisfies RectangleOptions;

        this.options = {...def, ...this.options, ...options};
        return this.options;
    }

    #onDrawStart = (e: MouseEvent) => {
        if (e.button !== 0) {
            return;
        }

        this.#tmpImageData = this.getSnapshot();

        this.#isDrawing = true;
        [this.#startX, this.#startY] = this.getMousePosition(e);

        this.ctx.lineWidth   = this.options.lineWidth;
        this.ctx.lineCap     = this.options.strokeStyle;
        this.ctx.strokeStyle = this.options.strokeColor;
    };

    #onDraw = (e: MouseEvent) => {
        if (!this.#isDrawing) {
            return;
        }

        e.preventDefault();

        const [currentX, currentY] = this.getMousePosition(e);

        if (this.#tmpImageData != null) {
            this.ctx.putImageData(this.#tmpImageData, 0, 0);
        }

        const x = Math.min(this.#startX, currentX);
        const y = Math.min(this.#startY, currentY);

        const width = Math.abs(currentX - this.#startX);
        const height = Math.abs(currentY - this.#startY);

        this.ctx.beginPath();
        this.ctx.rect(x, y, width, height);

        if (this.options.fillColor != null) {
            this.ctx.fillStyle = this.options.fillColor;
            this.ctx.fill();
        }

        this.ctx.stroke();
    };

    #onDrawEnd = (e: MouseEvent) => {
        if (!this.#isDrawing) {
            return;
        }

        this.#isDrawing = false;

        const [endX, endY] = this.getMousePosition(e);

        if (this.#tmpImageData != null) {
            this.ctx.putImageData(this.#tmpImageData, 0, 0);
        }

        const x = Math.min(this.#startX, endX);
        const y = Math.min(this.#startY, endY);

        const width = Math.abs(endX - this.#startX);
        const height = Math.abs(endY - this.#startY);

        if (width > 0 && height > 0) {
            this.ctx.beginPath();
            this.ctx.rect(x, y, width, height);

            if (this.options.fillColor != null) {
                this.ctx.fillStyle = this.options.fillColor;
                this.ctx.fill();
            }

            this.ctx.stroke();
            this.#tmpImageData = null;

            this.ctx.saveSnapshot();
        }
    };
}
