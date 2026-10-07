export class CanvasHistory {
    #canvas: HTMLCanvasElement;
    #ctx: CanvasRenderingContext2D;

    #cursor?: number;
    #history: ImageData[] = [];

    constructor(canvas: HTMLCanvasElement) {
        this.#canvas = canvas;
        this.#ctx = canvas.getContext("2d")!;
        this.saveSnapshot();
    }

    saveSnapshot(): ImageData {
        const snapshot = this.#ctx.getImageData(0, 0, this.#canvas.width, this.#canvas.height);

        this.#history.push(snapshot);
        this.#cursor = undefined;

        return snapshot;
    }

    back() {
        const cursor = (this.#cursor ?? this.#history.length - 1) - 1;

        if (cursor < 0) {
            this.#cursor = 0;
            this.#ctx.clearRect(0, 0, this.#canvas.width, this.#canvas.height);

        } else if (cursor >= this.#history.length) {
            this.#cursor = undefined;

        } else {
            this.#cursor = cursor;
            this.#ctx.putImageData(this.#history[cursor], 0, 0);
        }

        return this.#cursor;
    }

    forward() {
        const cursor = (this.#cursor ?? this.#history.length - 1) + 1;

        if (cursor >= this.#history.length) {
            this.#cursor = undefined;

        } else {
            this.#cursor = cursor;
            this.#ctx.putImageData(this.#history[cursor], 0, 0);
        }

        return this.#cursor;
    }
}