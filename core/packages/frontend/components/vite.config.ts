import { resolve } from "path";
import UnoCSS from "unocss/vite";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [UnoCSS({
    mode: 'shadow-dom'
  })],
  publicDir: false,
  resolve: {
    alias: {
      "@": resolve(__dirname, "./src"),
    },
  },
  build: {
    lib: {
      entry: resolve(__dirname, "src/index.ts"),
      formats: ["es", "cjs"],
      fileName: (format) => {
        if (format === "es") return `core-components.mjs`;
        if (format === "cjs") return `core-components.cjs`;
        return `core-components.${format}.js`;
      },
    },
  },
});
