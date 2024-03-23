import UnoCSS from "unocss/vite";

/** @type { import('@storybook/web-components-vite').StorybookConfig } */
const config = {
  stories: [
    "../core/packages/frontend/components/src/**/*.stories.@(js|jsx|mjs|ts|tsx|mdx)",
    "../**/packages/frontend/infrastructure/src/adapters/inbound/**/**/*.stories.@(js|jsx|mjs|ts|tsx|mdx)",
  ],
  addons: [
    "@storybook/addon-links",
    "@storybook/addon-essentials",
    "@chromatic-com/storybook",
  ],
  framework: {
    name: "@storybook/web-components-vite",
    options: {},
  },
  docs: {
    autodocs: "tag",
  },
  viteFinal: (config) => {
    config.plugins?.push(
      UnoCSS({
        mode: "shadow-dom",
      })
    );
    return config;
  },
};
export default config;
