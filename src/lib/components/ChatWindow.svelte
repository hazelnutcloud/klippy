<script lang="ts">
	import { ScrollArea } from '$lib/components/ui/scroll-area';
	import { Separator } from '$lib/components/ui/separator';
	import ChatMessage from './ChatMessage.svelte';
	import ChatInput from './ChatInput.svelte';
	import { chatStore } from '$lib/stores/chat.svelte';
	import { fade } from 'svelte/transition';
	import { flip } from 'svelte/animate';

	async function handleMessage(message: string) {
		const userMessage = {
			id: crypto.randomUUID(),
			content: message,
			role: 'user' as const,
			timestamp: new Date()
		};

		chatStore.addMessage(userMessage);
	}
</script>

<Separator />

<ScrollArea class="flex-1">
	{#if chatStore.messages.length === 0}
		<div class="p-8 text-center text-muted-foreground">
			Start a new conversation by sending a message.
		</div>
	{/if}

	{#each chatStore.messages as message (message.id)}
		<ChatMessage {message} />
	{/each}
</ScrollArea>

<ChatInput onsubmit={handleMessage} />
