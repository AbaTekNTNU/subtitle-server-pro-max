// @ts-check
import node from "@astrojs/node";
import svelte from "@astrojs/svelte";
import tailwind from "@astrojs/tailwind";
import { defineConfig } from "astro/config";

// https://astro.build/config
export default defineConfig({
  output: "server",
  prefetch: {
    prefetchAll: true,
  },
  adapter: node({
    mode: "standalone",
  }),
  integrations: [tailwind(), svelte()],
});
