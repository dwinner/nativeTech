import { wasm_resize } from "@wasm-lib/wasm_lib";

import { Tool } from "@paint-tools/tool.ts";
import type { ToolOptions, CanvasPair } from "@paint-tools/tool.ts";

export interface RectSelectOptions extends ToolOptions {
    borderColor: string;
    borderWidth: number;
    dashPattern: number[];
    dashOffsetSpeed: number;
    handleSize: number;
    handleColor: string;
}

export interface SelectionRect {
    x: number;
    y: number;
    width: number;
    height: number;
    imageData: ImageData;
    cut: boolean;
}

export class RectSelect extends Tool {
    protected declare options: RectSelectOptions;

    #startX = 0;
    #startY = 0;
    #isSelecting = false;

    #tmpImageData: ImageData | null = null;
    #selection: SelectionRect | null = null;

    #dragMode: "move" | "resize-tl" | "resize-tr" | "resize-bl" | "resize-br" | null = null;
    #dragOffsetX = 0;
    #dragOffsetY = 0;

    #dashOffset = 0;
    #lastTime = 0;

    constructor(canvas: CanvasPair, options: Partial<RectSelectOptions> = {}) {
        super(canvas, options);
        this.setOptions(options);

        this.canvas.addEventListener("mousedown", this.#onMoveStart, {signal: this.signal});
        this.canvas.addEventListener("mousemove", this.#onMove, {signal: this.signal});
        this.canvas.addEventListener("mouseup", this.#onMoveEnd, {signal: this.signal});
        this.canvas.addEventListener("mouseout", this.#onMoveEnd, {signal: this.signal});
    }

    setOptions(options: Partial<RectSelectOptions>): RectSelectOptions {
        const def = {
            borderColor: "#0D6EFD",
            borderWidth: 1.5,
            dashPattern: [5, 3],
            dashOffsetSpeed: 80,
            handleSize: 8,
            handleColor: "#0D6EFD",
        } satisfies RectSelectOptions;

        this.options = {...def, ...this.options, ...options};
        return this.options;
    }

    override destroy() {
        super.destroy();

        if (this.#tmpImageData != null) {
            this.ctx.putImageData(this.#tmpImageData, 0, 0);
        }

        const s = this.#selection;

        if (s != null) {
            this.ctx.putImageData(s.imageData, s.x, s.y);
        }
    }

    #onMoveStart = (e: MouseEvent) => {
        if (e.button !== 0) {
            return;
        }

        const [x, my] = this.getMousePosition(e);

        const s = this.#selection;

        if (s == null) {
            this.#isSelecting = true;
            [this.#startX, this.#startY] = [x, my];

            if (this.#tmpImageData == null) {
                this.#tmpImageData = this.getSnapshot();

            } else {
                this.ctx.putImageData(this.#tmpImageData, 0, 0);
            }

            this.#dashOffset = 0;
            this.#lastTime = performance.now();

        } else {
            const hs = this.options.handleSize ?? 8;

            const handles = [
                {type: "resize-tl", x: s.x - hs / 2, y: s.y - hs / 2},
                {type: "resize-tr", x: s.x + s.width - hs / 2, y: s.y - hs / 2},
                {type: "resize-bl", x: s.x - hs / 2, y: s.y + s.height - hs / 2},
                {type: "resize-br", x: s.x + s.width - hs / 2, y: s.y + s.height - hs / 2},
            ];

            for (const handle of handles) {
                if (x >= handle.x && x <= handle.x + hs && my >= handle.y && my <= handle.y + hs) {
                    this.#dragMode = handle.type as "resize-tl" | "resize-tr" | "resize-bl" | "resize-br";
                    this.#dragOffsetX = x - s.x;
                    this.#dragOffsetY = my - s.y;
                    return;
                }
            }

            // Клик внутри выделения — перемещение
            if (
                x >= s.x && x <= s.x + s.width &&
                my >= s.y && my <= s.y + s.height
            ) {
                this.#dragMode = "move";
                this.#dragOffsetX = x - s.x;
                this.#dragOffsetY = my - s.y;

                this.#cutSelection();

            } else {
                this.#tmpImageData = this.#redrawWithoutBorders(true);
                this.#selection = null;
                this.#onMoveStart(e);
            }
        }
    };

    #onMove = (e: MouseEvent) => {
        e.preventDefault();

        const [x, y] = this.getMousePosition(e);

        const s = this.#selection;

        if (this.#dragMode) {
            if (s == null) {
                return;
            }

            if (this.#dragMode === "move") {
                s.x = x - this.#dragOffsetX;
                s.y = y - this.#dragOffsetY;

            } else {
                const {width: originalWidth, height: originalHeight} = s;

                switch (this.#dragMode) {
                    case "resize-tl":
                        s.width += s.x - x;
                        s.height += s.y - y;
                        s.x = x;
                        s.y = y;
                        break;

                        case "resize-tr":
                        s.width = x - s.x;
                        s.height += s.y - y;
                        s.y = y;
                        break;

                    case "resize-bl":
                        s.width += s.x - x;
                        s.height = y - s.y;
                        s.x = x;
                        break;

                        case "resize-br":
                        s.width = x - s.x;
                        s.height = y - s.y;
                        break;
                }

                // Фиксация пропорций с Shift
                if (e.shiftKey) {
                    const ratio = s.imageData.width / s.imageData.height;
                    s.height = s.width / ratio;
                }

                if (s.width < 0) {
                    s.width = 0;
                }

                if (s.height < 0) {
                    s.height = 0;
                }

                this.#cutSelection();

                const bytes = wasm_resize(
                    new Uint8Array(s.imageData.data.buffer),
                    originalWidth,
                    originalHeight,
                    s.width,
                    s.height
                );

                const data = new Uint8ClampedArray(bytes.buffer);

                // @ts-expect-error (Некогда разбираться)
                s.imageData = new ImageData(data, s.width, s.height);
            }

            this.#redraw();

        } else if (this.#isSelecting) {
            if (this.#tmpImageData != null) {
                this.ctx.putImageData(this.#tmpImageData, 0, 0);
            }

            this.#drawPreviewRect(this.#startX, this.#startY, x, y);
        }
    };

    #onMoveEnd = (e: MouseEvent) => {
        if (this.#dragMode) {
            this.#dragMode = null;
            this.#redraw(true);

        } else if (this.#isSelecting) {
            this.#isSelecting = false;

            const [endX, endY] = this.getMousePosition(e);

            const x = Math.min(this.#startX, endX);
            const y = Math.min(this.#startY, endY);

            const width = Math.abs(endX - this.#startX);
            const height = Math.abs(endY - this.#startY);

            if (width >= 4 && height >= 4) {
                const selectedImageData = this.ctx.getImageData(x, y, width, height);
                this.#selection = {x, y, width, height, imageData: selectedImageData, cut: false};

            } else {
                this.#selection = null;
            }

            this.#redraw();
        }
    };

    #redrawWithoutBorders(snapshot: true): ImageData;
    #redrawWithoutBorders(snapshot?: false): void;
    #redrawWithoutBorders(snapshot = false): ImageData | void {
        if (this.#tmpImageData != null) {
            this.ctx.putImageData(this.#tmpImageData, 0, 0);
        }

        const s = this.#selection;

        if (s != null) {
            this.ctx.putImageData(s.imageData, s.x, s.y);
        }

        return snapshot ? this.getSnapshot() : undefined;
    }

    #redraw(saveSnapshot = false): void {
        this.#redrawWithoutBorders();

        if (saveSnapshot) {
            this.ctx.saveSnapshot();
        }

        const s = this.#selection;

        if (s != null) {
            this.#drawPreviewRect(s.x, s.y, s.x + s.width, s.y + s.height);
        }
    }

    #drawPreviewRect(x: number, y: number, x2: number, y2: number) {
        this.ctx.save();

        const o = this.options;

        this.ctx.lineWidth = o.borderWidth;
        this.ctx.strokeStyle = o.borderColor;
        this.ctx.setLineDash(o.dashPattern);

        const now = performance.now();
        const delta = (now - this.#lastTime) / 1000;

        this.#dashOffset -= (o.dashOffsetSpeed ?? 80) * delta;
        this.#lastTime = now;

        this.ctx.lineDashOffset = this.#dashOffset;

        this.ctx.beginPath();
        this.ctx.rect(Math.min(x, x2), Math.min(y ?? this.#startY, y2), Math.abs(x2 - x), Math.abs(y2 - y));

        this.ctx.stroke();
        this.ctx.restore();

        const hs = o.handleSize;

        this.ctx.fillStyle = o.handleColor;
        this.ctx.fillRect(x - hs / 2, y - hs / 2, hs, hs);
        this.ctx.fillRect(x2 - hs / 2, y - hs / 2, hs, hs);
        this.ctx.fillRect(x - hs / 2, y2 - hs / 2, hs, hs);
        this.ctx.fillRect(x2 - hs / 2, y2 - hs / 2, hs, hs);
    }

    #cutSelection() {
        const s = this.#selection;

        if (s != null && !s.cut) {
            s.cut = true;

            if (this.#tmpImageData != null) {
                this.ctx.putImageData(this.#tmpImageData, 0, 0);
            }

            this.ctx.clearRect(s.x, s.y, s.width, s.height);
            this.#tmpImageData = this.getSnapshot();
        }
    }
}
