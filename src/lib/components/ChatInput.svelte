<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';

	// let input: HTMLInputElement;

	const { disabled = false, onsubmit }: { disabled?: boolean; onsubmit: (message: string) => {} } =
		$props();

	let message = $state('');
	// let displayMessage = $derived(parseMessage(message));

	const trimmedMessage = $derived(message.trim());

	// function parseMessage(message: string) {
	// 	const sanitized = message.replace(/[&<>"']/g, (char) => {
	// 		const entities: Record<string, string> = {
	// 			'&': '&amp;',
	// 			'<': '&lt;',
	// 			'>': '&gt;',
	// 			'"': '&quot;',
	// 			"'": '&#39;'
	// 		};
	// 		return entities[char];
	// 	});

	// 	const aliasRegexp = createRegExp(
	// 		maybe(oneOrMore(whitespace)).grouped(),
	// 		exactly('@').and(oneOrMore(not.whitespace)).notAfter(not.whitespace).grouped(),
	// 		maybe(oneOrMore(whitespace)).grouped(),
	// 		['g']
	// 	);

	// 	return sanitized.replace(
	// 		aliasRegexp,
	// 		(_, space, tag, space2) =>
	// 			`${space ? space.replace(' ', '&nbsp') : ''}<span class="text-blue-500 bg-blue-100 rounded">${tag}</span>${space2 ? space2.replace(' ', '&nbsp') : ''}`
	// 	);
	// }

	function handleSubmit(e: SubmitEvent & { currentTarget: EventTarget & HTMLFormElement }) {
		e.preventDefault();
		if (!trimmedMessage) return;
		onsubmit(trimmedMessage);
		message = '';
	}
</script>

<form class="flex gap-2 p-4" onsubmit={handleSubmit}>
	<!-- <div class="relative flex-1"> -->
	<!-- <Input
			type="text"
			bind:value={message}
			class="relative z-10 bg-transparent text-transparent caret-foreground"
		/> -->
	<!-- <div
			class="whitespace-preserve absolute left-0 top-0 z-0 flex h-full w-full items-center px-3 py-2 text-sm"
		>
			{@html displayMessage}
		</div> -->
	<!-- </div> -->
	<Input type="text" bind:value={message} />
	<Button type="submit" disabled={disabled || !trimmedMessage}>Send</Button>
</form>

<!-- <style>
	.whitespace-preserve {
		white-space-collapse: preserve;
	}
</style> -->
