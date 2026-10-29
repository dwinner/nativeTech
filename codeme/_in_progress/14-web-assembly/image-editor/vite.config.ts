import { defineConfig } from "vite";

import wasm from "vite-plugin-wasm";
import tsconfigPath from "vite-tsconfig-paths";

export default defineConfig({
    plugins: [
        wasm(),
        tsconfigPath()
    ]
});