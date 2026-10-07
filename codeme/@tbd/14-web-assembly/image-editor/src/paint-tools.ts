import { getCanvasById } from "@/canvas.ts";
import type { CanvasPair } from "@/canvas.ts";

import styles from "@paint-tools/styles.css?raw";
import template from "@paint-tools/template.html?raw";

import type { Tool } from "@paint-tools/tool.ts";
import * as tools from "@paint-tools/tools.ts";

export class PaintTools extends HTMLElement {
    #canvas!: CanvasPair;
    #activeTool: Tool | null = null;

    static get observedAttributes() {
        return ["canvas"];
    }

    get fgColor(): HTMLInputElement {
        const color= this.shadowRoot?.getElementById("fg-color");

        if (color instanceof HTMLInputElement) {
            return color;
        }

        throw new Error("Unable to parse color");
    }

    get bgColor() {
        const color= this.shadowRoot?.getElementById("bg-color");

        if (color instanceof HTMLInputElement) {
            return color;
        }

        throw new Error("Unable to parse color");
    }

    constructor() {
        super();
        this.attachShadow({mode: "open"});
    }

    connectedCallback() {
        this.#render();
        this.#initCanvas();
        this.#initTools();
    }

    attributeChangedCallback(attrName: string) {
        if (attrName === "canvas") {
            this.#initCanvas();
        }
    }

    useUnknown() {
        throw new Error("Can't use unknown tool");
    }

    useInsertImage() {
        return new tools.InsertImage(this.#canvas);
    }

    useGaussianWasmBlur() {
        return new tools.GaussianBlur(this.#canvas, {
            sigma: 10,
            mode: "wasm"
        });
    }

    useGaussianWasmBlur2() {
        return new tools.GaussianBlur(this.#canvas, {
            sigma: 10,
            mode: "wasm2"
        });
    }

    useGaussianJSBlur() {
        return new tools.GaussianBlur(this.#canvas, {
            sigma: 10,
            mode: "js"
        });
    }

    useRectSelect() {
        const tool = new tools.RectSelect(this.#canvas);

        this.bgColor.addEventListener("change", () => {
            this.#activeTool?.setOptions({strokeColor: this.bgColor.value});
        }, {signal: tool.signal});

        return tool;
    }

    useBrush() {
        const tool = new tools.Brush(this.#canvas, {
            color: this.fgColor.value,
            size: 16
        });

        this.fgColor.addEventListener("change", () => {
            this.#activeTool?.setOptions({color: this.fgColor.value});
        }, {signal: tool.signal});

        return tool;
    }

    usePencil() {
        const tool = new tools.Brush(this.#canvas, {
            color: this.fgColor.value,
            lineCap: "square",
            size: 2
        });

        this.fgColor.addEventListener("change", () => {
            this.#activeTool?.setOptions({color: this.fgColor.value});
        }, {signal: tool.signal});

        return tool;
    }

    useEraser() {
        const tool = new tools.Brush(this.#canvas, {
            mode: "erase",
        });

        return tool;
    }

    useFill() {
        const tool = new tools.Fill(this.#canvas, {
            color: this.bgColor.value
        });

        this.bgColor.addEventListener("change", () => {
            this.#activeTool?.setOptions({strokeColor: this.bgColor.value});
        }, {signal: tool.signal});

        return tool;
    }

    useRectangle() {
        const tool = new tools.Rectangle(this.#canvas, {
            strokeColor: this.fgColor.value,
            fillColor: this.bgColor.value
        });

        this.fgColor.addEventListener("change", () => {
            this.#activeTool?.setOptions({strokeColor: this.fgColor.value});
        }, {signal: tool.signal});

        this.bgColor.addEventListener("change", () => {
            this.#activeTool?.setOptions({fillColor: this.bgColor.value});
        }, {signal: tool.signal});

        return tool;
    }

    useEllipse() {
        const tool = new tools.Ellipse(this.#canvas, {
            strokeColor: this.fgColor.value,
            fillColor: this.bgColor.value
        });

        this.fgColor.addEventListener("change", () => {
            this.#activeTool?.setOptions({strokeColor: this.fgColor.value});
        }, {signal: tool.signal});

        this.bgColor.addEventListener("change", () => {
            this.#activeTool?.setOptions({fillColor: this.bgColor.value});
        }, {signal: tool.signal});

        return tool;
    }

    useLine() {
        const tool = new tools.Line(this.#canvas, {
            strokeColor: this.fgColor.value
        });

        this.fgColor.addEventListener("change", () => {
            this.#activeTool?.setOptions({strokeColor: this.fgColor.value});
        }, {signal: tool.signal});

        return tool;
    }

    #render() {
        if (this.shadowRoot != null) {
            this.shadowRoot.innerHTML = `<style>${styles}</style>${template}`;
        }
    }

    #initCanvas() {
        const canvasId = this.getAttribute("canvas");

        if (canvasId == null) {
            throw new Error("Canvas is id is missing");
        }

        this.#canvas = getCanvasById(canvasId);
    }

    #initTools() {
        this.shadowRoot?.getElementById("insert-file")?.addEventListener("input", (e) => {
            this.#activeTool?.destroy();

            // @ts-expect-error (Нет времени разбираться)
            const file = e.target?.files[0];

            if (file !== null) {
                const image = this.useInsertImage();
                this.#activeTool = image;
                void image.addImage(file);
            }
        });

        this.shadowRoot?.addEventListener("click", (e) => {
            const target = e.target;

            if (!(target instanceof Element)) {
                return;
            }

            const tool = target.closest<HTMLElement>("[data-tool]");

            if (tool == null) {
                return;
            }

            this.#activeTool?.destroy();

            const use = tool.dataset.tool as keyof this;

            if (use in this && typeof this[use] == "function") {
                this.#activeTool = this[use]();

            } else {
                this.#activeTool = null;
                this.useUnknown();
            }
        });
    }
}

customElements.define('paint-tools', PaintTools);
