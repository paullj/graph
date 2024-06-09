<script lang="ts">
	import { getHighlighter, type Highlighter } from 'shiki';
	import graph from '$lib/graphLanguage.json';
	import { untrack } from 'svelte';

	import type { FormEventHandler, KeyboardEventHandler, UIEventHandler } from 'svelte/elements';

	interface CodeEditorProps {
		contents: string;
	}

	let { contents = $bindable() }: CodeEditorProps = $props();
	let highlighted = $state('');
	let highlightedElement: HTMLElement;

	let highlighter: Highlighter;

	$effect(() => {
		(async () => {
			highlighter = await getHighlighter({
				langs: [],
				themes: ['vitesse-light']
			});
			await highlighter.loadLanguage(graph);
			untrack(() => {
				updateCode(contents);
			});
		})();
	});

	const updateCode = (code: string) => {
		if (!highlighter) return;
		contents = code;

		if (code[code.length - 1] == '\n') code += ' ';

		highlighted = highlighter.codeToHtml(code, {
			lang: 'graph',
			theme: 'vitesse-light'
		});
	};

	const updateScroll = (target: HTMLTextAreaElement) => {
		highlightedElement.scrollTop = target.scrollTop;
		highlightedElement.scrollLeft = target.scrollLeft;
	};

	const handleOnInput: FormEventHandler<EventTarget> = (event) => {
		updateCode((event.target as HTMLTextAreaElement).value);
		updateScroll(event.target as HTMLTextAreaElement);
	};
	const handleScroll: UIEventHandler<HTMLTextAreaElement> = (event) => {
		updateScroll(event.target as HTMLTextAreaElement);
	};
	const handleTab: KeyboardEventHandler<EventTarget> = (event) => {
		let target = event.target as HTMLTextAreaElement;
		if (event.key.toLowerCase() == 'tab') {
			event.preventDefault();
			let start = target.selectionStart;
			let end = target.selectionEnd;
			let selectedText = target.value.slice(start, end);
			let replacementText: string;
			const matches = selectedText.match(new RegExp('\n', 'g'));

			target.selectionEnd = start - (end - start) + 2;
			if (matches) {
				let lines = selectedText.split('\n');
				let indentedLines = lines.map((line) => '  ' + line);
				replacementText = indentedLines.join('\n');
			} else {
				replacementText = '  ' + selectedText;
			}
			target.value = target.value.slice(0, start) + replacementText + target.value.slice(end);
			target.selectionEnd = matches ? end + 2 * (matches.length + 1) : start - (end - start) + 2;
			target.selectionStart = matches ? start + 2 : target.selectionEnd;
			updateCode(target.value);
		}
	};
</script>

<div class="h-full relative font-mono">
	<div class="h-full overflow-auto pt-4 px-4" bind:this={highlightedElement}>
		{@html highlighted}
	</div>
	<textarea
		id="editing"
		class="h-full border-0 pt-4 px-4 whitespace-pre overflow-auto absolute inset-0 z-10 bg-transparent text-transparent caret-black tab-2 resize-none"
		value={contents}
		oninput={handleOnInput}
		onscroll={handleScroll}
		onkeydown={handleTab}
		spellcheck="false"
		autocapitalize="off"
		autocorrect="off"
		autocomplete="off"
	></textarea>
</div>
