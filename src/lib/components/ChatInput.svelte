<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Command from '$lib/components/ui/command';

	let inputDiv: HTMLDivElement;
	let popupInput: HTMLInputElement | null = $state(null);
	$effect(() => {
		console.log(popupInput);
	});

	const { disabled = false, onsubmit }: { disabled?: boolean; onsubmit: (message: string) => {} } =
		$props();

	let textinput = $state('');

	let popupActive = $state(false);
	let popupPosition = $state({ top: 0, left: 0 });

	function handleKeydown(e: KeyboardEvent & { currentTarget: EventTarget & HTMLDivElement }) {
		if (e.key === '@') {
			const selection = window.getSelection();
			const range = selection?.getRangeAt(0);

			if (range) {
				const startOffset = range.startOffset;
				const textContent = e.currentTarget.textContent || '';

				// Check if we're at the start or if previous char is whitespace
				const isAtStart = startOffset === 0;
				const prevChar = startOffset > 0 ? textContent[startOffset - 1] : '';
				const nextChar = startOffset > 0 ? (textContent[startOffset] ?? ' ') : '';
				const isPrevCharWhitespace = /\s/.test(prevChar);
				const isNextCharWhitespace = /\s/.test(nextChar);

				if (isAtStart || (isPrevCharWhitespace && isNextCharWhitespace)) {
					// Get caret position
					const rect = range.getBoundingClientRect();

          console.log(rect)

					popupPosition = {
						top: rect.top - 160, // 160 = popup height (40) * 4
						left: rect.left
					};

					popupActive = true;
					setTimeout(() => {
						popupInput?.focus();
					}, 0);
					return;
				}
			}
		}
	}

	function handleInput() {
		textinput = inputDiv.textContent ?? '';
	}
</script>

<div class="relative flex gap-2 p-4">
	{#if popupActive}
		<div class="fixed h-40" style="top: {popupPosition.top}px; left: {popupPosition.left}px">
			<Command.Root class="outline outline-1 outline-muted">
				<Command.Input placeholder="Add some context..." bind:ref={popupInput}></Command.Input>
				<Command.List>
					<Command.Empty>Nothing found...</Command.Empty>
					<Command.Group heading="suggestions">
						<Command.Item
							onclick={() => {
								popupActive = false;
								inputDiv.focus();
							}}
						>
							Image
						</Command.Item>
					</Command.Group>
				</Command.List>
			</Command.Root>
		</div>
	{/if}
	<div
		bind:this={inputDiv}
		class="flex w-full items-center rounded p-2 outline outline-1 outline-muted focus:outline-muted-foreground"
		contenteditable="true"
		oninput={handleInput}
		onkeydown={handleKeydown}
		role="textbox"
		tabindex="0"
	></div>
	<Button type="submit" {disabled}>Send</Button>
</div>
