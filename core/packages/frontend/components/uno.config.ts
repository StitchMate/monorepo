import { defineConfig, presetAttributify } from "unocss";
import presetWind from '@unocss/preset-wind'


export default defineConfig({
    presets: [
        presetAttributify(),
        presetWind()
    ]
})