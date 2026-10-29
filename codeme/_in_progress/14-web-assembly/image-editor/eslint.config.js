import js from "@eslint/js";
import globals from "globals";
import tseslint from "typescript-eslint";
import { defineConfig, globalIgnores } from "eslint/config";

export default defineConfig([
    globalIgnores(["./wasm-lib/pkg/", "./dist"]),

    {
        files: ["**/*.{js,mjs,cjs,ts,mts,cts}"],
        plugins: {js},
        extends: ["js/recommended"],
        languageOptions: {globals: globals.browser},
        rules: {
            "semi": ["error", "always"],
            "no-extra-semi": "error"
        }
    },

    tseslint.configs.recommended
]);
