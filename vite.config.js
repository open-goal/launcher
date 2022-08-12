import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import { fileURLToPath, URL } from "url";

/** @type {import('vite').UserConfig} */
export default defineConfig({
	plugins: [sveltekit()],
	resolve: {
		alias: {
			$assets: fileURLToPath(new URL("./src/assets", import.meta.url)),
		},
	},
});
