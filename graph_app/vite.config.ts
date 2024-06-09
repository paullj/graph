import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';
import unocss from 'unocss/vite'
import extractorSvelte from '@unocss/extractor-svelte'


export default defineConfig({
	plugins: [
		unocss({
			extractors: [
				extractorSvelte()
			]
		}),
		sveltekit(),
	],
	server: {
		proxy: {
			'/api': {
				target: 'http://localhost:8000/',
				changeOrigin: true,
			}
		}
	},
	test: {
		include: ['src/**/*.{test,spec}.{js,ts}']
	}
});
