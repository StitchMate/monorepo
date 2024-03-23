import { defineConfig, presetAttributify } from "unocss";
import presetWind from "@unocss/preset-wind";

export default defineConfig({
  presets: [presetAttributify(), presetWind()],
  content: {
    pipeline: {
      include: [
        /\.(vue|svelte|[jt]sx|mdx?|astro|elm|php|phtml|html)($|\?)/,
        "../core/packages/frontend/components/src/**/*.stories.@(js|jsx|mjs|ts|tsx|mdx)",
        "../**/packages/frontend/frontend/infrastructure/src/inbound/**/**/*.stories.@(js|jsx|mjs|ts|tsx|mdx)",
      ],
    },
  },
});
