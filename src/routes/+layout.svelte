<script lang="ts">
	import '../app.css';
	import Container from '$lib/components/Container.svelte';
	let { children } = $props();
</script>

<Container>
	{@render children()}
</Container>
