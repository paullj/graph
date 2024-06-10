<script lang="ts">
	import Editor from '$lib/components/editor.svelte';
	import Viewer from '$lib/components/viewer.svelte';

	let contents = $state('graph down\n  a[Node A] --> b[Node B]\n  c[Node C] --> b');
</script>

<div class="w-full h-full flex space-x-4">
	<div class="w-1/2 py-4 pl-4">
		<div class="w-full h-full">
			<Editor bind:contents></Editor>
		</div>
	</div>
	<div class="w-1/2 py-4 pr-4">
		<Viewer bind:contents></Viewer>
	</div>
</div>
