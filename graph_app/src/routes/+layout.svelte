<script lang="ts">
	import 'virtual:uno.css';
	import '@unocss/reset/tailwind.css';

	import { browser } from '$app/environment';
	if (browser) import('virtual:unocss-devtools');

	let { children } = $props();
</script>

<div class="w-screen h-screen">
	{@render children()}
</div>
