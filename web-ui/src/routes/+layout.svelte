<script lang="ts">
	import '../app.css';

	import { isLoading } from 'svelte-i18n';
	import type { Snippet } from 'svelte';

	let { children }: { children: Snippet } = $props();
</script>

{#if !$isLoading}
	{@render children()}
{/if}
