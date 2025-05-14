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
		position: fixed;
		top: 0;
		width: 100%;

		padding-top: 12px;
		padding-right: 55px;
		padding-bottom: 12px;
		padding-left: 55px;

		border-bottom: 1px solid #e182824f;

		background-color: #0a0a0a;

		display: flex;
		justify-content: space-between;
	}
</style>
