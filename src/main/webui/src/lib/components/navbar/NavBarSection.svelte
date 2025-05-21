<script lang="ts">
	import { onMount, type Snippet } from 'svelte';
	import { navElements, navMenuElements } from './stores.svelte';

	const { order, menu, children }: { order: number; menu?: true; children?: Snippet } = $props();
	const id = $props.id();

	onMount(() => {
		if (!children) {
			return;
		}

		const targetElements = menu ? navMenuElements : navElements;

		if (Object.values(targetElements).some((element) => element.order === order)) {
			throw new Error('Duplicate ordering of navbar element');
		}

		targetElements[id] = { order, content: children };
		return () => {
			delete targetElements[id];
		};
	});
</script>

<!-- No actual content rendered -->
