<script>
	import Header from './Header.svelte';
	import { Card } from './ui/card';

	const { children } = $props();
</script>

<Card class="flex h-svh flex-col">
	<Header></Header>
	{@render children()}
</Card>
