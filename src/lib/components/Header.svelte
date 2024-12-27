<script>
	import { PanelLeft } from 'lucide-svelte';

	import { Button } from './ui/button';
</script>

<div class="flex items-center gap-1 p-4 outline outline-1 outline-card">
	<Button href="/menu" variant="ghost" size="icon">
		<PanelLeft></PanelLeft>
	</Button>
	<h2 class="text-2xl font-bold">altdot.</h2>
</div>
