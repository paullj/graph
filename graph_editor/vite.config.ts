import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';
import windicss from 'vite-plugin-windicss'

export default defineConfig({
	plugins: [
		sveltekit(),
		windicss(),
	],
	test: {
		include: ['src/**/*.{test,spec}.{js,ts}']
	}
});