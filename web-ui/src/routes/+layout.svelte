<script lang="ts">
	import '../app.css';

	import { isLoading } from 'svelte-i18n';
	import type { Snippet } from 'svelte';
	import Snackbar from '$lib/components/Snackbar.svelte';
	import { global } from '$lib/errors.svelte';

	let { children }: { children: Snippet } = $props();
</script>

{#if !$isLoading}
	{@render children()}
{/if}

<Snackbar bind:this={global.snackbar} />
