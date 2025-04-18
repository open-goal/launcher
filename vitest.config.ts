import { defineConfig } from "vitest/config";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import { svelteTesting } from "@testing-library/svelte/vite";
import { fileURLToPath, URL } from "url";

export default defineConfig({
  plugins: [
    svelte({
      compilerOptions: {
        hmr: !process.env.VITEST,
      },
    }),
    svelteTesting(),
  ],
  resolve: {
    alias: {
      $lib: fileURLToPath(new URL("./src/lib", import.meta.url)),
      $assets: fileURLToPath(new URL("./src/assets", import.meta.url)),
    },
  },
  test: {
    alias: [{ find: /^svelte$/, replacement: "svelte/internal" }],
    include: ["./src/**/*.test.ts"],
    globals: true,
    environment: "jsdom",
    setupFiles: ["./src/tests/setup.ts"],
  },
});
