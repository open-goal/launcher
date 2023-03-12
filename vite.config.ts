import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import { fileURLToPath, URL } from "url";

// https://vitejs.dev/config/
export default defineConfig({
  server: {
    port: 3000, // The port the server will listen on.
  },
  plugins: [svelte()],
  resolve: {
    alias: {
      $lib: fileURLToPath(new URL("./src/lib", import.meta.url)),
      $assets: fileURLToPath(new URL("./src/assets", import.meta.url)),
    },
  },
  optimizeDeps: { exclude: ["svelte-navigator"] },
  build: {
    rollupOptions: {
      input: {
        main: fileURLToPath(new URL("./index.html", import.meta.url)),
        splash: fileURLToPath(
          new URL("./src/splash/index.html", import.meta.url)
        ),
      },
    },
  },
});
