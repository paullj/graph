<script lang="ts">
	import 'virtual:windi.css';

	import { browser } from '$app/environment';
	if (browser) import('virtual:windi-devtools');

	let { children } = $props();
</script>

{@render children()}
