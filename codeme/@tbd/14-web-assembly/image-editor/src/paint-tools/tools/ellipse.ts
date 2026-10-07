import { Tool } from "@paint-tools/tool.ts";
import type { ToolOptions, CanvasPair } from "@paint-tools/tool.ts";

export interface EllipseOptions extends ToolOptions {
    lineWidth: number;
    strokeColor: string | CanvasGradient | CanvasPattern;
    fillColor?: string | CanvasGradient | CanvasPattern;
    strokeStyle: CanvasLineCap;
}

export class Ellipse extends Tool {
    protected declare  options: EllipseOptions;

    #startX = 0;
    #startY = 0;
    #isDrawing = false;
    #tmpImageData: ImageData | null = null;

    constructor(canvas: CanvasPair, options: Partial<EllipseOptions> = {}) {
        super(canvas, options);

        this.canvas.addEventListener("mousedown", this.#onDrawStart, {signal: this.signal});
        this.canvas.addEventListener("mousemove", this.#onDraw, {signal: this.signal});
        this.canvas.addEventListener("mouseup", this.#onDrawEnd, {signal: this.signal});
        this.canvas.addEventListener("mouseout", this.#onDrawEnd, {signal: this.signal});
    }

    setOptions(options: Partial<EllipseOptions>): EllipseOptions {
        const def = {
            lineWidth: 2,
            strokeColor: "#000",
            strokeStyle: "butt"
        } satisfies EllipseOptions;

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

        const centerX = (this.#startX + currentX) / 2;
        const centerY = (this.#startY + currentY) / 2;

        const radiusX = Math.abs(currentX - this.#startX) / 2;
        const radiusY = Math.abs(currentY - this.#startY) / 2;

        this.ctx.beginPath();
        this.ctx.ellipse(
            centerX,
            centerY,
            radiusX,
            radiusY,
            0,
            0,
            Math.PI * 2
        );

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

        const centerX = (this.#startX + endX) / 2;
        const centerY = (this.#startY + endY) / 2;

        const radiusX = Math.abs(endX - this.#startX) / 2;
        const radiusY = Math.abs(endY - this.#startY) / 2;

        if (radiusX >= 0.5 && radiusY >= 0.5) {
            this.ctx.beginPath();
            this.ctx.ellipse(
                centerX,
                centerY,
                radiusX,
                radiusY,
                0,
                0,
                Math.PI * 2
            );

            if (this.options.fillColor) {
                this.ctx.fillStyle = this.options.fillColor;
                this.ctx.fill();
            }

            this.ctx.stroke();
            this.#tmpImageData = null;

            this.ctx.saveSnapshot();
        }
    };
}
