import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import { fileURLToPath, URL } from "url";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte()],
  resolve: {
    alias: {
      $lib: fileURLToPath(new URL("./src/lib", import.meta.url)),
      $assets: fileURLToPath(new URL("./src/assets", import.meta.url)),
    },
  },
  optimizeDeps: { exclude: ["svelte-navigator"] },
});
