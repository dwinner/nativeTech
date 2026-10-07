import { Tool } from "@paint-tools/tool.ts";
import type { ToolOptions, CanvasPair } from "@paint-tools/tool.ts";

export interface LineOptions extends ToolOptions {
    lineWidth: number;
    strokeColor: string | CanvasGradient | CanvasPattern;
    strokeStyle: CanvasLineCap;
}

export class Line extends Tool {
    protected declare options: LineOptions;

    #startX = 0;
    #startY = 0;
    #isDrawing = false;
    #tmpImageData: ImageData | null = null;

    constructor(canvas: CanvasPair, options: Partial<LineOptions> = {}) {
        super(canvas, options);

        this.canvas.addEventListener("mousedown", this.#onDrawStart, {signal: this.signal});
        this.canvas.addEventListener("mousemove", this.#onDraw, {signal: this.signal});
        this.canvas.addEventListener("mouseup", this.#onDrawEnd, {signal: this.signal});
        this.canvas.addEventListener("mouseout", this.#onDrawEnd, {signal: this.signal});
    }

    setOptions(options: Partial<LineOptions>): LineOptions {
        const def = {
            lineWidth: 2,
            strokeColor: "#000",
            strokeStyle: "round",
        } satisfies LineOptions;

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

        if (this.#tmpImageData !== null) {
            this.ctx.putImageData(this.#tmpImageData, 0, 0);
        }

        this.ctx.beginPath();
        this.ctx.moveTo(this.#startX, this.#startY);
        this.ctx.lineTo(currentX, currentY);
        this.ctx.stroke();
    };

    #onDrawEnd = (e: MouseEvent) => {
        if (!this.#isDrawing) {
            return;
        }

        this.#isDrawing = false;

        const [endX, endY] = this.getMousePosition(e);

        if (this.#tmpImageData !== null) {
            this.ctx.putImageData(this.#tmpImageData, 0, 0);
        }

        this.#tmpImageData = null;

        if (Math.abs(endX - this.#startX) >= 1 || Math.abs(endY - this.#startY) >= 1) {
            this.ctx.beginPath();
            this.ctx.moveTo(this.#startX, this.#startY);
            this.ctx.lineTo(endX, endY);
            this.ctx.stroke();
            this.ctx.saveSnapshot();
        }
    };
}
