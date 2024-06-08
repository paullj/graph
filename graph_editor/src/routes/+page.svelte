<script lang="ts">
	let contents = $state('graph down\n\ta --> b');
	let result = $state('');

	$effect(() => {
		post_graph(contents);
	});

	const post_graph = async (contents: string) => {
		const res = await fetch('http://localhost:8000/graph', {
			method: 'POST',
			body: contents
		});
		if (res.status !== 200) {
			result = `Error: ${res.status}`;
			return;
		}
		result = await res.text();
	};

	const submit = async (e: SubmitEvent) => {
		e.preventDefault();
		const data = new FormData(e.target);
		const text = data.get('text');
		post_graph(text as string);
	};
</script>

<div class="w-screen h-screen flex">
	<div class="bg-red w-1/2">
		<form action="submit" method="post" onsubmit={submit}>
			<textarea id="graph-content" class="font-mono whitespace-pre" bind:value={contents}
			></textarea>
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
