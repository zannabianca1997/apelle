<script lang="ts">
	import type { Snippet } from 'svelte';
	import { _ } from 'svelte-i18n';

	let { children }: { children?: Snippet } = $props();
</script>

<nav>
	{@render children?.()}
</nav>

<style lang="scss">
	nav {
		position: absolute;

		top: 12px;
		left: 55px;

		display: flex;
		gap: 10px;

		border-radius: 4px;

		background: linear-gradient(180deg, #911616 0%, #691010 77.5%);
	}
</style>
