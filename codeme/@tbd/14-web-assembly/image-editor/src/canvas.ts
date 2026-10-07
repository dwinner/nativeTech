import { CanvasHistory } from "@canvas/canvas-history.ts";

export interface PersistentCanvasRenderingContext2D extends CanvasRenderingContext2D {
    saveSnapshot(): ImageData;
}

export type CanvasPair = [HTMLCanvasElement, PersistentCanvasRenderingContext2D];

const cache = new Map<string, CanvasPair>();

const historyLog: CanvasHistory[] = [];

export function getCanvasById(id: string): CanvasPair {
    if (!cache.has(id)) {
        const canvas = document.getElementById(id);

        if (canvas == null || !(canvas instanceof HTMLCanvasElement)) {
            throw new Error("Can't find canvas");
        }

        const ctx = canvas.getContext("2d", {willReadFrequently: true});

        if (ctx == null) {
            throw new Error("Can't resolve canvas context");
        }

        const history = new CanvasHistory(canvas);

        Object.defineProperty(ctx, "saveSnapshot", {
            configurable: true,
            writable: true,
            value: () => {
                historyLog.push(history);
                return history.saveSnapshot();
            }
        });

        cache.set(id, [canvas, ctx as PersistentCanvasRenderingContext2D]);
    }

    return cache.get(id)!;
}

document.addEventListener("keydown", (e) => {
    const isUndo = (e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'z' && !e.shiftKey;

    const isRedo =
        ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'y') ||
        ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'z' && e.shiftKey);

    if (isUndo) {
        e.preventDefault();
        historyLog.at(-1)?.back();

    } else if (isRedo) {
        e.preventDefault();
        historyLog.at(-1)?.forward();
    }
});
