<script lang="ts">
	import { debounce } from '$lib/utils';
	import { panZoom } from '$lib/panZoom';
	import { fade } from 'svelte/transition';

	interface GraphViewerProps {
		contents: string;
	}
	let { contents = $bindable() }: GraphViewerProps = $props();

	let graph = $state('');

	let fetching = $state(false);
	let panX = $state(0);
	let panY = $state(0);
	let scale = $state(1);

	$effect(() => {
		fetch_graph(contents);
	});

	const fetch_graph = debounce(async (contents: string) => {
		const delay = 200;
		const timeoutId = setTimeout(() => {
			fetching = true;
		}, delay);
		const res = await fetch(`${import.meta.env.VITE_API_ENDPOINT}/api/graph`, {
			method: 'POST',
			body: contents.trim(),
			headers: {
				'Content-Type': 'text/plain',
				Accept: 'application/svg+xml'
			}
		});
		clearTimeout(timeoutId);

		if (res.status === 200) {
			graph = await res.text();
		}
		fetching = false;
	}, 100);
</script>

<div class="relative rounded-lg border-2 w-full h-full overflow-hidden">
	<div class="absolute right-2 bottom-2 z-10 space-y-2 flex flex-col justify-end items-center">
		{#if fetching}
			<span transition:fade={{ duration: 100 }}>
				<div class="i-teenyicons:loader-outline w-4 h-4 animate-spin"></div>
			</span>
		{/if}
		<button
			onclick={() => {
				scale = 1;
				panX = 0;
				panY = 0;
			}}
		>
			<div class="i-teenyicons:home-outline w-4 h-4"></div>
		</button>
		<input
			type="range"
			name=""
			id=""
			min="0.125"
			max="4"
			step="0.1"
			bind:value={scale}
			class="bg-gray-200 appearance-none rounded-full"
			style="writing-mode: vertical-lr; direction: rtl; vertical-align: middle;"
		/>
	</div>
	<div
		use:panZoom={{ panX, panY, scale }}
		onzoom={(e) => (scale = e.detail)}
		onpanX={(e) => (panX = e.detail)}
		onpanY={(e) => (panY = e.detail)}
	>
		{@html graph}
	</div>
</div>
