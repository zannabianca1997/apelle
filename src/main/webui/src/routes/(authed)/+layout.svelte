<script lang="ts">
	import NavBar from '$lib/components/navbar/NavBar.svelte';
	import type { Snapshot } from './$types';
	import type { Snippet } from 'svelte';

	let { children }: { children: Snippet } = $props();

	let collapsed = $state(true);
	export const snapshot: Snapshot<{ collapsed: boolean }> = {
		capture: () => {
			return {
				collapsed
			};
		},
		restore: (value) => (collapsed = value.collapsed)
	};
</script>

<NavBar bind:collapsed />
{@render children()}
