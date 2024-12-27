<script lang="ts">
	import type { Message } from '$lib/types';
	import { User } from 'lucide-svelte';
	import { unified } from 'unified';
	import remarkParse from 'remark-parse';
	import remarkRehype from 'remark-rehype';
	import rehypeStringify from 'rehype-stringify';
	import { fade } from 'svelte/transition';

	const { message }: { message: Message } = $props();

	const markdownProcessor = unified().use(remarkParse).use(remarkRehype).use(rehypeStringify);

	const isUser = $derived(message.role === 'user');
</script>

<div class="px-4 py-2" in:fade={{ duration: 200 }}>
	<div
		class={[
			'flex items-center gap-2 rounded-lg p-4',
			isUser && 'ml-4 w-fit bg-secondary/50',
			!isUser && 'flex-1 outline outline-1 outline-muted'
		]}
	>
		{#if isUser}
			<User size={20}></User>
		{/if}
		<div class={'mt-1 text-sm'}>
			{#if isUser}
				{message.content}
			{:else}
				<div
					class="prose prose-zinc dark:prose-invert prose-sm prose-code:font-sans prose-code:text-sm contents"
				>
					{#await markdownProcessor.process(message.content) then markup}
						{@html markup}
					{/await}
				</div>
			{/if}
		</div>
	</div>
</div>
