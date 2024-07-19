import { resolve } from "path";
import UnoCSS from "unocss/vite";
import { defineConfig } from "vite";
import dts from "vite-plugin-dts";

export default defineConfig({
  plugins: [
    UnoCSS({
      mode: "shadow-dom",
    }),
    dts({ outDir: "./dist/typings" }),
  ],
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
        if (format === "es") return `user-infrastructure.mjs`;
        if (format === "cjs") return `user-infrastructure.cjs`;
        return `user-infrastructure.${format}.js`;
      },
    },
  },
});
