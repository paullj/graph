<script lang="ts">
	import type { EventHandler } from 'svelte/elements';
	import { preventDefault, debounce } from '$lib/utils';

	let contents = $state('graph down\n\ta --> b');
	let result = $state('');

	$effect(() => {
		post_graph(contents);
	});

	const post_graph = debounce(async (contents: string) => {
		const res = await fetch('http://localhost:8000/graph', {
			method: 'POST',
			body: contents
		});
		if (res.status !== 200) {
			result = `Error: ${res.status}`;
			return;
		}
		result = await res.text();
	}, 300);

	// TODO: Progressively enahnce form handling
	const submit: EventHandler<SubmitEvent> = async (e) => {
		if (e.target == null) return;
		const data = new FormData(e.target as HTMLFormElement);
		const text = data.get('text');
		post_graph(text as string);
	};
</script>

<div class="w-screen h-screen flex">
	<div class="bg-red w-1/2">
		<form class="w-full h-full" action="submit" method="post" onsubmit={preventDefault(submit)}>
			<textarea
				id="graph-content"
				class="w-full h-full font-mono whitespace-pre"
				bind:value={contents}
			></textarea>
			<!-- TODO: Hide when JS is enabled -->
			<button type="submit"> Submit </button>
		</form>
	</div>
	<div class="w-1/2">
		{@html result}
	</div>
</div>

<!-- svelte-ignore css_unused_selector -->
<style>
	svg {
		width: 100%;
		height: 100%;
	}
</style>
