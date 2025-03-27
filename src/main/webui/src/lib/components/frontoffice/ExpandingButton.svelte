<script lang="ts">
	import type { Snippet } from 'svelte';

	const assignedId = $props.id();
	let {
		id = assignedId,
		text,
		children,
		expanded = $bindable(null)
	}: { id?: string; text: string; children: Snippet; expanded?: string | null } = $props();
	const thisExpanded = $derived(expanded == id);

	function onclick() {
		expanded = id;
	}
</script>

<div class={{ expanded: thisExpanded }}>
	<button {onclick}>{text}</button>
	{#if thisExpanded}
		{@render children?.()}
	{/if}
</div>

<style lang="scss">
	div {
		transition: 0.5s;

		&.expanded {
			border: 2px solid var(--theme-color);
			border-radius: 6px;
			padding: 6px;
			flex-grow: 1;
		}

		button {
			width: 100%;
			height: 48px;

			border: 0px;
			border-radius: 4px;

			padding-top: 6px;
			padding-right: 12px;
			padding-bottom: 6px;
			padding-left: 12px;

			font-weight: 900;
			font-size: 16px;
			line-height: 100%;
			letter-spacing: 0%;

			background-color: var(--theme-color);
			color: white;

			cursor: pointer;
		}
	}
</style>
