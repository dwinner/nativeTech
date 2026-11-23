import type { CanvasPair, PersistentCanvasRenderingContext2D } from "@/canvas";

export type { CanvasPair, PersistentCanvasRenderingContext2D } from "@/canvas";

export type ToolOptions = Record<string, unknown>;

export abstract class Tool {
    protected canvas: HTMLCanvasElement;
    protected ctx: PersistentCanvasRenderingContext2D;

    protected options: ToolOptions;
    protected controller = new AbortController();

    get signal(): AbortSignal {
        return this.controller.signal;
    }

    constructor(canvas: CanvasPair, options: Partial<ToolOptions>) {
        [this.canvas, this.ctx] = canvas;
        this.options = this.setOptions(options);
        this.ctx.save();
    }

    abstract setOptions(options: ToolOptions): ToolOptions;

    getSnapshot(): ImageData {
        return this.ctx.getImageData(0, 0, this.ctx.canvas.width, this.ctx.canvas.height);
    }

    destroy() {
        this.controller.abort();
        this.ctx.restore();
    }

    [Symbol.dispose]() {
        this.destroy();
    }

    protected getMousePosition(e: MouseEvent): [number, number] {
        const rect = this.canvas.getBoundingClientRect();
        return [e.clientX - rect.left, e.clientY - rect.top];
    }
}
