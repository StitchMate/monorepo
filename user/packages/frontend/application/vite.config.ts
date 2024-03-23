import { resolve } from "path";
import { defineConfig } from "vite";
import dts from "vite-plugin-dts";

export default defineConfig({
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
        if (format === "es") return `user-application.mjs`;
        if (format === "cjs") return `user-application.cjs`;
        return `user-application.${format}.js`;
      },
    },
  },
  plugins: [dts({ rollupTypes: true })],
});
