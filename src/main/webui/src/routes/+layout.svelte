<script>
	import { isLoading } from 'svelte-i18n';

	let { children } = $props();
	import '../app.css';
</script>

{#if !$isLoading}
	{@render children()}
{/if}
