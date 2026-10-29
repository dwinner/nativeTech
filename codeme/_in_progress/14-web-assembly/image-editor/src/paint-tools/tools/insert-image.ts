import { Tool } from "@paint-tools/tool.ts";
import type { ToolOptions, CanvasPair } from "@paint-tools/tool.ts";

export interface ImageToolOptions extends ToolOptions {
    defaultScale: number;
    opacity: number;
    borderOnDrag: boolean;
}

interface PlacedImage {
    img: HTMLImageElement;
    x: number;
    y: number;
    width: number;
    height: number;
    rotation?: number;
}

export class InsertImage extends Tool {
    protected declare options: ImageToolOptions;

    #images: PlacedImage[] = [];
    #draggedImage: PlacedImage | null = null;

    #dragOffsetX = 0;
    #dragOffsetY = 0;
    #isDragging = false;

    constructor(canvas: CanvasPair, options: Partial<ImageToolOptions> = {}) {
        super(canvas, options);
        this.setOptions(options);

        this.canvas.addEventListener("mousedown", this.#onDragStart, { signal: this.signal });
        this.canvas.addEventListener("mousemove", this.#onDrag, { signal: this.signal });
        this.canvas.addEventListener("mouseup", this.#onDragEnd, { signal: this.signal });
        this.canvas.addEventListener("mouseleave", this.#onDragEnd, { signal: this.signal });
    }

    setOptions(options: Partial<ImageToolOptions>): ImageToolOptions {
        const def = {
            defaultScale: 1.0,
            opacity: 1.0,
            borderOnDrag: true,
        } satisfies ImageToolOptions;

        this.options = { ...def, ...this.options, ...options };
        return this.options;
    }

    async addImage(fileOrUrl: File | string) {
        let img: HTMLImageElement;

        if (typeof fileOrUrl === "string") {
            img = await this.#loadImageFromUrl(fileOrUrl);

        } else {
            img = await this.#loadImageFromFile(fileOrUrl);
        }

        const canvasRect = this.canvas.getBoundingClientRect();
        const centerX = canvasRect.width / 2;
        const centerY = canvasRect.height / 2;

        const scale = this.options.defaultScale;

        const placed: PlacedImage = {
            img,
            x: centerX - (img.width * scale) / 2,
            y: centerY - (img.height * scale) / 2,
            width: img.width * scale,
            height: img.height * scale,
        };

        this.#images.push(placed);
        this.#redraw();

        this.ctx.saveSnapshot();
    }

    #onDragStart = (e: MouseEvent) => {
        if (e.button !== 0) {
            return;
        }

        const [mx, my] = this.getMousePosition(e);

        // Ищем, по какому изображению кликнули (сверху вниз — последнее добавленное сверху)
        for (let i = this.#images.length - 1; i >= 0; i--) {
            const imgObj = this.#images[i];
            if (
                mx >= imgObj.x &&
                mx <= imgObj.x + imgObj.width &&
                my >= imgObj.y &&
                my <= imgObj.y + imgObj.height
            ) {
                this.#startDragging(imgObj, mx, my);
                e.preventDefault();
                return;
            }
        }
    };

    #onDrag = (e: MouseEvent) => {
        if (!this.#isDragging || !this.#draggedImage) {
            return;
        }

        e.preventDefault();

        const [mx, my] = this.getMousePosition(e);

        this.#draggedImage.x = mx - this.#dragOffsetX;
        this.#draggedImage.y = my - this.#dragOffsetY;

        this.#redraw();
    };

    #onDragEnd = () => {
        if (!this.#isDragging) {
            return;
        }

        this.#isDragging = false;
        this.#draggedImage = null;

        this.#redraw();
        this.ctx.saveSnapshot();
    };

    #startDragging(imgObj: PlacedImage, mx: number, my: number) {
        this.#draggedImage = imgObj;
        this.#isDragging = true;
        this.#dragOffsetX = mx - imgObj.x;
        this.#dragOffsetY = my - imgObj.y;
    }

    #redraw() {
        this.ctx.clearRect(0, 0, this.ctx.canvas.width, this.ctx.canvas.height);

        for (const imgObj of this.#images) {
            this.ctx.globalAlpha = this.options.opacity;

            if (this.#isDragging && imgObj === this.#draggedImage && this.options.borderOnDrag) {
                this.ctx.strokeStyle = "#0D6EFD";
                this.ctx.lineWidth = 2;
                this.ctx.setLineDash([6, 4]);
                this.ctx.strokeRect(imgObj.x, imgObj.y, imgObj.width, imgObj.height);
                this.ctx.setLineDash([]);
            }

            this.ctx.drawImage(
                imgObj.img,
                imgObj.x,
                imgObj.y,
                imgObj.width,
                imgObj.height
            );
        }

        this.ctx.globalAlpha = 1.0;
    }

    async #loadImageFromUrl(url: string): Promise<HTMLImageElement> {
        return new Promise((resolve, reject) => {
            const img = new Image();
            img.crossOrigin = "anonymous";
            img.onload = () => resolve(img);
            img.onerror = reject;
            img.src = url;
        });
    }

    async #loadImageFromFile(file: File): Promise<HTMLImageElement> {
        return new Promise((resolve, reject) => {
            const reader = new FileReader();

            reader.onload = (e) => {
                const img = new Image();
                img.onload = () => resolve(img);
                img.onerror = reject;
                img.src = e.target?.result as string;
            };

            reader.onerror = reject;
            reader.readAsDataURL(file);
        });
    }
}