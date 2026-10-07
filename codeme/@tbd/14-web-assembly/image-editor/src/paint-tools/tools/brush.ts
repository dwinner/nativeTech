import { Tool } from "@paint-tools/tool.ts";
import type { ToolOptions, CanvasPair } from "@paint-tools/tool.ts";

export interface BrushOptions extends ToolOptions {
    size: number;
    color: string | CanvasGradient | CanvasPattern;
    opacity: number;
    lineCap: CanvasLineCap;
    lineJoin: CanvasLineJoin;
    mode: "draw" | "erase";
}

export class Brush extends Tool {
    protected declare options: BrushOptions;

    #x = 0;
    #y = 0;
    #isDrawing = false;

    constructor(canvas: CanvasPair, options: Partial<BrushOptions> = {}) {
        super(canvas, options);

        this.canvas.addEventListener('mousedown', this.#onDrawStart, {signal: this.signal});
        this.canvas.addEventListener('mousemove', this.#onDraw, {signal: this.signal});
        this.canvas.addEventListener('mouseup', this.#onDrawEnd, {signal: this.signal});
        this.canvas.addEventListener('mouseout', this.#onDrawEnd, {signal: this.signal});
    }

    setOptions(options: Partial<BrushOptions>): BrushOptions {
        const def = {
            size: 16,
            color: "#000",
            opacity: 1.0,
            lineCap: "round",
            lineJoin: "round",
            mode: "draw"
        } satisfies BrushOptions;

        this.options = {...def, ...this.options, ...options};
        return this.options;
    }

    #onDrawStart = (e: MouseEvent)=> {
        this.#isDrawing = true;
        [this.#x, this.#y] = this.getMousePosition(e);

        this.ctx.lineWidth   = this.options.size;
        this.ctx.lineCap     = this.options.lineCap;
        this.ctx.lineJoin    = this.options.lineJoin;
        this.ctx.globalAlpha = this.options.opacity;

        if (this.options.mode === "erase") {
            this.ctx.globalCompositeOperation = "destination-out";

        } else {
            this.ctx.globalCompositeOperation = "source-over";
            this.ctx.strokeStyle = this.options.color;
        }

        this.ctx.beginPath();
        this.ctx.moveTo(this.#x, this.#y);
    };

    #onDraw = (e: MouseEvent)=> {
        if (!this.#isDrawing) {
            return;
        }

        e.preventDefault();

        const [x, y] = this.getMousePosition(e);

        this.ctx.lineTo(x, y);
        this.ctx.stroke();

        // Добавляем дополнительные точки для более плавной линии
        // Это улучшает качество рисования при быстром движении мыши
        for (let i = 0; i < 3; i++) {
            const midX = (this.#x + x) / 2;
            const midY = (this.#y + y) / 2;
            this.ctx.lineTo(midX, midY);
            this.ctx.stroke();
        }

        [this.#x, this.#y] = [x, y];
    };

    #onDrawEnd = () => {
        if (!this.#isDrawing) {
            return;
        }

        this.#isDrawing = false;
        this.ctx.closePath();

        if (this.options.mode === "erase") {
            this.ctx.globalCompositeOperation = "source-over";
            this.ctx.globalAlpha = 1.0;
        }

        this.ctx.saveSnapshot();
    };
}
